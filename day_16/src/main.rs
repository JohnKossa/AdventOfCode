#[macro_use] extern crate scan_fmt;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

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

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve{
    flow_rate: i32,
    connected_valves: Vec<&'static str>
}

fn dist_from(start: &'static str, end: &str, valves: &HashMap<&str, Valve>) -> i32{
    dist_from_recurse(start, end, valves, &mut HashSet::new())
}

fn dist_from_recurse(start: &'static str, end: &str, valves: &HashMap<&str, Valve>, visited: &mut HashSet<&str>) -> i32{
    let start_valve = valves.get(start).unwrap();
    if start == end{
        return 0;
    }
    if start_valve.connected_valves.contains(&end){
        return 1;
    }
    //visited.insert(start);
    let unvisited_children: Vec<&str> = start_valve.connected_valves.iter().filter(|&name|!visited.contains(name)).map(|x|*x).collect();
    return 1 + unvisited_children.iter().map(|name|dist_from_recurse(name, end, valves, visited)).min().unwrap_or(0)
}

//key, opened_on map

fn path_value(key_turn_map: &HashMap<&str, i32>, valves: &HashMap<&str, Valve>)->i32{
    key_turn_map.iter().map(|(key, turn)|valves.get(key).unwrap().flow_rate * turn).sum()
}

fn key_list_to_opening_history(key_list: Vec<&'static str>, valves: &HashMap<&str, Valve>) -> HashMap<&'static str, i32>{
    let mut current_position = "AA";
    let mut current_turn = 30;
    let mut to_return = HashMap::new();
    let mut iter_keys = key_list.iter();
    while let Some(key) = iter_keys.next(){
        let turn_cost = dist_from(current_position, key, valves)+1;
        if turn_cost > current_turn{
            //can't get there and open the valve in time, break
            println!("broke early for {:?}", &key_list);
            break;
        }
        to_return.insert(*key, current_turn - turn_cost);
        current_turn -= turn_cost;
        current_position = key;
    }
    /*for key in &key_list{
        let turn_cost = dist_from(current_position, key, valves)+1;
        if turn_cost > current_turn{
            //can't get there and open the valve in time, break
            println!("broke early for {:?}", &key_list);
            break;
        }
        to_return.insert(*key, 30 - dist_from(current_position, key, valves) - 1);
        current_turn -= dist_from(current_position, key, valves) + 1;
        current_position = key;
    }*/
    to_return
}

/*fn dfs_search(minute: u32, current_location: &str, flow_rate: i32, current_score: u64, open_vlocation_history: Vec<&str>) -> Option<u64>{
    if minute == 0{
        return Some(current_score)
    }
}*/

fn part_1(){
    //let contents = include_str!("../files/input.txt");
    let contents = include_str!("../files/sample.txt");
    let lines = contents
        .trim()
        .split("\n")
        .map(|x|x.trim());
    let mut valves: HashMap<&str, Valve> = HashMap::new();
    lines.for_each(|line|{
        let mut splits = line.split(" ");
        let name = splits.nth(1).unwrap();
        splits.next();
        splits.next();
        let flow_rate = scan_fmt!(splits.next().unwrap(), "rate={};", i32).unwrap();
        let mut connected_valves: Vec<&str> = Vec::new();
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
    println!("Valves {:?}", valves);

    let valve_id_mappings: Vec<(usize, &str)> = valves.keys().enumerate().map(|(i, &k)|(i,k)).collect();
    println!("valve id mappings {:?}", valve_id_mappings);

    let valve_count = valves.len();

    let inf = i32::max_value();
    let dist_aj = dist_from("AA", "JJ", &valves);
    println!("Distance from AA to JJ is  {}", dist_aj);
    /*let mut curr_postition: &str = "AA";
    let mut open_valves: Vec<&str> = Vec::new();
    let mut open_valve_history: HashMap<&str, i32> = HashMap::new();
    let mut time_remaining = 30;
    let all_valves: Vec<&str> = valves.keys().map(|x|*x).collect();
    let mut expected_values_1:Vec<(&str, i32)> = valves.iter().map(|(key, valve)|{
        //expected value is flowrate * (time_remaining-dist-1)
        (*key, valve.flow_rate * (time_remaining-1-dist_from(curr_postition, key, &valves)))
    }).filter(|(_k,v)|*v!=0).collect();
    expected_values_1.sort_by(|(_, v1), (_,v2)| v2.partial_cmp(v1).unwrap());
    println!("Expected values rd1 {:?}", expected_values_1);

    time_remaining -= (1+ dist_from(curr_postition, expected_values_1[0].0, &valves));
    curr_postition = expected_values_1[0].0;
    open_valve_history.insert(expected_values_1[0].0, (1+ dist_from(curr_postition, expected_values_1[0].0, &valves)));
    println!("score after rd1 {}", path_value(&open_valve_history, &valves));
    let mut expected_values_2:Vec<(&str, i32)> = valves.iter().filter(|(k,v)|!open_valve_history.contains_key(*k)).map(|(key, valve)|{
        //expected value is flowrate * (time_remaining-dist-1)
        (*key, valve.flow_rate * (time_remaining-1-dist_from(curr_postition, key, &valves)))
    }).filter(|(_k,v)|*v!=0).collect();
    expected_values_2.sort_by(|(_, v1), (_,v2)| v2.partial_cmp(v1).unwrap());
    println!("Expected values rd2 {:?}", expected_values_2);*/

    let dist_db = dist_from("DD", "BB", &valves);
    println!("Distance from DD to BB is  {}", dist_aj);
    let non_zero_valve_keys: Vec<&str> = valves.iter().filter(|(k, v)|v.flow_rate > 0).map(|(k,_v)|*k).collect();
    let permutations = non_zero_valve_keys.iter().permutations(non_zero_valve_keys.len()).unique();
    let mut histories: Vec<(_,HashMap<&str, i32>)> = permutations.map(|p|{
        (p.iter().map(|x|*x).join(","), key_list_to_opening_history(p.iter().map(|x|**x).collect(), &valves))
    }).collect::<Vec<(_,HashMap<&str, i32>)>>();
    histories.sort_by(|(h1, s1), (h2, s2)| h1.partial_cmp(h2).unwrap());
    let mut scores: Vec<(_, i32)> = histories.iter().map(|(p,h)|(p, path_value(&h, &valves))).collect();
    //scores.sort_by(|(h1, s1), (h2, s2)| s2.partial_cmp(s1).unwrap());
    scores.sort_by(|(h1, s1), (h2, s2)| h1.partial_cmp(h2).unwrap());
    for hist in &histories{
        println!("{:?}", hist);
    }
    let dist_ad = dist_from("AA", "DD", &valves);
    println!("Distance from AA to DD is  {}", dist_ad);
    let dist_db = dist_from("DD", "BB", &valves);
    println!("Distance from DD to BB is  {}", dist_db);
    let dist_bj = dist_from("BB", "JJ", &valves);
    println!("Distance from BB to JJ is  {}", dist_bj);
    let dist_jh = dist_from("JJ", "HH", &valves);
    println!("Distance from JJ to HH is  {}", dist_jh);
    /*for score in &scores{
        println!("{:?}", score);
    }*/
    //println!("scores {:?}", scores);
    let max_score = scores.iter().map(|(p, s)|s).max().unwrap();
    println!("Answer 1: {}", max_score);





    //1 minute per move
    //1 minute per open
    //score = time at open * flow_rate

    //get dist from any to any with matrix multiplication
    //expected value = (time_remaining - time_to_reach)* flow_rate

    //start at aa


    //let time = 30;
    //let score = 0;
    //println!("Answer 1: {}", 1);
}

fn part_2(){
    println!("Answer 2: {}", 2);
}
