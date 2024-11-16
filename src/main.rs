use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args = env::args();
    

    let config = Config::new(args).unwrap_or_else(|err|{
        println!("{}", err);    
        process::exit(1);
    });
    println!("Searching for {} in file {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Run error: {e}");
        process::exit(1);
    }
    
}

