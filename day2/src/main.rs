use std::fs::File;
use std::io::Read;

// write a function to read from file into vector of vector. each line only have two char, sepreted by
// blank
fn read_file(filename: &str) -> Vec<Vec<char>> {
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in content.lines() {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if c != ' ' {
                chars.push(c);
            }
        }
        lines.push(chars);
    }
    lines
}

// fn read_file(filename: &str) -> Vec<Split<char>> {
//     let mut file = File::open(filename).expect("file not found");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)
//         .expect("something went wrong reading the file");
//
//     let mut v = Vec::new();
//     for line in contents.lines() {
//         v.push(line.split(' '));
//     }
//     v
// }

enum Result {
    Win,
    Lose,
    Draw,
}

fn is_win(op: char, me: char) -> Result {
    match op {
        'A' => match me {
            'X' => Result::Draw,
            'Y' => Result::Win,
            'Z' => Result::Lose,
            _ => Result::Draw,
        },
        'B' => match me {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => Result::Draw,
        },
        'C' => match me {
            'X' => Result::Win,
            'Y' => Result::Lose,
            'Z' => Result::Draw,
            _ => Result::Draw,
        },
        _ => panic!("invalid input"),
    }
}

fn cal_outcome_score(op: char, me: char) -> i32 {
    match is_win(op, me) {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Lose => 0,
    }
}

fn cal_score(op: char, me: char) -> i32 {
    let mut res = cal_outcome_score(op, me);
    match me {
        'X' => res += 1,
        'Y' => res += 2,
        'Z' => res += 3,
        _ => panic!("invalid input"),
    }
    res
}

fn main() {
    let s = read_file("input.txt");
    let mut res = 0;
    for round in s {
        let (op, me) = (round[0], round[1]);
        res += cal_score(op, me)
    }
    println!("{}", res);
}
