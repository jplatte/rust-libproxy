use std::ffi::{CStr, CString};

use error::{Error, ProxyResolutionError};
use proxies::Proxies;
use proxy_sys;

/// A ProxyFactory is used to query the system for potential proxies to use in order to reach a
/// given URL.
///
/// A ProxyFactory should be kept around as long as possible as it contains cached data to increase
/// performance. Memory usage should be minimal (cache is small) and the cache lifespan is handled
/// automatically.
pub struct ProxyFactory(*mut proxy_sys::pxProxyFactory);

impl ProxyFactory {
    /// Creates a new ProxyFactory.
    pub fn new() -> Option<ProxyFactory> {
        let ptr = unsafe { proxy_sys::px_proxy_factory_new() };
        if ptr.is_null() {
            None
        } else {
            Some(ProxyFactory(ptr))
        }
    }

    /// Get which proxies to use for the specified URL.
    ///
    /// If the first proxy fails, the second should be tried, etc...
    ///
    /// This method always blocks. In most cases, the time required to complete this function call
    /// is simply the time required to read the configuration (i.e. from gconf, kconfig, etc).
    ///
    /// In the case of PAC, if no valid PAC is found in the cache (i.e. configuration has changed,
    /// cache is invalid, etc), the PAC file is downloaded and inserted into the cache. This is the
    /// most expensive operation as the PAC is retrieved over the network. Once a PAC exists in the
    /// cache, it is merely a javascript invocation to evaluate the PAC. One should note that DNS
    /// can be called from within a PAC during javascript invocation.
    ///
    /// In the case of WPAD, WPAD is used to automatically locate a PAC on the network. Currently,
    /// we only use DNS for this, but other methods may be implemented in the future. Once the PAC
    /// is located, normal PAC performance (described above) applies.
    ///
    /// The format of the returned proxy strings are as follows:
    ///
    ///   - http://[username:password@]proxy:port
    ///   - socks://[username:password@]proxy:port
    ///   - socks5://[username:password@]proxy:port
    ///   - socks4://[username:password@]proxy:port
    ///   - \<procotol\>://[username:password@]proxy:port
    ///   - direct://
    ///
    /// Please note that the username and password in the above URLs are optional and should be use
    /// to authenticate the connection if present.
    ///
    /// For SOCKS proxies, when the protocol version is specified (socks4:// or sock5://), it is
    /// expected that only this version is used. When only socks:// is set, the client MUST try
    /// SOCKS version 5 protocol and, on connection failure, fallback to SOCKS version 4.
    ///
    /// Other proxying protocols may exist. It is expected that the returned  configuration scheme
    /// shall match the network service name of the proxy protocol or the service name of the
    /// protocol being proxied if the previous does not exist. As an example, on Mac OS X you can
    /// configure a RTSP streaming proxy. The expected returned configuration would be:
    ///
    ///   - rtsp://[username:password@]proxy:port
    ///
    /// Example:
    ///
    /// ```rust
    /// # use libproxy::ProxyFactory;
    /// let url = "http://example.com/";
    /// let factory = ProxyFactory::new().unwrap();
    ///
    /// println!("Proxies to try for {:?}:", url);
    /// for proxy in &factory.get_proxies(url).unwrap() {
    ///     println!(" * {:?}", proxy);
    /// }
    /// ```
    pub fn get_proxies(&self, url: &str) -> Result<Vec<String>, Error> {
        let url_c = CString::new(url)?;

        self.get_proxies_raw(&url_c)?
            .into_iter()
            .map(|c_str| {
                String::from_utf8(c_str.to_bytes().to_owned()).map_err(From::from)
            })
            .collect()
    }

    /// Get which proxies to use for the specified URL.
    ///
    /// This is the lower-level version of `get_proxies`. Instead of allocating a `Vec` like
    /// `get_proxies`, it returns a wrapper type that allows safe access to the array returned by
    /// the C API.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use std::ffi::CString;
    /// # use libproxy::ProxyFactory;
    /// let url = CString::new("http://example.com/").unwrap();
    /// let factory = ProxyFactory::new().unwrap();
    ///
    /// println!("Proxies to try for {:?}:", url);
    /// for proxy in &factory.get_proxies_raw(&url).unwrap() {
    ///     println!(" * {:?}", proxy.as_c_str());
    /// }
    /// ```
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

unsafe impl Send for ProxyFactory {}
unsafe impl Sync for ProxyFactory {}
