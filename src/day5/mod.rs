mod test;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Debug)]
struct CPU {
    pc: isize,
    jump_list : Vec<isize>,
    upgrade : bool,
}

impl CPU {
    pub fn new(jump_list : Vec<isize>, upgrade : bool) -> CPU 
    {
        CPU{jump_list,pc : 0, upgrade}
    }

    /** Executes a list of jump instructons and returns
    a number of steeps needed to reach the exit (i.e. current 
    instruction goes outside of jump list)  **/
    pub fn execute(&mut self) -> u64
    {
        let mut couter = 0;
        while self.pc >= 0 && self.pc < self.jump_list.len() as isize
        {
            self.execute_jump();
            couter += 1;
        }
        couter
    }

    fn execute_jump(&mut self)
    {
        let offset = self.jump_list[self.pc as usize];
        if self.upgrade && offset >= 3 
        {
            self.jump_list[self.pc as usize] -= 1;
        }
        else {
            self.jump_list[self.pc as usize] += 1;
        }
        
        self.pc += offset; 
    }
}

pub fn run_problem1()
{
    let input = File::open("./src/day5/input.txt").unwrap();
    let br = BufReader::new(input);
    let list : Vec<isize> = br.lines().map(|x| x.unwrap().parse::<isize>().unwrap()).collect();
    let mut cpu = CPU::new(list,false);
    println!("Steps: {}",cpu.execute());
}


pub fn run_problem2()
{
    let input = File::open("./src/day5/input.txt").unwrap();
    let br = BufReader::new(input);
    let list : Vec<isize> = br.lines().map(|x| x.unwrap().parse::<isize>().unwrap()).collect();
    let mut cpu = CPU::new(list,true);
    println!("Steps: {}",cpu.execute());
}
