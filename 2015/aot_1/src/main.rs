use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    iterator_basement();
}

fn iterator() {
    let mut count: i32 = 0;

    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                for c in line.chars() {
                    match c {
                        '(' => count = count + 1,
                        ')' => count = count - 1,
                        _ => break,
                    }
                }
            }
        }
        println!("{count}");
    }
}

fn count() {
    if let Ok(file) = fs::read_to_string("src/input.txt") {
        let up_count = file.chars().filter(|&c| c == '(').count();
        let down_count = file.chars().filter(|&c| c == ')').count();
        let floor = up_count - down_count;
        println!("{floor}")
    }
}

fn iterator_basement() {
    let mut count: i32 = 0;

    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        'principal: for line in reader.lines() {
            if let Ok(line) = line {
                for c in line.chars().into_iter().enumerate() {
                    match c.1 {
                        '(' => count = count + 1,
                        ')' => count = count - 1,
                        _ => break,
                    }
                    if count == -1 {
                        let position = c.0 + 1;
                        println!("entrou no porão na posição {position}");
                        break 'principal;
                    }
                }
            }
        }
    }
}
