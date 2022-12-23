use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input file.");
    let paths = input.lines().map(|line| {
        line.split(" -> ")
            .map(|point_string| {
                point_string
                    .split_once(',')
                    .expect("Expect x, y coordinates.")
            })
            .map(|(x_str, y_str)| {
                (
                    x_str.parse::<usize>().unwrap(),
                    y_str.parse::<usize>().unwrap(),
                )
            })
    });

    let (left, depth) = paths.clone().fold((usize::MAX, 0), |(left, depth), path| {
        path.fold((left, depth), |(left, depth), point| {
            let left = if point.0 < left { point.0 } else { left };
            let depth = if point.1 > depth { point.1 } else { depth };
            (left, depth)
        })
    });

    let floor = depth + 2;
    // Point (500, 0) is the starting point, so we offset that by the left bound
    // then add the width of the pile (height/depth of pile - 1) * 2
    let width = 500 - left + (floor - 1) * 2;
    let mut cave = (0..=floor)
        .map(|y| (0..=width).map(|_| y == floor).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for mut path in paths {
        let (mut curr_x, mut curr_y) = path
            .next()
            .expect("Should be at least two points in a path");

        while let Some((next_x, next_y)) = path.next() {
            let dx = next_x as i32 - curr_x as i32;
            let dy = next_y as i32 - curr_y as i32;

            for x in 0..=dx.abs() {
                cave[curr_y][((curr_x - left) as i32 + x * dx.signum()) as usize + depth] = true;
            }
            for y in 0..=dy.abs() {
                cave[(curr_y as i32 + y * dy.signum()) as usize][curr_x - left + depth] = true;
            }
            (curr_x, curr_y) = (next_x, next_y);
        }
    }

    let start = (500 - left + depth, 0);
    // debug_cave(start, &cave);

    let mut num_sand = 0;
    let mut print_part_one = true;
    'sand_simulation: loop {
        let mut position = start;
        if cave[position.1][position.0] {
            break;
        }

        loop {
            if position.1 > depth && print_part_one {
                // part one:
                // fallen through the bottom
                println!("{}", num_sand);
                print_part_one = false;
            }

            // try to drop down
            if cave[position.1 + 1][position.0] == false {
                position.1 += 1;
                continue;
            }

            // try to drop diagonally, left and down
            if cave[position.1 + 1][position.0 - 1] == false {
                position.0 -= 1;
                position.1 += 1;
                continue;
            }

            // try to drop diagonally, right and down
            if cave[position.1 + 1][position.0 + 1] == false {
                position.0 += 1;
                position.1 += 1;
                continue;
            }

            // settle here
            cave[position.1][position.0] = true;
            break;
        }
        num_sand += 1;
    }

    // debug_cave(start, &cave);

    println!("{}", num_sand);
}

fn debug_cave(start: (usize, usize), cave: &Vec<Vec<bool>>) {
    for (y, row) in cave.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            print!(
                "{}",
                if x == start.0 && y == start.1 {
                    "+"
                } else {
                    if col {
                        "#"
                    } else {
                        "."
                    }
                }
            );
        }
        println!();
    }
}
