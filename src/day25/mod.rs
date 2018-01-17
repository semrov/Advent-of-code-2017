use std::collections::HashSet;

#[derive(Debug,PartialEq,Clone,Copy)]
enum State
{
    A,
    B,
    C,
    D,
    E,
    F
}

struct TurningMachine
{
    tape : HashSet<isize>,
    cursor : isize,
    state : State,
}

impl TurningMachine {
    pub fn new() -> TurningMachine
    {
        TurningMachine
        {
            tape : HashSet::new(),
            cursor : 0,
            state : State::A,
        }
    }

    #[inline]
    fn move_left(&mut self)
    {
        self.cursor -= 1;
    }

    #[inline]
    fn move_right(&mut self)
    {
        self.cursor += 1;
    }

    fn next(&mut self)
    {
        match self.state
        {
            State::A =>
            {
                if self.tape.contains(&self.cursor)
                {
                    self.tape.remove(&self.cursor);
                    self.move_left();
                    self.state = State::C;
                }
                else 
                {
                    self.tape.insert(self.cursor);
                    self.move_right();
                    self.state = State::B;
                }
            },
            State::B => 
            {
                if self.tape.contains(&self.cursor)
                {
                    self.move_right();
                    self.state = State::D;
                }
                else
                {
                    self.tape.insert(self.cursor);
                    self.move_left();
                    self.state = State::A;
                }
            },
            State::C => 
            {
                if self.tape.contains(&self.cursor)
                {
                    self.tape.remove(&self.cursor);
                    self.move_left();
                    self.state = State::E;
                }
                else
                {
                    self.tape.insert(self.cursor);
                    self.move_right();
                    self.state = State::A;
                }
            },
            State::D => 
            {
                if self.tape.contains(&self.cursor)
                {
                    self.tape.remove(&self.cursor);
                    self.state = State::B;
                }
                else
                {
                    self.tape.insert(self.cursor);
                    self.state = State::A;
                }
                self.move_right();
            },
            State::E => 
            {
                if self.tape.contains(&self.cursor)
                {
                    self.state = State::C;
                }
                else
                {
                    self.tape.insert(self.cursor);
                    self.state = State::F;
                }
                self.move_left();
            },
            State::F => 
            {
                if self.tape.contains(&self.cursor)
                {
                    self.state = State::A;
                }
                else
                {
                    self.tape.insert(self.cursor);
                    self.state = State::D;
                }
                self.move_right();
            },
        }
    }

    pub fn run(&mut self, n : u32)
    {
        for _ in 0..n
        {
            self.next();
        }
    }

    pub fn diagnostic_checksum(&self) -> u64
    {
        self.tape.len() as u64
    }
}

pub fn run_problem1()
{
    let mut tm = TurningMachine::new();
    tm.run(12173597);
    println!("Problem1: Diagnostic checksum is {}", tm.diagnostic_checksum());
}