use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should be able to read the input file.");

    let stacks_input = input.lines()
        .take_while(|line| !line.is_empty());

    let mut stacks = Vec::<VecDeque<char>>::new();

    for line in stacks_input {
        for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
            if stacks.len() <= i {
                stacks.push(VecDeque::<char>::new());
            }

            if char == ' ' || char.is_digit(10) {
                continue;
            }

            let stack = &mut stacks[i];
            stack.push_front(char);
        }
    }

    let instructions = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip_while(|line| line.is_empty())
        .map(|instruction| {
            instruction
                .split(' ')
                .filter(|str| str.parse::<i32>().is_ok())
                .map(|str| str.parse::<i32>().unwrap() as usize)
                .collect::<Vec<usize>>()
        });
        
    part_one(instructions.clone(), stacks.clone());
    part_two(instructions.clone(), stacks.clone());
}

fn part_two<I>(instructions: I, mut stacks: Vec<VecDeque<char>>)
where
    I: Iterator<Item = Vec<usize>>,
{
    for instruction in instructions {

        let num = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;

        let mut stack = Vec::<char>::new();

        for _ in 0..num {
            let item = stacks[from].pop_back().unwrap();
            stack.push(item);
        }

        for _ in 0..num {
            let item = stack.pop().unwrap();
            stacks[to].push_back(item);
        }
    }
    for stack in &mut stacks {
        print!("{}", stack.pop_back().unwrap_or(' '));
    }
    println!();
}

fn part_one<I>(instructions: I, mut stacks: Vec<VecDeque<char>>)
where
    I: Iterator<Item = Vec<usize>>,
{
    for instruction in instructions {

        let num = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;

        for _ in 0..num {
            let item = stacks[from].pop_back().unwrap();
            stacks[to].push_back(item);
        }
    }
    for stack in &mut stacks {
        print!("{}", stack.pop_back().unwrap_or(' '));
    }
    println!();
}
