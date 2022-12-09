use std::fs;

fn main() {
    let now = std::time::Instant::now();
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    part_1(&contents);
    part_2(&contents);
    println!("Execution time: {:?}", now.elapsed());
}

fn part_1(contents: &str){
    println!("Result for part 1: {}", 1);
}

fn part_2(contents: &str){
    println!("Result for part 2: {}", 2);
}
