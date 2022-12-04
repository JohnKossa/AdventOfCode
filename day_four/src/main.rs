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
    lines.map(|line|line.trim()).for_each(|line|{
        let assignments = line.split(',').collect::<Vec<&str>>();
        let mut first_split = assignments[0].split('-');
        let first_start: i32 = first_split.next().unwrap().parse().unwrap();
        let first_end: i32 = first_split.next().unwrap().parse().unwrap();
        let mut second_split = assignments[1].split('-');
        let second_start = second_split.next().unwrap().parse().unwrap();
        let second_end = second_split.next().unwrap().parse().unwrap();
        if first_start <= second_start && first_end>=second_end{
            total_overlaps += 1;
        }else if second_start <= first_start && second_end>=first_end{
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
    lines.map(|line|line.trim()).for_each(|line| {
        let assignments = line.split(',').collect::<Vec<&str>>();
        let mut first_split = assignments[0].split('-');
        let first_start: i32 = first_split.next().unwrap().parse().unwrap();
        let first_end: i32 = first_split.next().unwrap().parse().unwrap();
        let mut second_split = assignments[1].split('-');
        let second_start = second_split.next().unwrap().parse().unwrap();
        let second_end = second_split.next().unwrap().parse().unwrap();
        if first_start >= second_start && first_start <= second_end {
            total_overlaps += 1;
        } else if first_end >= second_start && first_end <= second_end {
            total_overlaps += 1;
        } else if second_start >= first_start && second_start <= first_end {
            total_overlaps += 1;
        } else if second_start >= first_start && second_end <= first_end {
            total_overlaps += 1;
        }
    });
    println!("Total overlaps for part 2: {}", total_overlaps);
}
