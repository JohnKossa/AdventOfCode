use std::cmp::Ordering;

fn main() {
    let now = std::time::Instant::now();
    let contents = include_str!("../files/input.txt");
    part_1(&contents);
    part_2(&contents);
    println!("Execution time: {:?}", now.elapsed());
}

#[derive(Debug, Clone)]
enum Token{
    Value(i32),
    List(Vec<Token>),
    Blank,
}

impl Token{
    fn from_str(line: &str) -> (usize, Token) {
        if line.is_empty() || line.starts_with("]"){
            return (0, Token::Blank);
        }
        if line.chars().next().unwrap().is_ascii_digit() {
            //found a digit, keep taking chars while we keep finding digits
            let mut numeric_chars = line.chars();
            let mut i = 0;
            while numeric_chars.next().unwrap().is_ascii_digit() {
                i += 1;
            }
            let value = line[0..i].parse::<i32>().unwrap();
            return (i, Token::Value(value));
        }

        //line should start with [ here

        let mut list: Vec<Token> = Vec::new();
        let mut i: usize = 1;
        while i < line.len() {
            //keep parsing chunks until we run out
            let (j, data) = Token::from_str(&line[i..]);
            i += j;
            list.push(data);
            if line.chars().nth(i).unwrap() == ']' {
                i += 1;
                break;
            }
            //line[i] should be ","
            i += 1;
        }

        (i, Token::List(list))
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Token{}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Token) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Token{
    fn cmp(&self, other: &Token) -> Ordering {
        match (self, other) {
            (Token::Blank, Token::Blank) => Ordering::Equal,
            (Token::Blank, _) => Ordering::Less,
            (_, Token::Blank) => Ordering::Greater,
            (Token::Value(a), Token::Value(b)) => a.cmp(b),
            (Token::Value(a), Token::List(_))=>{
                Token::List(vec![Token::Value(*a)]).cmp(other)
            }
            (Token::List(_), Token::Value(b))=>{
                self.cmp(&Token::List(vec![Token::Value(*b)]))
            }
            (Token::List(a), Token::List(b)) =>{
                for(first, second) in a.iter().zip(b.iter()){
                    match first.cmp(second) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

fn part_1(contents: &str){
    let mut lines = contents.trim().split("\n").map(|x|x.trim());
    let mut pair_num = 1;
    let mut in_order_pairs: Vec<usize> = Vec::new();
    while let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()){
        let (_,first_token) = Token::from_str(first_line.trim());
        let (_, second_token) = Token::from_str(second_line.trim());
        if first_token < second_token{
            in_order_pairs.push(pair_num);
        }
        lines.next();
        pair_num += 1;
    }
    let answer:usize = in_order_pairs.iter().sum();
    println!("Answer for part 1: {}", answer);
}

fn part_2(contents: &str){
    let lines = contents.trim().split("\n").map(|x|x.trim()).filter(|x|!x.is_empty());
    let (_, first_extra) = Token::from_str("[[2]]");
    let (_, second_extra) = Token::from_str("[[6]]");
    let mut all_packets = lines.map(|line| Token::from_str(line).1).collect::<Vec<Token>>();
    all_packets.push(first_extra.clone());
    all_packets.push(second_extra.clone());
    all_packets.sort();
    let first_divider_location = all_packets.iter().position(|x|*x == first_extra).unwrap();
    let second_divider_location = all_packets.iter().position(|x|*x == second_extra).unwrap();
    println!("Answer for part 2: {}", (1+first_divider_location)*(1+second_divider_location));
}
