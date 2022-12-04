use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file.");

    part_one(input.lines());
    part_two(input.lines());
}

fn part_two(rucksacks: std::str::Lines) {
    let mut total_priority = 0;
    let count = rucksacks.clone().count();

    let mut items = rucksacks.map(|rucksack| rucksack);

    for _ in (0..count).step_by(3) {
        let group = (items.next(), items.next(), items.next());
        let shared_item =
            match group {
                (Some(a), Some(b), Some(c)) => {
                    a.chars()
                        .filter(|item| b.contains(*item) && c.contains(*item))
                        .next()
                        .unwrap_or_default()
                }
                _ => break,
            };
        total_priority += get_priority(shared_item);
    }

    println!("{total_priority}");
}

fn part_one(rucksacks: std::str::Lines) {
    let mut total_priority = 0;

    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

        let items_first: HashSet<char> = first_compartment.chars().collect();
        let items_second: HashSet<char> = second_compartment.chars().collect();

        let shared_item = items_first.intersection(&items_second).next().unwrap();

        total_priority += get_priority(*shared_item);
    }
    println!("{total_priority}");
}

fn get_priority(item: char) -> i32 {
    return match item {
        'A'..='Z' => item as i32 - 'A' as i32 + 27,
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        _ => 0,
    };
}
