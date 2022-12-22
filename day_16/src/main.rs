#[macro_use] extern crate scan_fmt;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use rayon::prelude::*;

type ValveKey = &'static str;
type Path = Vec<ValveKey>;
type ValveMap = HashMap<ValveKey, Valve>;
type ValveDistMap = HashMap<String, u32>;
type PathHistory = HashMap<ValveKey, i32>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve{
    flow_rate: i32,
    connected_valves: Vec<ValveKey>
}

fn main() {
    let now = std::time::Instant::now();
    let contents = include_str!("../files/input.txt");
    //let contents = include_str!("../files/input2.txt");
    //let contents = include_str!("../files/input3.txt");
    //let contents = include_str!("../files/input4.txt");
    //let contents = include_str!("../files/sample.txt");
    let lines = contents
        .trim()
        .split("\n")
        .map(|x|x.trim());
    let mut valves: ValveMap = HashMap::new();
    lines.for_each(|line|{
        let mut splits = line.split(" ");
        let name = splits.nth(1).unwrap();
        splits.next();
        splits.next();
        let flow_rate = scan_fmt!(splits.next().unwrap(), "rate={};", i32).unwrap();
        let mut connected_valves: Vec<ValveKey> = Vec::new();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        //9th and onward are connected valves
        while let Some(name) = splits.next(){
            connected_valves.push(&name[0..=1]);
        }
        valves.insert(name, Valve{
            flow_rate,
            connected_valves
        });
    });

    let tstamp_parse = now.elapsed();
    let now1 = std::time::Instant::now();
    part_1(&valves);
    let tstamp_pt1 = now1.elapsed();
    let now2 = std::time::Instant::now();
    part_2(&valves);
    let tstamp_pt2 = now2.elapsed();

    println!("        Parse time: {:?}", tstamp_parse);
    println!("        Execution time 1: {:?}", tstamp_pt1);
    println!("        Execution time 2: {:?}", tstamp_pt2);
    println!("        Execution time total: {:?}", now.elapsed());
}

fn part_1(valves: &ValveMap){
    let time_limit:i32 = 30;
    let mut all_path_keys: Vec<ValveKey>= valves
                                            .iter()
                                            .filter(|(_k,v)|v.flow_rate!=0)
                                            .map(|(k, _v)|*k)
                                            .collect();
    all_path_keys.push("AA");
    let valve_distances: ValveDistMap = all_path_keys
                                            .iter()
                                            .permutations(2)
                                            .map(|pair| {
                                                ((pair[0].to_string()+","+pair[1]).to_string(), dist_from(pair[0], pair[1],&valves) as u32)
                                            })
                                            .collect();
    //println!("Valve distances {:?}", valve_distances);

    let non_zero_valve_keys: Vec<ValveKey> = valves
                                                .iter()
                                                .filter(|(_k, v)|v.flow_rate > 0)
                                                .map(|(k,_v)|*k)
                                                .collect();

    //try building paths up from the bottom
    let non_zero_key_count = non_zero_valve_keys.len();
    let mut cost_cache: HashMap<Path, u32> = HashMap::new();
    non_zero_valve_keys.iter().for_each(|key|{
        let path_cost = path_cost_with_cache(vec![key], &valve_distances, &cost_cache);
        if path_cost < time_limit as u32{
            cost_cache.insert(vec![key], path_cost);
        }
    });
    for i in 1..non_zero_key_count{
        let cost_cache_copy = cost_cache.clone();
        let valid_paths = cost_cache_copy.keys().filter(|path|path.len()==i).collect::<Vec<_>>();
        valid_paths.iter().for_each(|path|{
            non_zero_valve_keys
                .iter()
                .filter(|key|path.iter().all(|x|&x!=key))
                .for_each(|key|{
                    let mut new_path = Vec::from(path.as_slice());
                    new_path.push(key);
                    let path_cost = path_cost_with_cache(new_path.to_vec(), &valve_distances, &cost_cache);
                    if path_cost < time_limit as u32 {
                        cost_cache.insert(new_path.to_vec(), path_cost);
                    }
                });
        })
    }
    let valid_permutations = cost_cache.keys().collect::<Vec<_>>();
    println!("Pt1: {} unique paths", valid_permutations.len());
    let max_score = valid_permutations
        .into_par_iter()
        .map(|perm|{
            permutation_to_score(&(perm.iter().map(|x|*x).collect()), &valves, &valve_distances, time_limit)
        })
        .max()
        .unwrap();

    //let best_permutation = cost_cache.keys().find(|p|permutation_to_score(&(p.iter().map(|x|*x).collect()), &valves, &valve_distances, time_limit) == max_score).unwrap();
    //println!("best permutation {:?}", best_permutation);

    println!("Answer 1: {}", max_score);
}

fn part_2(valves: &ValveMap){
    let time_limit:i32 = 26;
    //create a list of all valves that are either the start node or a nonzero flow rate
    let mut all_path_nodes: Vec<ValveKey>= valves
                                            .iter()
                                            .filter(|(_k,v)|v.flow_rate > 0)
                                            .map(|(k, _v)|*k)
                                            .collect();
    all_path_nodes.push("AA");

    //create a map of all possible travel distances from path nodes
    let valve_distances: ValveDistMap = all_path_nodes
        .iter()
        .permutations(2)
        .map(|pair| {
            let new_key = (pair[0].to_string()+","+pair[1]).to_string();
            (new_key, dist_from(pair[0], pair[1], &valves) as u32)
        })
        .collect();

    //println!("valve distances {:?}", valve_distances);
    //println!("Valve distances {:?}", valve_distances);

    //create a map of all nodes with a nonzero flow rate (Excludes the entrance: AA)
    let non_zero_valve_keys: Vec<ValveKey> = valves
                                                .iter()
                                                .filter(|(_k, v)|v.flow_rate > 0)
                                                .map(|(k,_v)|*k)
                                                .collect();
    let non_zero_key_count = non_zero_valve_keys.len();

    //Build paths from the bottom up, starting with single node paths
    let mut cost_cache: HashMap<Path, u32> = HashMap::new();
    //populate initial target destinations
    non_zero_valve_keys.iter().for_each(|key|{
        let path_cost = path_cost_with_cache(vec![key], &valve_distances, &cost_cache);
        if path_cost < time_limit as u32{
            cost_cache.insert(vec![key], path_cost);
        }
    });
    //for each destination in the chain, add each other destination as a possible path
    for i in 1..non_zero_key_count{
        let cost_cache_copy = cost_cache.clone(); //create a clone to prevent multi-borrowing
        let valid_paths = cost_cache_copy
                            .keys()
                            .filter(|path|path.len()==i)
                            .collect::<Vec<_>>(); //find all "full" paths, excluding any that terminated in an earlier iteration of the loop
        valid_paths.iter().for_each(|path|{
            non_zero_valve_keys
                .iter()
                .filter(|key|path.iter().all(|x|&x!=key))
                .for_each(|key|{
                    //if this path contains the current key, skip it
                    let mut new_path = Vec::from(path.as_slice()); //create a new path vector that we will insert
                    new_path.push(key); //add the key to the path
                    let path_cost = path_cost_with_cache(new_path.to_vec(), &valve_distances, &cost_cache); //get the total cost of the path
                    if path_cost < time_limit as u32{
                        //if it's over 26, we can't hit all those nodes
                        //if it's equal to 26, the last valve is turned on on the last day and doesn't contribute to the score
                        //otherwise save the path into our costs object
                        cost_cache.insert(new_path.to_vec(), path_cost);
                    }
                });
        })
    }
    let valid_permutations: Vec<&Path> = cost_cache.keys().collect(); //keys of this object are the paths
    println!("Pt2: {} unique paths", valid_permutations.len());
    let path_pairs: Vec<Vec<&&Path>> = valid_permutations
                                        .iter()
                                        .permutations(2)//FIXME this line here is the cause of all the slowdowns in the program
                                        .filter(|pair|{
                                            //if the pair of paths contains duplicate paths, it cannot be correct, so exclude it
                                            let mut set_of_valves = HashSet::new();
                                            pair[0].iter().for_each(|name|{set_of_valves.insert(name);});
                                            pair[1].iter().for_each(|name|{set_of_valves.insert(name);});
                                            set_of_valves.len() == (pair[0].len() + pair[1].len())
                                        })
                                        .collect();
    println!("Pt2: {} unique permutations with elephant", path_pairs.len());
    println!("now for the hard part...again");
    let answer_2 = path_pairs
                    .into_par_iter()
                    .map(|pair| permutation_to_score(pair[0], &valves, &valve_distances, time_limit) + permutation_to_score(pair[1], &valves, &valve_distances, time_limit)) //merge the path histories and calculate scores
                    .max()
                    .unwrap();

    println!("Answer 2: {}", answer_2);
}

fn dist_from(start: ValveKey, end: ValveKey, valves: &ValveMap) -> i32{
    dist_from_iterative(start, end, valves)
}

fn dist_from_iterative(start: ValveKey, end: ValveKey, valves: &ValveMap) -> i32{
    let start_valve = valves.get(start).unwrap();
    if start == end{
        return 0;
    }
    //this check isn't necessary, but it speeds up the recursion a bit
    if start_valve.connected_valves.contains(&end){
        return 1;
    }

    let mut process_queue:VecDeque<(ValveKey, i32)> = VecDeque::new();
    let mut visited: HashSet<ValveKey> = HashSet::new();
    process_queue.push_back((start, 0));
    while let Some((current_node, depth)) = process_queue.pop_front(){
        if current_node == end{
            return depth;
        }else if valves.get(current_node).unwrap().connected_valves.contains(&end){
            //not technically needed, but should speed up recursion
            return depth+1;
        }else{
            visited.insert(current_node);
            valves.get(current_node).unwrap().connected_valves.iter().for_each(|name|{
                if !visited.contains(name){
                    process_queue.push_back((name, 1+depth));
                }
            });
        }
    }
    return 99999;
}

fn path_cost_with_cache(mut path: Path, valve_distances: &ValveDistMap, cache: &HashMap<Vec<ValveKey>, u32>) -> u32{
    let path_length = path.len();
    if path_length == 1{
        //return 1 + distance from AA to the only path node
        return valve_distances
                .get(format!("AA,{}", path[0]).as_str())
                .unwrap() + 1;
    }
    if cache.contains_key(&path[0..path.len()-1]){
        //cache hit
        //return 1 + previous path cost + distance to last valve
        1 + cache.get(&path[0..path.len()-1]).unwrap() + valve_distances.get(format!("{},{}", path[path_length -2], path[path_length -1]).as_str()).unwrap()
    } else {
        //cache miss, shouldn't be possible
        //if we get here somehow, append AA to the front of path and get distances between all path pairs
        //sum them up and return result
        let mut full_path = vec!["AA"];
        full_path.append(&mut path);
        let path_segments = full_path.windows(2);
        path_segments.map(|path| {
            valve_distances.get(format!("{},{}", path[0], path[1]).as_str()).unwrap() + 1
        }).sum()
    }
}

fn path_value(history: &PathHistory, valves: &ValveMap) ->i32{
    history
        .iter()
        .map(|(key, turn)|{
            valves.get(key).unwrap().flow_rate * turn
        })
        .sum()
}

fn path_to_path_history(key_list: Path, valve_distances: &ValveDistMap, time_limit: i32) -> PathHistory{
    let mut current_position = "AA";
    let mut current_turn = time_limit;
    let mut to_return = HashMap::new();
    let mut iter_keys = key_list.iter();
    while let Some(key) = iter_keys.next(){
        let turn_cost = *(valve_distances.get(format!("{},{}", current_position, key).as_str()).unwrap()) as i32 + 1;
        if turn_cost > current_turn{
            //can't get there and open the valve in time, break and return what we have
            break;
        }
        to_return.insert(*key, current_turn - turn_cost);
        current_turn -= turn_cost;
        current_position = key;
    }
    to_return
}

fn permutation_to_score(permutation: &Path, valves: &ValveMap, valve_distances: &ValveDistMap, time_limit: i32) -> i32{
    let history: HashMap<ValveKey, i32>  = path_to_path_history(permutation.iter().map(|x|*x).collect(), valve_distances, time_limit);
    let score = path_value(&history, &valves);
    score
}
