
use config;
use project;

mod eacore;

pub trait Backend {
    // XXX: need a better name for this
    type T;
}

pub fn get_suite(t: config::Backend) -> impl Backend {
    match t {
        config::Backend::EventAssembler => eacore::EACore {},
        _ => unimplemented!()
    }
}

