use std::fs;
use std::collections::HashSet;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn part_1(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let input_line = contents.trim();

    for last_idx in 4..input_line.chars().count(){
        let substr = &input_line[..last_idx+1];
        match (substr.chars().nth(last_idx-3), substr.chars().nth(last_idx-2), substr.chars().nth(last_idx-1), substr.chars().nth(last_idx)){
            (Some(a), Some(b), Some(c), Some(d)) if a!=b && a!=c && a!=d && b!=c && b!=d && c!=d => {
                println!("Chars are {} {} {} {}", a,b, c, d);
                println!("Last index is: {}", last_idx+1);
                break
            }
            _ => ()
        }
    }
}

fn part_2(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let input_line = contents.trim();

    for last_idx in 14..input_line.chars().count(){
        let substr = &input_line[..last_idx+1];
        let test_vec = vec![substr.chars().nth(last_idx-13),
            substr.chars().nth(last_idx-12),
            substr.chars().nth(last_idx-11),
            substr.chars().nth(last_idx-10),
            substr.chars().nth(last_idx-9),
            substr.chars().nth(last_idx-8),
            substr.chars().nth(last_idx-7),
            substr.chars().nth(last_idx-6),
            substr.chars().nth(last_idx-5),
            substr.chars().nth(last_idx-4),
            substr.chars().nth(last_idx-3),
            substr.chars().nth(last_idx-2),
            substr.chars().nth(last_idx-1),
            substr.chars().nth(last_idx)];
        let uniqueness_test_set: HashSet<char> = test_vec.iter().map(|elem|elem.unwrap()).collect();
        if uniqueness_test_set.len() == 14{
            println!("Last index is: {}", last_idx+1);
            break;
        }
    }
}
