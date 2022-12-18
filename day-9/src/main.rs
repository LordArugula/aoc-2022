use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the in_1le.");

    let movements = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(dir, steps)| (dir, steps.parse::<i32>().unwrap()))
        .map(|(dir, steps)| match dir {
            "L" => (-steps, 0),
            "R" => (steps, 0),
            "U" => (0, steps),
            "D" => (0, -steps),
            _ => (0, 0),
        });

    part_one(movements);
}

fn part_one<I>(movements: I)
where
    I: Iterator<Item = (i32, i32)>,
{
    let mut walked_positions = HashSet::new();
    let (mut head_x, mut head_y) = (0, 0);
    let (mut tail_x, mut tail_y) = (0, 0);
    walked_positions.insert((tail_x, tail_y));
    movements.for_each(|(x, y)| {
        let steps = if x.abs() > y.abs() { x.abs() } else { y.abs() };
        for _ in 0..steps {
            (head_x, head_y) = (head_x + x.signum(), head_y + y.signum());

            let (dx, dy) = (head_x - tail_x, head_y - tail_y);
            if dx * dx + dy * dy > 2 {
                (tail_x, tail_y) = (tail_x + dx.signum(), tail_y + dy.signum());
                walked_positions.insert((tail_x, tail_y));
            }
        }
    });
    println!("{}", walked_positions.len());
}
