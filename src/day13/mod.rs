use std::collections::HashMap;

mod test;

struct Firewall
{
                 // <depth, range>
    layers : HashMap<u32,u32>,
}

impl Firewall {
    pub fn from_str(input : &str) -> Firewall
    {
        let layers : HashMap<u32,u32> = input.lines().map(|line| {
            let mut line = line.split(":");
            (line.next().unwrap().trim().parse().unwrap(),line.next().unwrap().trim().parse().unwrap())
        }).collect();
        Firewall{layers}
    }

    fn caught(&self, delay : u32) -> bool
    {
        self.layers.iter().any(|(&deepth,&range)| range == 1 || (deepth+delay) % ((range-1)*2) == 0 )
    }

    pub fn severity(&self) -> u32 
    {
        self.layers.iter().fold(0,|severity,(&deepth,&range)| { severity +
            if range == 1 || deepth % ((range-1)*2) == 0 {deepth*range} 
            else {0}
            })
    }

    pub fn get_delay(&self) -> u32
    {
        (0..u32::max_value()).find(|&delay| !self.caught(delay)).unwrap()
    }
}


pub fn run_problem1()
{
    let input = include_str!("input.txt");
    let firewall = Firewall::from_str(input);
    let severity = firewall.severity();
    println!("Severity: {}", severity);
}

pub fn run_problem2()
{
    let input = include_str!("input.txt");
    let firewall = Firewall::from_str(input);
    let min_delay = firewall.get_delay();
    println!("Minimal delay to pass firewall: {}", min_delay);
}
