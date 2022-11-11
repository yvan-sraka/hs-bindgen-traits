//! # `hs-bingen-traits`
//!
//! Utility traits behind [`hs-bindgen`](https://github.com/yvan-sraka/hs-bindgen)
//! ergonomics. It helps user to easily define wrapper function to derive a Rust
//! type from and into a C-FFI safe target type (that match the memory layout of
//! an Haskell type).
//!
//! ## What's this library for?
//!
//! [Does `repr(C)` define a trait I can use to check structs were declared with `#repr(C)`?](https://users.rust-lang.org/t/16323)
//! The answer is sadly no ... that's what this library trying to provide, like
//! what [`safer_ffi`](https://docs.rs/safer-ffi/latest/safer_ffi/layout/trait.ReprC.html)
//! does, but in a simpler and more minimal way, since the goal here is only to
//! target Haskell FFI.
//!
//! ## Acknowledgments
//!
//! ⚠️ This is still a working experiment, not yet production ready.
//!
//! This project was part of a work assignment as an
//! [IOG](https://github.com/input-output-hk) contractor.
//!
//! ## License
//!
//! Licensed under either of [Apache License](LICENSE-APACHE), Version 2.0 or
//! [MIT license](LICENSE-MIT) at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this project by you, as defined in the Apache-2.0 license,
//! shall be dual licensed as above, without any additional terms or conditions.

mod hs;
mod str;
mod vec;

pub use self::{hs::*, str::*, vec::*};
use core::ffi::*;

/// Generate C-FFI cast from a given Rust type.
pub trait ReprC<T>: private::CFFISafe {
    fn from(_: T) -> Self;
}

/// Generate safe Rust wrapper from a given C-FFI type.
pub trait ReprRust<T: private::CFFISafe> {
    fn from(_: T) -> Self;
}

mod private {
    /// The trait `CFFISafe` is sealed and cannot be implemented for types outside this crate.
    /// c.f. https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed
    pub trait CFFISafe {}

    macro_rules! c_ffi_safe {
        ($($ty:ty),*) => {$(
            impl CFFISafe for $ty {}
            impl CFFISafe for *const $ty {}
        )*};
    }

    // C-FFI safe types (the previous macro avoid redundant code)
    c_ffi_safe![(), i8, i16, i32, i64, u8, u16, u32, u64, f32, f64];
}

macro_rules! transparent {
    ($($ty:ty),*) => {$(
        impl ReprRust<$ty> for $ty {
            #[inline]
            fn from(x: $ty) -> Self { x }
        }
        impl ReprC<$ty> for $ty {
            #[inline]
            fn from(x: $ty) -> Self { x }
        }
        impl ReprRust<*const $ty> for *const $ty {
            #[inline]
            fn from(x: *const $ty) -> Self { x }
        }
        impl ReprC<*const $ty> for *const $ty {
            #[inline]
            fn from(x: *const $ty) -> Self { x }
        }
    )*};
}

// C-FFI safe type trivially implement both traits
transparent![i8, i16, i32, i64, u8, u16, u32, u64, f32, f64];

/// Turn a given Rust type into his `HsType` target.
///
/// Deducing what's the right Haskell type target given an arbitrary Rust type
/// is provided by `antlion` feature of `hs-bingen-derive` and rely mostly on
/// Rust type inference through this trait.
pub trait ReprHs {
    fn into() -> HsType;
}

macro_rules! repr_hs {
    ($($ty:ty => $ident:ident,)*) => {$(
        impl ReprHs for $ty {
            fn into() -> HsType {
                HsType::$ident
            }
        }
    )*};
}
pub(crate) use repr_hs;

repr_hs! {
    c_char   => CChar,
    c_double => CDouble,
    c_float  => CFloat,
    c_int    => CInt,
    c_long   => CLong,
    c_short  => CShort,
    c_uchar  => CUChar,
    c_uint   => CUInt,
    c_ulong  => CULong,
    c_ushort => CUShort,
}

impl<T> ReprHs for *const T
where
    T: ReprHs,
{
    fn into() -> HsType {
        HsType::Ptr(Box::new(T::into()))
    }
}

/// This is used by Rust function that doesn’t return any value
/// (`void` C equivalent).
impl ReprC<()> for () {
    fn from(_: ()) -> Self {}
}
repr_hs! { () => Empty, }
