
mod test;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader,Read};


#[derive(Debug)]
struct Program
{
    name : String,
    children : HashMap<String,Option<Program>>,
    weight : i32
}

impl Program {
    pub fn new(line : &str) -> Program
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) \((\d+)\)( -> [\w,\s]+)?").unwrap();
        }

        lazy_static! {
            static ref RE2: Regex = Regex::new(r" -> (.+)").unwrap();
        }

        let caps = RE.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str().to_owned();
        let weight = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

        let children : HashMap<String,Option<Program>> = match caps.get(3) {
            Some(cap) =>
                RE2.captures(cap.as_str())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| (s.to_owned(),None))
                .collect()
            ,
            None => HashMap::new(), 
        };

        let program = Program{name, children, weight };
        program
    }

    fn is_parent(&self) -> bool 
    {
        !self.children.is_empty()
    }

    pub fn find_root(input : &str) -> Option<String>
    {
        for program in input.lines().map(|line| Program::new(line))
        {
            if program.is_parent()
            {
                if input.lines().map(|line| Program::new(line))
                .any(|other_program| other_program.children.contains_key(&program.name)) 
                == false 
                {
                    return Some(program.name);
                }
            }
        }

        None
    }


}

pub fn run_problem1()
{
    let f = File::open("./src/day7/input.txt").unwrap();
    let mut br = BufReader::new(f);
    let mut buffer = String::new();
    br.read_to_string(&mut buffer);
    println!("Root is: {}", Program::find_root(&buffer).unwrap());
}
