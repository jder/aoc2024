use std::fs;

fn main() {
    println!("{}", fs::read_to_string("input/day1.txt").unwrap());
}
