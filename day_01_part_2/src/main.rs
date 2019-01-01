use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashSet;
 
fn main() {
    let file = File::open("input1.txt").unwrap();
    let changes = BufReader::new(file).lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut frequencies = HashSet::new();
    let mut current_frequency = 0;
    frequencies.insert(current_frequency);

    let repeated_frequency = 'outer: loop {
        for change in changes.iter() {
            current_frequency += change;
            if !frequencies.contains(&current_frequency) {
                break 'outer current_frequency;
            }
            frequencies.insert(current_frequency);
        }
    };

    println!("{}", repeated_frequency);
}
