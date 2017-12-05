mod dec1;
mod dec2;
mod dec3;
use std::fs::File;
use std::io::prelude::*;
extern crate itertools;

fn main() {
    let mut input1 = File::open("data/input1.txt").expect("input 1 not found!");

    let mut contents = String::new();

    input1.read_to_string(&mut contents).expect("couldn't read");
    println!("1-1: {}", dec1::captcha(&contents));

    let mut input2 = File::open("data/input2.txt").expect("input 2 not found!");
    let mut contents2 = String::new();
    input2.read_to_string(&mut contents2).expect("couldn't read 2");

    println!("2-1: {}", dec2::checksum(&contents2));

    println!("3-1: {}", dec3::spiral(265149));
}
