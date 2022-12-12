use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should be able to read the input file.");

    let chars = input.chars().collect::<Vec<char>>();

    for i in 0..chars.len() {
        let slice = chars.iter().skip(i).take(4);
        if is_start_of_packet_marker(slice.clone()) {
            println!("{}", i + 4);
            break;
        }
    }
}

fn is_start_of_packet_marker<'a, I>(sequence: I) -> bool
where
    I: Iterator<Item = &'a char>,
{
    return sequence.collect::<HashSet<&char>>().len() == 4;
}
