mod test;
use std::collections::{HashMap,HashSet,VecDeque};

fn visited(graph :&HashMap<u32,Vec<u32>>, group : u32) -> HashSet<u32>
{
    let mut visited : HashSet<u32> = HashSet::new();
    let mut queue : VecDeque<u32> = VecDeque::new();
    queue.push_back(group);
    while !queue.is_empty() 
    {
        let pipe = queue.pop_front().unwrap();
        visited.insert(pipe);
        graph.get(&pipe).unwrap().iter().for_each(|p| if !visited.contains(p) {queue.push_back(*p) });
    }
    visited
}

fn groups(graph : &HashMap<u32,Vec<u32>> ) -> u32
{
    let mut count = 0;
    let mut connected : HashSet<u32> = HashSet::new();
    for node in graph.keys().clone()
    {
        if !connected.contains(node)
        {
            for n in visited(&graph,*node).into_iter()
            {
                connected.insert(n);
            }
           count += 1;
        }
    }
    count
}

pub fn read(data : &str) -> HashMap<u32,Vec<u32>>
{
    let mut graph : HashMap<u32,Vec<u32>> = HashMap::new();
    for line in data.lines()
    {   
        let line = line.split("<->").collect::<Vec<_>>();
        let id = line[0].trim().parse::<u32>().unwrap();
        let pipes = line[1].trim().split(", ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
        graph.insert(id,pipes);
    }
    graph
}

pub fn run_problem1()
{
    let data = include_str!("input.txt");
    let graph = read(data);
    println!("Number of groups: {}",visited(&graph,0).len());
}

pub fn run_problem2()
{
    let data = include_str!("input.txt");
    let graph = read(data);

   println!("Number of groups: {}",groups(&graph));
}

