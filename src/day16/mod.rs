mod test;

use std::collections::VecDeque;

#[derive(Debug,PartialEq)]
pub enum DanceMove
{
  Spin(u8),
  Exchange((u8,u8)),
  Partner((char,char)),
}

impl DanceMove {
    pub fn from_str(step : &str) -> DanceMove
    {
        //sX xA/B pA/B
        match step.get(0..1).unwrap() 
        {
            "s" => DanceMove::Spin(step.get(1..).unwrap().parse().unwrap()),
            "x" => {
                let mut positions = step.get(1..).unwrap().split("/");
                DanceMove::Exchange((positions.next().unwrap().parse().unwrap(),
                                    positions.next().unwrap().parse().unwrap())) 
            },
            "p" => 
            {
                let mut programs = step.get(1..).unwrap().split("/");
                DanceMove::Partner((programs.next().unwrap().chars().nth(0).unwrap(),
                                    programs.next().unwrap().chars().nth(0).unwrap()))
            }
            x => panic!("Invalid step {}", x),
        }
    }
}

pub struct Dance
{
    list : VecDeque<char>
}

impl Dance {
    pub fn new() -> Dance
    {
        let list = (0..16).into_iter().map(|i| (('a' as u8) + i) as char).collect();
        Dance{list}
    }
    fn from_vec(list : VecDeque<char>) -> Dance
    {
        Dance{list}
    }

    fn move_step(&mut self, step : &DanceMove)
    {
        match *step
        {
            DanceMove::Spin(num) => 
            {
                for _ in 0..num
                {
                    let c = self.list.pop_back().unwrap();
                    self.list.push_front(c);
                }
            },
            DanceMove::Exchange((i,j)) => self.list.swap(i as usize, j as usize),
            DanceMove::Partner((a,b)) => 
            {
                let i = self.list.iter().position(|&c| c==a).unwrap();
                let j = self.list.iter().position(|&c| c==b).unwrap();
                self.list.swap(i,j); 
            }
        }
    }

    pub fn dance(&mut self, steps : &Vec<DanceMove>)
    {
        for step in steps
        {
            self.move_step(step);
        }
    }

    pub fn str_to_dance_moves(input : &str) -> Vec<DanceMove>
    {
        input.split(",").map(|step| DanceMove::from_str(step)).collect()
    }

    pub fn get_count_to_cycle(&mut self, steps : &Vec<DanceMove>) -> u32 
    {
        let mut count = 0;
        let clone = self.list.clone();
        loop {
            self.dance(steps);
            count += 1;
            if self.list == clone 
            {
                return count;
            }
        }
    } 
}

pub fn run_problem1(){
    let input = include_str!("input.txt");
    let mut dance = Dance::new();
    dance.dance(&Dance::str_to_dance_moves(&input));
    println!("Order after dance: {}", dance.list.iter().collect::<String>());
}

pub fn run_problem2(){
    let input = include_str!("input.txt");
    let mut dance = Dance::new();
    let steps = Dance::str_to_dance_moves(&input);
    //run dance billion times ... to slow to execute
    //instead calculate how many steps it takes that sequence of steps begins repeating
    let cycle_count = dance.get_count_to_cycle(&steps);
    let num_dances = 1000000000 % cycle_count;
    for _ in 0..num_dances 
    {
        dance.dance(&steps);
    }
    println!("Order after dance: {}", dance.list.iter().collect::<String>());
}