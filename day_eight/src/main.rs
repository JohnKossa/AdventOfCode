use itertools::Itertools;
use std::fs;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn part_1(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");

    println!("Result for part 1: {}", 1);
}

fn part_2(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");

    println!("Result for part 2: {}", 2);
}
