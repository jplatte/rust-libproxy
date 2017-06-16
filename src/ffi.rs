use std::borrow::Borrow;
use std::ffi::CStr;
use std::ops::Deref;

use libc::{c_char, c_void, free};

/// Like `CString`, but with the underlying memory managed by libc's malloc and free.
///
/// Useful for when you receive a string from C code that you have to free yourself, or for when a
/// C API that you are calling expects to take ownership of a string provided by you. (the latter
/// is not implemented yet though)
pub struct MallocCString(*mut c_char);

impl MallocCString {
    /// Takes ownership of a string allocated by `malloc`.
    pub unsafe fn from_raw(ptr: *mut c_char) -> MallocCString {
        MallocCString(ptr)
    }

    /// Transfers ownership of the string back to a C function.
    ///
    /// The new owner will be responsible of `free`ing the string.
    pub fn into_raw(self) -> *mut c_char {
        self.0
    }

    /// Extracts a `CStr` slice containing the entire string.
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }

    // TODO: impl Debug for MalloCString
    // TODO if this leaves libproxy to become more general-purpose:
    //   * somehow make sure that the representation of MallocCString doesn't change,
    //     OR rewrite Proxies::new so it doesn't cast `*mut *mut c_char` to `*mut MallocCString`
    //   * add new (like CString::new)
    //   * add from_slice_unchecked (like CString::from_vec_unchecked)
    //   * add into_* and as_* (like CString, with the exception of into_boxed_c_str)
}

impl AsRef<CStr> for MallocCString {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Borrow<CStr> for MallocCString {
    fn borrow(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Deref for MallocCString {
    type Target = CStr;

    fn deref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Drop for MallocCString {
    fn drop(&mut self) {
        unsafe { free(self.0 as *mut c_void) }
    }
}
