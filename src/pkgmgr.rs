
extern crate git2;

use config;

static FEU_GITHUB_BASE_URL : &'static str =
    "https://github.com/{USER}/{NAME}.git";

pub fn fetch_tool((name, cfg): (String, config::ToolVer)) {
    let repo_url =
        str::replace(FEU_GITHUB_BASE_URL, "{USER}", cfg.owner.as_str());

    // TODO
}

