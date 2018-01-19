mod test;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender,Receiver,channel};

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
    Jgz{op1: Operand, op2: Operand}
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
                let op1 = Operand::from_str(instruction.next().unwrap());
                let op2 = Operand::from_str(instruction.next().unwrap());
                Instruction::Jgz{op1,op2}
            },
            x => panic!("Invalid instruction {}",x)
        }
    }
}


pub struct Computer
{
    pc : isize,
    register : HashMap<char,i64>,
    program : Vec<Instruction>,
    running : bool,
    values_send : u32,
    receiver : Receiver<i64>,
    sender : Option<Sender<i64>>,
    id : i64
}

impl Computer {
    pub fn new(program : Vec<Instruction>, id : i64) -> (Computer,Sender<i64>)
    {
        let (sender,receiver) = channel();
        let mut register = HashMap::new();
        register.insert('p',id);
        (Computer{pc : 0, register, program, running : true,
            values_send : 0, receiver, sender : None,id}, sender)
    }

    pub fn parse_program(program : &str) -> Vec<Instruction>
    {
        program.lines().map(|line| Instruction::from_str(line)).collect()
    }
    pub fn is_running(&self) -> bool
    {
        self.running
    }

    pub fn set_sender(&mut self, sender : Sender<i64>)
    {
        self.sender = Some(sender);
    }

    fn send_val(&self, value : i64)
    {
        self.sender.as_ref().expect(&format!("Computer {} has no sender!",self.id)).send(value).unwrap();
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

    fn execute_instruction(&mut self)
    {
        let instruction = match self.fetch()
        {
            Some(instruction) => instruction,
            None => 
            {
                self.running = false;
                return;
            },
        };

        match instruction
        {
            Instruction::Snd{op} => 
            {
                self.values_send += 1;
                let val = self.get_val_from_operand(&op);
                self.send_val(val);
            },
            Instruction::Set{reg,op} => 
            {
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg = val;
            },
            Instruction::Add{reg,op} =>
            {
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg += val;
            },
            Instruction::Mul{reg,op} =>
            {
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg *= val;
            },
            Instruction::Mod{reg,op} =>  
            {
                let val = self.get_val_from_operand(&op);
                let mut reg = self.register.entry(reg).or_insert(0); 
                *reg %= val;
            },
            Instruction::Jgz{op1,op2} => 
            {
                let condition_val = self.get_val_from_operand(&op1);
                if condition_val > 0
                {
                    let offset = self.get_val_from_operand(&op2);
                    self.pc += offset as isize;
                    return;
                }
            },
            Instruction::Rcv{reg} => 
            {
                match self.receiver.recv_timeout(Duration::from_secs(5))
                {
                    Ok(val) => 
                    {
                        self.register.insert(reg,val);
                        self.running = true;
                    },
                    _ => 
                    {
                        self.running = false;
                    },
                }    
            },
        }
        self.increment_pc();
    }

    pub fn run_channels(&mut self)
    {
        while self.is_running() {
            self.execute_instruction();
        }
    }


}

pub fn run_problem2() {
    let input = include_str!("input.txt");
    let instructions = Computer::parse_program(input);

    let (mut comp0,sender0) = Computer::new(instructions.clone(),0);
    let (mut comp1,sender1) = Computer::new(instructions.clone(),1);
    comp0.set_sender(sender1);
    comp1.set_sender(sender0);
    let t1 = thread::spawn(move || comp0.run_channels());
    let t2 = thread::spawn(move || {
        comp1.run_channels();
        println!("Send called times: {}", comp1.values_send);
        }
    );

    // solution problem2: 6858
    t1.join().expect("Thread 1 failed");
    t2.join().expect("Thread 2 failed");    
}