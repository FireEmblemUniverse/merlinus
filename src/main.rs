
extern crate toml;
extern crate git2;

#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
extern crate lazy_static;

mod config;
mod pkgmgr;
mod backends;

fn main() {
    println!("Hello, world!");
}

