use std::fs::File;
use std::io::Read;

// write a function to read from file into int vector, split by blank line
fn read_file(filename: &str) -> Vec<i32> {
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut count: i32 = 0;
    let mut v = Vec::new();
    for line in contents.lines() {
        if line == "" {
            v.push(count);
            count = 0;
            continue;
        }
        let num: i32 = line.parse().unwrap();
        count += num;
        v.push(num);
    }
    v
}

fn main() {
    let mut calories = read_file("input.txt");
    calories.sort();
    calories.reverse();
    let mut res = 0;
    for i in 0..3 {
        res += calories[i];
    }
    // let minValue = calories.iter().max();

    // println!("max value is {}", minValue.unwrap());
    println!("total of max 3 elf is {}", res);
}
