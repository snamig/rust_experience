use std::env::{args, Args};

fn main() {
    let arg: Args = args();
    println!("{:?} f", arg);
}
