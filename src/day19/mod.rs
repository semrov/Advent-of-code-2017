mod test;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Line
{
    VerticalBar,
    Minus,
    Plus,
    Whitespace,
    Letter(char),
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum Direction
{
    Up,
    Down,
    Left,
    Right,
}


pub struct Network
{
    diagram : Vec<Vec<Line>>,
    curr_pos : (usize,usize),
    curr_dir : Direction,
}

impl Network {
    fn from_str(input : &str) -> Network
    {
        let mut diagram : Vec<Vec<Line>> = Vec::new();
        for line in input.lines()
        {
            let line = line.chars().map(|c| {
                match c
                {
                    '|' => Line::VerticalBar,
                    '-' => Line::Minus,
                    '+' => Line::Plus,
                    x if x.is_whitespace() => Line::Whitespace,
                    x if x.is_alphabetic() => Line::Letter(x),
                    x => panic!("Invalid character {}",x),
                }
            }).collect();
            diagram.push(line);
        }

        let curr_pos = Self::find_start(&diagram);    
        Network{diagram,curr_pos,curr_dir : Direction::Down}
    }

    fn find_start(diagram : &Vec<Vec<Line>>) -> (usize,usize)
    {
        let y = diagram.get(0).unwrap().iter().position(|&c| c==Line::VerticalBar).unwrap();
        (0,y)
    }

    fn is_valid_coord(&self, row : isize, col : isize ) -> bool
    {
        row  >= 0 && (row as usize) <  self.diagram.len() 
        && col >= 0 && (col as usize) < self.diagram.get(row as usize).unwrap().len()
    }

    fn get_line(&self) -> Option<Line>
    {
        let (r,c) = self.curr_pos;
        match self.diagram.get(r) {
            Some(column) => 
                match column.get(c){
                    Some(&line) => Some(line),
                    None => None,
                },
            None => None,
        }
    }

    fn has_left(&self) -> bool
    {
        match self.diagram.get(self.curr_pos.0) 
        {
            Some(col) => {
                match col.get(self.curr_pos.1 -1) {
                    Some(&line) => line != Line::Whitespace,
                    None => false,
                }
            },
            None => false,
        }
    }

    fn has_up(&self) -> bool 
    {
        match self.diagram.get(self.curr_pos.0 -1) {
            Some(row) => {
                match row.get(self.curr_pos.1)
                {
                    Some(&line) => line != Line::Whitespace,
                    None => false,
                }
            },
            None =>  return false,
        }
    }

    fn move_to_next_position(&mut self)
    {
        match self.curr_dir
        {
            Direction::Up    => 
            {
                if self.curr_pos.0 > 0
                { self.curr_pos.0 -= 1; }
            },
            Direction::Down  => 
            {
                if self.curr_pos.0 + 1 < self.diagram.len()
                { self.curr_pos.0 += 1; }
            },
            Direction::Left  => 
            {   
                if self.curr_pos.1 > 0
                {self.curr_pos.1 -= 1;}
            },
            Direction::Right => 
            {
                if self.curr_pos.1 + 1 < self.diagram.get(self.curr_pos.0).unwrap().len()
                {self.curr_pos.1 += 1;}
            },
        }
    }

    fn next(&mut self) -> Option<Line>
    {
        self.move_to_next_position();

        match self.get_line() {
            Some(line) => {
                match line {
                    Line::Whitespace => None,
                    Line::Plus => {
                        if self.curr_dir == Direction::Up || self.curr_dir == Direction::Down
                        {
                            if self.has_left()
                            {
                                self.curr_dir = Direction::Left;
                            }
                            else
                            {
                                self.curr_dir = Direction::Right;
                            }
                        }
                        else
                        {
                            if self.has_up()
                            {
                                self.curr_dir = Direction::Up;
                            }
                            else
                            {
                                self.curr_dir = Direction::Down;
                            }
                        }
                        Some(Line::Plus)
                    },
                    line => Some(line),
                }
            },
            None => None,
        }
    }
}



fn follow_path(network : &mut Network) -> Vec<Line>
{
    let mut v : Vec<Line> = Vec::new();
    while let Some(line) = network.next()
    {
        v.push(line);
    }
    v
}

fn path(input : &str) -> (String,u32) 
{
    let mut network = Network::from_str(input);
    let mut out = String::new();
    let mut count = 0;
    for line in  follow_path(&mut network).into_iter()
    {
        match line 
        {
            Line::Letter(c) => 
            {
                out.push(c);
                count += 1;
            },
            _ => {count += 1;},
        }
    }
    (out,count+1)
}


pub fn run()
{
    let input = include_str!("input.txt");
    let (out_str,count) = path(input);
    println!("Output string {}, number of steps: {}", out_str, count);
}
