use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("1:\n{}\n", naloga1("input1.txt"));
    println!("2:\n{}\n", naloga2("input2.txt"));
    println!("3:\n{}\n", naloga3("input3.txt"));
    println!("4:\n{}\n", naloga4("input4.txt"));
    println!("5:\n{}\n", naloga5("input5.txt"));
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

fn naloga4(path: &str) -> usize {
    let mut sum: usize = 0;

    let overlap = |a: u32, b: u32, c: u32, d: u32| {
        // partial overlap
        if a <= c && b <= d && b >= c { return true };
        if c <= a && d <= b && d >= a { return true };

        // complete overlap
        if a <= c && b >= d { return true };
        if c <= a && d >= b { return true };

        // no overlap
        return false;
    };

    for line in get_reader(path).lines() {
        let line = line.unwrap();
        if line.len() == 0 { break }

        let sections = line.split(|c| c == ',' || c == '-').collect::<Vec<&str>>();
        let (first_start, first_end, second_start, second_end) = (sections[0].parse::<u32>().unwrap(), 
                                                                  sections[1].parse::<u32>().unwrap(), 
                                                                  sections[2].parse::<u32>().unwrap(), 
                                                                  sections[3].parse::<u32>().unwrap());

        sum += if overlap(first_start, first_end, second_start, second_end) { 1 } else { 0 };
    }

    return sum;
}

fn naloga5(path: &str) -> String {
    let mut lines = get_reader(path).lines();
    let line1: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let num_stacks = (line1.len() + 1) / 4;
    let mut crates: Vec<Vec<char>> = vec![vec![]; num_stacks];
    
    for i in 0..num_stacks {
        let c = line1[4*i + 1];
        if c.is_ascii_alphabetic() {
            crates[i].insert(0, c);
        }
    }

    loop {
        let line: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
        if (line.len() + 1) / 4 < num_stacks { break; }

        for i in 0..num_stacks {
            let c = line[4*i + 1];
            if c.is_ascii_alphabetic() {
                crates[i].insert(0, c);
            }
        }
    }

    let move_crates = &mut |from: usize, to: usize, count: usize| {
        let i = crates[from - 1].len() - count;
        for _ in 0..count {
            let c = crates[from - 1].remove(i);
            crates[to - 1].push(c);
        }
    };

    loop {
        let line = match lines.next() {
            Some(l) => l.unwrap(),
            None => break,
        };
        
        let instructions: Vec<usize> = line
            .split(' ')
            .filter(|s| *s != "move" && *s != "from" && *s != "to")
            .map(|s| s.parse().unwrap())
            .collect();

        if let [count, from, to] = instructions.as_slice() {
            move_crates(*from, *to, *count);
        }
    }

    let mut top = String::new();
    for stack in crates {
        top.push(*stack.last().unwrap());
    }

    return top;
}

fn get_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    return BufReader::new(file);
}
