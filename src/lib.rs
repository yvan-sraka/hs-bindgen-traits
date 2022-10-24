//! # `hs-bingen-traits`
//!
//! Utility traits behind [`hs-bindgen`](https://github.com/yvan-sraka/hs-bindgen)
//! ergonomics. It helps user to easily define wrapper function to derive a Rust
//! type from and into a C-FFI safe target type (that match the memory layout of
//! an Haskell type).

use displaydoc::Display;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use thiserror::Error;

/// Enumeration of all Haskell F-FFI safe types as the string representation of
/// their token in Haskell.
pub enum HsType {
    CString,
    Empty,
    IO(Box<HsType>),
}

impl std::fmt::Display for HsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HsType::CString => "CString".to_string(),
                HsType::Empty => "()".to_string(),
                HsType::IO(x) => format!("IO {}", x),
            }
        )
    }
}

#[derive(Display, Error, Debug)]
pub enum Error {
    /** type `{0}` isn't in the list of supported Haskell types.
     * Consider opening an issue https://github.com/yvan-sraka/hs-bindgen-traits
     */
    UnsupportedHsType(String),
}

impl FromStr for HsType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if &s[..2] == "IO" {
            Ok(HsType::IO(Box::new(s[2..].parse()?)))
        } else {
            match s.trim() {
                "CString" => Ok(HsType::CString),
                "()" => Ok(HsType::Empty),
                ty => Err(Error::UnsupportedHsType(ty.to_string())),
            }
        }
    }
}

impl HsType {
    /// Get the C-FFI Rust type that match the memory layout of a given HsType.
    ///
    /// This function return a `OUTPUT: proc_macro2::TokenStream` that should
    /// be valid (considered as FFI-safe by `rustc`) in the context of a block
    /// of form: `quote! { extern C fn _(_: #OUTPUT) {} }`
    pub fn quote(&self) -> TokenStream {
        match self {
            HsType::CString => quote! { *const std::os::raw::c_char },
            HsType::Empty => quote! { () },
            HsType::IO(x) => x.quote(),
        }
    }
}

/// Turn a given Rust type into his `HsType` target.
///
/// FIXME: derive this trait for most of `std` types!
pub trait ReprHs {
    fn into() -> HsType;
}

impl ReprHs for String {
    fn into() -> HsType {
        HsType::CString
    }
}

impl ReprHs for &str {
    fn into() -> HsType {
        HsType::CString
    }
}

impl ReprHs for () {
    fn into() -> HsType {
        HsType::Empty
    }
}

/// Generate C-FFI safe wrapper from a given Rust type.
pub trait ReprC<T> {
    fn from(_: T) -> Self;
}

impl ReprC<*const std::os::raw::c_char> for &str {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(x: *const std::os::raw::c_char) -> Self {
        unsafe { std::ffi::CStr::from_ptr(x) }.to_str().unwrap()
    }
}

impl ReprC<*const std::os::raw::c_char> for String {
    fn from(x: *const std::os::raw::c_char) -> Self {
        let r: &str = ReprC::from(x);
        r.to_string()
    }
}

/// Generate Rust safe wrapper from a given C-FFI type.
pub trait ReprRust<T> {
    fn from(_: T) -> Self;
}

impl ReprRust<()> for () {
    fn from(_: ()) -> Self {
        ()
    }
}
