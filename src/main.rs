use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should be able to read the input file.");

    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calorie| calorie.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect();
    elves.sort_by(|a, b| b.cmp(a));

    let max_calories = elves.iter()
        .max()
        .expect("Expected an i32.");
    println!("{max_calories}");

    let top_three_calories = elves.iter()
        .take(3)
        .sum::<i32>();
    println!("{top_three_calories}");
}
