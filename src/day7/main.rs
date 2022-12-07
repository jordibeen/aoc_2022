use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum LineType {
    ChangeDirectory,
    List,
    File,
    Directory,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    size: usize,
    parent_id: Option<usize>,
    children_ids: Vec<usize>,
}

fn main() {
    println!("--- Day 7: No Space Left On Device ---");
    let input: String = fs::read_to_string("./src/day7/input.txt").expect("File should exist");

    let tree = parse_input(&input);

    let part1: usize = part1(&tree);
    println!("Part One: {}", part1);
    let part2: usize = part2(&tree);
    println!("Part Two: {}", part2);
}

// Get sum of all directories smaller than 100000
fn part1(tree: &HashMap<usize, Directory>) -> usize {
    let mut total_size: usize = 0;
    tree.iter().for_each(|(_dir_id, dir)| {
        if dir.size < 100000 {
            total_size += dir.size;
        }
    });
    total_size
}

// Return smallest dir size to delete for update
fn part2(tree: &HashMap<usize, Directory>) -> usize {
    let total_diskspace: usize = 70000000;
    let update_diskspace: usize = 30000000;

    // Calc needed diskspace
    let root_dir = tree
        .get(&0_usize)
        .expect("Root directory should always exist");
    let free_diskspace = total_diskspace - root_dir.size;
    let needed_diskspace: usize = update_diskspace - free_diskspace;

    // Create vector with sizes of deletable directories
    let mut deletable_dir_sizes: Vec<usize> = Vec::new();
    tree.iter().for_each(|(_dir_id, dir)| {
        if dir.size > needed_diskspace {
            deletable_dir_sizes.push(dir.size);
        }
    });

    // Return the smallest deletable directory size
    deletable_dir_sizes
        .iter()
        .min()
        .expect("Should have a minimum size")
        .clone()
}

fn parse_input(input: &String) -> HashMap<usize, Directory> {
    // Initialize HashMap to map unique ids to directories
    // Eg. {1: Directory, 2: Directory, 3: Directory, 4: Directory}
    let mut tree: HashMap<usize, Directory> = HashMap::new();

    // Initialize HashMap to map directory paths to the tree id
    // Eg. {"/": 1, "/a": 2, "/a/e": 3, "/d": 4}
    let mut paths: HashMap<String, usize> = HashMap::new();

    let mut curr_path: String = String::from("");
    let mut dir_id: usize = 0;
    for line in input.lines() {
        match parse_line(&line) {
            LineType::ChangeDirectory => {
                let move_to_dir = line.replace("$ cd ", "");
                match move_to_dir.as_str() {
                    "/" => {
                        // Add root directory to tree
                        tree.insert(
                            dir_id,
                            Directory {
                                name: String::from(""),
                                parent_id: None,
                                children_ids: Vec::new(),
                                files: Vec::new(),
                                size: 0,
                            },
                        );
                        paths.insert(String::from(""), dir_id);
                    }
                    ".." => {
                        // Set path to parent directory
                        let cloned_path = curr_path.clone();
                        let mut folders: Vec<&str> = cloned_path.split("/").collect();
                        folders.pop();
                        curr_path = folders.join("/");
                    }
                    _ => {
                        // Set path to specified directory
                        curr_path = [curr_path, move_to_dir].join("/");
                    }
                }
            }
            LineType::Directory => {
                dir_id += 1;

                // Get current directory path
                let new_dir_name = line.replace("dir ", "");
                let new_dir_path = [curr_path.clone(), new_dir_name].join("/");

                // Get parent directory id
                let mut folders: Vec<&str> = new_dir_path.split("/").collect();
                folders.pop();
                let parent_dir_path = folders.join("/");
                let parent_dir_id = paths.get(&parent_dir_path).expect("Should exist");

                // Add new directory to tree
                tree.insert(
                    dir_id,
                    Directory {
                        name: new_dir_path.to_owned(),
                        parent_id: Some(parent_dir_id.to_owned()),
                        children_ids: Vec::new(),
                        files: Vec::new(),
                        size: 0,
                    },
                );
                // Add directory as parent's child
                tree.get_mut(&parent_dir_id)
                    .expect("Should exist in tree")
                    .children_ids
                    .push(dir_id);
                // Save directory path to id mapping
                paths.insert(new_dir_path, dir_id);
            }
            LineType::File => {
                // Get filename and size
                let re_file = Regex::new(r"(?P<size>\d+) (?P<filename>.*)")
                    .unwrap()
                    .captures(&line)
                    .expect("Input file lines should have this format");
                let filename: &str = re_file
                    .name("filename")
                    .expect("File line should have a filename")
                    .as_str();
                let size: usize = re_file
                    .name("size")
                    .expect("File line should have a size")
                    .as_str()
                    .parse()
                    .expect("Should be a number");

                // Get current directory according to current path
                let current_dir_id = paths
                    .get(&curr_path)
                    .expect("An id should exist for this path");
                let current_dir = tree
                    .get_mut(&current_dir_id)
                    .expect("A directory should exist for this id");

                // Add new file to tree
                current_dir.files.push(File {
                    name: filename.to_owned(),
                    size,
                });

                // Add file size to current directory, as well as all parent directories
                let mut dir = current_dir;
                dir.size += &size;
                loop {
                    if let Some(parent_dir) = dir.parent_id {
                        dir = tree
                            .get_mut(&parent_dir)
                            .expect("Parent directory should exist");
                        dir.size += size;
                    } else {
                        break;
                    }
                }
            }
            LineType::List => continue,
        };
    }

    tree
}

fn parse_line(line: &str) -> LineType {
    let line: String = line.to_owned();
    if line.contains("$ cd") {
        LineType::ChangeDirectory
    } else if line.contains("$ ls") {
        LineType::List
    } else if line.contains("dir ") {
        LineType::Directory
    } else {
        LineType::File
    }
}
