use std::fs;
use std::collections::HashSet;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}
fn part_1(moves: &Vec<char>){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let lines = contents.trim().split("\n");

    lines.map(|line|{

    });
    println!("Result for part 1: {}", 1);
}

fn part_2(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let lines = contents.trim().split("\n");

    lines.map(|line|{

    });
    println!("Result for part 1: {}", 1);
}
