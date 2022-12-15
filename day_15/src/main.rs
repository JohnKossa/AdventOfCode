#[macro_use] extern crate scan_fmt;
extern crate fxhash;
use fxhash::FxHashSet;

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
    part_1(&sensor_beacon_pairs, &sensor_clear_distances);
    part_2(&sensor_clear_distances);
    println!("Execution time: {:?}", now.elapsed());
}

type Coordinate = (i32, i32);
type Square = (Coordinate, Coordinate);

fn part_1(sensor_beacon_pairs: &Vec<(Coordinate, Coordinate)>, sensor_clear_distances: &Vec<(Coordinate, i32)>){
    let min_beacon_x = sensor_clear_distances
        .iter()
        .map(|((x,_),d)|x-d)
        .min()
        .unwrap();
    let max_beacon_x = sensor_clear_distances
        .iter()
        .map(|((x,_),d)|x+d)
        .max()
        .unwrap();
    let y = 2000000;
    let mut clear_locations: FxHashSet<Coordinate> = FxHashSet::default();
    for x in min_beacon_x..=max_beacon_x{
        //if distance to any beacon is less than that beacon's range, spot is clear
        if sensor_clear_distances
            .iter()
            .map(|((sx, sy), d)| (sx-x).abs() + (sy-y).abs() <= *d).any(|x|x){
                clear_locations.insert((x, y));
        }
    }
    let beacon_locations: Vec<Coordinate> = sensor_beacon_pairs
        .iter()
        .filter(|(_,(_,by))|*by==y)
        .map(|(_, beacon)|*beacon)
        .collect();
    beacon_locations
        .iter()
        .for_each(|pair|{
            clear_locations.remove(pair);
        });
    println!("Answer for part 1 {}", clear_locations.iter().count());
}

fn square_can_contain_unseen(((x0, y0), (x1,y1)): Square, sensor:Coordinate, sensor_dist: i32) -> bool{
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
                    let score :i64 = 4000000 * (min.0 as i64) + (min.1 as i64);
                    println!("Answer for part 2: {}", score);
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
                if sensor_clear_distances.iter().all(|(pos, d)| square_can_contain_unseen(*square, *pos, *d)){
                    //square is possible, push it to search grid
                    search_squares.push(*square)
                }
            }
        }
    }
    println!("No result found!");
}
