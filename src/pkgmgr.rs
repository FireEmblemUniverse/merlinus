
extern crate git2;

use config;
use std::io;
use std::fs;
use std::path;

static FEU_GITHUB_BASE_URL : &'static str =
    "https://github.com/{USER}/{NAME}.git";

pub fn init_metadata() -> io::Result<()> {
    fs::create_dir(&config::CURRENT_CONFIG.meta_loc)
}

pub fn fetch_tool((name, cfg): (String, config::ToolVer)) {
    // XXX - this sucks, stylistically, We could use std::fmt to do it more
    // nicely, but then we need to have the magic string in the method (instead
    // of using a static global like above
    let repo_url =
        str::replace(
            str::replace(FEU_GITHUB_BASE_URL,
                         "{USER}", cfg.owner.as_str()).as_str(),
            "{NAME}", name.as_str()
        );

    let mut repo_local = path::PathBuf::new();
    repo_local.push(&config::CURRENT_CONFIG.meta_loc);
    repo_local.push(name);

    match git2::Repository::clone(repo_url.as_str(), repo_local) {
        _ => ()
    }
}

