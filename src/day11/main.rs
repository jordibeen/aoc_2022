use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: String,
    divisibility_test: usize,
    throw_true_id: usize,
    throw_false_id: usize,
    inspections: usize,
}

fn main() {
    println!("--- Day 11: Monkey in the Middle ---");

    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    let input: String = fs::read_to_string("./src/day11/input.txt").expect("File should exist");

    input.split("\n\n").for_each(|monkey_input| {
        let monkey = parse_monkey(monkey_input);
        monkeys.insert(monkey.id, monkey);
    });

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(&i).unwrap();

            // Keep track of throws as a tuple containing monkey id and worry level
            // Eg (0, 74)
            let mut throws: Vec<(usize, usize)> = Vec::new();

            // Loop through Monkey's items
            monkey.items.retain(|item| {
                // Perform worry level operation
                let mut worry_level: usize = 0;

                // Perform Monkey's worry level operation
                match monkey.operation.split_whitespace().collect::<Vec<&str>>()[..] {
                    ["old", operator, "old"] => match operator {
                        "*" => {
                            worry_level = item * item;
                        }
                        "+" => {
                            worry_level = item + item;
                        }
                        _ => {}
                    },
                    ["old", operator, new] => match operator {
                        "*" => {
                            worry_level = item * new.parse::<usize>().unwrap();
                        }
                        "+" => {
                            worry_level = item + new.parse::<usize>().unwrap();
                        }
                        _ => {}
                    },
                    _ => {}
                }

                // Divide worry level by 3
                worry_level /= 3;

                // Test item's worry level divisibility
                if worry_level % monkey.divisibility_test == 0 {
                    throws.push((monkey.throw_true_id, worry_level));
                } else {
                    throws.push((monkey.throw_false_id, worry_level));
                }

                // Save Monkey's item inspection count
                monkey.inspections += 1;

                // Remove item from Monkey's items
                false
            });

            // Perform throws
            throws.iter().for_each(|throw| {
                let monkey_to = monkeys.get_mut(&throw.0).unwrap();
                monkey_to.items.push(throw.1);
            });
        }
    }

    let mut number_of_inspects: Vec<usize> = Vec::new();
    monkeys.iter().for_each(|(_, monkey)| {
        number_of_inspects.push(monkey.inspections);
    });
    number_of_inspects.sort();
    number_of_inspects.reverse();

    println!(
        "Part One: {}",
        number_of_inspects[0] * number_of_inspects[1]
    );
}

fn parse_monkey(monkey_input: &str) -> Monkey {
    let monkey_split: Vec<&str> = monkey_input.split("\n").collect();

    let id: usize = Regex::new(r"Monkey (?P<number>\d):")
        .unwrap()
        .captures(monkey_split[0])
        .unwrap()
        .name("number")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let items: Vec<usize> = Regex::new(r"(?P<item>\d+)")
        .unwrap()
        .find_iter(monkey_split[1])
        .filter_map(|starting_item| starting_item.as_str().parse().ok())
        .collect();

    let operation: String = Regex::new(r"Operation: new = (?P<operation>.*)")
        .unwrap()
        .captures(monkey_split[2])
        .unwrap()
        .name("operation")
        .unwrap()
        .as_str()
        .to_owned();

    let divisibility_test: usize = Regex::new(r"Test: divisible by (?P<divisible>\d*)")
        .unwrap()
        .captures(monkey_split[3])
        .unwrap()
        .name("divisible")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let throw_true_id: usize = Regex::new(r"If true: throw to monkey (?P<true>\d*)")
        .unwrap()
        .captures(monkey_split[4])
        .unwrap()
        .name("true")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    let throw_false_id: usize = Regex::new(r"If false: throw to monkey (?P<false>\d*)")
        .unwrap()
        .captures(monkey_split[5])
        .unwrap()
        .name("false")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    Monkey {
        id,
        items,
        operation,
        divisibility_test,
        throw_true_id,
        throw_false_id,
        inspections: 0,
    }
}
