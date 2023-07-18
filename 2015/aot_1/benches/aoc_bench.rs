use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("iterator", |f| f.iter(|| iterator()));
    c.bench_function("count", |f| f.iter(|| count()));
    c.bench_function("count2", |f| f.iter(|| count2()));
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
    }
}

fn count() {
    if let Ok(file) = fs::read_to_string("src/input.txt") {
        let up_count = file.chars().filter(|&c| c == '(').count();
        let down_count = file.chars().filter(|&c| c == ')').count();
        let _ = up_count - down_count;
    }
}

fn count2() {
    if let Ok(file) = fs::read_to_string("src/input.txt") {
        let up_count = file.chars().filter(|&c| c == '(').count();
        let down_count = file.len() - up_count;
        let _ = up_count - down_count + 1;
    }
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
