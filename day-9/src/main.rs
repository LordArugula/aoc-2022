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

    let num_knots_part_one = 2;
    count_positions_walked_by_tail_knot(movements.clone(), num_knots_part_one);

    let num_knots_part_two = 10;
    count_positions_walked_by_tail_knot(movements, num_knots_part_two);
}

fn count_positions_walked_by_tail_knot<I>(movements: I, num_knots: usize)
where
    I: Iterator<Item = (i32, i32)>,
{
    let mut walked_positions = HashSet::<(i32, i32)>::new();
    let mut knots = (0..num_knots)
        .map(|_| (0, 0))
        .collect::<Vec<(i32, i32)>>();

    walked_positions.insert((0, 0));

    for (x, y) in movements {
        let steps = if x.abs() > y.abs() { x.abs() } else { y.abs() };
        for _ in 0..steps {
            let (mut knot_prev_x, mut knot_prev_y) = knots[0];
            (knot_prev_x, knot_prev_y) = (knot_prev_x + x.signum(), knot_prev_y + y.signum());
            knots[0] = (knot_prev_x, knot_prev_y);

            for i in 1..num_knots {
                let (mut knot_x, mut knot_y) = knots[i];
                let (dx, dy) = (knot_prev_x - knot_x, knot_prev_y - knot_y);

                if dx * dx + dy * dy > 2 {
                    (knot_x, knot_y) = (knot_x + dx.signum(), knot_y + dy.signum());
                    knots[i] = (knot_x, knot_y);
                    if i == num_knots - 1 {
                        walked_positions.insert((knot_x, knot_y));
                    }
                }
                (knot_prev_x, knot_prev_y) = (knot_x, knot_y);
            }
        }
    }

    println!("{}", walked_positions.len());
}
