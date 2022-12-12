use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should be able to read the input file.");

    let chars = input.chars().collect::<Vec<char>>();

    part_one(chars.clone());
    part_two(chars.clone());
}

fn part_one(chars: Vec<char>) {
    let marker_size = 4;
    let start_of_packet_marker = start_of_marker(chars, marker_size).unwrap();
    println!("{}", start_of_packet_marker + marker_size);
}

fn part_two(chars: Vec<char>) {
    let marker_size = 14;
    let start_of_message_marker = start_of_marker(chars, marker_size).unwrap();
    println!("{}", start_of_message_marker + marker_size);
}

fn start_of_marker(chars: Vec<char>, marker_size: usize) -> Option<usize> {
    for i in 0..chars.len() {
        let slice = chars.iter().skip(i).take(marker_size);
        if is_marker(slice, marker_size) {
            return Option::Some(i);
        }
    }
    return Option::None;
}

fn is_marker<'a, I>(sequence: I, marker_size: usize) -> bool
where
    I: Iterator<Item = &'a char>,
{
    return sequence.collect::<HashSet<&char>>().len() == marker_size;
}
