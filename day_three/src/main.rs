use std::fs;
use std::collections::HashSet;

fn main() {
    part_1();
    part_2();
}

fn get_priority(c: char) -> i32{
    match c{
        c if (c as i32) > 97 => (c as i32) - 96,
        c if (c as i32) < 97 => (c as i32) - 64 + 26,
        _ => unreachable!()
    }
}

fn part_1(){
    let file_path = "files/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.trim().split("\n");
    let mut priority_sum =0;
    lines.for_each(|line|{
        let str_len = line.chars().count();
        let first_string = &line[..str_len/2];
        let second_string = &line[str_len/2..];
        let first_hash:HashSet<char> = HashSet::from(first_string.chars().collect());
        let second_hash:HashSet<char> = HashSet::from(second_string.chars().collect());
        let mut intersection = first_hash.intersection(&second_hash);
        priority_sum += get_priority(*intersection.next().unwrap())
    });
    println!("Total priority for part 1: {}", priority_sum);
}

fn part_2(){
    let file_path = "files/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = contents.trim().split("\n").peekable();
    let mut priority_sum =0;
    while lines.peek().is_some(){
        let first_hash: HashSet<char> = HashSet::from(lines.next().unwrap().trim().chars().collect());
        let second_hash: HashSet<char> = HashSet::from(lines.next().unwrap().trim().chars().collect());
        let third_hash: HashSet<char> = HashSet::from(lines.next().unwrap().trim().chars().collect());
        let diff = first_hash.iter().filter(|k| second_hash.contains(k) && third_hash.contains(k)).map(|&c|c).collect::<Vec<char>>();
        priority_sum+= get_priority(diff[0]);
    }
    println!("Total priority for part 2: {}", priority_sum);
}
