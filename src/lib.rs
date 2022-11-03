//! # `hs-bingen-traits`
//!
//! Utility traits behind [`hs-bindgen`](https://github.com/yvan-sraka/hs-bindgen)
//! ergonomics. It helps user to easily define wrapper function to derive a Rust
//! type from and into a C-FFI safe target type (that match the memory layout of
//! an Haskell type).
//!
//! What's this library for?
//! [Does `repr(C)` define a trait I can use to check structs were declared with `#repr(C)`?](https://users.rust-lang.org/t/16323)
//! The answer is sadly no ... that's what this library trying to provide, like
//! what [`safer_ffi`](https://docs.rs/safer-ffi/latest/safer_ffi/layout/trait.ReprC.html)
//! does, but in a simpler and more minimal way, since the goal here is only to
//! target Haskell FFI.

mod hs;
mod str;

pub use self::{hs::HsType, str::*};
use core::ffi::*;

/// Generate C-FFI cast from a given Rust type.
pub trait ReprC<T>: private::CFFISafe {
    fn from(_: T) -> Self;
}

/// Generate safe Rust wrapper from a given C-FFI type.
pub trait ReprRust<T: private::CFFISafe> {
    fn from(_: T) -> Self;
}

// FIXME: study what could be a good `Vec<T>`/`&[T]` traits ergonomics ...
// n.b. the concept of `slice` have no C equivalent ...
// https://users.rust-lang.org/t/55118
//
// impl ReprRust<(*const u8, usize)> for &[u8] {
//     fn from((data, bytes_length): (*const u8, usize)) -> Self {
//         unsafe { slice::from_raw_parts(data, bytes_length) }
//     }
// }

mod private {
    use core::ffi::*;

    /// The trait `CFFISafe` is sealed and cannot be implemented for types outside this crate.
    /// c.f. https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed
    pub trait CFFISafe {}

    macro_rules! c_ffi_safe {
        ($($ty:ty),*) => {$(
            impl CFFISafe for $ty {}
        )*};
    }

    // C-FFI safe types (the previous macro avoid redundant code)
    c_ffi_safe![
        (),
        *const c_char,
        c_char,
        c_double,
        c_float,
        c_int,
        c_long,
        c_short,
        c_uchar,
        c_uint,
        c_ulong,
        c_ushort
    ];
}

macro_rules! transparent {
    ($($ty:ty),*) => {$(
        impl ReprRust<$ty> for $ty {
            fn from(x: $ty) -> Self { x }
        }
        impl ReprC<$ty> for $ty {
            fn from(x: $ty) -> Self { x }
        }
    )*};
}

// C-FFI safe type trivially implement both traits
transparent![
    (),
    *const c_char,
    c_char,
    c_double,
    c_float,
    c_int,
    c_long,
    c_short,
    c_uchar,
    c_uint,
    c_ulong,
    c_ushort
];

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
