mod test;

use std::collections::HashMap;

#[derive(Debug)]
enum Modifier
{
    Increment(i32),
    Decrement(i32)
}

impl Modifier {
    fn from(modifier_string : &str, value : i32) -> Modifier
    {
        match modifier_string {
            "inc" => Modifier::Increment(value),
            "dec" => Modifier::Decrement(value),
            _ => panic!("Invalid modifier {}", modifier_string),
        }
    }
}

#[derive(Debug,Clone,Copy)]
enum Comparison {
    LessThan,
    LessOrEqual,
    Equal,
    NotEqual,
    GreaterOrEqual,
    Greater,
}

impl Comparison {
    fn from(cmp_string : &str) -> Comparison 
    {
        match cmp_string {
            "<"  => Comparison::LessThan,
            "<=" => Comparison::LessOrEqual,
            "==" => Comparison::Equal,
            "!=" => Comparison::NotEqual,
            ">=" => Comparison::GreaterOrEqual,
            ">"  => Comparison::Greater,
            _ => panic!("Invalid comparison string: {}", cmp_string),
        }
    }
}

#[derive(Debug)]
struct Condition {
    register : String,
    cmp : Comparison,
    value : i32
}

#[derive(Debug)]
struct Instruction
{
    register : String,
    modifier : Modifier,
    condition : Condition,
}

impl Instruction {
    fn from_str(instruction : &str) -> Instruction
    {
        // t inc -927 if t < -2695
        let parts = instruction.split_whitespace().collect::<Vec<&str>>();
        Instruction
        {
            register : parts[0].to_owned(),
            modifier : Modifier::from(parts[1],parts[2].parse().unwrap()),
            condition : Condition
                {
                    register : parts[4].to_owned(),
                    cmp : Comparison::from(parts[5]),
                    value : parts[6].parse().unwrap(),
                }
        }
    } 
}




pub struct CPU 
{
    registers : HashMap<String,i32>,
    max_val_ever : i32,
}

impl CPU {
    pub fn new() -> CPU
    {
        CPU{ registers : HashMap::new(), max_val_ever : 0 }
    }

/*
#[derive(Debug)]
struct Condition {
    register : String,
    cmp : Comparison,
    value : i32
}

*/
    fn check_condition(&self, condition : &Condition) -> bool
    {

        let reg_val = *self.registers.get(&condition.register).unwrap_or(&0);

        match condition.cmp 
        {
            Comparison::LessThan => reg_val < condition.value,
            Comparison::LessOrEqual => reg_val <= condition.value,
            Comparison::Equal => reg_val == condition.value,
            Comparison::NotEqual => reg_val != condition.value,
            Comparison::GreaterOrEqual => reg_val >= condition.value,
            Comparison::Greater => reg_val > condition.value,
        }

    }

    fn execute(&mut self, instructions : Vec<Instruction>)
    {
        
        // t inc -927 if t < -2695
        instructions.iter().for_each(|instruction|
        {
            //println!("{:?}", instruction);
            if self.check_condition(&instruction.condition)
            {
                let register_val = self.registers
                .entry(instruction.register.to_owned())
                .or_insert(0);

                match instruction.modifier
                {
                    Modifier::Decrement(ref val) => *register_val -= *val,
                    Modifier::Increment(ref val) => *register_val += *val,
                }

                if self.max_val_ever < *register_val 
                {
                    self.max_val_ever = *register_val;
                }
            }
        })
    }

}

fn get_instrutions(input : &str) -> Vec<Instruction>
{
    input.lines()
    .map(|line| Instruction::from_str(line))
    .collect()
}

fn largest_value(input : &str) -> i32
{
    let instructions = get_instrutions(input);

    let mut cpu = CPU::new();
    cpu.execute(instructions);
    *cpu.registers.values().max().unwrap()
}

fn largest_value_ever(input : &str) -> i32
{
    let instructions = get_instrutions(input);

    let mut cpu = CPU::new();
    cpu.execute(instructions);
    cpu.max_val_ever
}


pub fn run_problem1()
{
    let input = include_str!("./input.txt");
    println!("Largest value:  {}", largest_value(input));
}

pub fn run_problem2()
{
    let input = include_str!("./input.txt");
    println!("Largest value ever:  {}", largest_value_ever(input));
}