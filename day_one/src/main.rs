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

    let mut highest_elf_value = 0;
    let mut current_elf_value = 0;
    lines.for_each(|line|{
        match line{
            "\n" | "" => {
                highest_elf_value = if current_elf_value > highest_elf_value { current_elf_value }else{ highest_elf_value };
                current_elf_value = 0;
            },
            a => current_elf_value = current_elf_value + a.parse::<i32>().unwrap(),
        }
    });
    println!("Highest elf value: {highest_elf_value}");
}

fn part_2(){
    let file_path = "files/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut highest_elf_values = vec![0,0,0];
    let mut current_elf_value = 0;
    lines.for_each(|line|{
        match line{
            "\n" | "" => {
                if current_elf_value > highest_elf_values[0]{
                    highest_elf_values.push(current_elf_value);
                    highest_elf_values.sort();
                    highest_elf_values.reverse();
                    highest_elf_values.pop();
                    highest_elf_values.reverse();

                }
                current_elf_value = 0;
            },
            a => current_elf_value = current_elf_value + a.parse::<i32>().unwrap(),
        }
    });
    println!("Highest elf values: {:?}", highest_elf_values);
    let sum:i32 = highest_elf_values.iter().sum();
    println!("Sum of highest: {}", sum)
}
