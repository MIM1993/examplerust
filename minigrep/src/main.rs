use std::env;
use std::process;

use minigrep;

fn main() {
    //接收参数
    let args = env::args();
    let cfg = minigrep::Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //捕获错误
    if let Err(e) = minigrep::run(cfg){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
