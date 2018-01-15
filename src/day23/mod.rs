mod test;
use primal;


#[derive(Debug,Copy,Clone)]
pub enum Operand
{
    Register(u8),
    Literal(i64),
}

impl Operand {
    fn from_str(op : &str)->Operand
    {
        if let Some(literal) = op.parse::<i64>().ok()
        {
            return Operand::Literal(literal);
        }
        return Operand::Register((op.chars().nth(0).unwrap() as u8) - ('a' as u8));
    }

    fn get_reg_name(&self) -> u8
    {
        match self 
        {
            &Operand::Register(reg) => reg,
            &Operand::Literal(_) => panic!("Not a register!"),
        }
    }
}


#[derive(Debug,Copy,Clone)]
pub enum Instruction
{
    Set{reg: Operand, op: Operand},
    Sub{reg: Operand, op: Operand},
    Mul{reg: Operand, op: Operand},
    Jnz{reg: Operand, op: Operand}
}

impl Instruction {
    pub fn from_str(instr : &str)->Instruction
    {
        let mut instruction = instr.split_whitespace();
        match instruction.next().unwrap() {
            "set" => 
            {
                let reg = Operand::from_str(instruction.next().unwrap());
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Set{reg,op}
            },
            "sub" => 
            {
                let reg = Operand::from_str(instruction.next().unwrap());
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Sub{reg,op}
            },
            "mul" => 
            {
                let reg = Operand::from_str(instruction.next().unwrap());
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Mul{reg,op}
            },
            "jnz" => 
            {
                let reg = Operand::from_str(instruction.next().unwrap());
                let op = Operand::from_str(instruction.next().unwrap());
                Instruction::Jnz{reg,op}
            },
            x => panic!("Invalid instruction {}",x)
        }
    }
}

pub struct Coprocessor
{
    pc : isize,
    register : [i64; 8],
    program : Vec<Instruction>,
    mul_count : u32,
    running : bool,
}

impl Coprocessor {
    pub fn new(program : &str) -> Coprocessor
    {
        let program = program.lines()
                             .map(|line| Instruction::from_str(line.trim()))
                             .collect();
        Coprocessor{program, register : [0; 8], pc : 0, mul_count : 0, running : false}
    }

    #[inline]
    fn increment_pc(&mut self)
    {
        self.pc += 1;
    }

    fn get_val_from_operand(&self, op : &Operand) -> i64
    {
        match op {
            &Operand::Register(reg) => self.register[reg as usize],
            &Operand::Literal(literal) => literal 
        }
    }

    fn instruction_fetch(&mut self) -> Option<Instruction>
    {
        match self.program.get(self.pc as usize) 
        {
            Some(&instruction) => 
            {
                self.increment_pc();
                Some(instruction)
            }
            None => None,
        }
    }

    fn execute_instruction(&mut self) -> bool
    {
        let instruction = match self.instruction_fetch()
        {
            Some(instruction) => instruction,
            None => return false,
        };

        match instruction 
        {
            Instruction::Set{reg,op} => 
            {
                self.register[reg.get_reg_name() as usize] = self.get_val_from_operand(&op);
            },
            Instruction::Sub{reg,op} => 
            {
                self.register[reg.get_reg_name() as usize] -= self.get_val_from_operand(&op);
            },
            Instruction::Mul{reg,op} =>
            {
                self.register[reg.get_reg_name() as usize] *= self.get_val_from_operand(&op);
                self.mul_count += 1;
            },
            Instruction::Jnz{reg,op} =>
            {
                let reg = self.get_val_from_operand(&reg);
                if reg != 0
                {
                    self.pc += (self.get_val_from_operand(&op)-1) as isize;
                }
            }
        }
        true
    }

    fn run(&mut self)
    {
        self.running = true;
        while self.running 
        {
            self.running = self.execute_instruction();
        }
    }
}

pub fn run_problem1()
{
    let input = include_str!("input.txt");
    let mut coprocessor = Coprocessor::new(input);
    coprocessor.run();

    println!("Times mul instruction invoked: {} ",coprocessor.mul_count);
}

pub fn run_problem2()
{
    let input = include_str!("input3.txt");
    //let mut coprocessor = Coprocessor::new(input);
    //coprocessor.register[0] = 1;
    ///coprocessor.run();
     println!(
        "P2: {}",
        (0u64..1001)
            .filter(|bb| !primal::is_prime(108_400 + 17 * bb))
            .count()
    );
    //println!("Value of register h: {} ",coprocessor.register[('h' as usize) - ('a' as usize)]);
}


