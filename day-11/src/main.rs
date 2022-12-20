use std::fs;

struct Monkey {
    item_worry_levels: Vec<u64>,
    operator: bool,
    operand: u64,
    test: u64,
    throw_if_true: usize,
    throw_if_false: usize,
    num_times_inspected_an_item: usize,
}

impl Monkey {
    fn from_string_input(string: &str) -> Self {
        let mut fields_iter = string.lines().skip(1);

        let item_worry_levels = fields_iter
            .next()
            .unwrap()
            .split(&[' ', ','])
            .filter(|&str| str.parse::<u64>().is_ok())
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut operation_str = fields_iter.next().unwrap().split(" ").skip(6); // Skips "_" "_" "Operation:" "new" "=" "old"

        let operator = operation_str.next().unwrap() == "*"; // true => multiple, false => addition

        let operand = operation_str.next().unwrap();

        let operand = operand.parse::<u64>().unwrap_or(0);

        let test = fields_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let throw_if_true = fields_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let throw_if_false = fields_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self {
            item_worry_levels,
            operator,
            operand,
            test,
            throw_if_true,
            throw_if_false,
            num_times_inspected_an_item: 0,
        }
    }

    fn inspect_item(&mut self, item_index: usize, max: u64, with_relief: bool) {
        let item_worry_level = self.item_worry_levels[item_index];
        let operand = if self.operand == 0 {
            item_worry_level
        } else {
            self.operand
        };

        let mut result = if self.operator {
            item_worry_level * operand
        } else {
            item_worry_level + operand
        };

        result = if with_relief { result / 3 } else { result % max};

        self.item_worry_levels[item_index] = result;

        self.num_times_inspected_an_item += 1;
    }

    fn test_item(&mut self, index: usize) -> usize {
        let item_worry_level = self.item_worry_levels[index];

        if item_worry_level % self.test == 0 {
            self.throw_if_true
        } else {
            self.throw_if_false
        }
    }

    fn num_items(&self) -> usize {
        self.item_worry_levels.len()
    }
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Self {
            item_worry_levels: self.item_worry_levels.clone(),
            operator: self.operator.clone(),
            operand: self.operand.clone(),
            test: self.test.clone(),
            throw_if_true: self.throw_if_true.clone(),
            throw_if_false: self.throw_if_false.clone(),
            num_times_inspected_an_item: self.num_times_inspected_an_item.clone(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the in_1le.");

    let monkeys = input
        .split("\n\n")
        .map(|monkey| Monkey::from_string_input(monkey))
        .collect::<Vec<Monkey>>();

    simulate_monkeys(&mut monkeys.to_vec(), 20, true);
    simulate_monkeys(&mut monkeys.to_vec(), 10000, false);
}

fn simulate_monkeys(monkeys: &mut Vec<Monkey>, num_rounds: usize, with_relief: bool) {
    let max = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test);

    for _ in 0..num_rounds {
        for monkey_index in 0..monkeys.len() {
            for item_index in 0..monkeys[monkey_index].num_items() {
                monkeys[monkey_index].inspect_item(item_index, max, with_relief);
                let throw_index = monkeys[monkey_index].test_item(item_index);

                let item_worry_level = monkeys[monkey_index].item_worry_levels[item_index];
                monkeys[throw_index]
                    .item_worry_levels
                    .push(item_worry_level);
            }
            // cleaning up old values; thrown everything to another monkey
            monkeys[monkey_index].item_worry_levels.clear();
        }
    }
    let mut times_monkey_inspected_items = monkeys
        .iter()
        .map(|monkey| monkey.num_times_inspected_an_item)
        .collect::<Vec<usize>>();
    times_monkey_inspected_items.sort_by(|a, b| b.cmp(a));
    let monkey_business_level = times_monkey_inspected_items
        .iter()
        .take(2)
        .fold(1, |acc, x| acc * x);
    println!("{}", monkey_business_level);
}
