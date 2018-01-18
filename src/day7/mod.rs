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
    weight : i32,
    total_weight : Option<i32>,
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

        let program = Program{name, children, weight, total_weight : None };
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

    fn find_root_index_from_vec(programs : &Vec<Program>) ->Option<usize>
    {
        for (index,program) in programs.iter().enumerate()
        {
            if !programs.iter().any(|other_prog| other_prog.children.contains_key(&program.name))
            {
                return Some(index);
            }
        }
        None
    }

    fn find_childrens_recursive(&mut self, programs : &mut Vec<Program>)
    {
        let childrens : Vec<String> = self.children.keys().map(|key| key.to_owned()).collect();

        for child_name in childrens
        {
            if let Some(index) = programs.iter().position(|p| p.name == child_name)
            {
                let mut program = programs.remove(index);
                program.find_childrens_recursive(programs);
                self.children.insert(child_name,Some(program));
            }
        }
    }

    fn total_weight(&mut self) -> i32 
    {
        if self.total_weight.is_none()
        {
            let subtower_weight :i32 = self.children.values_mut().map(|child| child.as_mut().expect("Program not yet found!").total_weight()).sum();
            let sum = self.weight + subtower_weight;
            self.total_weight = Some(sum);
        }
        return self.total_weight.unwrap();
    }

    pub fn find_balance_weight(&mut self) -> Option<i32>
    {
        if self.children.is_empty() 
        {
            return None;
        }

        let mut weights = Vec::new();
        let mut names = Vec::new();

        for child in self.children.values_mut()
        {
            if let Some(weight) = child.as_mut().expect("Program not yet found!").find_balance_weight()
            {
                return Some(weight);
            }
            weights.push(child.as_mut().expect("Program not yet found!").total_weight());
            names.push(child.as_ref().unwrap().name.to_owned());
        }

        let max = weights.iter().enumerate().max_by(|x,y| x.1.cmp(y.1)).unwrap();
        let min = weights.iter().enumerate().min_by(|x,y| x.1.cmp(y.1)).unwrap();

        if(max.1 != min.1)
        {
            let max_count = weights.iter().filter(|&&x| x == *max.1).count();
            let min_count = weights.iter().filter(|&&x| x == *min.1).count();
            let diff = max.1 - min.1;

            if max_count < min_count
            {
                return Some(self.children.get(&names[max.0]).unwrap().as_ref().unwrap().weight - diff);
            }
            else
            {
                return Some(self.children.get(&names[min.0]).unwrap().as_ref().unwrap().weight + diff);
            }
        }
    None
    }

    pub fn create_tree(input : &str) -> Program
    {
        let mut programs = input.lines().map(|line| Program::new(line)).collect::<Vec<Program>>();
        let root_index = Self::find_root_index_from_vec(&programs).unwrap();
        let mut root = programs.remove(root_index);
        root.find_childrens_recursive(&mut programs);
        root
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

pub fn run_problem2()
{
    let input = include_str!("input.txt");
    let mut root = Program::create_tree(input);
    let w = root.find_balance_weight().unwrap();
    println!("Corrected weight: {}", w); 
}
