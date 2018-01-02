mod test;

use day10::KnotHash;
use std::str::FromStr;
use std::iter::Extend;
use std::ops::IndexMut;


fn hex_char_to_strbin(hex : char) -> &'static str
{
    match hex 
    {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        x => panic!("Invalid hex number: {}", x),
    }
}



fn hex_string_to_bin_string(v : String) -> String
{
    v.chars().map(|hex_char| hex_char_to_strbin(hex_char)).collect()
}

fn count_ones(s : &str) -> u32 
{
    s.chars().filter(|&c| c == '1').count() as u32
}

fn used_squares(key : &str) -> u32
{
    (0..128).into_iter()
            .map(|x| count_ones(&hex_string_to_bin_string(
                    KnotHash::hash_to_string(KnotHash::new_full().hash_code(&format!("{}-{}",key,x))))))
            .sum()
}

fn is_valid_coord(row : i32, col : i32) -> Option<(i32,i32)>
{
    if row >= 0 && row < 128 && col >= 0 && col < 128 {Some((row,col))} else {None} 
}

fn dfs_ij(graph : &mut Vec<Vec<char>>, row : i32 ,col : i32) -> u32
{
    let mut stack = Vec::new();
    let mut region : u32 = 0;
    stack.push((row,col));
    while !stack.is_empty() {
        let (row,col) = stack.pop().unwrap();
        let square = graph.index_mut(row as usize).index_mut(col as usize);
        if *square == '1' 
        {
            stack.extend([(row-1,col),(row,col+1),(row+1,col),(row,col-1)]
                         .into_iter()
                         .filter_map(|coord| is_valid_coord(coord.0,coord.1)).collect::<Vec<(i32, i32)>>());
            region += 1;
            *square = '0';
        }
    }
    region
}
 

fn num_of_regions(key : &str) -> u32
{
    let mut graph = (0..128).into_iter()
                            .map(|x| hex_string_to_bin_string(
                    KnotHash::hash_to_string(KnotHash::new_full().hash_code(&format!("{}-{}",key,x))))
                            .chars().collect::<Vec<char>>()
                    ).collect::<Vec<_>>();
    let mut regions : u32 = 0;       
    for i in 0..128
    {
        for j in 0..128
        {
            if dfs_ij(&mut graph, i,j) > 0
            {
                regions += 1;
            }
        }
    }
    regions
}


pub fn run_problem1()
{
    let used_squares = used_squares("jxqlasbh");
    println!("Squares used: {}",used_squares);
}

pub fn run_problem2()
{
    let regions = num_of_regions("jxqlasbh");
    println!("Regions present: {}",regions);
}