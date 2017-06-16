use std::ffi::{CStr, CString};

use error::{Error, ProxyResolutionError};
use proxies::Proxies;
use proxy_sys;

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
        let url_c = CString::new(url)?;

        self.get_proxies_raw(&url_c)?
            .into_iter()
            .map(|c_str| {
                String::from_utf8(c_str.to_bytes().to_owned()).map_err(From::from)
            })
            .collect()
    }

    pub fn get_proxies_raw(&self, url: &CStr) -> Result<Proxies, ProxyResolutionError> {
        Proxies::new(unsafe {
            proxy_sys::px_proxy_factory_get_proxies(self.0, url.as_ptr())
        })
    }
}

impl Drop for ProxyFactory {
    fn drop(&mut self) {
        unsafe { proxy_sys::px_proxy_factory_free(self.0) }
    }
}
