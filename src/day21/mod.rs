mod test;

use ndarray::{Array2,Axis};
use std::collections::HashMap;

fn parse(matrix : &str) -> Array2<bool> 
{
    let v = matrix.chars().filter_map(|c| match c {
        '.' => Some(false),
        '#' => Some(true),
        _ => None
        }
    ).collect::<Vec<bool>>();

    let size = (v.len() as f64).sqrt() as usize;

    Array2::from_shape_vec((size,size),v).unwrap()
}

fn get_replacements() -> HashMap<Array2<bool>,Array2<bool>>
{
    let input = include_str!("input.txt");
    let mut replacements = HashMap::new();


    for line in input.lines()
    {
        let parts = line.split(" => ").collect::<Vec<&str>>();

        let mut from = parse(parts[0].trim());
        let to = parse(parts[1].trim());

        for _ in 0..4 
        {
            replacements.entry(from.clone()).or_insert(to.clone());
            from = from.reversed_axes();
            replacements.entry(from.clone()).or_insert(to.clone());
            from.invert_axis(Axis(0));
        }
        
    }
    replacements
}

fn enhance(pattern : Array2<bool>, replacements : &HashMap<Array2<bool>,Array2<bool>>) -> Array2<bool>
{
    let old_size = pattern.shape()[0] as isize;
    let old_step = if old_size % 2 == 0 {2} else {3};
    let new_size = old_size*(old_step+1) / old_step;
    let new_step = old_step+1;

    let mut enhanced_array : Array2<bool> = Array2::default((new_size as usize, new_size as usize));

    for (i1,i2) in (0..old_size).step_by(old_step as usize).zip((0..new_size).step_by(new_step as usize))
    {
        for (j1,j2) in (0..old_size).step_by(old_step as usize).zip((0..new_size).step_by(new_step as usize))
        {
            let from = pattern.slice(s![i1..(i1+old_step), j1..(j1+old_step)]);

            if let Some(replacement) = replacements.get(&from.to_owned())
            {
                enhanced_array.slice_mut(s![i2..(i2+new_step), j2..(j2+new_step)]).assign(replacement);
            }
        }
    }
    enhanced_array
}

pub fn run()
{
    let mut pattern = parse(".#./..#/###");
    let replacements = get_replacements();
    
    for _ in 0..5
    {
        pattern = enhance(pattern,&replacements )
    }

    let pixels_on = pattern.iter().filter(|p| **p).count();

    println!("Pixels on after 5 iterations: {}", pixels_on);

    for _ in 5..18
    {
        pattern = enhance(pattern,&replacements )
    }

    let pixels_on = pattern.iter().filter(|p| **p).count();

    println!("Pixels on after 5 iterations: {}", pixels_on);
    
}



