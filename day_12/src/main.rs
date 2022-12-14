use std::collections::VecDeque;

fn main() {
	let now = std::time::Instant::now();
	
	let mut lines = include_str!("../files/input.txt")
		.trim()
		.split("\n");
	let mut height_map:Vec<Vec<u32>> = Vec::new();
	let mut start:(usize, usize) = (0,0);
	let mut end:(usize, usize) = (0,0);
	let mut row_num: usize = 0;
	while let Some(line) = lines.next(){
		height_map.push(line
			.trim()
			.chars()
			.enumerate()
			.map(|(idx, c)|{
				match (idx, c){
					(x, 'S') => {
						start = (x, row_num);
						1
					},
					(x, 'E')=>{
						end = (x, row_num);
						26
					},
					(_,a)=>  (a as u32)-96
				}
			})
			.collect::<Vec<u32>>());
		row_num+=1;
	}
	part_1(&height_map, start, end);
	part_2(&height_map, end);
	println!("Execution time: {:?}", now.elapsed());
}

type Coordinate=(usize, usize);

fn all_visited(visit_grid: &Vec<Vec<bool>>) -> bool{
	!visit_grid.iter().flatten().any(|&x|!x)
}

fn neighbors_of(coords: Coordinate, width: usize, height: usize) -> Vec<Coordinate>{
	[(-1,0), (1,0), (0,-1), (0,1)]
		.iter()
		.map(|(x, y)|(coords.0 as i32 +x, coords.1 as i32+y))
		.filter(|(x, y)|x >= &0 && x < &(width as i32) && y >= &0 && y < &(height as i32))
		.map(|(a, b)|(a as usize, b as usize))
		.collect()
}

fn part_1(height_map: &Vec<Vec<u32>>, start: Coordinate, end: Coordinate){
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
		if (x,y) == end {
			println!("Answer for part 1: {}", current_dist);
			break;
		}
		let neighbors: Vec<Coordinate> = neighbors_of((x, y), grid_width, grid_height)
			.iter()
			.filter(|coord|!visited[coord.1][coord.0])
			.filter(|coord|{
				let coord_height = height_map[coord.1][coord.0];
				coord_height <= current_height || coord_height-current_height == 1
			})
			.map(|x|*x)
			.collect();
		neighbors
			.iter()
			.for_each(|coord|{
				if !visited[coord.1][coord.0]{
					visited[coord.1][coord.0] = true;
					dist_map[coord.1][coord.0] = current_dist+1;
					paths.push_back((coord.0, coord.1));
				}
			});
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
		let neighbors: Vec<Coordinate> = neighbors_of((x, y), grid_width, grid_height)
			.iter()
			.filter(|coord|{
				!visited[coord.1][coord.0]
			})
			.filter(|coord|{
				let coord_height = height_map[coord.1][coord.0];
				current_height <= coord_height || current_height - coord_height == 1
			})
			.map(|x|*x).collect();
		neighbors
			.iter()
			.for_each(|coord|{
				if !visited[coord.1][coord.0]{
					visited[coord.1][coord.0] = true;
					dist_map[coord.1][coord.0] = current_dist+1;
					paths.push_back((coord.0, coord.1));
				}
			});
	}
}
