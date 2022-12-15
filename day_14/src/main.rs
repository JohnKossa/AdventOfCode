#[macro_use] extern crate scan_fmt;
extern crate fxhash;
use fxhash::FxHashSet;

fn main() {
    let now = std::time::Instant::now();
    let contents = include_str!("../files/input.txt");
    let lines = contents.trim().split("\n").map(|x|x.trim());
    let mut walls: FxHashSet<Coordinate> = FxHashSet::default();
    lines.for_each(|line|{
        let segment_endpoints = line.split(" -> ").map(|str|{
            scan_fmt!(str, "{},{}", usize, usize).unwrap()
        }).collect::<Vec<Coordinate>>();
        let mut windows = segment_endpoints.windows(2);
        while let Some(&[first,second]) = windows.next(){
            let (x1, y1) = first;
            let (x2, y2) = second;
            if x1 < x2{
                for x in x1 as u32..=x2 as u32{
                    walls.insert((x as usize, y1));
                }
            }else if x1 > x2 {
                for x in x2 as u32..=x1 as u32{
                    walls.insert((x as usize, y1));
                }
            }else if y1 < y2 {
                for y in y1 as u32..=y2 as u32{
                    walls.insert((x1, y as usize));
                }
            }else if y1 > y2{
                for y in y2 as u32..=y1 as u32{
                    walls.insert((x1, y as usize));
                }
            }
        };
    });
    let floor_height = 2 + *(walls.iter().map(|(_, y)|y).max().unwrap());
    part_1(&walls, floor_height);
    part_2(&walls, floor_height);
    println!("Execution time: {:?}", now.elapsed());
}

type Coordinate = (usize, usize);

fn part_1(walls: &FxHashSet<Coordinate>, floor_height: usize){
    let mut rest_sand: FxHashSet<Coordinate> = FxHashSet::default();
    let sand_origin: Coordinate = (500,0);
    let mut i = 0;
    'generate_sand: loop{
        let (mut sand_x, mut sand_y) = sand_origin;
        'sand_fall: loop{
            if sand_y >= floor_height{
                println!("Answer for part 1: {}", i);
                break 'generate_sand;
            }
            if !walls.contains(&(sand_x, sand_y+1)) && !rest_sand.contains(&(sand_x, sand_y+1)){
                sand_y += 1;
            }else if !walls.contains(&&(sand_x-1, sand_y+1)) && !rest_sand.contains(&(sand_x-1, sand_y+1)){
                sand_y += 1;
                sand_x -=1;
            }
            else if !walls.contains(&(sand_x+1, sand_y+1)) && !rest_sand.contains(&(sand_x+1, sand_y+1)){
                sand_y += 1;
                sand_x +=1;
            }else{
                rest_sand.insert((sand_x, sand_y));
                break 'sand_fall;
            }
        }
        i +=1;
    }
}

fn part_2(walls: &FxHashSet<Coordinate>, floor_height: usize){
    let mut rest_sand: FxHashSet<Coordinate> = FxHashSet::default();
    let sand_origin: Coordinate = (500,0);
    let mut count = 0;
    while !rest_sand.contains(&(500,0)){
        count += 1;
        let (mut sand_x, mut sand_y) = sand_origin;
        loop{
            if sand_y == floor_height-1{
                rest_sand.insert((sand_x, sand_y));
                break;
            }
            if !walls.contains(&(sand_x, sand_y+1)) && !rest_sand.contains(&(sand_x, sand_y+1)){
                sand_y += 1;
            }else if !walls.contains(&&(sand_x-1, sand_y+1)) && !rest_sand.contains(&(sand_x-1, sand_y+1)){
                sand_y += 1;
                sand_x -=1;
            }
            else if !walls.contains(&(sand_x+1, sand_y+1)) && !rest_sand.contains(&(sand_x+1, sand_y+1)){
                sand_y += 1;
                sand_x +=1;
            }else{
                rest_sand.insert((sand_x, sand_y));
                break;
            }
        }
    }
    println!("Answer for part 2 {}", count);
}
