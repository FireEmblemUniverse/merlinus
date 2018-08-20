    /* config.rs -- Configuration definitions and implementations.
 *
 * Edit this file if you want to extend the configuration options!
 */

extern crate toml;
extern crate lazy_static;

// XXX
use std::io::prelude::*;
use std::fs;
use std::collections::HashMap;

static CONFIG_FILE: &'static str = "merlinus.conf";

// XXX: I'm torn on whether this goes here or in main.rs. I'm putting it here
// for now so the namespacing works nicely, but there's something to be said
// for keeping the general program IO in main.rs.
lazy_static! {
    pub static ref CURRENT_CONFIG: BaseConfig = {
        let mut buf = String::new();
        // TODO: somehow allow this to be non-constant
        match fs::File::open(CONFIG_FILE) {
            // TODO: we can probably be more specific than just randomly
            // panicking and displaying no actually useful information
            Err(_) => panic!("Unable to open file!"),
            Ok(ref mut f) => {
                let _ = f.read_to_string(&mut buf);
                toml::from_str(buf.as_str()).unwrap()
            }
        }
    };
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BaseConfig {
    #[serde(default = "get_default_name")]
    pub title: String,
    #[serde(default)]
    pub backend: Backend,
    #[serde(default = "get_default_location")]
    pub meta_loc: String,
    #[serde(default = "HashMap::new")]
    pub tools: HashMap<String, ToolVer>,
}

fn get_default_name() -> String {
    "merlinus_project".to_string()
}

fn get_default_location() -> String {
    "convoy".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
//#[serde(untagged)]
pub enum Backend {
    EventAssembler,
    Native,
}

impl Default for Backend {
    fn default() -> Self {
        Backend::EventAssembler
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolVer {
    #[serde(default = "default_version")]
    pub version: String,
    pub variant: Option<String>,
}

fn default_version() -> String {
    "*".to_string()
}

/* ---------------------------------------------------------------------- */

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn basic_enums() {
        let cfg = r#"
        title = "debug_string"
        backend = "EventAssembler"
        "#;

        let result: BaseConfig = toml::from_str(cfg).unwrap();

        assert_eq!(result.title, "debug_string".to_string());
        match result.backend {
            Backend::EventAssembler => (),
            _ => panic!("Wrong backend!")
        }
    }

    #[test]
    fn defaults() {
        let cfg = r#""#;
        let result: BaseConfig = toml::from_str(cfg).unwrap();

        assert_eq!(result.title, "merlinus_project".to_string());
        match result.backend {
            Backend::EventAssembler => (),
            _ => panic!("Wrong backend!")
        }
    }
}

