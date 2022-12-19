use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the in_1le.");

    let mut cycle_number = 0;
    let mut value = 1;
    let mut total_signal_strength = 0;

    for line in input.lines() {
        if tick_cycle_number(&mut cycle_number) {
            total_signal_strength += calc_signal_strength(cycle_number, value);
        }

        print_pixel(cycle_number, value);

        match line.split(" ").last().unwrap().parse::<i32>() {
            Ok(x) => {
                if tick_cycle_number(&mut cycle_number) {
                    total_signal_strength += calc_signal_strength(cycle_number, value);
                }

                print_pixel(cycle_number, value);

                value += x;
            }
            Err(_) => {}
        };
    }

    println!("{}", total_signal_strength);
}

fn calc_signal_strength(cycle_number: i32, value: i32) -> i32 {
    cycle_number * value
}

fn tick_cycle_number(cycle_number: &mut i32) -> bool {
    *cycle_number += 1;
    return (*cycle_number + 20) % 40 == 0;
}

fn print_pixel(cycle_number: i32, value: i32) {
    if value <= (cycle_number % 40) && value + 2 >= (cycle_number % 40) {
        print!("#");
    } else {
        print!(".");
    }

    if cycle_number % 40 == 0 {
        println!();
    }
}

/*
########.###..####.##....##..##.#..##...
..####......####...#.......####.####....
.....##...##..###..###.......##.####.##.
##..##......####......######....#####..#
....##...##.####......####......####...#
###.##...###..##...##.....##...##.#.....
*/

/*
##..##..#..##...##.##..##..##..##..##..#
###..####..###...###...####..####..###..
##.....#####...###.....####.....##.....#
####.....##.#......#####.....####.......
###.##.....##.###......######......##.#.
###.##.......####.#........########.....
*/