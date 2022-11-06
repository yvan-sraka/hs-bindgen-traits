use crate::{private, HsType, ReprC, ReprHs, ReprRust};

// FIXME: study what could be a good `Vec<T>`/`&[T]` traits ergonomics ...
// n.b. the concept of `slice` have no C equivalent ...
// https://users.rust-lang.org/t/55118

impl<T, const N: usize> ReprRust<*const T> for &[T; N]
where
    *const T: private::CFFISafe,
{
    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(ptr: *const T) -> Self {
        unsafe {
            let x = std::slice::from_raw_parts(ptr, N);
            std::mem::transmute_copy::<&[T], &[T; N]>(&x)
        }
    }
}

impl<T> ReprC<Vec<T>> for *const T
where
    *const T: private::CFFISafe,
{
    #[inline]
    fn from(v: Vec<T>) -> Self {
        let x = v.as_ptr();
        // since the value is passed to Haskell runtime we want Rust to never
        // drop it!
        std::mem::forget(v);
        // FIXME: I should double-check that this does not leak memory and
        // that the value is well handled by GHC tracing Garbage Collector
        x
        // if not, we should export a utility function to let user drop
        // the value, this technique was suggested e.g. here:
        // https://stackoverflow.com/questions/39224904
    }
}

impl<T> ReprHs for Vec<T>
where
    T: ReprHs,
{
    fn into() -> HsType {
        HsType::Ptr(Box::new(T::into()))
    }
}

impl<T, const N: usize> ReprHs for &[T; N]
where
    T: ReprHs,
{
    fn into() -> HsType {
        HsType::Ptr(Box::new(T::into()))
    }
}

#[test]
fn _1() {
    let x = &[1, 2, 3]; // FIXME: use Arbitrary crate
    let y: &[i32; 3] = ReprRust::from(ReprC::from(x.to_vec()));
    assert!(x == y);
}
