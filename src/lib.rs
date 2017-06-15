extern crate libc;

mod error;
mod proxy_sys;

use std::ffi::{CStr, CString};
use libc::{c_void, free};
use error::Error;

pub struct ProxyFactory(*mut proxy_sys::pxProxyFactory);

impl ProxyFactory {
    pub fn new() -> Option<ProxyFactory> {
        let ptr = unsafe { proxy_sys::px_proxy_factory_new() };
        if ptr.is_null() {
            None
        } else {
            Some(ProxyFactory(ptr))
        }
    }

    pub fn get_proxies(&self, url: &str) -> Result<Vec<String>, Error> {
        // TODO: Implement zero-cost version of this that uses CStr / libc-allocated CString

        let url_c = CString::new(url)?;
        let mut res = Vec::new();

        unsafe {
            let proxies = proxy_sys::px_proxy_factory_get_proxies(self.0, url_c.as_ptr());

            if proxies.is_null() {
                return Err(Error::ProxyResolveError);
            }

            let mut proxy_ptr = proxies;
            while !(*proxy_ptr).is_null() {
                let bytes = CStr::from_ptr(*proxy_ptr).to_bytes().to_owned();
                res.push(String::from_utf8(bytes)?);
                free(*proxy_ptr as *mut c_void);
                proxy_ptr = proxy_ptr.offset(1);
            }

            free(proxies as *mut c_void);
        }

        Ok(res)
    }
}

impl Drop for ProxyFactory {
    fn drop(&mut self) {
        unsafe { proxy_sys::px_proxy_factory_free(self.0) }
    }
}
