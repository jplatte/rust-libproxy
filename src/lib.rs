use std::ffi::CStr;

mod proxy_sys;

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

    pub fn get_proxies(&self, url: &CStr) /* -> NullTerminatedArray<OwnedCStr> */ {
        let proxies = unsafe { proxy_sys::px_proxy_factory_get_proxies(self.0, url.as_ptr()) };
        // TODO:
        // * Create OwnedCStr: wrapper around *mut char that calls into C for drop
        //   * Deref into CStr via CStr::from_ptr
        // * Create NullTerminatedArray: wrapper around *mut *??? T where NULL indicates the end
        //   * Deref into &{,mut} [T] with std::slice::from_raw_parts{,_mut}
        // * Somehow make the two work together
    }
}

impl Drop for ProxyFactory {
    fn drop(&mut self) {
        unsafe { proxy_sys::px_proxy_factory_free(self.0) }
    }
}
