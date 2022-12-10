use std::fs;

fn main() {
    let now = std::time::Instant::now();
    let x_values = part_1();
    part_2(x_values);
    println!("Execution time: {:?}", now.elapsed());
}

enum Command{
    AddX(i32),
    Noop
}

fn part_1() -> Vec<i32>{
    let contents = include_str!("../files/input.txt");
    let lines = contents.trim().split("\n");
    let command_list: Vec<Command> = lines.map(|line|line.trim().split(' ')).map(|mut tokens|{
        match (tokens.next(), tokens.next()) {
            (Some("noop"), None) => Command::Noop,
            (Some("addx"), Some(i)) => Command::AddX(i.parse::<i32>().unwrap()),
            _ => unreachable!()
        }
    }).collect();
    let mut x_values: Vec<i32> = vec![1];
    for cmd in command_list{
        match cmd{
            Command::AddX(a) => {
                x_values.push(x_values[x_values.len()-1]);
                x_values.push(x_values[x_values.len()-1]+a);
            },
            Command::Noop => {
                x_values.push(x_values[x_values.len()-1]);
            }
        }
    }
    let answer: i32 = [20, 60, 100, 140, 180, 220].iter().map(|i|{
        x_values[i-1] * (*i as i32)
    }).sum();
    println!("Result for part 1: {:?}", answer);
    x_values
}

fn part_2(x_values: Vec<i32>){
    let display_indices: Vec<i32> = (0..241).collect();
    let display: String = display_indices.iter().map(|cycle_num|{
        let val = x_values[*cycle_num as usize];
        (val-(cycle_num%40)).abs() <= 1
    }).map(|lit|{
        match lit{
            true => '#',
            false => '.',
        }
    }).collect();
    let line1 = &display[..40];
    let line2 = &display[40..80];
    let line3 = &display[80..120];
    let line4 = &display[120..160];
    let line5 = &display[160..200];
    let line6 = &display[200..];
    println!("Result pt 2:");
    println!("{}", line1);
    println!("{}", line2);
    println!("{}", line3);
    println!("{}", line4);
    println!("{}", line5);
    println!("{}", line6);
}
