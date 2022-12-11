use std::fs;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}


fn part_1(){
    let contents = include_str!("../files/input.txt");
    let lines = contents.trim().split("\n");
    lines.map(|line|line.trim()).for_each(|line| {

    });
    println!("Result for part 1: {:?}", 1);
}

fn part_2(){

}
