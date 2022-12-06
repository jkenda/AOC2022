use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("1:\n{}\n", naloga1("input1.txt"));
    println!("2:\n{}\n", naloga2("input2.txt"));
    println!("3:\n{}\n", naloga3("input3.txt"));
}

fn naloga1(path: &str) -> i32 {
    let mut cur_calories = 0;
    let mut max_calories = [0, 0, 0];

    for line in get_reader(path).lines() {
        let line = line.unwrap();

        if line.len() > 0 {
            cur_calories += line.parse::<i32>().unwrap();
        }
        else {
            if cur_calories > max_calories[0] {
                max_calories[2] = max_calories[1];
                max_calories[1] = max_calories[0];
                max_calories[0] = cur_calories;
            }
            else if cur_calories > max_calories[1] {
                max_calories[2] = max_calories[1];
                max_calories[1] = cur_calories;
            }
            else if cur_calories > max_calories[2] {
                max_calories[2] = cur_calories;
            }
            cur_calories = 0;
        }
    }

    if cur_calories > max_calories[0] {
        max_calories[2] = max_calories[1];
        max_calories[1] = max_calories[0];
        max_calories[0] = cur_calories;
    }
    else if cur_calories > max_calories[1] {
        max_calories[2] = max_calories[1];
        max_calories[1] = cur_calories;
    }
    else if cur_calories > max_calories[2] {
        max_calories[2] = cur_calories;
    }

    return max_calories.iter().sum();
}

fn naloga2(path: &str) -> i32 {
    let mut score = 0;

    for line in get_reader(path).lines() {
        let line = line.unwrap();
        let turn = line.split(' ').collect::<Vec<&str>>();

        score += match turn.as_slice() {
            &[opponent, "X"] => 0 + match opponent { "A" => 3, "B" => 1, _ => 2 },
            &[opponent, "Y"] => 3 + match opponent { "A" => 1, "B" => 2, _ => 3 },
            &[opponent, "Z"] => 6 + match opponent { "A" => 2, "B" => 3, _ => 1 },
            _ => 0,
        };
    }

    return score;
}

fn naloga3(path: &str) -> usize {
    let mut sum: usize = 0;
    let mut lines = [HashSet::<u8>::new(), HashSet::<u8>::new(), HashSet::<u8>::new()];
    
    for (i, line) in get_reader(path).lines().enumerate() {
        let items = line.unwrap().into_bytes();
        let len = items.len();
        if len == 0 { continue }

        lines[i % 3].clear();
        lines[i % 3].extend(items.iter());

        if i % 3 == 2 {
            let char = *(&(&lines[0] & &lines[1]) & &lines[2]).iter().next().unwrap() as u8;
            sum += match char.is_ascii_uppercase() {
                false => char - 'a' as u8 + 1,
                true  => char - 'A' as u8 + 27,
            } as usize;
        }
    }
    
    return sum;
}

fn get_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    return BufReader::new(file);
}
