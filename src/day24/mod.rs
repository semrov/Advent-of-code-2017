mod test;

use std::collections::HashSet;

#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq)]
struct Component(u8,u8);

impl Component {
    pub fn new(port1 : u8, port2 : u8) -> Component
    {
        Component(port1,port2)
    }
    pub fn is_type_of(&self, port : u8) -> Option<u8>
    {
        if self.0 == port {Some(self.1)} else if self.1 == port {Some(self.0)} else {None}
    } 
    pub fn strength(&self) -> u32
    {
        self.0 as u32 + self.1 as u32
    }
    pub fn get_port_end(&self, port_in : u8) -> u8
    {
        if self.0 == port_in {self.1 } else {self.0}
    } 
    fn parse_components(input : &str) -> Vec<Component>
    {
        input.lines().map(|line|{
            let mut ports = line.trim().split("/");
            let port1 = ports.next().unwrap().parse().unwrap();
            let port2 = ports.next().unwrap().parse().unwrap();
            Component::new(port1,port2)
        }).collect()
    }
}

fn rec(port : u8, bridge :&mut HashSet<Component>, bridges :&mut Vec<Vec<Component>>, components : &Vec<Component>)
{
    for c in components.iter()
    {
        if !bridge.contains(c) 
        {
            if let Some(end_port) = c.is_type_of(port)
            {
                bridge.insert(*c);
                bridges.push(bridge.iter().map(|comp| *comp).collect());
                rec(end_port,bridge,bridges,components);
                bridge.remove(c);
            }
        }
    }
}

fn process(input : &str) -> (u32,u32)
{
    let mut bridge : HashSet<Component> =  HashSet::new();
    let mut bridges : Vec<Vec<Component>> = Vec::new();
    let components : Vec<Component> = Component::parse_components(input);
    rec(0,&mut bridge,&mut bridges,&components);    

    let max_strength = bridges.iter().map(|bridge|
                        bridge.iter().fold(0,|acc,c| acc + c.strength())).max().unwrap();

    let max_length = bridges.iter().map(|bridge| bridge.len()).max().unwrap() as usize;  
    let max_strength_of_longest =  bridges.iter().map(|bridge|
                        if bridge.len() == max_length
                        {
                            bridge.iter().fold(0,|acc,c| acc + c.strength())
                        }
                        else {0}
                        ).max().unwrap();                  

    (max_strength, max_strength_of_longest)                    
}

pub fn run_problem()
{
    let input = include_str!("input.txt");    
    let (max_strength, max_strength_of_longest) = process(input);
    println!("Problem 1: strength of the strongest bridge: {}", max_strength); 
    println!("Problem 2: max strength of the longest bridge: {}", max_strength_of_longest);                    
}







