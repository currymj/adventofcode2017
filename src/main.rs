mod dec1;
use dec1::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input1 = File::open("data/input1.txt").expect("input 1 not found!");

    let mut contents = String::new();

    input1.read_to_string(&mut contents).expect("couldn't read");
    println!("{}", captcha(&contents));
}
