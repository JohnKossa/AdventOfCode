use std::fs;

fn main() {
    println!("PART 1:");
    part_1();
    println!("PART 2:");
    part_2();
}

fn part_1(){
    // --snip--
    let file_path = "files/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");


}

fn part_2(){
    let file_path = "files/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
}
