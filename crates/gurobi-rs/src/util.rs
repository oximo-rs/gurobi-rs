use crate::ffi;
use crate::{Error, Result};
use std::{
    ffi::{CStr, CString},
    path::Path,
};

/// Copy a raw C-string into a String
///
/// To quote the Gurobi docs:
/// Note that all interface routines that return string-valued attributes are returning pointers
/// into internal Gurobi data structures. The user should copy the contents of the pointer to a
/// different data structure before the next call to a Gurobi library routine. The user should
/// also be careful to never modify the data pointed to by the returned character pointer.
pub(crate) unsafe fn copy_c_str(s: ffi::c_str) -> String {
    debug_assert!(!s.is_null());
    unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned() // to_string_lossy().into_owned() ALWAYS clones
}

// FIXME: this needs to be re-done, as_mut_ptr should take &mut self,
// but this will likely cause a bunch of breaking changes.
pub(crate) trait AsPtr {
    type Ptr;
    /// Return the underlying Gurobi pointer for [`Model`](crate::Model) and [`Env`](crate::Env) objects
    ///
    /// # Safety
    /// One of the following conditions must hold
    /// - self is mutable
    /// - the resulting pointer is passed only to Gurobi C routines
    unsafe fn as_mut_ptr(&self) -> *mut Self::Ptr;

    #[allow(unused)]
    /// Return the underling Gurobi pointer
    fn as_ptr(&self) -> *const Self::Ptr {
        (unsafe { self.as_mut_ptr() }).cast_const()
    }
}

pub(crate) fn path_to_cstring<P: AsRef<Path>>(p: P) -> Result<CString> {
    let path = p.as_ref().to_string_lossy().into_owned().into_bytes();
    CString::new(path).map_err(Error::NulError)
}

#[test]
fn conversion_must_succeed() {
    use std::ffi::CString;
    let s1 = "mip1.log";
    let cs = CString::new(s1).unwrap();
    let s2 = unsafe { copy_c_str(cs.as_ptr()) };
    assert!(s1 == s2);
}

/// Convert an input count to the range accepted by the C API.
pub(crate) fn checked_count(len: usize) -> crate::Result<crate::ffi::c_int> {
    crate::ffi::c_int::try_from(len)
        .map_err(|_| crate::Error::AlgebraicError("input count exceeds C API range".into()))
}

pub(crate) fn checked_next_count(len: usize) -> crate::Result<()> {
    let next = len
        .checked_add(1)
        .ok_or_else(|| crate::Error::AlgebraicError("input count exceeds C API range".into()))?;
    checked_count(next).map(|_| ())
}

pub(crate) fn collect_indices(
    iter: impl IntoIterator<Item = crate::Result<i32>>,
) -> crate::Result<Vec<i32>> {
    let mut result = Vec::new();
    for index in iter {
        checked_next_count(result.len())?;
        result.push(index?);
    }
    Ok(result)
}

#[cfg(test)]
mod checked_count_tests {
    #[test]
    fn boundaries() {
        assert_eq!(super::checked_count(0).unwrap(), 0);
        assert_eq!(super::checked_count(i32::MAX as usize).unwrap(), i32::MAX);
        assert!(super::checked_count(i32::MAX as usize + 1).is_err());
        assert!(super::checked_next_count(i32::MAX as usize).is_err());
        assert!(super::checked_next_count(i32::MAX as usize - 1).is_ok());
    }
}
