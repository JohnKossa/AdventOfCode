use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1(){
    let file_path = "files/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.trim().split("\n");
    let mut total_overlaps = 0;
    lines.for_each(|line|{
        let assignments = line.trim().split(',').collect::<Vec<&str>>();
        let first: Vec<i32> = assignments[0].split('-').map(|part|part.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let second: Vec<i32> = assignments[1].split('-').map(|part|part.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if first[0] <= second[0] && first[1]>=second[1]{
            total_overlaps += 1;
        }else if second[0] <= first[0] && second[1]>=first[1]{
            total_overlaps += 1;
        }
    });
    println!("Total overlaps for part 1: {}", total_overlaps);
}

fn part_2(){
    let file_path = "files/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.trim().split("\n");
    let mut total_overlaps = 0;
    lines.for_each(|line| {
        let assignments = line.trim().split(',').collect::<Vec<&str>>();
        let first: Vec<i32> = assignments[0].split('-').map(|part|part.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let second: Vec<i32> = assignments[1].split('-').map(|part|part.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if first[0] >= second[0] && first[0] <= second[1] {
            total_overlaps += 1;
        } else if first[1] >= second[0] && first[1] <= second[1] {
            total_overlaps += 1;
        } else if second[0] >= first[0] && second[0] <= first[1] {
            total_overlaps += 1;
        } else if second[0] >= first[0] && second[1] <= first[1] {
            total_overlaps += 1;
        }
    });
    println!("Total overlaps for part 2: {}", total_overlaps);
}
