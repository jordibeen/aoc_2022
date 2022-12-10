use std::fs;

fn main() {
    println!("--- Day 10: Cathode-Ray Tube ---");
    let input: String = fs::read_to_string("./src/day10/input.txt").expect("File should exist");

    let mut cycle: isize = 0;
    let mut x_value: isize = 1;
    let mut signal_strenghts: Vec<isize> = Vec::new();

    input.lines().for_each(
        |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => {
                // noop takes 1 cycle to complete
                cycle = add_cycle(cycle, x_value, &mut signal_strenghts);
            }
            ["addx", amount_to_add] => {
                let amount = amount_to_add.parse::<isize>().expect("Should be a number");

                // addx takes two cycles to complete, and adds the value on the last cycle
                for i in 0..2 {
                    cycle = add_cycle(cycle, x_value, &mut signal_strenghts);

                    if i == 1 {
                        x_value += amount;
                    }
                }
            }
            _ => {}
        },
    );

    let signal_strength_sum: isize = signal_strenghts.iter().sum();
    println!("Part One: {:?}", signal_strength_sum);
}

fn add_cycle(cycle: isize, x_value: isize, signal_strenghts: &mut Vec<isize>) -> isize {
    let end_cycle = cycle + 1;
    if end_cycle >= 20 && (end_cycle - 20) % 40 == 0 {
        signal_strenghts.push(end_cycle * x_value);
    };
    end_cycle
}
