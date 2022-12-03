use std::{fs, collections::{HashSet}};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should be able to read the input file.");

    let mut total_priority = 0;

    let rucksacks = input.lines();
    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

        let items_first: HashSet<char> = first_compartment.chars().collect();
        let items_second: HashSet<char> = second_compartment.chars().collect();
        
        let shared_item = items_first.intersection(&items_second)
            .next()
            .unwrap();

        total_priority += get_priority(*shared_item);
    }
    println!("{total_priority}");
}

fn get_priority(item: char) -> i32 {
    return match item {
        'A'..='Z' => item as i32 - 'A' as i32 + 27,
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        _ => 0
    };
}
