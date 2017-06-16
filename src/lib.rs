//! # libproxy bindings for Rust.
//!
//! This library provides bindings to [libproxy].
//!
//! [libproxy]: https://libproxy.github.io/libproxy/

extern crate libc;

pub use factory::ProxyFactory;

pub mod error;
pub mod ffi;
pub mod proxies;

mod factory;
mod proxy_sys;
