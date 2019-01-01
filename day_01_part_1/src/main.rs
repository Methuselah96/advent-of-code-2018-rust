use std::io::{BufReader,BufRead};
use std::fs::File;
 
fn main() {
    let file = File::open("input1.txt").unwrap();
    let total = BufReader::new(file).lines().map(|line| line.unwrap().parse::<i32>().unwrap()).sum::<i32>();
    println!("{}", total);
}
