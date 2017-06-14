use std::os::raw::c_char;

#[allow(non_camel_case_types)]
pub enum pxProxyFactory {}

#[link(name = "proxy")]
extern "C" {
    #[no_mangle]
    pub fn px_proxy_factory_new() -> *mut pxProxyFactory;

    #[no_mangle]
    pub fn px_proxy_factory_get_proxies(proxy_factory: *mut pxProxyFactory, url: *const c_char)
        -> *mut *mut c_char;

    #[no_mangle]
    pub fn px_proxy_factory_free(proxy_factory: *mut pxProxyFactory);
}
