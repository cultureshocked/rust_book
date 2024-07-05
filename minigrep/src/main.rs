use std::process;
use std::env;

use minigrep::Config;

fn main() {
    let conf = Config::new(env::args())
        .unwrap_or_else(|e| { 
            eprintln!("Problem parsing arguments: {}", e);
            eprintln!("usage: {} [pattern] [filename]", env::args().next().unwrap());
            process::exit(1);
        });
    
    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
