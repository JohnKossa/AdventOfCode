use itertools::Itertools;
use std::fs;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn read_initial_stacks(mut lines: Vec<&str>) -> Vec<Vec<char>>{
    let last_line: &str = lines.pop().unwrap();
    let stack_count: u32 = last_line.chars().last().unwrap().to_digit(10).unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new();stack_count as usize];
    while let Some(s) = lines.pop(){
        let mut chars = s.clone().chars();
        let mut stack_num = 0;
        while let (Some(_lbracket), Some(item), Some(_rbracket)) = (chars.next(), chars.next(), chars.next()){
            if item != ' '{
                stacks[stack_num].push(item)
            }
            stack_num += 1;//move on to next stack
            chars.next();//discard the blank space if there is one
        }
    }
    stacks
}

fn read_moves(lines: Vec<&str>) -> Vec<(i32, i32, i32)>{
    lines.into_iter().map(|line| {
        let mut split_line = line.split(' ');
        split_line.next();
        let count = split_line.next().unwrap().parse::<i32>().unwrap();
        split_line.next();
        let source = split_line.next().unwrap().parse::<i32>().unwrap();
        split_line.next();
        let dest = split_line.next().unwrap().parse::<i32>().unwrap();
        (count, source, dest)
    }).collect()
}

fn process_move_1(stacks: &mut Vec<Vec<char>>, count: i32, source_idx: i32, dest_idx: i32){
    for _i in 0..count{
        let elem = stacks[(source_idx-1) as usize].pop();
        match elem {
            Some(c) =>  stacks[(dest_idx-1) as usize].push(c),
            _ => continue,
        }
    }
}

fn process_move_2(stacks: &mut Vec<Vec<char>>, count: i32, source_idx: i32, dest_idx: i32){
    let mut moved_elems: Vec<char> = Vec::new();
    for _i in 0..count{
        moved_elems.push(stacks[(source_idx-1) as usize].pop().unwrap());
    }
    while let Some(elem) = moved_elems.pop(){
        stacks[(dest_idx-1) as usize].push(elem);
    }
}

fn part_1(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let (initial_stacks_lines, moves_lines) = contents.trim().split("\n\n").next_tuple().unwrap();
    let mut stacks =  read_initial_stacks(initial_stacks_lines.split('\n').collect::<Vec<&str>>());
    read_moves(moves_lines.split('\n').collect::<Vec<&str>>()).into_iter().for_each(|item| process_move_1(&mut stacks, item.0, item.1, item.2));
    let mut result = "".to_owned();
    for mut stack in stacks {
        result = result + &stack.pop().unwrap().to_string();
    }
    println!("Result for part 1: {}", result);
}

fn part_2(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let (initial_stacks_lines, moves_lines) = contents.trim().split("\n\n").next_tuple().unwrap();
    let mut stacks =  read_initial_stacks(initial_stacks_lines.split('\n').collect::<Vec<&str>>());
    read_moves(moves_lines.split('\n').collect::<Vec<&str>>()).into_iter().for_each(|item| process_move_2(&mut stacks, item.0, item.1, item.2));
    let mut result = "".to_owned();
    for mut stack in stacks {
        result = result + &stack.pop().unwrap().to_string();
    }
    println!("Result for part 2: {}", result);
}
