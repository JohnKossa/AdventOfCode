use std::fs;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn scenic_score(grid: &Vec<Vec<u32>>, grid_width: usize, grid_height:usize, x: usize, y: usize) -> u32{
    let score_l = trees_visible_left(grid, grid_width, grid_height, x, y);
    let score_r = trees_visible_right(grid, grid_width, grid_height, x, y);
    let score_u = trees_visible_up(grid, grid_width, grid_height, x, y);
    let score_d = trees_visible_down(grid, grid_width, grid_height, x, y);
    score_l * score_r * score_u * score_d
}

fn is_visible(grid: &Vec<Vec<u32>>, grid_width: usize, grid_height:usize, y: usize, x: usize) -> bool{
    let my_height = grid[x][y];

    if x == 0 || y==0 || x==grid_width-1 || y==grid_height-1{
        //println!("edge tree {},{}", x, y);
        return true;
    }
    //print!("{},{} {}    ", x, y, my_height);
    let mut all_left_lower = true;
    for i in 0..x {
        if grid[i][y] >= my_height{
            all_left_lower = false;
            //print!("left {},{} {}   ",i, y, grid[i][y]);
            break;
        }
    }

    let mut all_right_lower = true;
    for i in (x+1)..grid_width{
        if grid[i][y] >= my_height{
            all_right_lower = false;
            //print!("right {},{} {}   ",i, y, grid[i][y]);
            break;
        }
    }
    let mut all_above_lower = true;
    for i in 0..y {
        if grid[x][i] >= my_height{
            all_above_lower = false;
            //print!("up {},{} {}   ",x, i, grid[x][i]);
            break;
        }
    }
    let mut all_below_lower = true;
    for i in (y+1)..grid_height{
        if grid[x][i] >= my_height{
            all_below_lower = false;
            //print!("down {},{} {}   ",x, i, grid[x][i]);
            break;
        }
    }
    /*if all_above_lower || all_below_lower || all_left_lower || all_right_lower{
        println!("Interior visible tree at {},{}", x, y);
    }else{
        println!("Hidden tree at {},{}", x, y);
    }*/
    all_above_lower || all_below_lower || all_left_lower || all_right_lower
}

fn trees_visible_left(grid: &Vec<Vec<u32>>, _grid_width: usize, _grid_height:usize, x: usize, y: usize) -> u32{
    if x == 0{
        return 0;
    }
    if x == 1{
        return 1;
    }
    let mut ret_val= 0;
    let my_height = grid[y][x];
    for i in (0..x).rev(){
        if grid[y][i] < my_height{
            ret_val+=1;
        }else if grid[y][i] >= my_height{
            ret_val+=1;
            break;
        }
    }
    ret_val
}
fn trees_visible_right(grid: &Vec<Vec<u32>>, grid_width: usize, _grid_height:usize, x: usize, y: usize) -> u32 {
    if x == grid_width - 1 {
        return 0;
    }
    if x == grid_width - 2 {
        return 1;
    }
    let mut ret_val= 0;
    let my_height = grid[y][x];
    for i in (x+1)..grid_width {
        if grid[y][i] < my_height{
            ret_val+=1;
        }else if grid[y][i] >= my_height{
            ret_val+=1;
            break;
        }
    }
    ret_val
}
fn trees_visible_up(grid: &Vec<Vec<u32>>, _grid_width: usize, _grid_height:usize, x: usize, y: usize) -> u32{
    if y == 0{
        return 0;
    }
    if y == 1{
        return 1;
    }
    let mut ret_val= 0;
    let my_height = grid[y][x];
    for i in (0..y).rev(){
        if grid[i][x] < my_height{
            ret_val+=1;
        }else if grid[i][x] >= my_height{
            ret_val+=1;
            break;
        }
    }
    ret_val
}

fn trees_visible_down(grid: &Vec<Vec<u32>>, _grid_width: usize, grid_height:usize, x: usize, y: usize) -> u32{
    if y == grid_height-1{
        return 0;
    }
    if y == grid_height-2{
        return 1;
    }
    let mut ret_val= 0;
    let my_height = grid[y][x];
    for i in (y+1)..grid_height{
        if grid[i][x] < my_height{
            ret_val+=1;
        }else if grid[i][x] >= my_height{
            ret_val+=1;
            break;
        }
    }
    ret_val
}


fn part_1(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let mut lines = contents.trim().split("\n");
    let grid_height = contents.trim().split("\n").count();
    let grid_width  = contents.trim().split("\n").next().unwrap().trim().chars().count();
    let mut tree_grid: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next(){
        tree_grid.push(Vec::from(line.trim().chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>()));
    }
    let mut visible_mask: Vec<Vec<bool>> = vec![vec![true; grid_height]; grid_width];
    for y in 0..grid_height {
        for x in 0..grid_width {
            if !is_visible(&tree_grid, grid_width, grid_height, x, y){
                visible_mask[x][y] = false;
            }
        }
    }
    //count visible trees
    let mut visible_count = 0;
    for y in 0..grid_height {
        for x in 0..grid_height {
            if visible_mask[x][y]{
                visible_count+=1;
            }
        }
    }
    println!("Result for part 1: {}", visible_count);
}

fn part_2(){
    let contents = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let mut lines = contents.trim().split("\n");
    let grid_height = contents.trim().split("\n").count();
    let grid_width  = contents.trim().split("\n").next().unwrap().trim().chars().count();
    let mut tree_grid: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next(){
        tree_grid.push(Vec::from(line.trim().chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>()));
    }
    let mut score_grid: Vec<Vec<u32>> = vec![vec![0; grid_height]; grid_width];
    for y in 0..grid_height {
        for x in 0..grid_width {
            score_grid[y][x] = scenic_score(&tree_grid, grid_width, grid_height, x, y);
        }
    }
    let mut max_score = 0;
    for x in 0..grid_width{
        for y in 0..grid_width{
            if score_grid[y][x] > max_score{
                max_score = score_grid[y][x]
            }
        }
    }
    println!("Result for part 2: {}", max_score);
}
