fn main() {
    part_one();
    part_two();
}

fn part_one() {
    calculate_hash("00000");
}

fn part_two() {
    calculate_hash("000000");
}

fn calculate_hash(start: &str) {
    let input = "yzbqklnj";
    let mut answer = 0;
    let mut hash = String::from("");

    while !hash.starts_with(start) {
        answer += 1;
        hash = format!("{:x}", md5::compute(format!("{}{}", input, answer)));
    }

    println!("{answer}");
}
