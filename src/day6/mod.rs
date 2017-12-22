mod test;

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::cmp::Ordering;

struct Memory
{
    banks : Vec<u16>,
}

impl Memory {
    pub fn new(input : &str) -> Memory
    {
        let blocks = input.split_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        Memory{banks : blocks}
    }

    fn max(&mut self) -> (usize,u16)
    {
        let max = self.banks.iter_mut()
            .enumerate()
            .max_by(|x,y| (x.1.cmp(&y.1)).then_with(|| y.0.cmp(&x.0)))
            .unwrap();
        let max_value = *max.1;
        *max.1 = 0;
        (max.0,max_value)
    }

    fn reallocate(&mut self)
    {
        let (max_index,max_value) = self.max();

        let idx_range = (0..self.banks.len()).cycle().skip(max_index+1);
        for (_,idx) in (0..max_value).zip(idx_range)
        {
            self.banks[idx] += 1;
        }
      
    }

    pub fn how_many_cycles(&mut self) -> u32
    {
        let mut counter = 0;
        let mut states : Vec<Vec<u16>> = Vec::new();
        loop {
            self.reallocate();
            counter += 1;
            if states.contains(&self.banks)
            {
                return counter;   
            }
            states.push(self.banks.clone());
        }
    }

    pub fn how_many_cycles_again(&mut self) -> u32
    {
        let mut counter = 0;
        let mut states : Vec<Vec<u16>> = Vec::new();
        let again_state;

        loop {
            self.reallocate();
            if states.contains(&self.banks)
            {
                again_state = self.banks.clone();   
                break;
            }
            states.push(self.banks.clone());
        }

        loop {
            self.reallocate();
            counter += 1;
            if self.banks.cmp(&again_state) == Ordering::Equal
            {
                return counter;   
            }
        }


    }

}

pub fn run_problem1()
{
    let f = File::open("./src/day6/input.txt").unwrap();
    let mut bf = BufReader::new(f);
    let mut string = String::new();
    bf.read_line(&mut string).unwrap();
    let mut memory = Memory::new(&mut string);
    println!("Number of cycles: {}", memory.how_many_cycles());
}

pub fn run_problem2()
{
    let f = File::open("./src/day6/input.txt").unwrap();
    let mut bf = BufReader::new(f);
    let mut string = String::new();
    bf.read_line(&mut string).unwrap();
    let mut memory = Memory::new(&mut string);
    println!("Number of cycles to reach the same state: {}", memory.how_many_cycles_again());
}