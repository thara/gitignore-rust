use std::env;

#[macro_use]
extern crate error_chain;
extern crate env_logger;

extern crate ignor;
extern crate reqwest;

error_chain!{
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

fn run() -> Result<()> {
    env_logger::init().expect("Failed to initialize logger");

    let args: Vec<String> = env::args().skip(1).collect();

    let res = if args.len() == 0 {
        ignor::list()
    } else {
        ignor::search(&args)
    };

    let _ = std::io::copy(&mut res?, &mut std::io::stdout())?;
    Ok(())
}

quick_main!(run);
