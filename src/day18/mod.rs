mod test;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug,Copy,Clone)]
pub enum Operand
{
    Register(char),
    Literal(i64),
}

impl Operand {
    fn from_str(op : &str)->Operand
    {
        if let Some(literal) = op.parse::<i64>().ok()
        {
            return Operand::Literal(literal);
        }
        return Operand::Register(op.chars().nth(0).unwrap());
    }
}


#[derive(Debug,Copy,Clone)]
pub enum Instruction
{
    Snd{op: Operand},
    Set{reg: char, op: Operand},
    Add{reg: char, op: Operand},
    Mul{reg: char, op: Operand},
    Mod{reg: char, op: Operand},
    Rcv{reg: char},
    Jgz{reg: char, op: Operand}
}

impl Instruction {
    pub fn from_str(instr : &str)->Instruction
    {
        let mut instruction = instr.split_whitespace();
        match instruction.next().unwrap() {
            "snd" => Instruction::Snd{op: Operand::from_str(instruction.next().unwrap())},
            "set" => 
            {
                let reg = instruction.next().unwrap().chars().nth(0).unwrap();
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Set{reg,op}
            },
            "add" => 
            {
                let reg = instruction.next().unwrap().chars().nth(0).unwrap();
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Add{reg,op}
            },
            "mul" => 
            {
                let reg = instruction.next().unwrap().chars().nth(0).unwrap();
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Mul{reg,op}
            },
            "mod" => 
            {
                let reg = instruction.next().unwrap().chars().nth(0).unwrap();
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Mod{reg,op}
            },
            "rcv" => Instruction::Rcv{reg : instruction.next().unwrap().chars().nth(0).unwrap()},
            "jgz" => 
            {
                let reg = instruction.next().unwrap().chars().nth(0).unwrap();
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Jgz{reg,op}
            },
            x => panic!("Invalid instruction {}",x)
        }
    }
}


pub struct Computer<'a>
{
    pc : isize,
    register : HashMap<char,i64>,
    program : &'a Vec<Instruction>,
    running : bool,
    message_queue : VecDeque<i64>,
    id : i64
}

impl<'a> Computer<'a> {
    pub fn new(program : &'a Vec<Instruction>, id : i64) -> Computer
    {
        let mut register = HashMap::new();
        register.insert('p',id);
        Computer{pc : 0, register, program, running : true, message_queue : VecDeque::new(),id}
    }

    pub fn parse_program(program : &str) -> Vec<Instruction>
    {
        program.lines().map(|line| Instruction::from_str(line)).collect()
    }
    pub fn is_running(&self) -> bool
    {
        self.running
    }

    pub fn add_to_queue(&mut self, val : i64)
    {
        self.message_queue.push_back(val);
    }

    fn increment_pc(&mut self)
    {
        self.pc += 1;
    }

    fn get_val_from_operand(&self, op : &Operand) -> i64 
    {
        match op {
            &Operand::Literal(literal) => literal,
            &Operand::Register(register) => *self.register.get(&register).unwrap_or(&0),
        }
    }

    fn fetch(&mut self) -> Option<Instruction>
    {
        match self.program.get(self.pc as usize) 
        {
            Some(&instruction) => 
            {
                Some(instruction)
            },
            None => None,
        }
    }

    fn execute_instruction(&mut self ) -> Option<i64>
    {
        let instruction = match self.fetch()
        {
            Some(instruction) => instruction,
            None => 
            {
                self.running = false;
                return None;
            },
        };


        match instruction
        {
            Instruction::Snd{op} => 
            {
                self.increment_pc();
                let val = self.get_val_from_operand(&op);
                return Some(val);
            },
            Instruction::Set{reg,op} => 
            {
                self.increment_pc();
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg = val;
                return None;
            },
            Instruction::Add{reg,op} =>
            {
                self.increment_pc();
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg += val;
                return None;
            },
            Instruction::Mul{reg,op} =>
            {
                self.increment_pc();
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg *= val;
                return None;
            },
            Instruction::Mod{reg,op} =>  
            {
                self.increment_pc();
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg %= val;
                return None;
            },
            Instruction::Jgz{reg,op} => 
            {
                match self.register.get(&reg)
                {
                    Some(&x) => if x > 0
                    {
                        let offset = self.get_val_from_operand(&op);
                        self.pc += offset as isize;
                        return None;
                    },
    	            None => {},
                }
                self.increment_pc();
                return None;
            },
            Instruction::Rcv{reg} => 
            {
                match self.message_queue.pop_front() 
                {
                    Some(val) => 
                    {
                        self.register.insert(reg,val);
                        self.increment_pc();
                        self.running = true;
                    },
                    None => 
                    {
                        self.running = false;
                    },
                }
                return None;
            },
        }
    }


}

pub fn run_problem2() {
    let input = include_str!("input.txt");
    let instructions = Computer::parse_program(input);

    let mut comp0 = Computer::new(&instructions,0);
    let mut comp1 = Computer::new(&instructions,1);
    let mut count = 0;
    while comp0.is_running() || comp1.is_running()
    {
        let status = (comp0.execute_instruction(),comp1.execute_instruction());
        match status 
        {
            (Some(val), None) => 
            {
                //println!("Comp0 sending {}",val);
                comp1.add_to_queue(val);
            },
            (None, Some(val)) => {
                //println!("Comp1 sending {}",val);
                comp0.add_to_queue(val);
                count += 1;
            },
            (Some(val0),Some(val1)) => 
            {
                //println!("Comp0 sending {}",val0);
                //println!("Comp1 sending {}",val1);
                comp0.add_to_queue(val1);
                comp1.add_to_queue(val0);
                count += 1;
            },
            _ => {} ,
        }
    }
    println!("Program 1 send a value {} times.", count);
    
}