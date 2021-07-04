use std::env;

extern crate ignor;
extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let res = if args.len() == 0 {
        ignor::list()
    } else {
        ignor::search(&args)
    };

    let _ = std::io::copy(&mut res?, &mut std::io::stdout())?;
    Ok(())
}
