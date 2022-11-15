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

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), forbid(unsafe_code))]

#[cfg(feature = "std")]
mod hs;
#[cfg(feature = "std")]
mod str;
#[cfg(feature = "std")]
mod vec;
#[cfg(feature = "std")]
pub use self::{hs::*, str::*, vec::*};

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
            // `*const T` is C-FFI safe if `T` is C-FFI safe
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

/// This is used by Rust function that doesn’t return any value
/// (`void` C equivalent).
impl ReprC<()> for () {
    #[inline]
    fn from(_: ()) -> Self {}
}
