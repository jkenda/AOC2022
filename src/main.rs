use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("1: {}", naloga1("input1.txt"));
    println!("2: {}", naloga2("input2.txt"));
    println!("3: {}", naloga3("input3.txt"));
    println!("4: {}", naloga4("input4.txt"));
    println!("5: {}", naloga5("input5.txt"));
    println!("6: {}", naloga6("input6.txt"));
}

fn naloga1(path: &str) -> u32 {
    let mut cals = 0;
    let mut max_cals = [0, 0, 0];

    let lines: Vec<String> = get_reader(path).lines()
        .map(|line| line.unwrap())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        if line.len() > 0 && i < lines.len() {
            cals += line.parse::<u32>().unwrap();
        }
        else {
            max_cals = match max_cals {
                [m1, m2,  _] if cals > m1 => [cals, m1, m2],
                [m1, m2,  _] if cals > m2 => [m1, cals, m2],
                [m1, m2, m3] if cals > m3 => [m1, m2, cals],
                _ => max_cals,
            };
            cals = 0;
        }
    }

    max_cals.iter().sum()
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

    score
}

fn naloga3(path: &str) -> usize {
    let mut sum = 0;
    let mut lines = [HashSet::<u8>::new(), HashSet::new(), HashSet::new()];
    
    for (i, line) in get_reader(path).lines().enumerate() {
        let items = line.unwrap().into_bytes();
        if items.is_empty() {
            continue
        }

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
    
    sum
}

fn naloga4(path: &str) -> usize {
    let mut sum = 0;

    for line in get_reader(path).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break
        }

        let nums: Vec<usize> = line
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse().unwrap())
            .collect();

        sum += match nums.as_slice() {
            // partial overlap
            [a, b, c, d] if a <= c && b <= d && b >= c => 1,
            [a, b, c, d] if c <= a && d <= b && d >= a => 1,
            // complete overlap
            [a, b, c, d] if a <= c && b >= d => 1,
            [a, b, c, d] if c <= a && d >= b => 1,
            [_, _, _, _] => 0,
            _ => panic!("Invalid input!")
        };
    }

    sum
}

fn naloga5(path: &str) -> String {
    let mut lines = get_reader(path).lines();
    let line1: Vec<u8> = lines.next().unwrap().unwrap().into_bytes();
    let num_stacks = (line1.len() + 1) / 4;
    let mut crates: Vec<Vec<u8>> = vec![vec![]; num_stacks];
    
    // read initial state
    for i in 0..num_stacks {
        let c = line1[4*i + 1];
        if c.is_ascii_alphabetic() {
            crates[i].insert(0, c);
        }
    }

    loop {
        let line: Vec<u8> = lines.next().unwrap().unwrap().into_bytes();
        if line.is_empty() {
            break;
        }

        for i in 0..num_stacks {
            let c = line[4*i + 1];
            if c.is_ascii_alphabetic() {
                crates[i].insert(0, c);
            }
        }
    }

    // read delta and advance state
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

        let (count, from, to) = (instructions[0], instructions[1] - 1, instructions[2] - 1);

        let i = crates[from].len() - count;
        for _ in 0..count {
            let c = crates[from].remove(i);
            crates[to].push(c);
        }
    }

    let top = crates.iter()
        .map(|stack| *stack.last().unwrap())
        .map(|byte| char::from_u32(byte as u32).unwrap())
        .collect();

    top
}

fn naloga6(path: &str) -> usize {
    let mut buffer = Vec::new();
    get_reader(path).read_to_end(&mut buffer).unwrap();

    let distinct = |start: usize, end: usize| {
        for i in start..end {
            for j in i+1..end {
                if buffer[i] == buffer[j] {
                    return false;
                }
            }
        }
        return true;
    };

    for i in 14..=buffer.len() {
        if distinct(i - 14, i) {
            return i;
        }
    }

    return 0;
}

fn get_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    return BufReader::new(file);
}
