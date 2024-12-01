use std::collections::HashSet;
extern crate fxhash;
use fxhash::FxHashSet;

type Coordinate = (i64, i64);

enum Move{
    Left,
    Right
}

enum Shape{
    HLine,
    Cross,
    BackL,
    VLine,
    Square
}

impl Shape{
    fn get_blocks(&self, pos: Coordinate) -> Vec<Coordinate>{
        let pos_map = match self {
            Shape::HLine => vec![(0,0), (1,0), (2,0), (3,0)],
            Shape::Cross => vec![(1,0), (0,1), (1,1), (2,1), (1,2)],
            Shape::BackL => vec![(0,0), (1,0), (2,0), (2,1), (2,2)],
            Shape::VLine => vec![(0,0),(0,1),(0,2),(0,3)],
            Shape::Square => vec![(0,0), (0,1), (1,0), (1,1)],
        };
        pos_map.iter().map(|(x,y)|(pos.0+x,pos.1+y)).collect()
    }
    fn down_check_locations(&self, pos: Coordinate) -> Vec<Coordinate>{
        let pos_map = match self {
            Shape::HLine => vec![(0,-1), (1,-1), (2,-1), (3,-1)],
            Shape::Cross => vec![(1,-1), (0,0), (2,0)],
            Shape::BackL => vec![(0,-1), (1,-1), (2,-1)],
            Shape::VLine => vec![(0,-1)],
            Shape::Square => vec![(0,-1), (1,-1)],
        };
        pos_map.iter().map(|(x,y)|(pos.0+x,pos.1+y)).collect()
    }
    fn left_check_locations(&self, pos: Coordinate) -> Vec<Coordinate>{
        let pos_map = match self {
            Shape::HLine => vec![(-1,0)],
            Shape::Cross => vec![(0,0), (-1,1), (0,2)],
            Shape::BackL => vec![(-1,0), (1,1), (1,2)],
            Shape::VLine => vec![(-1,0),(-1,1), (-1,2), (-1,3)],
            Shape::Square => vec![(-1,0), (-1,1)],
        };
        pos_map.iter().map(|(x,y)|(pos.0+x,pos.1+y)).collect()
    }
    fn right_check_locations(&self, pos: Coordinate) -> Vec<Coordinate>{
        let pos_map = match self {
            Shape::HLine => vec![(4,0)],
            Shape::Cross => vec![(2,0), (3,1), (2,2)],
            Shape::BackL => vec![(3,0), (3,1), (3,2)],
            Shape::VLine => vec![(1,0),(1,1), (1,2), (1,3)],
            Shape::Square => vec![(2,0), (2,1)],
        };
        pos_map.iter().map(|(x,y)|(pos.0+x,pos.1+y)).collect()
    }
}

fn shape_from_count(count: usize) -> Shape{
    match count%5{
        0=> Shape::HLine,
        1=> Shape::Cross,
        2=> Shape::BackL,
        3=>Shape::VLine,
        4=>Shape::Square,
        _=>unreachable!()
    }
}

fn main() {
    let now = std::time::Instant::now();
    let tstamp_parse = now.elapsed();
    let now1 = std::time::Instant::now();
    part_1();
    let tstamp_pt1 = now1.elapsed();
    let now2 = std::time::Instant::now();
    part_2();
    let tstamp_pt2 = now2.elapsed();

    println!("        Parse time: {:?}", tstamp_parse);
    println!("        Execution time 1: {:?}", tstamp_pt1);
    println!("        Execution time 2: {:?}", tstamp_pt2);
    println!("        Execution time total: {:?}", now.elapsed());
}

fn part_1(){
    let contents = include_str!("../files/input.txt");
    //let contents = include_str!("../files/sample.txt");
    let movements = contents.trim().chars().map(|c|{
        match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unreachable!()
        }
    }).collect::<Vec<Move>>();
    let movement_pattern_length = movements.len();
    let mut highest_block_height: usize = 0;
    let cave_max_right = 6;
    //let mut static_blocks: HashSet<Coordinate> = HashSet::new();
    let mut static_blocks: FxHashSet<Coordinate> = FxHashSet::default();
    let mut jet_idx = 0;
    for i in 0..2022{
    //for i in 0..6{
        let block: Shape = shape_from_count(i);
        let mut block_pos:Coordinate = (2, (highest_block_height+3) as i64);
        let mut block_stopped = false;
        while !block_stopped{
            let movement: &Move = &movements[jet_idx % movement_pattern_length];
            match movement {
                Move::Left =>{

                    if block.left_check_locations(block_pos).iter().any(|pos| static_blocks.contains(&pos) || pos.0<0){
                        //prevent movement
                        //println!("LEFT BLOCKED");
                        ()
                    }else{
                        //println!("LEFT");
                        block_pos.0 -=1;
                    }
                },
                Move::Right => {
                    //println!("RIGHT");
                    if block.right_check_locations(block_pos).iter().any(|pos| static_blocks.contains(&pos) || pos.0 > cave_max_right){
                        //prevent movement
                        //println!("RIGHT BLOCKED");
                        ()
                    }else{
                        //println!("RIGHT");
                        block_pos.0 +=1;
                    }
                }
            }
            if block.down_check_locations(block_pos).iter().any(|pos| static_blocks.contains(&pos) || pos.1<0){
                //println!("BLOCK {} LANDED", i);
                //prevent movement
                //block.get_blocks().iter().map(|(x,y)| (x+block_pos.0, y+block_pos.1)).for_each(|(x, y)|{
                block.get_blocks(block_pos).iter().for_each(|(x, y)|{
                    static_blocks.insert((*x,*y));
                    if (*y+1) as usize > highest_block_height{
                        //println!("new height after rock {} is {}",i, (*y+1));
                        highest_block_height = (*y + 1) as usize;
                    }
                });
                //add all locations to static blocks
                //adjust top height if necessary
                block_stopped = true;
            }else{
                block_pos.1 -=1;
            }
            jet_idx +=1;
        }
        //println!("jet index is {} after rock {}", jet_idx%movement_pattern_length, i);
    }


    println!("Answer 1: {}", highest_block_height);
}

fn part_2(){
    let contents = include_str!("../files/input.txt");
    //let contents = include_str!("../files/sample.txt");
    let movements = contents.trim().chars().map(|c|{
        match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unreachable!()
        }
    }).collect::<Vec<Move>>();
    let movement_pattern_length = movements.len();
    let mut highest_block_height: usize = 0;
    let mut highest_floor_height = 0;
    let cave_max_right = 6;
    let mut static_blocks: HashSet<Coordinate> = HashSet::new();
    let mut jet_idx = 0;
    let mut tetris_lines:Vec<(usize,usize,usize)> = Vec::new();
    let now = std::time::Instant::now();
    let mut rock_num = 0;
    let mut cycle_found = false;
    'rocks_counter: loop {
        let block: Shape = shape_from_count(rock_num);
        let mut block_pos:Coordinate = (2, (highest_block_height+3) as i64);
        let mut block_stopped = false;
        'rockfall: loop{
            let movement: &Move = &movements[jet_idx % movement_pattern_length];
            match movement {
                Move::Left =>{
                    if block.left_check_locations(block_pos).iter().any(|pos| pos.0 < 0 || static_blocks.contains(&pos)){
                        //prevent movement
                        ()
                    }else{
                        block_pos.0 -=1;
                    }
                },
                Move::Right => {
                    if block.right_check_locations(block_pos).iter().any(|pos| pos.0 > cave_max_right || static_blocks.contains(&pos)){
                        //prevent movement
                        ()
                    }else{
                        block_pos.0 +=1;
                    }
                }
            }
            if block.down_check_locations(block_pos).iter().any(|pos| pos.1 < highest_floor_height || static_blocks.contains(&pos)){
                //println!("block landed");
                //prevent movement
                block.get_blocks(block_pos).iter().for_each(|(x, y)|{
                    static_blocks.insert((*x,*y));
                    if (*y+1) as usize > highest_block_height{
                        highest_block_height = (*y + 1) as usize;
                    }
                    if  *y+1 > highest_floor_height &&
                        static_blocks.contains(&(0,*y)) &&
                        static_blocks.contains(&(1,*y)) &&
                        static_blocks.contains(&(2,*y)) &&
                        static_blocks.contains(&(3,*y)) &&
                        static_blocks.contains(&(4,*y)) &&
                        static_blocks.contains(&(5,*y)) &&
                        static_blocks.contains(&(6,*y)){
                        highest_floor_height = *y + 1;
                        //println!("new floor set to {}", *y+1);
                        //remove all static blocks lower than floor
                        //let mut tetris_lines:Vec<(i64,i64,usize)> = Vec::new();

                        //println!("Tetris block rock {} pattern {} height {}",rock_num,jet_idx % movement_pattern_length, highest_block_height );
                        if highest_block_height == highest_floor_height as usize && tetris_lines.iter().any(|(rock, pos, _height)| rock%5 == rock_num%5 && *pos == jet_idx % movement_pattern_length){
                            cycle_found=true;
                            println!("Reset Tetris block rock {} pattern {} height {}",rock_num,jet_idx % movement_pattern_length, highest_block_height );
                            tetris_lines.push((rock_num, jet_idx % movement_pattern_length, highest_block_height));
                            println!("CYCLE FOUND");
                        }else if highest_block_height == highest_floor_height as usize{
                            println!("Reset Tetris block rock {} pattern {} height {}",rock_num,jet_idx % movement_pattern_length, highest_block_height );
                            tetris_lines.push((rock_num, jet_idx % movement_pattern_length, highest_block_height));
                        }
                        //static_blocks = static_blocks.iter().filter(|(_,y)| y >= &highest_floor_height).map(|x|*x).collect();
                    }
                });
                block_stopped = true;
            }else{
                block_pos.1 -=1;
            }
            jet_idx +=1;
            if cycle_found{
                break 'rocks_counter;
            }
            if block_stopped{
                break 'rockfall;
            }
        }
        rock_num +=1;
    }
    let (rocknum_end, pattern_pos_end, highest_level_end) = tetris_lines[tetris_lines.len()-1];
    for i in (0..tetris_lines.len()-1).rev(){
        let (rocknum_start, pattern_pos_start, highest_level_start) = &tetris_lines[i];
        if rocknum_start%5 == rocknum_end%5 && pattern_pos_end==pattern_pos_start{
            //we found the other one. get the diffs and start calculation
        }
    }
    //last element of tetris_lines should have a match earlier in the vector
    //get both and subtract rock ids and heights
    //offset_rocks = first.rock_id
    //offset_height = first.height
    //cycles_needed = (1000000000000 - offset_rocks) / rockid_diff
    //cycle_height = cycles_needed * height_diff



    println!("Answer 2: {}", highest_block_height);
}