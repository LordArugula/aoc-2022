use std::fs;

pub struct Dir {
    pub parent: usize,
    pub subdirs: Vec<usize>,
    pub files: Vec<File>,
    pub name: String,
}

pub struct File {
    pub size: usize,
    pub name: String,
}

impl Dir {
    pub fn new(name: &str, parent: usize) -> Self {
        Self {
            parent,
            subdirs: Vec::new(),
            files: Vec::new(),
            name: name.to_owned(),
        }
    }

    pub fn total_size(&self, tree: &Vec<Dir>) -> usize {
        let total_file_size = self.files.iter().map(|file| file.size).sum::<usize>();
        let total_subdir_size = self
            .subdirs
            .iter()
            .map(|index| match tree.get(*index) {
                Some(dir) => dir.total_size(tree),
                None => 0,
            })
            .sum::<usize>();
        total_file_size + total_subdir_size
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file.");

    let tree = build_dir_tree(input);
    //part_one(&tree);
    part_two(&tree);
}

fn part_one(tree: &Vec<Dir>) {
    let sum_dirs_less_100000 = tree
        .iter()
        .map(|dir| dir.total_size(&tree))
        .filter(|size| *size <= 100000)
        .sum::<usize>();
    println!("{}", sum_dirs_less_100000);
}

fn part_two(tree: &Vec<Dir>) {
    let maximum_size = 70000000;
    let update_size = 30000000;

    let used_size = match tree.get(0) {
        Some(dir) => dir.total_size(tree),
        None => 0,
    };

    let unused_space = maximum_size - used_size;
    let needed_space = update_size - unused_space;
    // want to delete the smallest directory that would
    // allow us to have enough space
    let deletion_candidate_dir_size = tree
        .iter()
        .map(|dir| dir.total_size(tree))
        .filter(|&size| size > needed_space)
        .min()
        .unwrap();

    println!("{}", deletion_candidate_dir_size);
}

fn build_dir_tree<'a>(input: String) -> Vec<Dir> {
    let mut dir_tree = Vec::<Dir>::new();

    let root_dir = Dir::new("/", !0);
    dir_tree.push(root_dir);

    let root_index = 0;
    let mut pwd_index = root_index;

    for line in input.lines() {
        if line.starts_with('$') {
            // commands
            let args = line.split(' ').skip(1).collect::<Vec<&str>>();
            match args[0] {
                "cd" => {
                    let dir_name = args[1];
                    match dir_name {
                        "/" => pwd_index = root_index,
                        ".." => {
                            if pwd_index > 0 {
                                match dir_tree.get(pwd_index) {
                                    Some(dir) => pwd_index = dir.parent,
                                    None => println!("Could not find parent index"),
                                }
                            } else {
                                println!("{}", pwd_index);
                            }
                        }
                        _ => match dir_tree
                            .iter()
                            .position(|dir| dir.parent == pwd_index && dir.name == dir_name)
                        {
                            Some(index) => pwd_index = index,
                            None => println!("Could not find directory!"),
                        },
                    }
                }
                "ls" => {}
                _ => println!("Unexpected command"),
            };
        } else {
            // output
            let output = line.split(' ').collect::<Vec<&str>>();
            if output[0] == "dir" {
                // dir
                let subdir = Dir::new(output[1], pwd_index);
                let subdir_index = dir_tree.len();

                let dir = dir_tree
                    .get_mut(pwd_index)
                    .expect("Should be a directory at this index!");

                dir.subdirs.push(subdir_index);
                dir_tree.push(subdir);
            } else {
                // file
                let file_size = output[0].parse::<usize>().unwrap();
                let file_name = output[1];
                let dir = dir_tree
                    .get_mut(pwd_index)
                    .expect("Should be a directory at this index!");

                dir.files.push(File {
                    size: file_size,
                    name: file_name.to_owned(),
                });
            }
        }
    }
    dir_tree
}
