use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut area: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0, 0);

    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                for char in line.chars() {
                    process_coords(char, &mut x, &mut y, &mut area);
                }
            }
        }

        let count = area.into_iter().count();
        println!("{count}");
    }
}

fn part_two() {
    let mut santa_area: HashSet<(i32, i32)> = HashSet::new();
    let (mut santa_x, mut santa_y) = (0, 0);

    let mut robo_area: HashSet<(i32, i32)> = HashSet::new();
    let (mut robo_x, mut robo_y) = (0, 0);

    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                for (position, char) in line.chars().enumerate() {
                    if position % 2 == 0 {
                        process_coords(char, &mut santa_x, &mut santa_y, &mut santa_area);
                    } else {
                        process_coords(char, &mut robo_x, &mut robo_y, &mut robo_area);
                    }
                }
            }
        }

        let all_areas = santa_area.union(&robo_area).count();
        println!("{all_areas}");
    }
}

fn process_coords(char: char, x: &mut i32, y: &mut i32, area: &mut HashSet<(i32, i32)>) {
    match char {
        '>' => *x += 1,
        '<' => *x -= 1,
        '^' => *y += 1,
        'v' => *y -= 1,
        _ => println!("unexpected token"),
    }
    area.insert((*x, *y));
}
