use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    part_one_iterator_approach();
    part_one_count_approach();
    part_two_iterator_approach();
}

fn part_one_iterator_approach() {
    let mut floor = 0;

    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                for c in line.chars() {
                    match c {
                        '(' => floor += 1,
                        ')' => floor -= 1,
                        _ => break,
                    }
                }
            }
        }
        println!("{floor}");
    }
}

fn part_one_count_approach() {
    if let Ok(file) = fs::read_to_string("src/input.txt") {
        let up_count = file.chars().filter(|&c| c == '(').count();
        let down_count = file.chars().filter(|&c| c == ')').count();
        let floor = up_count - down_count;
        println!("{floor}")
    }
}

fn part_two_iterator_approach() {
    let mut floor = 0;

    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        'outer: for line in reader.lines() {
            if let Ok(line) = line {
                for (position, char) in line.chars().into_iter().enumerate() {
                    match char {
                        '(' => floor += 1,
                        ')' => floor -= 1,
                        _ => break,
                    }
                    if floor == -1 {
                        println!("{position}");
                        break 'outer;
                    }
                }
            }
        }
    }
}
