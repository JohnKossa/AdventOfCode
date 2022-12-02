use std::fs;

fn main() {
    println!("PART 1:");
    part_1();
    println!("PART 2:");
    part_2();
}

#[derive(Clone, Copy, PartialEq)]
enum Choice{
    Rock,
    Paper,
    Scissors,
}

fn beats(first: Choice, second: Choice) -> bool{
    match (first, second) {
        (Choice::Rock, Choice::Scissors) => true,
        (Choice::Scissors, Choice::Paper) => true,
        (Choice::Paper, Choice::Rock) => true,
        (_,_) => false
    }
}

fn get_winning_move(input:Choice) -> Choice{
    match input {
        Choice::Rock=> Choice::Paper,
        Choice::Paper=> Choice::Scissors,
        Choice::Scissors=> Choice::Rock
    }
}
fn get_losing_move(input:Choice) -> Choice{
    match input {
        Choice::Rock=> Choice::Scissors,
        Choice::Paper=> Choice::Rock,
        Choice::Scissors=> Choice::Paper
    }
}
fn get_draw_move(input:Choice) -> Choice{
    input
}

fn get_points_for_choice(input:Choice) -> i32{
    match input {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3
    }
}

fn convert_to_choice_pt1(input: &str) -> Choice{
    match input {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!()
    }
}

fn convert_to_choice_pt2(input: &str) -> Choice{
    match input {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!()
    }
}

fn is_win_pt_2(second: &str) -> bool{
    match second{
        "Z" => true,
        _   => false
    }
}

fn is_draw_pt_2(second: &str) -> bool{
    match second{
        "Y" => true,
        _   => false
    }
}

fn is_loss_pt_2(second: &str) -> bool{
    match second{
        "X" => true,
        _   => false
    }
}

fn part_2(){
    let file_path = "files/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total_score = 0;
    lines.for_each(|line|{
        if line == ""{
            return;
        }
        let mut split_text = line.split(" ");
        let first =  convert_to_choice_pt2(split_text.next().unwrap().trim());
        let desired_result = split_text.next().unwrap().trim();
        let mut round_score = 0;
        match desired_result {
            a if is_win_pt_2(a) => {
                round_score+=6;
                round_score+=get_points_for_choice(get_winning_move(first));
            },
            a if is_draw_pt_2(a) => {
                round_score+=3;
                round_score+=get_points_for_choice(get_draw_move(first));
            },
            a if is_loss_pt_2(a) => {
                round_score+=0;
                round_score+=get_points_for_choice(get_losing_move(first));
            },
            _ => panic!()
        }
        total_score += round_score;
    });
    println!("Total Score: {}", total_score);
}

fn part_1(){
    let file_path = "files/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total_score = 0;
    lines.for_each(|line|{
        if line == ""{
            return;
        }
        let mut split_text = line.split(" ");
        let first = convert_to_choice_pt1(split_text.next().unwrap().trim());
        let second = convert_to_choice_pt1(split_text.next().unwrap().trim());
        let mut round_score = 0;
        match (first, second) {
            (a,b) if beats(b, a) => {
                round_score+=6;
                round_score+=get_points_for_choice(b);
            }
            (a,b) if beats(a, b) => {
                round_score+=get_points_for_choice(b);
            }
            (a,b) if a==b =>{
                round_score += 3;
                round_score+= get_points_for_choice(b);
            }
            (_,_) => panic!()
        }
        total_score += round_score;
    });
    println!("Total Score: {}", total_score);
}
