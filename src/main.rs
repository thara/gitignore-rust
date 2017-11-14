use std::env;

#[macro_use]
extern crate error_chain;
extern crate env_logger;
extern crate reqwest;

// See https://www.gitignore.io/docs
const GITIGNORE_BASEURL: &str = "https://www.gitignore.io/api";
const GITIGNORE_LIST_URL: &str = "https://www.gitignore.io/api/list";

error_chain!{
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn list() -> reqwest::Result<reqwest::Response> {
    reqwest::get(GITIGNORE_LIST_URL)
}

fn search(search_items: &Vec<String>) -> reqwest::Result<reqwest::Response> {
    let base_url = GITIGNORE_BASEURL.to_owned();
    let search_strs = search_items.join(",");
    let url = base_url + "/" + &search_strs;
    reqwest::get(&url)
}

fn run() -> Result<()> {
    env_logger::init().expect("Failed to initialize logger");

    let args: Vec<String> = env::args().skip(1).collect();

    let res = if args.len() == 0 {
        list()
    } else {
        search(&args)
    };

    let _ = std::io::copy(&mut res?, &mut std::io::stdout())?;
    Ok(())
}

quick_main!(run);
