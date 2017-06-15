extern crate libproxy;

use std::env;
use libproxy::ProxyFactory;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: proxy <url>");
        return;
    }

    let factory = ProxyFactory::new().unwrap();

    for proxy in factory.get_proxies(&args.nth(1).unwrap()).unwrap() {
        print!("{} ", proxy);
    }
    println!();
}
