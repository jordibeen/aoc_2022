use std::fs;

fn main() {
    println!("--- Day 10: Cathode-Ray Tube ---");
    let input: String = fs::read_to_string("./src/day10/input.txt").expect("File should exist");

    let mut cycle: isize = 0;
    let mut x_value: isize = 1;
    let mut signal_strenghts: Vec<isize> = Vec::new();

    let mut screen: Vec<Vec<&str>> = Vec::new();

    input.lines().for_each(|line| {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => {
                // noop takes 1 cycle to complete
                cycle = add_cycle(cycle, x_value, &mut signal_strenghts);
                draw_pixel(&mut screen, cycle, x_value);
            }
            ["addx", amount_str] => {
                // addx takes two cycles to complete, and adds the value on the last cycle
                let amount = amount_str.parse::<isize>().expect("Should be a number");
                cycle = add_cycle(cycle, x_value, &mut signal_strenghts);
                draw_pixel(&mut screen, cycle, x_value);
                cycle = add_cycle(cycle, x_value, &mut signal_strenghts);
                draw_pixel(&mut screen, cycle, x_value);
                x_value += amount;
            }
            _ => {}
        }
    });

    let signal_strength_sum: isize = signal_strenghts.iter().sum();
    println!("Part One: {:?}", signal_strength_sum);

    println!("Part Two:");
    screen.iter().for_each(|row| {
        println!("{}", row.join(""));
    })
}

fn add_cycle(cycle: isize, x_value: isize, signal_strenghts: &mut Vec<isize>) -> isize {
    let end_cycle = cycle + 1;

    if end_cycle >= 20 && (end_cycle - 20) % 40 == 0 {
        signal_strenghts.push(end_cycle * x_value);
    };

    end_cycle
}

fn draw_pixel(screen: &mut Vec<Vec<&str>>, cycle: isize, x_value: isize) {
    let sprite_pos: Vec<isize> = vec![x_value - 1, x_value, x_value + 1];
    let crt_pos = (cycle - 1) % 40;

    if crt_pos == 0 {
        screen.push(Vec::new());
    }

    if sprite_pos.contains(&crt_pos) {
        screen
            .last_mut()
            .expect("Should have a last element")
            .push("#");
    } else {
        screen
            .last_mut()
            .expect("Should have a last element")
            .push(".");
    }
}
