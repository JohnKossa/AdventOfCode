#[macro_use] extern crate scan_fmt;
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;

fn main() {
    let now = std::time::Instant::now();
    //let tstamp_parse = now.elapsed();
    let now1 = std::time::Instant::now();
    part_1();
    let tstamp_pt1 = now1.elapsed();
    let now2 = std::time::Instant::now();
    part_2();
    let tstamp_pt2 = now2.elapsed();
    //println!("        Parse time: {:?}", tstamp_parse);
    println!("        Execution time 1: {:?}", tstamp_pt1);
    println!("        Execution time 2: {:?}", tstamp_pt2);
    println!("        Execution time total: {:?}", now.elapsed());
}

fn part_1(){
    let contents = include_str!("../files/input.txt");
    //let contents = include_str!("../files/sample.txt");
    let lines = contents
        .trim()
        .split("\n")
        .map(|x|x.trim());
    println!("Answer 1: {}", 1);
}

fn part_2(){
    println!("Answer 2: {}", 2);
}
