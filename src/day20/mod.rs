mod test;

use std::ops::{Add,AddAssign};
use regex::Regex;
use std::collections::HashMap;


#[derive(Debug,Clone,Copy,PartialEq,Hash,Eq)]
struct V3(i64,i64,i64);

impl Add for V3 {
    type Output = V3;
    fn add(self,other : V3) -> Self::Output
    {
        V3(self.0 + other.0,self.1 + other.1,self.1 + other.1)
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, other : V3)
    {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct Particle
{
    position : V3,
    velocity : V3,
    acceleration : V3
}

impl Particle {
    pub fn from_line_str(line : &str) -> Particle
    {
        //p=<1199,-2918,1457>, v=<-13,115,-8>, a=<-7,8,-10>
        lazy_static! {                      
            static ref RE: Regex = Regex::new(r"p=<(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)>, v=<(?P<vx>-?\d+),(?P<vy>-?\d+),(?P<vz>-?\d+)>, a=<(?P<ax>-?\d+),(?P<ay>-?\d+),(?P<az>-?\d+)>").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let x = caps["x"].parse::<i64>().unwrap();
        let y = caps["y"].parse::<i64>().unwrap();
        let z = caps["z"].parse::<i64>().unwrap();
        let vx = caps["vx"].parse::<i64>().unwrap();
        let vy = caps["vy"].parse::<i64>().unwrap();
        let vz = caps["vz"].parse::<i64>().unwrap();
        let ax = caps["ax"].parse::<i64>().unwrap();
        let ay = caps["ay"].parse::<i64>().unwrap();
        let az = caps["az"].parse::<i64>().unwrap();

        Particle
        {
            position: V3(x,y,z),
            velocity: V3(vx,vy,vz),
            acceleration: V3(ax,ay,az)
        }
    }

    pub fn tick(&mut self)
    {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    pub fn manhattan_distance(&self) -> i64
    {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }
}

pub fn run_problem1()
{
    let input = include_str!("input.txt");
    let mut particles = input.lines()
                         .map(|line| Particle::from_line_str(line))
                         .collect::<Vec<Particle>>();

    for _ in 0..5000
    {
        for particle in particles.iter_mut()
        {
            particle.tick();
        }
    }

    let closest = particles.iter()
                            .enumerate()
                            .min_by_key(|&(_,p)| p.manhattan_distance()).unwrap();

    println!("Closest particle: {}",closest.0);
}


pub fn run_problem2()
{
    let input = include_str!("input.txt");
    let mut particles = input.lines()
                         .map(|line| Particle::from_line_str(line))
                         .collect::<Vec<Particle>>();

    for _ in 0..5000
    {
        let mut collisions : HashMap<V3,u32> = HashMap::with_capacity(particles.len()*2);
        for particle in particles.iter_mut()
        {
            particle.tick();
            let mut collision = collisions.entry(particle.position).or_insert(0);
            *collision +=1;
        }
        particles.retain(|&p| collisions[&p.position] == 1);
    }

    println!("Particles are left after all collisions: {}", particles.len());
}