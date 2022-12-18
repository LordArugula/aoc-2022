use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file.");

    let tree_heightmap = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    part_one(&tree_heightmap);
    part_two(&tree_heightmap);
}

fn part_one(tree_heightmap: &Vec<Vec<i32>>) {
    let mut num_visible_trees = 0;
    for row in 0..tree_heightmap.len() {
        let heights = tree_heightmap.get(row).unwrap();

        for col in 0..heights.len() {
            if is_visible(row, col, &tree_heightmap) {
                num_visible_trees += 1;
            }
        }
    }
    println!("{}", num_visible_trees);
}

fn part_two(tree_heightmap: &Vec<Vec<i32>>) {
    let mut highest_scenic_score = 0;

    for row in 0..tree_heightmap.len() {
        let heights = tree_heightmap.get(row).unwrap();

        for col in 0..heights.len() {
            let scenic_score = calc_scenic_score(row, col, tree_heightmap);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }
    println!("{}", highest_scenic_score);
}

fn calc_scenic_score(row: usize, col: usize, tree_heightmap: &Vec<Vec<i32>>) -> usize {
    let height = tree_heightmap[row][col];

    let num_visible_from_left = {
        let mut num = 0;
        for y in (0..col).rev() {
            num += 1;

            if height <= tree_heightmap[row][y] {
                break;
            }
        }
        num
    };

    let num_visible_from_right = {
        let mut num = 0;
        for y in (col + 1)..tree_heightmap[row].len() {
            num += 1;

            if height <= tree_heightmap[row][y] {
                break;
            }
        }
        num
    };

    let num_visible_from_top = {
        let mut num = 0;
        for x in (0..row).rev() {
            num += 1;

            if height <= tree_heightmap[x][col] {
                break;
            }
        }
        num
    };

    let num_visible_from_bottom = {
        let mut num = 0;
        for x in (row + 1)..tree_heightmap.len() {
            num += 1;

            if height <= tree_heightmap[x][col] {
                break;
            }
        }
        num
    };

    let scenic_score = num_visible_from_left
        * num_visible_from_right
        * num_visible_from_top
        * num_visible_from_bottom;
    scenic_score
}

fn is_visible(row: usize, col: usize, tree_heightmap: &Vec<Vec<i32>>) -> bool {
    let height = tree_heightmap[row][col];

    if row == 0
        || row == tree_heightmap.len() - 1
        || col == 0
        || col == tree_heightmap[row].len() - 1
    {
        return true;
    }

    let visible_from_left = {
        let mut y = 0;
        let result = loop {
            if height <= tree_heightmap[row][y] {
                break false;
            }
            y += 1;
            if y == col {
                break true;
            }
        };
        result
    };

    let visible_from_right = {
        let mut y = tree_heightmap[row].len() - 1;
        let result = loop {
            if height <= tree_heightmap[row][y] {
                break false;
            }
            y -= 1;
            if y == col {
                break true;
            }
        };
        result
    };

    let visible_from_top = {
        let mut x = 0;
        let result = loop {
            if height <= tree_heightmap[x][col] {
                break false;
            }
            x += 1;
            if x == row {
                break true;
            }
        };
        result
    };

    let visible_from_bottom = {
        let mut x = tree_heightmap.len() - 1;
        let result = loop {
            if height <= tree_heightmap[x][col] {
                break false;
            }
            x -= 1;
            if x == row {
                break true;
            }
        };
        result
    };

    visible_from_left || visible_from_right || visible_from_top || visible_from_bottom
}
