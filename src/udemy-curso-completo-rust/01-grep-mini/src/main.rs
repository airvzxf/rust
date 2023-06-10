use std::env;
use grep_mini;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:#?}", args);

    grep_mini::run(&args);
}
