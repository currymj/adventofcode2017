mod dec1;
mod dec2;
mod dec3;
mod dec4;
mod dec5;
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

    let mut input4 = File::open("data/input4.txt").expect("input 4 not found!");
    let mut contents4 = String::new();
    input4.read_to_string(&mut contents4).expect("couldn't read 4");

    let valids: Vec<_> = contents4.split("\n").filter(|x| { dec4::passphrase(x)}).collect();
    let numvalid = valids.len();
    println!("4-1: {}", numvalid);

    let mut input5 = File::open("data/input5.txt").expect("input 5 not found!");
    let mut contents5 = String::new();
    input5.read_to_string(&mut contents5).expect("couldn't read 5");
    let xvec: Vec<_> = contents5.split("\n").flat_map(|z| z.parse::<i64>()).collect();
    println!("5-1: {}", dec5::followjumps(&xvec));
}
