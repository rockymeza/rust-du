//! `du -sb` implemented in Rust.
//!
//! This was solely written for the purpose of learning Rust. Do not use it in production code.
extern crate getopts;
extern crate du;

use std::env;
use std::io::Write;

use getopts::Options;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <path> [<path> ..]", program);
    println!("{}", opts.usage(&brief));
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut stderr = std::io::stderr();

    let mut opts = Options::new();
    opts.optflag("", "help", "Print this help menu and exit.");

    match opts.parse(&args[1..]) {
        Ok(matches) => {
            if matches.opt_present("help") || matches.free.is_empty() {
                print_usage(&args[0], opts);
                return;
            }

            for path in &matches.free {
                match du::get_size(path) {
                    Ok(size) => {
                        println!("{}\t{}", size, path);
                    }
                    Err(err) => {
                        writeln!(&mut stderr, "Error: {}", err).unwrap();
                    }
                }
            }
        }
        Err(err) => {
            writeln!(&mut stderr, "{}", err).unwrap();
            print_usage(&args[0], opts);
        }
    }
}
