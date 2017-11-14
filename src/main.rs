use std::env;

#[macro_use]
extern crate error_chain;
extern crate env_logger;
extern crate reqwest;

// See https://www.gitignore.io/docs
const GITIGNORE_BASEURL: &str = "https://www.gitignore.io/api";

error_chain!{
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn run() -> Result<()> {
    env_logger::init().expect("Failed to initialize logger");

    let args: Vec<String> = env::args().skip(1).collect();

    let base_url = GITIGNORE_BASEURL.to_owned();
    let url = if args.len() == 0 {
        base_url + "/list"
    } else {
        let search_strs = args.join(",");
        base_url + "/" + &search_strs
    };

    let mut res = reqwest::get(&url)?;

    let _ = std::io::copy(&mut res, &mut std::io::stdout())?;
    Ok(())
}

quick_main!(run);
