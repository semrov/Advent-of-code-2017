mod test;

use std::ops::{AddAssign,SubAssign};
use std::cmp::PartialEq;
use std::i32;

#[derive(Debug)]
struct StepTuple(i32,i32,i32,i32,i32,i32); //n,ne,se,s,sw,nw

impl StepTuple {

    pub fn new() -> StepTuple 
    {
        StepTuple(0,0,0,0,0,0)
    }

    pub fn from_str(step : &str) -> StepTuple
    {
        match step
        {
            "n"  => StepTuple(1,0,0,0,0,0),
            "ne" => StepTuple(0,1,0,0,0,0),
            "se" => StepTuple(0,0,1,0,0,0),
            "s"  => StepTuple(0,0,0,1,0,0),
            "sw" => StepTuple(0,0,0,0,1,0),
            "nw" => StepTuple(0,0,0,0,0,1),
            _ => panic!("Invalid step: {}",step),
        }
    }

    fn sum_steptuples(input : &str) -> StepTuple
    {
        let mut sum = StepTuple::new();
        input.split(",")
            .map(|step| StepTuple::from_str(step))
            .for_each(|step| sum += step);
        sum
    }

    pub fn add(&mut self, other : &StepTuple)
    {
        *self = StepTuple(self.0+other.0,
                 self.1+other.1,
                 self.2+other.2,
                 self.3+other.3,
                 self.4+other.4,
                 self.5+other.5);
    }

    pub fn reduce(&mut self) -> i32 
    {

        // reduce n+s=0
        let min_ns = self.0.min(self.3); 
        // reduce ne+sw=0
        let min_nesw = self.1.min(self.4); 
        // reduce se+nw=0
        let min_senw = self.2.min(self.5);

        *self -= StepTuple(min_ns,min_nesw,min_senw,min_ns,min_nesw,min_senw);


        //reduce n + sw = nw
        let min_nsw = self.0.min(self.4);
        *self -= StepTuple(min_nsw,0,0,0,min_nsw,-min_nsw);
        //reduce n + se = ne
        let min_nse = self.0.min(self.2);
        *self -= StepTuple(min_nse,-min_nse,min_nse,0,0,0);
        //reduce s + nw = sw
        let min_snw = self.3.min(self.5);
        *self -= StepTuple(0,0,0,min_snw,-min_snw,min_snw);
        // reduce s + ne = se
        let min_sne = self.3.min(self.1);
        *self -= StepTuple(0,min_sne,-min_sne,min_sne,0,0);

        //n,ne,se,s,sw,nw
        // reduce nw + ne = n
        let min_nwne = self.5.min(self.1);
        *self -= StepTuple(-min_nwne,min_nwne,0,0,0,min_nwne);
        //reduce sw + se = s
        let min_swse = self.4.min(self.2);
        *self -= StepTuple(0,0,min_swse,-min_swse,min_swse,0);

        self.0 + self.1 + self.2 + self.3 + self.4 + self.5
    }
}

impl AddAssign for StepTuple {

    fn add_assign(&mut self, other : StepTuple)
    {
        *self = StepTuple(self.0+other.0,
                 self.1+other.1,
                 self.2+other.2,
                 self.3+other.3,
                 self.4+other.4,
                 self.5+other.5);
    }
}

impl SubAssign for StepTuple {
    fn sub_assign(&mut self, other : StepTuple)
    {
        *self = StepTuple(self.0-other.0,
                 self.1-other.1,
                 self.2-other.2,
                 self.3-other.3,
                 self.4-other.4,
                 self.5-other.5);
    }
}

impl PartialEq for StepTuple {
    fn eq(&self, other : &StepTuple) -> bool
    {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 &&
        self.4 == other.4 && self.5 == other.5
    }
}



pub fn run_problem1()
{
    let input = include_str!("./input.txt");
    let min_steps = StepTuple::sum_steptuples(input).reduce();
    println!("Min steps: {}",min_steps);
}

pub fn run_problem2()
{
    let input = include_str!("./input.txt");
    let mut furthest = 0;
    let steps = input.split(",")
         .map(|step| StepTuple::from_str(step))
         .collect::<Vec<_>>();  

     for i in 0..steps.len()
    {
        let mut step_tuple = StepTuple::new();
        for j in 0..i+1
        {
            step_tuple.add(steps.get(j).unwrap());
        }
        let num_steps = step_tuple.reduce();
        if num_steps > furthest 
        {
            furthest = num_steps;
        }
    }

     println!("Furthest step: {}",furthest);
}