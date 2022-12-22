use std::{cmp::Ordering, fs};

enum ListItem {
    Sublist(Vec<ListItem>),
    Value(usize),
}

impl ListItem {
    fn from_string(input: &str) -> ListItem {
        let mut list = ListItem::Sublist(Vec::new());
        list.parse(input.chars().collect::<Vec<char>>().as_slice());
        list
    }

    fn parse(&mut self, slice: &[char]) -> usize {
        let mut index = 0;
        if let Self::Sublist(items) = self {
            while index < slice.len() {
                let char = slice[index];
                match char {
                    '[' => {
                        let subslice = &slice[index + 1..];
                        let mut sublist = Self::Sublist(Vec::new());
                        index += sublist.parse(subslice);
                        items.push(sublist);
                    }
                    ']' => {
                        index += 1;
                        break;
                    }
                    ',' => {}
                    '0'..='9' => {
                        let subslice = &slice[index..];
                        let end_of_value = subslice
                            .iter()
                            .position(|c| !c.is_numeric())
                            .expect("Expected delimiter");
                        let value = (&subslice[0..end_of_value])
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .expect("Expected number");
                        items.push(Self::Value(value));
                        index += end_of_value - 1;
                    }
                    _ => println!("Unexpected character: {}", index),
                }
                index += 1;
            }
        }
        index
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Value(self_value), Self::Value(other_value)) => self_value.cmp(other_value),
            (Self::Sublist(self_list), Self::Sublist(other_list)) => {
                for (self_item, other_item) in self_list.iter().zip(other_list) {
                    match self_item.cmp(other_item) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                self_list.len().cmp(&other_list.len())
            }
            (Self::Sublist(_), Self::Value(other_value)) => {
                let mut other_items = Vec::new();
                other_items.push(Self::Value(*other_value));
                let other_list = Self::Sublist(other_items);
                self.cmp(&other_list)
            }
            (Self::Value(self_value), Self::Sublist(_)) => {
                let mut self_items = Vec::new();
                self_items.push(Self::Value(*self_value));
                let self_list = Self::Sublist(self_items);
                self_list.cmp(other)
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the in_1le.");

    let packet_pairs = input
        .split("\n\n")
        .map(|pair| pair.split('\n'))
        .map(|mut split_pair| (split_pair.next().unwrap(), split_pair.next().unwrap()))
        .map(|(left, right)| (ListItem::from_string(left), ListItem::from_string(right)));

    part_one(packet_pairs.clone());
    part_two(packet_pairs);
}

fn part_one<I>(packet_pairs: I)
where
    I: Iterator<Item = (ListItem, ListItem)>,
{
    let sum_correct_pairs_indices = packet_pairs
        .enumerate()
        .map(|(index, (left, right))| match left.cmp(&right) {
            Ordering::Greater => 0,
            _ => index + 1,
        })
        .sum::<usize>();
    println!("{}", sum_correct_pairs_indices);
}

fn part_two<I>(packet_pairs: I)
where
    I: Iterator<Item = (ListItem, ListItem)>,
{
    let mut packets = Vec::new();

    packet_pairs.for_each(|(left, right)| {
        packets.push(left);
        packets.push(right)
    });

    let divider_packet_one = ListItem::from_string("[[2]]");
    let divider_packet_two = ListItem::from_string("[[6]]");
    packets.push(divider_packet_one);
    packets.push(divider_packet_two);

    packets.sort_by(|left, right| left.cmp(right));

    let index_divider_packet_one = packets
        .iter()
        .position(|list| match list.cmp(&ListItem::from_string("[[2]]")) {
            Ordering::Equal => true,
            _ => false,
        })
        .expect("Expect to find divider packet [[2]]")
        + 1;

    let index_divider_packet_two = packets
        .iter()
        .position(|list| match list.cmp(&ListItem::from_string("[[6]]")) {
            Ordering::Equal => true,
            _ => false,
        })
        .expect("Expect to find divider packet [[2]]")
        + 1;

    let decoder_key = index_divider_packet_one * index_divider_packet_two;
    println!("{}", decoder_key);
}
