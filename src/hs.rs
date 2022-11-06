use displaydoc::Display;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use thiserror::Error;

/// Enumeration of all Haskell C-FFI safe types as the string representation of
/// their token in Haskell.
///
/// FIXME: `Errno(c_int)` should be implemented as a Rust `enum` ...
/// https://hackage.haskell.org/package/base/docs/Foreign-C-Error.html
/// ... using `#[repr(i32)]` https://doc.rust-lang.org/nomicon/other-reprs.html
pub enum HsType {
    /// `Int32`
    CInt,
    /// `Int8`
    CChar,
    /// `Int8`
    CSChar,
    /// `Word8`
    CUChar,
    /// `Int16`
    CShort,
    /// `Word16`
    CUShort,
    /// `Word32`
    CUInt,
    /// `Int64`
    CLong,
    /// `Word64`
    CULong,
    /// `Int64`
    CLLong,
    /// `Word64`
    CULLong,
    /// `Word8`
    CBool,
    /// `Ptr CChar`
    CString,
    /// `Double`
    CDouble,
    /// `Float`
    CFloat,
    /// `()`
    Empty,
    /// `Ptr T`
    Ptr(Box<HsType>),
    /// `IO T`
    IO(Box<HsType>),
}

impl std::fmt::Display for HsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HsType::CBool => "CBool".to_string(),
                HsType::CChar => "CChar".to_string(),
                HsType::CDouble => "CDouble".to_string(),
                HsType::CFloat => "CFloat".to_string(),
                HsType::CInt => "CInt".to_string(),
                HsType::CLLong => "CLLong".to_string(),
                HsType::CLong => "CLong".to_string(),
                HsType::CSChar => "CSChar".to_string(),
                HsType::CShort => "CShort".to_string(),
                HsType::CString => "CString".to_string(),
                HsType::CUChar => "CUChar".to_string(),
                HsType::CUInt => "CUInt".to_string(),
                HsType::CULLong => "CULLong".to_string(),
                HsType::CULong => "CULong".to_string(),
                HsType::CUShort => "CUShort".to_string(),
                HsType::Empty => "()".to_string(),
                HsType::Ptr(x) => format!("Ptr ({})", x),
                HsType::IO(x) => format!("IO ({})", x),
            }
        )
    }
}

#[derive(Display, Error, Debug)]
pub enum Error {
    /** type `{0}` isn't in the list of supported Haskell C-FFI types.
     * Consider opening an issue https://github.com/yvan-sraka/hs-bindgen-traits
     *
     * The list of available Haskell C-FFI types could be found here:
     * https://hackage.haskell.org/package/base/docs/Foreign-C.html
     */
    UnsupportedHsType(String),
    /// found an open `(` without the matching closing `)`
    UnmatchedParenthesis,
}

impl FromStr for HsType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == "()" {
            Ok(HsType::Empty)
        } else if !s.is_empty() && &s[..1] == "(" {
            Ok(s[1..]
                .strip_suffix(')')
                .ok_or(Error::UnmatchedParenthesis)?
                .parse()?)
        } else if s.len() >= 2 && &s[..2] == "IO" {
            Ok(HsType::IO(Box::new(s[2..].parse()?)))
        } else if s.len() >= 3 && &s[..3] == "Ptr" {
            Ok(HsType::Ptr(Box::new(s[3..].parse()?)))
        } else {
            match s {
                "CBool" => Ok(HsType::CBool),
                "CChar" => Ok(HsType::CChar),
                "CDouble" => Ok(HsType::CDouble),
                "CFloat" => Ok(HsType::CFloat),
                "CInt" => Ok(HsType::CInt),
                "CLLong" => Ok(HsType::CLLong),
                "CLong" => Ok(HsType::CLong),
                "CSChar" => Ok(HsType::CSChar),
                "CShort" => Ok(HsType::CShort),
                "CString" => Ok(HsType::CString),
                "CUChar" => Ok(HsType::CUChar),
                "CUInt" => Ok(HsType::CUInt),
                "CULLong" => Ok(HsType::CULLong),
                "CULong" => Ok(HsType::CULong),
                "CUShort" => Ok(HsType::CUShort),
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
    ///
    /// c.f. https://doc.rust-lang.org/core/ffi/
    pub fn quote(&self) -> TokenStream {
        match self {
            // FIXME: add https://doc.rust-lang.org/core/ffi/enum.c_void.html
            HsType::CBool => quote! { bool },
            HsType::CChar => quote! { core::ffi::c_char },
            HsType::CDouble => quote! { core::ffi::c_double },
            HsType::CFloat => quote! { core::ffi::c_float },
            HsType::CInt => quote! { core::ffi::c_int },
            HsType::CLLong => quote! { core::ffi::c_longlong },
            HsType::CLong => quote! { core::ffi::c_long },
            HsType::CSChar => quote! { core::ffi::c_schar },
            HsType::CShort => quote! { core::ffi::c_short },
            HsType::CString => HsType::Ptr(Box::new(HsType::CChar)).quote(),
            HsType::CUChar => quote! { core::ffi::c_uchar },
            HsType::CUInt => quote! { core::ffi::c_uint },
            HsType::CULLong => quote! { core::ffi::c_ulonglong },
            HsType::CULong => quote! { core::ffi::c_ulong },
            HsType::CUShort => quote! { core::ffi::c_ushort },
            HsType::Empty => quote! { () },
            HsType::Ptr(x) => {
                let ty = x.quote();
                quote! { *const #ty }
            }
            HsType::IO(x) => x.quote(),
        }
    }
}
