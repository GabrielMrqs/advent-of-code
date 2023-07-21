use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let mut total_area = 0;
    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let (l, w, h) = get_dimensions(line);
                let sides = HashMap::from([((l, w), l * w), ((w, h), w * h), ((h, l), h * l)]);
                let min_side = sides.into_iter().min_by_key(|x| x.1).unwrap().0;
                let formula = l * w * h;
                let sum_sides = 2 * min_side.0 + 2 * min_side.1;
                let total_box_area = formula + sum_sides;
                total_area += total_box_area;
            }
        }
        println!("{total_area}");
    }
}

fn part_one() {
    let mut total_area = 0;
    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let (l, w, h) = get_dimensions(line);
                let min_side = vec![l * w, w * h, h * l].into_iter().min().unwrap();
                let formula = 2 * l * w + 2 * w * h + 2 * h * l;
                let total_box_area = formula + min_side;
                total_area += total_box_area;
            }
        }
        println!("{total_area}");
    }
}

fn get_dimensions(line: String) -> (i32, i32, i32) {
    let dimensions: Vec<i32> = line.split('x').filter_map(|x| x.parse().ok()).collect();
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];
    (l, w, h)
}
