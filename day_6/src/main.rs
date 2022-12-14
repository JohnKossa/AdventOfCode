use std::fs;
use itertools::Itertools;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn part_1(){
    let input = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    for last_idx in 4..input.chars().count(){
        if input[last_idx-4..last_idx].chars().all_unique(){
            println!("Last index for pt1 is: {}", last_idx+1);
            break;
        }
    }
}

fn part_2(){
    let input = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    for last_idx in 14..input.chars().count(){
        if input[last_idx-14..last_idx].chars().all_unique(){
            println!("Last index for pt2 is: {}", last_idx+1);
            break;
        }
    }
}
