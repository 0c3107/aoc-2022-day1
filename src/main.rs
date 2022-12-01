use std::fs;

fn main() {
    let input = parse_input_file();
    let max = input.iter().max().unwrap();
    println!("Max is: {}", max);
    let index = input.iter().position(|&x| x == *max).unwrap();
    println!("Index of max: {}", index);
    println!("Part two answer: {}", part_two(input));
}

fn part_two(mut vec: Vec<usize>) -> usize {
    vec.sort();
    let last_three = vec.iter().rev().take(3).sum();
    last_three
}

fn parse_input_file() -> Vec<usize> {
    let input = fs::read_to_string("resource/input").unwrap();
    let input_lines = input.lines();
    let mut calories: Vec<usize> = vec![];
    let mut runner = 0;
    for line in input_lines {
        if line != "" {
            let number = line.parse::<usize>().unwrap();
            runner += number;
        } else {
            calories.push(runner);
            runner = 0;
        }
    }
    calories.push(runner);
    calories
}
