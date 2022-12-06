use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;

fn main() {
    println!("1:\n{}\n", naloga1("input1.txt"));
    println!("2:\n{}\n", naloga2("input2.txt"));
}

fn naloga1(path: &str) -> i32 {
    let mut cur_calories = 0;
    let mut max_calories = 0;

    for line in get_reader(path).lines() {
        let line = line.unwrap();

        if line.len() > 0 {
            cur_calories += line.parse::<i32>().unwrap();
        }
        else {
            max_calories = max(max_calories, cur_calories);
            cur_calories = 0;
        }
    }

    return max_calories;
}

fn naloga2(path: &str) -> i32 {
    let mut score = 0;

    for line in get_reader(path).lines() {
        let line = line.unwrap();
        let turn = line.split(' ').collect::<Vec<&str>>();

        score += match turn.as_slice() {
            &[opponent, "X"] => 1 + match opponent { "A" => 3, "B" => 0, _ => 6 },
            &[opponent, "Y"] => 2 + match opponent { "A" => 6, "B" => 3, _ => 0 },
            &[opponent, "Z"] => 3 + match opponent { "A" => 0, "B" => 6, _ => 3 },
            _ => 0,
        };
    }

    return score;
}

fn get_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    return BufReader::new(file);
}
