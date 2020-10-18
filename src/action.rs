use std::path::PathBuf;

use crate::target::Target;

pub enum Action {
    MkFile(PathBuf, Box<dyn Fn(Vec<Target>) -> String>),
}
