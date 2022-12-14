use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::FileType;

use crate::io;

#[derive(Debug)]
struct Dir {
    path: String,
    name: String,
    content: Vec<String>,
}

#[derive(Debug)]
struct File {
    path: String,
    name: String,
    size: usize,
}

#[derive(Debug)]
enum ItemType {
    Dir(Dir),
    File(File),
}

pub(crate) fn aoc_puzzle_07_part_1() -> usize {
    let data = io::load_data_to_str("data/07/input.txt");

    let lines = data.split("\n");

    let mut ls = false;
    let mut pwd = Vec::<&str>::new();

    let mut file_index = HashMap::<String, ItemType>::new();

    let mut root_dir = ItemType::Dir(
        Dir {
            content: vec!(),
            path: "/".to_string(),
            name: "".to_string(),
        }
    );

    file_index.insert("".to_string(), root_dir);

    pwd.push("");
    for l in lines {
        // Command
        if l.len() <= 0 {
            continue;
        }

        if l.starts_with("$") {
            let s = l.split(" ").collect::<Vec<&str>>();
            let command = s[1];

            match command {
                "cd" => {
                    ls = false;
                    let path = s[2];
                    if path.starts_with("/") {
                        pwd.clear();
                    }

                    if path == "/" {
                        pwd.push("");
                    } else {
                        for path_component in path.split("/") {
                            if path_component == ".." {
                                pwd.pop();
                            } else {
                                pwd.push(path_component);
                            }
                        }
                    }

                    let new_current_path = pwd.join("/");
                    let mut entry = file_index.get(new_current_path.as_str());
                    if let None = entry {
                        let name = pwd.last().unwrap();
                        let new_dir = ItemType::Dir(
                            Dir {
                                path: new_current_path.to_string(),
                                name: name.to_string(),
                                content: vec![],
                            }
                        );
                        file_index.insert(pwd.join("/").to_owned() + "/" + path, new_dir);
                    }
                }
                "ls" => {
                    ls = true;
                    continue;
                }
                &_ => panic!("Expected command cd or ls")
            }
        }

        if ls {
            let current_path = pwd.join("/");

            file_index.get(current_path.as_str())
                .expect(format!("Path {} not found in index!", current_path).as_str());

            if l.starts_with("dir") {
                let new_dir_name = l.split(" ").skip(1).next().unwrap();
                let new_path = current_path.to_string().to_owned() + "/" + new_dir_name;
                let new_dir = ItemType::Dir(Dir {
                    name: new_dir_name.to_string(),
                    path: new_path.to_string().clone(),
                    content: vec![],
                });
                { file_index.insert(new_path.to_string().clone(), new_dir); }
                if let Some(ItemType::Dir(mut_dir)) = file_index.get_mut(current_path.as_str()) {
                    mut_dir.content.push(new_path.to_string().clone());
                }
            } else {
                let split = l.split(" ").collect::<Vec<&str>>();
                let size = split[0].parse::<usize>().unwrap();
                let name = split[1];
                let new_path = current_path.to_string().to_owned() + "/" + name;

                let file = ItemType::File(File {
                    name: name.to_string(),
                    size,
                    path: new_path.to_string().clone(),
                });

                { file_index.insert(new_path.to_string().clone(), file); }
                if let Some(ItemType::Dir(mut_dir)) = file_index.get_mut(current_path.as_str()) {
                    mut_dir.content.push(new_path.to_string().clone());
                }
            }
        }
    }

    let mut sizes: Vec<usize> = Vec::<usize>::new();
    let total_allocated = collect_size(&file_index, "".to_string(), &mut sizes);

    // Swap with solution_part1(..) to get solution for part 1
    solution_part2(&mut sizes, total_allocated)
}

fn solution_part1(sizes: &mut Vec<usize>) -> usize {
    let mut total_size: usize = 0;
    let limit: usize = 100000;
    for s in sizes {
        if *s <= limit {
            total_size += *s;
        }
    }
    total_size
}

fn solution_part2(sizes: &mut Vec<usize>, total_allocated: usize) -> usize {
    let total_disk_size: usize = 70000000;
    let free_disk_space_needed: usize = 30000000;

    let free_disk_space = total_disk_size - total_allocated;
    let disk_space_to_free = free_disk_space_needed - free_disk_space;

    let mut min: usize = usize::MAX;
    for s in sizes {
        if *s < min && *s >= disk_space_to_free {
            min = *s
        }
    }
    min
}

fn collect_size(file_index: &HashMap<String, ItemType>, path: String, sizes: &mut Vec<usize>) -> usize {
    let mut size: usize = 0;
    if let ItemType::Dir(dir) = file_index.get(path.as_str()).expect(format!("Expected path to be in file index: {}", path).as_str()) {
        for dir_file in &dir.content {
            match file_index.get(dir_file.as_str()) {
                Some(ItemType::Dir(dir)) => {
                    size += collect_size(file_index, dir.path.clone(), sizes);
                }
                Some(ItemType::File(file)) => {
                    size += file.size
                }
                _ => panic!("Expected command file or directory, got something else instead.")
            }
        }
        { sizes.push(size); }
    }
    size
}

