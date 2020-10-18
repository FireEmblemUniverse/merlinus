use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, fs::File};

use super::ProjectRepr;
use crate::error::{Error, Result};

pub(super) type ProjectFile = ProjectRepr<PathBuf>;

impl ProjectFile {
    fn from_path(p: impl AsRef<Path>) -> Result<Self> {
        let mut contents = String::new();
        let mut file = File::open(&p).map_err(Error::Io)?;
        file.read_to_string(&mut contents).map_err(Error::Io)?;

        let mut result: Self =
            toml::from_str(&contents).map_err(Error::ProjectParseError)?;

        let parent = match p.as_ref().parent() {
            None => return Ok(result),
            Some(p) => p,
        };

        for entry in fs::read_dir(parent).map_err(Error::Io)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let item = path.as_path().join(Path::new("convoy.toml"));
                if item.exists() {
                    result.add_item(item);
                }
            }
        }

        Ok(result)
    }
}
