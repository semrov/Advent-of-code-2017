mod test;
use std;

const FACTOR_A: u32 = 16807;
const FACTOR_B: u32 = 48271;
const MULTIPLE_OF_A: u32 = 4;
const MULTIPLE_OF_B: u32 = 8;
const MODULO: u64 = std::i32::MAX as u64;
const JUDGE_SAMPLE1 : u32 = 40_000_000;
const JUDGE_SAMPLE2 : u32 = 5_000_000;

pub trait Generate {
    fn next(&mut self) -> u32;
}


pub struct BasicGenerator
{
    previous_value : u32,
    factor : u32,
}

pub struct UpgradedGenerator
{
    generator : BasicGenerator,
    multiple_of : u32
}

impl BasicGenerator {
    pub fn new(start_value : u32, factor : u32) -> BasicGenerator
    {
        BasicGenerator{previous_value : start_value, factor}
    }    
}

impl UpgradedGenerator {
    pub fn new(start_value : u32, factor : u32, multiple_of : u32) -> UpgradedGenerator
    {
        let generator = BasicGenerator::new(start_value,factor);
        UpgradedGenerator{generator, multiple_of}
    }    
}

impl Generate for BasicGenerator {
    fn next(&mut self) -> u32 
    {
        let next_val = ((self.previous_value as u64) * (self.factor as u64) % MODULO) as u32;
        self.previous_value = next_val;
        next_val
    }
}

impl Generate for UpgradedGenerator {
    fn next(&mut self) -> u32
    {
        loop {
            let next_val = self.generator.next();
            if next_val != 0 && next_val % self.multiple_of == 0 
            { return next_val; }
        }
    }
}

pub struct Judge {
    generator_a : Box<Generate>,
    generator_b : Box<Generate>,
    total_matches: u32
}

impl Judge {
    pub fn new(generator_a : Box<Generate>, generator_b : Box<Generate>) -> Judge
    {
        Judge{generator_a,generator_b,total_matches : 0}
    }

    pub fn judge_next_values(&mut self) -> bool
    {
        if self.generator_a.next() & 0x0000FFFF == self.generator_b.next() & 0x0000FFFF
        {
            self.total_matches += 1;
            return true;
        } 
        false
    }

    pub fn judge_n_times(&mut self, n : u32) -> u32
    {
        for _ in 0..n 
        {
            self.judge_next_values();
        }
        self.total_matches
    }
}

pub fn run_problem1()
{
    //inputs
    //Generator A starts with 516
    //Generator B starts with 190
    let ga = Box::new(BasicGenerator::new(516,FACTOR_A));
    let gb = Box::new(BasicGenerator::new(190,FACTOR_B));
    let mut judge = Judge::new(ga,gb);
    let matches = judge.judge_n_times(JUDGE_SAMPLE1);
    println!("Number of matches: {}", matches);
}


pub fn run_problem2()
{
    //inputs
    //Generator A starts with 516
    //Generator B starts with 190
    let ga = Box::new(UpgradedGenerator::new(516,FACTOR_A,MULTIPLE_OF_A));
    let gb = Box::new(UpgradedGenerator::new(190,FACTOR_B,MULTIPLE_OF_B));
    let mut judge = Judge::new(ga,gb);
    let matches = judge.judge_n_times(JUDGE_SAMPLE2);
    println!("Number of matches: {}", matches);
}
