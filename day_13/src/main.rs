use std::cmp::Ordering;

fn main() {
    let now = std::time::Instant::now();
    let contents = include_str!("../files/input.txt");
    part_1(&contents);
    part_2(&contents);
    println!("Execution time: {:?}", now.elapsed());
}

#[derive(Clone)]
enum Token{
    Value(i32),
    List(Vec<Token>)
}

impl Token{
    fn from_str(line: &str) -> Self{
        if !line.starts_with("[") {
            Token::Value(line.parse().unwrap())
        } else {
            let mut inner_packets = Vec::new();
            let mut depth = 0;
            let mut start = 1;
            for i in 1..line.len() - 1 {
                let c = line.chars().nth(i).unwrap();
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => {
                        if depth == 0 {
                            let inner_packet = Token::from_str(&line[start..i]);
                            inner_packets.push(inner_packet);
                            start = i + 1;
                        }
                    }
                    _ => (),
                }
            }
            if line.len() > 2 {
                inner_packets.push(Token::from_str(&line[start..line.len() - 1]));
            }
            Token::List(inner_packets)
        }
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
        let first_token = Token::from_str(first_line.trim());
        //Token::from_str_2(first_line.trim());
        let second_token = Token::from_str(second_line.trim());
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
    let first_extra = Token::from_str("[[2]]");
    let second_extra = Token::from_str("[[6]]");
    let mut all_packets = lines.map(|line| Token::from_str(line)).collect::<Vec<Token>>();
    all_packets.push(first_extra.clone());
    all_packets.push(second_extra.clone());
    all_packets.sort();
    let first_divider_location = all_packets.iter().position(|x|*x == first_extra).unwrap();
    let second_divider_location = all_packets.iter().position(|x|*x == second_extra).unwrap();
    println!("Answer for part 2: {}", (1+first_divider_location)*(1+second_divider_location));
}
