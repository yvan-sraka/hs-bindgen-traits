//! This module defines convenient traits to let user-defined function take as
//! argument or return type either `CString`, `&CStr`, `String` or `&str`

use crate::{repr_hs, HsType, ReprC, ReprHs, ReprRust};
use std::ffi::{c_char, CStr, CString};

repr_hs! {
    CString => CString,
    &CStr   => CString,
    String  => CString,
    &str    => CString,
}

impl ReprRust<*const c_char> for CString {
    fn from(ptr: *const c_char) -> Self {
        let r: &str = ReprRust::from(ptr);
        CString::new(r).unwrap()
    }
}

impl ReprRust<*const c_char> for &CStr {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(ptr: *const c_char) -> Self {
        unsafe { CStr::from_ptr(ptr) }
    }
}

impl ReprRust<*const c_char> for String {
    fn from(ptr: *const c_char) -> Self {
        let r: &str = ReprRust::from(ptr);
        r.to_string()
    }
}

impl ReprRust<*const c_char> for &str {
    fn from(ptr: *const c_char) -> Self {
        let r: &CStr = ReprRust::from(ptr);
        r.to_str().unwrap()
    }
}

impl ReprC<CString> for *const c_char {
    fn from(s: CString) -> Self {
        s.as_ptr()
    }
}

impl ReprC<&CStr> for *const c_char {
    fn from(s: &CStr) -> Self {
        s.as_ptr()
    }
}

impl ReprC<String> for *const c_char {
    fn from(s: String) -> Self {
        ReprC::from(&*s)
    }
}

impl ReprC<&str> for *const c_char {
    fn from(s: &str) -> Self {
        ReprC::from(CString::new(s).unwrap())
    }
}
