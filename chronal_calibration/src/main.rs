use std::io::{BufReader,BufRead};
use std::fs::File;
 
fn main() {
    let file = File::open("input1.txt").unwrap();
    let mut total = 0;
    for line in BufReader::new(file).lines() {
        let number: i32 = line.unwrap().parse().unwrap();
        total += number;
    }
    println!("{}", total);
}
