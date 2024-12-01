use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let now = std::time::Instant::now();
    let contents = include_str!("../files/input.txt");
    let monkey_split = contents.trim().split("\n\n");
    let mut monkeys: Vec<Monkey> = monkey_split.map(|monkey_string_block|{
        let lines = monkey_string_block.split('\n').map(|x|x.trim()).collect::<Vec<&str>>();
        Monkey{
            items: VecDeque::from(lines[1][16..].split(", ").map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>()),
            mod_num: lines[3].split(" ").nth(3).unwrap().parse::<u64>().unwrap(),
            opcode: match lines[2].chars().nth(21).unwrap(){
                '+' => Operations::Add,
                '*' => Operations::Mult,
                _ => unreachable!(),
            },
            opval: match lines[2].split(" ").nth(5).unwrap(){
                "old" => OpVal::Old,
                other => OpVal::Lit(other.parse::<u64>().unwrap()),
            },
            true_target: lines[4].split(" ").nth(5).unwrap().parse::<usize>().unwrap(),
            false_target: lines[5].split(" ").nth(5).unwrap().parse::<usize>().unwrap(),
        }
    }).collect();
    part_1(&mut monkeys);
    part_2(&mut monkeys);
    println!("Execution time: {:?}", now.elapsed());
}

struct Monkey{
    items: VecDeque<u64>,
    mod_num: u64,
    opcode: Operations,
    opval: OpVal,
    true_target: usize,
    false_target: usize,
}

impl Monkey{
    fn operation(&self, old_val: u64) -> u64{
        match (self.opcode, self.opval){
            (Operations::Add, OpVal::Lit(a)) => old_val+a,
            (Operations::Add, OpVal::Old) => old_val+old_val,
            (Operations::Mult, OpVal::Lit(a)) => old_val*a,
            (Operations::Mult, OpVal::Old) => old_val*old_val,
        }
    }
    fn test(&self, val: u64) -> usize{
        match val%self.mod_num==0 {
            true => self.true_target,
            false => self.false_target,
        }
    }
}

#[derive(Copy, Clone)]
enum Operations{
    Add,
    Mult,
}

#[derive(Copy, Clone)]
enum OpVal{
    Old,
    Lit(u64),
}

fn part_1(monkeys: &mut Vec<Monkey>){
    let mut inspect_count: HashMap<usize, u64> = HashMap::new();
    for _round_num in 1..=20{
        for idx in 0..monkeys.iter().count(){
            while let Some(item) = monkeys[idx].items.pop_front(){
                let new_item = monkeys[idx].operation(item)/3;
                let target = monkeys[idx].test(new_item);
                monkeys[target].items.push_back(new_item);
                inspect_count.insert(idx, inspect_count.get(&idx).unwrap_or(&0)+1);
            }
        }
    }

    let mut top_two_mokeys: Vec<&u64> = inspect_count.values().collect();
    top_two_mokeys.sort();
    let result = top_two_mokeys[top_two_mokeys.len()-1] * top_two_mokeys[top_two_mokeys.len()-2];
    println!("Result for part 1: {:?}", result);
}

fn part_2(monkeys: &mut Vec<Monkey>){
    let total_mod = monkeys.iter().map(|m|m.mod_num).fold(1, |acc, x| acc*x);
    let mut inspect_count: HashMap<usize, u64> = HashMap::new();
    for _round_num in 1..=10000{
        for idx in 0..monkeys.iter().count(){
            while let Some(item) = monkeys[idx].items.pop_front(){
                let new_item = monkeys[idx].operation(item) % total_mod;
                let target = monkeys[idx].test(new_item);
                monkeys[target].items.push_back(new_item);
                inspect_count.insert(idx, inspect_count.get(&idx).unwrap_or(&0)+1);
            }
        }
    }
    let mut top_two_mokeys: Vec<&u64> = inspect_count.values().collect();
    top_two_mokeys.sort();
    let result = top_two_mokeys[top_two_mokeys.len()-1] * top_two_mokeys[top_two_mokeys.len()-2];
    println!("Result for part 2: {:?}", result);
}
