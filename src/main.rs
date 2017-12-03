use std::env;
use std::process;
extern crate advent;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        eprintln!("USAGE: one <input string>");
        process::exit(1);
    }
    let input = &args[1];
    println!("result: {}", advent::sum(&input[..]))
}
