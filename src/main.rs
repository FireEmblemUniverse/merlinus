
extern crate toml;

#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;

mod config;
mod pkgmgr;
mod project;
mod backends;

fn main() {
    let matches = clap_app!(merlinus =>
        (version: "0.1.0")
        (author: "CT075 via FEUniverse")
        (about: "project and build manager for FE romhacks")
        (@arg ACTION: +required "What are we doing?")
    ).get_matches();
}

