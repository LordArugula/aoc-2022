// use priority_queue::PriorityQueue;
use std::{collections::{HashMap, VecDeque}, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the in_1le.");

    let (mut start_x, mut start_y) = (0, 0);
    let (mut end_x, mut end_y) = (0, 0);

    let heightmap = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(column_index, c)| {
                    match c {
                        'a'..='z' => {
                            // heights: a is lowest, z is highest
                            c as i32 - 'a' as i32
                        }
                        'E' => {
                            // best position for signal at height z
                            (end_x, end_y) = (row_index, column_index);
                            'z' as i32 - 'a' as i32
                        }
                        'S' => {
                            // starting position at height a
                            (start_x, start_y) = (row_index, column_index);
                            'a' as i32 - 'a' as i32
                        }
                        _ => -1,
                    }
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // values represent the from position
    part_one((start_x, start_y), (end_x, end_y), &heightmap);
    part_two((end_x, end_y), &heightmap);
}

fn part_one((start_x, start_y): (usize, usize), (end_x, end_y): (usize, usize), heightmap: &Vec<Vec<i32>>) {
    let mut walked = HashMap::new();
    let mut open = VecDeque::new();
    open.push_back((start_x, start_y));
    while let Some((current_x, current_y)) = open.pop_front() {
        if current_x == end_x && current_y == end_y {
            break;
        }

        let height = heightmap[current_x][current_y];

        if current_x > 0
            && heightmap[current_x - 1][current_y] - height <= 1
            && !walked.contains_key(&(current_x - 1, current_y))
        {
            walked.insert((current_x - 1, current_y), (current_x, current_y));
            open.push_back((current_x - 1, current_y));
        }

        if current_x + 1 < heightmap.len()
            && heightmap[current_x + 1][current_y] - height <= 1
            && !walked.contains_key(&(current_x + 1, current_y))
        {
            walked.insert((current_x + 1, current_y), (current_x, current_y));
            open.push_back((current_x + 1, current_y));
        }

        if current_y > 0
            && heightmap[current_x][current_y - 1] - height <= 1
            && !walked.contains_key(&(current_x, current_y - 1))
        {
            walked.insert((current_x, current_y - 1), (current_x, current_y));
            open.push_back((current_x, current_y - 1));
        }

        if current_y + 1 < heightmap[current_x].len()
            && heightmap[current_x][current_y + 1] - height <= 1
            && !walked.contains_key(&(current_x, current_y + 1))
        {
            walked.insert((current_x, current_y + 1), (current_x, current_y));
            open.push_back((current_x, current_y + 1));
        }
    }
    
    let mut path = Vec::new();
    let (mut x, mut y) = (end_x, end_y);
    while x != start_x || y != start_y {
        path.push((x, y));
        (x, y) = walked[&(x, y)];
    }
    println!("{}", path.len());
}

fn part_two((end_x, end_y): (usize, usize), heightmap: &Vec<Vec<i32>>) {
    let mut walked = HashMap::new();
    let mut open = VecDeque::new();
    open.push_back((end_x, end_y));

    let (mut start_x, mut start_y) = (end_x, end_y);
    while let Some((current_x, current_y)) = open.pop_front() {
        let height = heightmap[current_x][current_y];

        if height == 0{
            (start_x, start_y) = (current_x, current_y);
            break ;
        }

        if current_x > 0
            && height - heightmap[current_x - 1][current_y] <= 1
            && !walked.contains_key(&(current_x - 1, current_y))
        {
            walked.insert((current_x - 1, current_y), (current_x, current_y));
            open.push_back((current_x - 1, current_y));
        }

        if current_x + 1 < heightmap.len()
            && height - heightmap[current_x + 1][current_y] <= 1
            && !walked.contains_key(&(current_x + 1, current_y))
        {
            walked.insert((current_x + 1, current_y), (current_x, current_y));
            open.push_back((current_x + 1, current_y));
        }

        if current_y > 0
            && height - heightmap[current_x][current_y - 1] <= 1
            && !walked.contains_key(&(current_x, current_y - 1))
        {
            walked.insert((current_x, current_y - 1), (current_x, current_y));
            open.push_back((current_x, current_y - 1));
        }

        if current_y + 1 < heightmap[current_x].len()
            && height - heightmap[current_x][current_y + 1] <= 1
            && !walked.contains_key(&(current_x, current_y + 1))
        {
            walked.insert((current_x, current_y + 1), (current_x, current_y));
            open.push_back((current_x, current_y + 1));
        }
    }
    
    let mut path = Vec::new();
    let (mut x, mut y) = (start_x, start_y);
    while x != end_x || y != end_y {
        path.push((x, y));
        (x, y) = walked[&(x, y)];
    }
    println!("{}", path.len());
}
