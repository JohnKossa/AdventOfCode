use std::fs;
use itertools::Itertools;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn test_unique(test_str: String) -> bool{
    test_str.chars().unique().count() == test_str.chars().count()
}

fn part_1(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let input_line = contents.trim();
    for last_idx in 4..input_line.chars().count(){
        let substr = &input_line[..last_idx+1];
        if test_unique(substr[substr.len()-4..].to_string()){
            println!("Last index for pt1 is: {}", last_idx+1);
            break;
        }
    }
}

fn part_2(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let input_line = contents.trim();

    for last_idx in 14..input_line.chars().count(){
        let substr = &input_line[..last_idx+1];
        if test_unique(substr[substr.len()-14..].to_string()){
            println!("Last index for pt2 is: {}", last_idx+1);
            break;
        }
    }
}
