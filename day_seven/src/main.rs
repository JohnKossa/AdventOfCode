use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let now = std::time::Instant::now();
    part_1();
    part_2();
    println!("Execution time: {:?}", now.elapsed());
}


fn directory_size(filesystem: &HashMap<String, usize>, path_string: String) -> usize{
    //search Hashmap for all entries containing path_string
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
                directories.push(format!("{}{}{}", &current_path.join("/"),"/".to_owned(), location ).to_string());
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
    //println!("{} files\n", files.len());
    for k in files.keys().sorted(){
        //println!("{:?} {:?}", files[k], k)
    }
    //println!("\n\n\n");
    //println!("{}\n\n\n", files.iter().map(|(name, size)|format!("{} {}", size, name)).join("\n"));
    directories.sort();
    //println!("directories {}\n\n\n", directories.iter().join("\n"));

    //println!("Directories unique? {} count {}\n\n\n", directories.iter().all_unique(), directories.len());
    //println!("Files unique? {} count {}\n\n\n", files.keys().all_unique(), files.keys().len());
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    directories.iter().for_each(|dirname|{
        dir_sizes.insert(dirname.to_string(), directory_size(&files, dirname.to_string()));
    });
    //println!("{} Dir sizes \n", dir_sizes.len());
    //println!("{}\n\n\n", dir_sizes.iter().map(|(name, size)|format!("{}   {}", size, name)).join("\n"));
    println!("{} Dir sizes\n", dir_sizes.len());
    for k in dir_sizes.keys().sorted(){
        println!("{:?} {:?}", k, dir_sizes[k])
    }
    //println!("\n\n\n");
    let total_pt1: usize = dir_sizes.iter().filter(|(_, size)|{
        return **size <= 100000;
    }).map(|(_, size)|size).sum();
    println!("Total for part 1 is {}", total_pt1);

    let summed = dir_sizes.iter().filter(|(_, size)|{
        return **size <= 100000;
    });
    let summed_count = dir_sizes.iter().filter(|(_, size)|{
        return **size <= 100000;
    }).count();
    //println!("{} Summed up\n", summed_count);
    //println!("{}\n\n\n", summed.map(|(name, size)|format!("{} {}", size, name)).join("\n"));



    //println!("File size for / is {}", directory_size(&files, "/".to_owned()));
    //println!("File size for /cmvqf/wcn/cqdzdnq/wqvv/rllbccjt/lnfvlqh/lvgzb {}", directory_size(&files, "/cmvqf/wcn/cqdzdnq/wqvv/rllbccjt/lnfvlqh/lvgzb".to_owned()));
    //println!("File size for /cmvqf/wcn/cqdzdnq/wqvv/rllbccjt/lnfvlqh is {}", directory_size(&files, "/cmvqf/wcn/cqdzdnq/wqvv/rllbccjt/lnfvlqh".to_owned()));
}

fn part_2(){
    let input = fs::read_to_string("files/input.txt").expect("Should have been able to read the file");
    for last_idx in 14..input.chars().count(){
        if input[last_idx-14..last_idx].chars().all_unique(){
            //println!("Last index for pt2 is: {}", last_idx+1);
            break;
        }
    }
}
