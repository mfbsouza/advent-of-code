use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process;

fn main() -> io::Result<()> {
    let path: String = env::args().nth(1).unwrap_or_else(|| {
        println!("Error: expected more command line arguments");
        println!("Syntax: {} </path/to/input>", env::args().next().unwrap());
        process::exit(1);
    });

    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut numbers: Vec<u32> = Vec::new();
    for ln in reader.lines() {
        let line = ln?;
        numbers.push(parse_line(&line).unwrap());
    }
    let sum: u32 = numbers.iter().sum();
    println!("the sum is: {}", sum);
    Ok(())
}

fn parse_line(line: &str) -> Option<u32> {
    let mut number = parse_first_digit(line).unwrap() * 10;
    number += reverse_parse_first_digit(line).unwrap();
    Some(number)
}

fn parse_first_digit(string: &str) -> Option<u32> {
    for (i, char) in string.chars().enumerate() {
        if char.is_ascii_digit() {
            return char.to_digit(10)
        }
        match parse_spelled_out_digit(&string[i..string.len()]) {
            Some(n) => return Some(n),
            None => continue,
        }
    }
    None
}

fn reverse_parse_first_digit(string: &str) -> Option<u32> {
    for (i, char) in string.chars().rev().enumerate() {
        if char.is_ascii_digit() {
            return char.to_digit(10)
        }
        match parse_spelled_out_digit(&string[string.len()-(i+1)..string.len()]) {
            Some(n) => return Some(n),
            None => continue,
        }
    }
    None
}

fn parse_spelled_out_digit(string: &str) -> Option<u32> {
    let dic = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    if string.len() > 1 {
        for (i, item) in dic.iter().enumerate() {
            if item[0..1] == string[0..1] {
                if item.len() > string.len() {
                    continue;
                }
                if item[..] == string[0..item.len()] {
                    let number = u32::try_from(i+1).expect("error converting usize to u32");
                    return Some(number);
                }
            }
        }
    }
    None
}
