use std::fs;
use std::collections::HashMap;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}

fn directory_size(filesystem: &HashMap<String, usize>, path_string: String) -> usize{
    //search Hashmap for all entries containing the path_string
    filesystem.iter().filter(|(path, _)|{
        path.starts_with(&path_string)
    }).map(|(_, size)|size).sum()
}

fn part_1(){
    let input = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let mut lines = input.trim().split("\n");
    lines.next();//discard $ cd /
    let mut files: HashMap<String, usize> = HashMap::new();
    let mut directories: Vec<String> = Vec::new();
    let mut current_path: Vec<String> = Vec::new();
    current_path.push("".to_owned());
    directories.push("".to_owned());
    while let Some(current_line) = lines.next(){
        if current_line.starts_with("$ cd"){
            let location = current_line.split(' ').nth(2).unwrap().trim();
            if location == ".."{
                if current_path.len() < 2{
                    panic!("Asked to pop last element!")
                }
                current_path.pop();
            }else{
                //println!("{}", location);
                directories.push(format!("{}{}{}{}", &current_path.join("/"),"/".to_owned(), location , "/".to_owned()).to_string());
                current_path.push(location.to_string());
            }
        }else if current_line.starts_with("$ ls"){
            ();//shouldn't need to do anything here
        }
        else{
            let mut split_line = current_line.trim().split(' ');
            match (split_line.next(), split_line.next()) {
                (Some("dir"), _) => (), //these should already be captured by the cd commands above
                (Some(size), Some(name)) => {
                    files.insert(format!("{}{}{}", &current_path.join("/"),"/".to_owned(), name ), size.parse::<usize>().unwrap());
                },
                _ => unreachable!()
            }
        }
    }
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    directories.iter().for_each(|dirname|{
        dir_sizes.insert(dirname.to_string(), directory_size(&files, dirname.to_string()));
    });
    let total_pt1: usize = dir_sizes.iter().filter(|(_, size)|{
        return **size <= 100000;
    }).map(|(_, size)|size).sum();

    println!("Total for part 1 is {}", total_pt1);
}

fn part_2(){
    let input = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    let mut lines = input.trim().split("\n");
    lines.next();//discard $ cd /
    let mut files: HashMap<String, usize> = HashMap::new();
    let mut directories: Vec<String> = Vec::new();
    let mut current_path: Vec<String> = Vec::new();
    current_path.push("".to_owned());
    directories.push("".to_owned());
    while let Some(current_line) = lines.next(){
        if current_line.starts_with("$ cd"){
            let location = current_line.split(' ').nth(2).unwrap().trim();
            if location == ".."{
                if current_path.len() < 2{
                    panic!("Asked to pop last element!")
                }
                current_path.pop();
            }else{
                //println!("{}", location);
                directories.push(format!("{}{}{}{}", &current_path.join("/"),"/".to_owned(), location , "/".to_owned()).to_string());
                current_path.push(location.to_string());
            }
        }else if current_line.starts_with("$ ls"){
            ();//shouldn't need to do anything here
        }
        else{
            let mut split_line = current_line.trim().split(' ');
            match (split_line.next(), split_line.next()) {
                (Some("dir"), _) => (), //these should already be captured by the cd commands above
                (Some(size), Some(name)) => {
                    let file_path = format!("{}{}{}", &current_path.join("/"),"/".to_owned(), name );
                    files.insert(file_path, size.parse::<usize>().unwrap());
                },
                _ => unreachable!()
            }
        }
    }
    let total_fs_size = 70000000;
    let used_size = directory_size(&files, "/".to_owned());
    let remaining_size = total_fs_size - used_size;
    let target = 30000000;
    let to_free = target - remaining_size;
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    directories.iter().for_each(|dirname|{
        dir_sizes.insert(dirname.to_string(), directory_size(&files, dirname.to_string()));
    });
    let viable_delete_files = dir_sizes.iter().filter(|(_, size)|{
        return **size >= to_free;
    });
    let mut smallest_viable_files: Vec<usize> = viable_delete_files.map(|(_, size)|*size).collect::<Vec<usize>>();
    smallest_viable_files.sort();
    let smallest_viable_dir = smallest_viable_files[0];
    println!("Total for part 2 is {}", smallest_viable_dir);
}
