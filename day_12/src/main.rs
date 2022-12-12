use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let now = std::time::Instant::now();
    let contents = include_str!("../files/input.txt");
    let mut height_map:Vec<Vec<u32>> = Vec::new();
    let mut start:(usize, usize) = (0,0);
    let mut end:(usize, usize) = (0,0);
    let mut lines = contents.trim().split("\n");
    let mut row_num: usize = 0;
    while let Some(line) = lines.next(){
        height_map.push(line.trim().chars().enumerate().map(|(idx, c)|{
            match (idx, c){
                (x, 'S') => {
                    start = (x, row_num);
                    0
                },
                (x, 'E')=>{
                    end = (x, row_num);
                    27
                },
                (_,a)=>{
                    (a as u32)-96
                }
            }
        }).collect::<Vec<u32>>());
        row_num+=1;
    }
    part_1(&height_map, start);
    part_2(&height_map, end);
    println!("Execution time: {:?}", now.elapsed());
}

type Coordinate=(usize, usize);

fn all_visited(visit_grid: &Vec<Vec<bool>>) -> bool{
    visit_grid.iter().flatten().all(|&x|x)
}

fn neighbors_of(coords: Coordinate, width: u32, height: u32) -> Vec<Coordinate>{
    let mut possible_neighbors :Vec<Coordinate> = Vec::new();
    if coords.0 > 0{
        possible_neighbors.push((coords.0-1, coords.1))
    }
    if coords.0 < (width-1) as usize{
        possible_neighbors.push((coords.0+1, coords.1))
    }
    if coords.1 > 0{
        possible_neighbors.push((coords.0, coords.1-1))
    }
    if coords.1 < (height-1) as usize{
        possible_neighbors.push((coords.0, coords.1+1))
    }
    possible_neighbors
}

fn part_1(height_map: &Vec<Vec<u32>>, start: Coordinate){
    let grid_height = height_map.len();
    let grid_width = height_map[0].len();
    let mut visited:Vec<Vec<bool>> = vec![vec![false;grid_width];grid_height];
    let mut dist_map:Vec<Vec<u32>> = vec![vec![u32::MAX;grid_width];grid_height];
    visited[start.1][start.0] = true;
    dist_map[start.1][start.0] = 0;
    let mut paths: VecDeque<Coordinate> = VecDeque::new();
    paths.push_back((start.0, start.1));
    while !all_visited(&visited) && paths.len() > 0{
        let (x, y) = paths.pop_front().unwrap();
        let current_height = height_map[y][x];
        let current_dist = dist_map[y][x];
        if height_map[y][x] == 27{
            println!("Answer for part 1: {}", current_dist);
            break;
        }
        let mut neighbors = neighbors_of((x, y), grid_width as u32, grid_height as u32);
        neighbors = neighbors.iter().filter(|coord|{
            !visited[coord.1 ][coord.0 ]
        }).filter(|coord|{
            let coord_height = height_map[coord.1][coord.0];
            coord_height <= current_height || coord_height-current_height == 1
        }).map(|x|*x).collect();
        neighbors.iter().for_each(|coord|{
            visited[y][x] = true;
            dist_map[coord.1][coord.0] = current_dist+1;
            paths.push_back((coord.0, coord.1));
        });
        paths = VecDeque::from(paths.iter().unique().map(|x|*x).collect::<Vec<Coordinate>>());
    }
}

fn part_2(height_map: &Vec<Vec<u32>>, end: Coordinate){
    let grid_height = height_map.len();
    let grid_width = height_map[0].len();
    let mut visited:Vec<Vec<bool>> = vec![vec![false;grid_width];grid_height];
    let mut dist_map:Vec<Vec<u32>> = vec![vec![u32::MAX;grid_width];grid_height];
    visited[end.1][end.0] = true;
    dist_map[end.1][end.0] = 0;
    let mut paths: VecDeque<Coordinate> = VecDeque::new();
    paths.push_back((end.0, end.1));
    while !all_visited(&visited) && paths.len() > 0{
        let (x, y) = paths.pop_front().unwrap();
        let current_height = height_map[y][x];
        let current_dist = dist_map[y][x];
        if height_map[y][x] == 1{
            println!("Answer for part 2: {}", current_dist);
            break;
        }
        let mut neighbors = neighbors_of((x, y), grid_width as u32, grid_height as u32);
        neighbors = neighbors.iter().filter(|coord|{
            !visited[coord.1][coord.0]
        }).filter(|coord|{
            let coord_height = height_map[coord.1][coord.0];
            current_height <= coord_height|| current_height - coord_height == 1
        }).map(|x|*x).collect();
        neighbors.iter().for_each(|coord|{
            visited[y][x] = true;
            dist_map[coord.1][coord.0] = current_dist+1;
            paths.push_back((coord.0, coord.1));
        });
        paths = VecDeque::from(paths.iter().unique().map(|x|*x).collect::<Vec<Coordinate>>());
    }
}
