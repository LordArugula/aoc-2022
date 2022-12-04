use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file.");

    let range_pairs = input
        .lines()
        .map(|line| line.split(','))
        .map(|mut ranges| (ranges.next().unwrap(), ranges.next().unwrap()))
        .map(|(range_a_str, range_b_str)| {
            let mut range_a = range_a_str.split('-').map(|x| x.parse::<i32>().unwrap());
            let mut range_b = range_b_str.split('-').map(|x| x.parse::<i32>().unwrap());
            (
                range_a.next().unwrap(),
                range_a.next().unwrap(),
                range_b.next().unwrap(),
                range_b.next().unwrap(),
            )
        })
        .into_iter()
        .collect::<Vec<(i32, i32, i32, i32)>>();

    part_one(&range_pairs);
    part_two(range_pairs);
}

fn part_one(range_pairs: &Vec<(i32, i32, i32, i32)>) {
    let num_subsets = range_pairs
        .iter()
        .map(|(a, b, x, y)| {
            if a <= x && b >= y || a >= x && b <= y {
                1
            } else {
                0
            }
        })
        .sum::<i32>();
    println!("{num_subsets}");
}

fn part_two(range_pairs: Vec<(i32, i32, i32, i32)>) {
    let num_overlap = range_pairs
        .iter()
        .map(|(a, b, x, y)| {
            if x <= b && b <= y || a <= y && y <= b {
                1
            } else {
                0
            }
        })
        .sum::<i32>();
    println!("{num_overlap}");
}
