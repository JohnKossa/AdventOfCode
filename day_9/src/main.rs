use std::fs;
use std::collections::HashSet;

fn main() {
    let now = std::time::Instant::now();
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let lines = contents.trim().split("\n");
    let mut moves: Vec<char> = Vec::new();
    lines.map(|line|{
        let mut pieces = line.trim().split(" ");
        (pieces.next().unwrap().chars().next().unwrap(), pieces.next().unwrap().parse::<usize>().unwrap())
    }).for_each(|(dir, amt)|{
        for _i in 0..amt{
            moves.push(dir)
        }
    });
    part_1(&moves);
    part_2(&moves);
    println!("Execution time: {:?}", now.elapsed());
}

fn taxi_dist(first: (i32, i32), second: (i32, i32)) -> i32{
    (first.0-second.0).abs() + (first.1-second.1).abs()
}

fn move_direction(coord: &mut (i32, i32), dir: char){
    match dir{
        'U' => coord.1 +=1,
        'D' => coord.1 -=1,
        'L' => coord.0 -=1,
        'R' => coord.0 += 1,
        _ => unreachable!()
    }
}

fn is_left_of(first: (i32, i32), second: (i32, i32)) -> bool{
    first.0 < second.0
}

fn is_right_of(first: (i32, i32), second: (i32, i32)) -> bool{
    first.0 > second.0
}

fn is_above(first: (i32, i32), second: (i32, i32)) -> bool{
    first.1 > second.1
}

fn is_below(first: (i32, i32), second: (i32, i32)) -> bool{
    first.1 < second.1
}

fn part_1(moves: &Vec<char>){
    let mut tail_positions :HashSet<(i32, i32)> = HashSet::new();
    let mut head_position = (0,0);
    let mut tail_position = (0,0);
    tail_positions.insert((0,0));
    let mut moves_iter = moves.iter();
    while let Some(dir) = moves_iter.next(){
        move_direction(&mut head_position, *dir);
        if tail_position == head_position{
            //overlapping
            continue;
        }
        if (tail_position.0 == head_position.0) && ((tail_position.1-head_position.1).abs() == 1){
            //tail directly above or below head
            continue;
        }
        if ((tail_position.0-head_position.0).abs() == 1) && (tail_position.1 == head_position.1) {
            //tail directly left or right of head
            continue;
        }
        if ((tail_position.0-head_position.0).abs() == 1) && ((tail_position.1-head_position.1).abs() == 1){
            //tail diagonal from head
            continue;
        }
        if (head_position.0 - tail_position.0) >= 2 && head_position.1==tail_position.1{
            //head to right, move tail to right
            move_direction(&mut tail_position, 'R');
            tail_positions.insert(tail_position);
            continue;
        }
        if (tail_position.0 - head_position.0 ) >= 2 && head_position.1==tail_position.1{
            //head to left, move tail to left
            move_direction(&mut tail_position,'L');
            tail_positions.insert(tail_position);
            continue;
        }
        if ( head_position.1 - tail_position.1 ) >= 2 && head_position.0==tail_position.0{
            //head above move tail above
            move_direction(&mut tail_position,'U');
            tail_positions.insert(tail_position);
            continue;
        }
        if (tail_position.1 - head_position.1 ) >= 2 && head_position.0==tail_position.0{
            //head below move tail below
            move_direction(&mut tail_position,'D');
            tail_positions.insert(tail_position);
            continue;
        }
        if taxi_dist(head_position, tail_position) >= 3{
            if is_left_of(head_position, tail_position) && is_above(head_position, tail_position){
                move_direction(&mut tail_position,'U');
                move_direction(&mut tail_position,'L');
                tail_positions.insert(tail_position);
                continue;
            }
            if is_left_of(head_position, tail_position) && is_below(head_position, tail_position){
                move_direction(&mut tail_position,'D');
                move_direction(&mut tail_position,'L');
                tail_positions.insert(tail_position);
                continue;
            }
            if is_right_of(head_position, tail_position) && is_above(head_position, tail_position){
                move_direction(&mut tail_position,'U');
                move_direction(&mut tail_position,'R');
                tail_positions.insert(tail_position);
                continue;
            }
            if is_right_of(head_position, tail_position) && is_below(head_position, tail_position){
                move_direction(&mut tail_position,'D');
                move_direction(&mut tail_position,'R');
                tail_positions.insert(tail_position);
                continue;
            }
        }
    }
    println!("Result for part 1: {}", tail_positions.iter().count());
}

fn part_2(moves: &Vec<char>){
    let mut moves_iter = moves.iter();
    let mut rope_positions: Vec<(i32,i32)> = vec![(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)];
    let mut tail_positions :HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0,0));
    while let Some(dir) = moves_iter.next(){
        move_direction(&mut rope_positions[0], *dir);
        for idx in 0..(rope_positions.len()-1){
            if rope_positions[idx+1] == rope_positions[idx]{
                //overlapping
                continue;
            }
            if (rope_positions[idx+1].0 == rope_positions[idx].0) && ((rope_positions[idx+1].1-rope_positions[idx].1).abs() == 1){
                //tail directly above or below head
                continue;
            }
            if ((rope_positions[idx+1].0-rope_positions[idx].0).abs() == 1) && (rope_positions[idx+1].1 == rope_positions[idx].1) {
                //tail directly left or right of head
                continue;
            }
            if ((rope_positions[idx+1].0-rope_positions[idx].0).abs() == 1) && ((rope_positions[idx+1].1-rope_positions[idx].1).abs() == 1){
                //tail diagonal from head
                continue;
            }
            if (rope_positions[idx].0 - rope_positions[idx+1].0) >= 2 && rope_positions[idx].1==rope_positions[idx+1].1{
                //head to right, move tail to right
                move_direction(&mut rope_positions[idx+1], 'R');
                continue;
            }
            if (rope_positions[idx+1].0 - rope_positions[idx].0 ) >= 2 && rope_positions[idx].1==rope_positions[idx+1].1{
                //head to left, move tail to left
                move_direction(&mut rope_positions[idx+1],'L');
                continue;
            }
            if ( rope_positions[idx].1 - rope_positions[idx+1].1 ) >= 2 && rope_positions[idx].0==rope_positions[idx+1].0{
                //head above move tail above
                move_direction(&mut rope_positions[idx+1],'U');
                continue;
            }
            if (rope_positions[idx+1].1 - rope_positions[idx].1 ) >= 2 && rope_positions[idx].0==rope_positions[idx+1].0{
                //head below move tail below
                move_direction(&mut rope_positions[idx+1],'D');
                continue;
            }
            if taxi_dist(rope_positions[idx], rope_positions[idx+1]) >= 3{
                if is_left_of(rope_positions[idx], rope_positions[idx+1]) && is_above(rope_positions[idx], rope_positions[idx+1]){
                    move_direction(&mut rope_positions[idx+1],'U');
                    move_direction(&mut rope_positions[idx+1],'L');
                    continue;
                }
                if is_left_of(rope_positions[idx], rope_positions[idx+1]) && is_below(rope_positions[idx], rope_positions[idx+1]){
                    move_direction(&mut rope_positions[idx+1],'D');
                    move_direction(&mut rope_positions[idx+1],'L');
                    continue;
                }
                if is_right_of(rope_positions[idx], rope_positions[idx+1]) && is_above(rope_positions[idx], rope_positions[idx+1]){
                    move_direction(&mut rope_positions[idx+1],'U');
                    move_direction(&mut rope_positions[idx+1],'R');
                    continue;
                }
                if is_right_of(rope_positions[idx], rope_positions[idx+1]) && is_below(rope_positions[idx], rope_positions[idx+1]){
                    move_direction(&mut rope_positions[idx+1],'D');
                    move_direction(&mut rope_positions[idx+1],'R');
                    continue;
                }
            }
        }
        tail_positions.insert(rope_positions[rope_positions.len()-1]);
    }
    println!("Result for part 2: {}", tail_positions.iter().count());
}
