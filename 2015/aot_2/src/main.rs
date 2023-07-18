use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut total_area: i32 = 0;
    if let Ok(file) = File::open("src/input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let dimensions: Vec<i32> = line.split('x').filter_map(|x| x.parse().ok()).collect();
                let l = dimensions[0];
                let w = dimensions[1];
                let h = dimensions[2];
                let min_side = vec![l * w, w * h, h * l].into_iter().min().unwrap();
                let formula = 2 * l * w + 2 * w * h + 2 * h * l;
                let total_box_area = formula + min_side;
                total_area += total_box_area;
            }
        }
        println!("{total_area}");
    }
}
