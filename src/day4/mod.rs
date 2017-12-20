mod test;

use std::path::Path;
use std::fs::File;
use std::io::Read;

fn is_valid_passphrase(line : &str) -> bool
{
    let words : Vec<_> = line.split_whitespace().collect();
    for i in 0..words.len()-1
    {
        if words[i+1..words.len()].contains(&words[i])
        {
            return false;
        }
    }
    true
}

fn is_valid_passphrase_no_anagrams(line : &str) -> bool
{
    let words : Vec<_> = line.split_whitespace()
        .map(|word| {
            let mut chars = word.chars().collect::<Vec<char>>();
            chars.sort();
            chars.into_iter().collect::<String>()} )
        .collect();

    //println!("Vector: {:?}",words);    
    
    for i in 0..words.len()-1
    {
        if words[i+1..words.len()].contains(&words[i])
        {
            return false;
        }
    }
    true
}

fn read_file(path : &Path) -> String
{
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn run_problem1()
{
    let buffer = read_file(Path::new("./src/day4/input1.txt"));
    let count = buffer.lines().filter(|line| is_valid_passphrase(line)).count();
    println!("Valid passphrases: {} ",count);
}

pub fn run_problem2()
{
    let buffer = read_file(Path::new("./src/day4/input1.txt"));
    let count = buffer.lines().filter(|line| is_valid_passphrase_no_anagrams(line)).count();
    println!("Valid passphrases: {} ",count);
}