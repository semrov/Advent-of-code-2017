/*
The spreadsheet consists of rows of apparently-random numbers. 
To make sure the recovery process is on the right track, they need you
 to calculate the spreadsheet's checksum. For each row, determine the difference 
 between the largest value and the smallest value; the checksum 
 is the sum of all of these differences.

For example, given the following spreadsheet:

5 1 9 5
7 5 3
2 4 6 8

    The first row's largest and smallest values are 9 and 1, and their difference is 8.
    The second row's largest and smallest values are 7 and 3, and their difference is 4.
    The third row's difference is 6.

In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
*/
mod test;

use std::path::Path;
use std::fs::File;
use std::io::{Error};
use std::result::Result;
use std::io::Read;



pub fn get_row_diff(line : &str) -> i32
{
    let mut min = ::std::i32::MAX;
    let mut max = ::std::i32::MIN+1; 
    for num in line.split_whitespace()
    {
        let val = num.parse::<i32>().unwrap();
        if val > max {max = val;}
        if val < min {min = val;}
    }
    max - min
}

pub fn get_row_dividable(line : &str) -> i32
{
    let v : Vec<i32> = line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    for i in 0..v.len() 
    {
        for j in 0..v.len()
        {
            if v[i] % v[j] == 0 && i != j
            {
                return v[i] / v[j];
            }
        }
    }
    0
}


fn read_file_as_string(path : &Path) -> Result<String, Error>
{
    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn calculate_checksum<F>(spread_sheet : &str, cheksum_function : F) -> i32 
    where F: Fn(&str) -> i32
{
    spread_sheet.lines().map(|line| cheksum_function(line)).sum()
}

pub fn run_part1()
{
    let path = Path::new("./src/day2/input.txt");
    let checksum = match read_file_as_string(path)
    {
        Ok(data) => calculate_checksum(&data,get_row_diff),
        Err(_) => panic!("Error reading file {:?}",path.file_name().unwrap()),
    };
    println!("Cheksum part 1: {}",checksum);
}


pub fn run_part2()
{
    let path = Path::new("./src/resorces_day2/input2.txt");
    let checksum = match read_file_as_string(path)
    {
        Ok(data) => calculate_checksum(&data,get_row_dividable),
        Err(_) => panic!("Error reading file {:?}",path.file_name().unwrap()),
    };
    println!("Cheksum part 2: {}",checksum);
}