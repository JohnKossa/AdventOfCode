#[macro_use] extern crate scan_fmt;
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;

fn main() {
    let now = std::time::Instant::now();
    let contents = include_str!("../files/input.txt");
    //let contents = include_str!("../files/sample.txt");
    let lines = contents
        .trim()
        .split("\n")
        .map(|x|x.trim());
    let sensor_beacon_pairs: Vec<(Coordinate, Coordinate)> = lines
        .map(|line|{
            let (sx, sy, bx, by) = scan_fmt!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", i32, i32, i32, i32).unwrap();
            ((sx, sy), (bx, by))
        })
        .collect();
    let sensor_clear_distances: Vec<(Coordinate, i32)> = sensor_beacon_pairs
        .iter()
        .map(|((sx, sy),(bx,by))|{
           ((*sx, *sy),(sx - bx).abs() + (sy - by).abs())
        })
        .collect();
    let tstamp_parse = now.elapsed();
    let now1 = std::time::Instant::now();
    part_1(&sensor_beacon_pairs, &sensor_clear_distances);
    let tstamp_pt1 = now1.elapsed();
    let now2 = std::time::Instant::now();
    part_2(&sensor_clear_distances);
    let tstamp_pt2 = now2.elapsed();
    println!("        Parse time: {:?}", tstamp_parse);
    println!("        Execution time 1: {:?}", tstamp_pt1);
    println!("        Execution time 2: {:?}", tstamp_pt2);
    println!("        Execution time total: {:?}", now.elapsed());
}

type Coordinate = (i32, i32);
type Square = (Coordinate, Coordinate);

fn part_1(sensor_beacon_pairs: &Vec<(Coordinate, Coordinate)>, sensor_clear_distances: &Vec<(Coordinate, i32)>){
    let search_y = 2000000;
    //filter for sensors in range of search y
    let relevant_sensors: Vec<(Coordinate, i32)> = sensor_clear_distances
        .iter()
        .filter(|((_, sy),d)|{
            (sy-search_y).abs() <= *d
        })
        .map(|x|*x)
        .collect();

    let ranges: Vec<(i32, i32)> = relevant_sensors
        .iter()
        .map(|((sx, sy), d)|{
            let remaining_x_dist = d - (sy - search_y).abs(); //get leftover Manhattan distance
            (sx-remaining_x_dist, sx+remaining_x_dist) //apply it to x in both directions
        })
        .collect();

    let mut starting_ranges = ranges.clone();
    
    let mut i = 0;
    while i< starting_ranges.len(){
        let mut merged_with: Vec<usize> = Vec::new();
        for ii in i+1..starting_ranges.len(){
            if starting_ranges[i].0 >= starting_ranges[ii].0 && starting_ranges[i].0 <= starting_ranges[ii].1 ||
                starting_ranges[i].1 >= starting_ranges[ii].0 && starting_ranges[i].1 <= starting_ranges[ii].1 ||
                starting_ranges[ii].0 >= starting_ranges[i].0 && starting_ranges[ii].0 <= starting_ranges[i].1 ||
                starting_ranges[ii].1 >= starting_ranges[i].0 && starting_ranges[ii].0 <= starting_ranges[i].1{
                starting_ranges[i] = (min(starting_ranges[i].0, starting_ranges[ii].0), max(starting_ranges[i].1, starting_ranges[ii].1));
                merged_with.push(ii);
            }
        }
        for idx in merged_with.iter().rev(){
            starting_ranges.remove(*idx);
        }
        if merged_with.len() == 0{
            i+=1;
        }
    }
    

    let clear_locations: i32 = starting_ranges
        .iter()
        .map(|(start, end)|1 + (end-start))
        .sum();

    let beacon_count: usize = sensor_beacon_pairs
        .iter()
        .map(|(_, beacon)|*beacon)
        .unique()
        .filter(|(_,by)|*by==search_y)
        .filter(|(bx,_)|{
            starting_ranges.iter().any(|(start, end)|{
                start <= bx && end >= bx
            })
        })
        .count();

    println!("Answer 1: {}", clear_locations-beacon_count as i32);
}

fn square_not_fully_covered(((x0, y0), (x1,y1)): Square, sensor:Coordinate, sensor_dist: i32) -> bool{
    //is the square completely covered by the sensor?
    let corners = [
        (x0, y0),
        (x0, y1),
        (x1, y0),
        (x1, y1)
    ];
    let largest_dist = corners
        .iter()
        .map(|(x, y)|(x-sensor.0).abs() + (y-sensor.1).abs())
        .max()
        .unwrap();
    largest_dist > sensor_dist
}

fn part_2(sensor_clear_distances: &Vec<(Coordinate, i32)>){
    let min_beacon_x = 0;
    let max_beacon_x = 4000000;
    //let max_beacon_x = 20; //use for sample
    let min_beacon_y = 0;
    let max_beacon_y = 4000000;
    //let max_beacon_y = 20; //use for sample
    let mut search_squares: Vec<Square> = vec![((min_beacon_x, min_beacon_y),(max_beacon_x, max_beacon_y))];
    while let Some((min, max)) = search_squares.pop(){
        if min == max{
            if sensor_clear_distances
                .iter()
                .all(|((sx, sy), d)| (sx-min.0).abs() + (sy-min.1).abs() > *d){
                    //println!("Unseen coord at {},{}", min.0, min.1);
                    let score :i64 = 4000000 * (min.0 as i64) + (min.1 as i64);
                    println!("Answer 2: {}", score);
                    return;
            }
        }else{
            //binary search grid
            let mid: Coordinate = ((min.0+max.0)/2, (min.1+max.1)/2);
            let new_squares:[Square;4] = [
                (min, mid),
                ((mid.0+1, min.1), (max.0, mid.1)),
                ((min.0, mid.1+1), (mid.0, max.1)),
                ((mid.0+1, mid.1+1),max)
            ];
            for square in new_squares.iter(){
                if square.0.0 > square.1.0 || square.0.1 > square.1.1 {
                    //square is inside out, stop iteration
                    continue;
                }
                if sensor_clear_distances
                    .iter()
                    .all(|(pos, d)| square_not_fully_covered(*square, *pos, *d)){
                        //square is possible, push it to search grid
                        search_squares.push(*square)
                }
            }
        }
    }
    println!("No result found!");
}
