use std::fs;

fn main() {
    println!("PART 1:");
    part_1();
    println!("PART 2:");
    part_2();
}

fn is_win_pt_1(first: &str, second: &str) -> bool{
    match (first, second){
        ("A","Y") => true,
        ("B","Z") => true,
        ("C","X") => true,
        (_,_)     => false
    }
}



fn is_draw_pt_1(first: &str, second: &str) -> bool{
    match (first, second){
        ("A","X") => true,
        ("B","Y") => true,
        ("C","Z") => true,
        (_,_)     => false
    }
}

fn is_loss_pt_1(first: &str, second: &str) -> bool{
    match (first, second){
        ("A","Z") => true,
        ("B","X") => true,
        ("C","Y") => true,
        (_,_)     => false
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

fn get_win_str(first: &str) -> &str{
    match first{
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _=>panic!("Shouldn't be possible"),
    }
}

fn get_draw_str(first: &str) -> &str{
    first
}

fn get_loss_str(first: &str) -> &str{
    match first{
        "A" => "C",
        "B" => "A",
        "C" => "B",
        _=>panic!("Shouldn't be possible"),
    }
}

fn get_point_value(choice :&str) -> i32{
    match choice {
        "A"=> 1,
        "B"=> 2,
        "C"=> 3,
        _ => panic!("Shouldn't be possible")
    }
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
        let first: &str = split_text.next().unwrap().trim();
        let second: &str = split_text.next().unwrap().trim();
        let mut round_score = 0;
        match (first, second) {
            (a,b) if is_win_pt_1(a, b) => {
                round_score+=6;
            },
            (a,b) if is_draw_pt_1(a, b) => {
                round_score+=3;
            },
            (a,b) if is_loss_pt_1(a, b) => {
                round_score+=0;
            },
            (_,_) => {
                println!("Invalid pattern {} {}", first, second);
                panic!("invalid pattern 1");
            },
        }
        match second {
            "X" => round_score+=1,
            "Y" => round_score+=2,
            "Z" => round_score+=3,
            _ => panic!("invalid pattern 2"),
        }

        total_score += round_score;
    });
    println!("Total Score: {}", total_score);
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
        let first: &str = split_text.next().unwrap().trim();
        let second: &str = split_text.next().unwrap().trim();
        let mut round_score = 0;
        match (first, second) {
            (a,b) if is_win_pt_2(b) => {
                round_score+=6;
                round_score+=get_point_value(get_win_str(a));
            },
            (a,b) if is_draw_pt_2(b) => {
                round_score+=3;
                round_score+=get_point_value(get_draw_str(a));
            },
            (a,b) if is_loss_pt_2(b) => {
                round_score+=0;
                round_score+=get_point_value(get_loss_str(a));
            },
            (_,_) => {
                println!("Invalid pattern {} {}", first, second);
                panic!("invalid pattern 1");
            },
        }
        total_score += round_score;
    });
    println!("Total Score: {}", total_score);
}
