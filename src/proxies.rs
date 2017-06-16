use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

use libc::{c_char, c_void, free};

use error::ProxyResolutionError;
use ffi::MallocCString;

/// A container type for holding the result of `px_proxy_factory_get_proxies`.
pub struct Proxies {
    data: *mut MallocCString,
    len: usize,
}

impl Proxies {
    pub(crate) fn new(data: *mut *mut c_char) -> Result<Proxies, ProxyResolutionError> {
        if data.is_null() {
            Err(ProxyResolutionError)
        } else {
            let mut len = 0;

            unsafe {
                let mut data_ptr = data;
                while !(*data_ptr).is_null() {
                    len += 1;
                    data_ptr = data_ptr.offset(1);
                }
            }

            Ok(Proxies {
                data: data as *mut MallocCString,
                len,
            })
        }
    }

    /// Extracts a slice containing the entire array.
    pub fn as_slice(&self) -> &[MallocCString] {
        unsafe { slice::from_raw_parts(self.data, self.len) }
    }

    /// Extracts a mutable slice of the entire array.
    pub fn as_mut_slice(&mut self) -> &mut [MallocCString] {
        unsafe { slice::from_raw_parts_mut(self.data, self.len) }
    }

    // TODO: impl Index(Mut) for Proxies
    // TODO: impl IntoIterator for Proxies
}

impl AsRef<[MallocCString]> for Proxies {
    fn as_ref(&self) -> &[MallocCString] {
        self.as_slice()
    }
}

impl AsMut<[MallocCString]> for Proxies {
    fn as_mut(&mut self) -> &mut [MallocCString] {
        self.as_mut_slice()
    }
}

impl Borrow<[MallocCString]> for Proxies {
    fn borrow(&self) -> &[MallocCString] {
        self.as_slice()
    }
}

impl BorrowMut<[MallocCString]> for Proxies {
    fn borrow_mut(&mut self) -> &mut [MallocCString] {
        self.as_mut_slice()
    }
}

impl Deref for Proxies {
    type Target = [MallocCString];

    fn deref(&self) -> &[MallocCString] {
        self.as_slice()
    }
}

impl DerefMut for Proxies {
    fn deref_mut(&mut self) -> &mut [MallocCString] {
        self.as_mut_slice()
    }
}

impl Drop for Proxies {
    fn drop(&mut self) {
        unsafe {
            // I don't fully understand it, but this (almost exactly) is what Vec does for dropping
            // its contained elements
            ptr::drop_in_place(self.as_mut_slice());

            free(self.data as *mut c_void)
        }
    }
}

impl<'a> IntoIterator for &'a Proxies {
    type Item = &'a MallocCString;
    type IntoIter = slice::Iter<'a, MallocCString>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Proxies {
    type Item = &'a mut MallocCString;
    type IntoIter = slice::IterMut<'a, MallocCString>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().into_iter()
    }
}
