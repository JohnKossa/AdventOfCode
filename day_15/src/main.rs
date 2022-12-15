fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn part_1(){
    let contents = include_str!("../files/input.txt");
    let lines = contents.trim().split("\n").map(|x|x.trim());
    lines.for_each(|line|{
    });
    println!("Answer for part 1 {}", 1);
}

fn part_2(){
    println!("Answer for part 2 {}", 2);
}
