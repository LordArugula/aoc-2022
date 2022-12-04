use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file.");

    let num_pairs = input.lines()
    .map(|line| line.split(','))
    .map(|mut ranges| (ranges.next().unwrap(), ranges.next().unwrap()))
    .map(|(range_a_str, range_b_str)| {
        let mut range_a = range_a_str.split('-').map(|x| x.parse::<i32>().unwrap());
        let mut range_b = range_b_str.split('-').map(|x| x.parse::<i32>().unwrap());
        let (a, b, x, y) = (range_a.next(), range_a.next(), range_b.next(), range_b.next());
        if a <= x && b >= y || a >= x && b <= y {
            1
        } else {
            0
        }
    })
    .sum::<i32>();

    println!("{num_pairs}");
}
