mod test;

use std::collections::HashMap;

type Position = (isize,isize);

#[derive(Debug,Copy,Clone,PartialEq)]
enum Direction
{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum Status
{
    Clean,
    Infected,
    Weakened,
    Flagged,
}

pub struct VirusCarrier
{
    curr_pos : Position,
    curr_dir : Direction,
    infection_map : HashMap<Position, Status>,
    infection_count : u32,
    evolved : bool
}

impl VirusCarrier {
    fn convert_line(line : &str) -> Vec<bool>
    {
        line.chars().map(|node| match node
        {
            '.' => false,
            '#' => true,
            x => panic!("Invalid infection status {}",x),
        }).collect()
    }

    pub fn from_map(map : &str, evolved : bool) -> VirusCarrier
    {
        let map = map.lines().map(|line| Self::convert_line(line.trim())).collect::<Vec<_>>();
        let width = map[0].len() as isize;
        let height = map.len() as isize;
        let mut infection_map = HashMap::with_capacity(100_000);
        

        for (inf_vector,row) in map.iter().zip((-(height/2)..((height/2)+1)).rev())
        {
            for (&is_infected, col) in inf_vector.iter().zip(-(width/2)..((width/2)+1))
            {
                if is_infected { infection_map.insert((row,col), Status::Infected);}
            }
        }

        VirusCarrier{curr_pos: (0,0), curr_dir: Direction::Up, infection_map,infection_count : 0,evolved}
    }

    fn move_forward(&mut self)
    {
        match self.curr_dir
        {
            Direction::Down => self.curr_pos.0 -= 1,
            Direction::Up => self.curr_pos.0 += 1,
            Direction::Left => self.curr_pos.1 -= 1,
            Direction::Right => self.curr_pos.1 += 1,
        }
    }

    fn turn_left(&mut self)
    {
        match self.curr_dir
        {
            Direction::Down => self.curr_dir = Direction::Right,
            Direction::Up => self.curr_dir = Direction::Left,
            Direction::Left => self.curr_dir = Direction::Down,
            Direction::Right => self.curr_dir = Direction::Up,
        }
    }

    fn reverse_direction(&mut self)
    {
        match self.curr_dir
        {
            Direction::Down => self.curr_dir = Direction::Up,
            Direction::Up => self.curr_dir = Direction::Down,
            Direction::Left => self.curr_dir = Direction::Right,
            Direction::Right => self.curr_dir = Direction::Left,
        }
    }

    fn turn_right(&mut self)
    {
        match self.curr_dir
        {
            Direction::Down => self.curr_dir = Direction::Left,
            Direction::Up => self.curr_dir = Direction::Right,
            Direction::Left => self.curr_dir = Direction::Up,
            Direction::Right => self.curr_dir = Direction::Down,
        }
    }

    pub fn burst(&mut self )
    {
        let status = match self.infection_map.get(&self.curr_pos)
        {
            None => Status::Clean,
            Some(&s) => {s},
        };

        match status
        {
            Status::Clean =>
            {
                self.turn_left();
                if self.evolved
                {
                    self.infection_map.insert(self.curr_pos,Status::Weakened);
                }
                else
                {
                    self.infection_map.insert(self.curr_pos,Status::Infected);
                    self.infection_count += 1;
                }
                
            },
            Status::Weakened => 
            {
                self.infection_map.insert(self.curr_pos,Status::Infected);
                self.infection_count += 1;
            },
            Status::Infected => 
            {
                self.turn_right();
                if self.evolved 
                {
                    self.infection_map.insert(self.curr_pos,Status::Flagged);
                }
                else
                {
                    self.infection_map.remove(&self.curr_pos);
                }
            },
            Status::Flagged => 
            {
                self.reverse_direction();
                self.infection_map.remove(&self.curr_pos);
            }
        }
        
        self.move_forward();
    }
    
}

pub fn run_problem1()
{
    let input = include_str!("input.txt");
    let mut vc = VirusCarrier::from_map(input,false);
    for _ in  0..10000
    {
        vc.burst();
    }

    println!("[Part 1] Times infected: {}",vc.infection_count );
}


pub fn run_problem2()
{
    let input = include_str!("input.txt");
    let mut vc = VirusCarrier::from_map(input,true);
    for _ in  0..10000000
    {
        vc.burst();
    }

    println!("[Part 2] Times infected: {}",vc.infection_count );
}