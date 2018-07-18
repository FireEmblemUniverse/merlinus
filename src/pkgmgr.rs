
use config;
use std::io;
use std::fs;
use std::path;

pub fn init_metadata() -> io::Result<()> {
    fs::create_dir(&config::CURRENT_CONFIG.meta_loc)
}

pub fn fetch_tool((name, cfg): (String, config::ToolVer)) {
}

