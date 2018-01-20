// input 102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216
mod test;

pub struct KnotHash
{
    list : Vec<u8>,
    current_position : usize,
    skip_size : usize
    //lengths : Vec<u8>,
}

impl KnotHash {
    pub fn new(max_range : u16) -> KnotHash
    {
        KnotHash{
            list : (0..max_range).map(|x| x as u8).collect(), 
            current_position : 0,
            skip_size : 0
        }
    }

    pub fn new_full() -> KnotHash
    {
        let mut kh = KnotHash{
            list : (0..255).collect::<Vec<u8>>(), 
            current_position : 0,
            skip_size : 0
        };
        kh.list.push(255);
        kh
    }

    fn get_norm_index(&self, index : usize) -> usize
    {
        index % self.list.len()
    }

    fn reverse(&mut self, length : usize)
    {
        for (i,j) in (0..length/2).zip((0..length).rev())
        {
            let i = self.get_norm_index(i + self.current_position);
            let j = self.get_norm_index(j + self.current_position);
            self.list.swap(i,j);
        }
    }

    fn hash(&mut self, lengths : &Vec<u8> )
    {
        let list_size = self.list.len();


        for length in lengths
        {
            let length = *length as usize;
            if(length <= list_size)
            {
                self.reverse(length);
                self.current_position += length+self.skip_size;
                self.skip_size += 1; 
            }
        }
    } 

    pub fn hash_code(&mut self, input : &str) -> Vec<u8>
    {
        let magic_sequence = &[17, 31, 73, 47, 23];
        let input = input.trim();
        let lenghts : Vec<u8> = input.as_bytes()
            .iter()
            .chain(magic_sequence)
            .map(|length| *length as u8)
            .collect();


        for _ in 0..64
        {
            self.hash(&lenghts);
        } 

        let out = self.list.chunks(16)
        .map(|chunk| chunk.into_iter().fold(0,|acc,val| acc ^ val ))
        .collect();
        out
    }

    pub fn hash_to_string(v : Vec<u8>) -> String 
    {
        v.iter().map(|byte| format!("{:02x}",*byte)).collect()
    }


}


pub fn run_problem1()
{
    let mut knot_hash = KnotHash::new(256);
    knot_hash.hash(&"102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216"
    .split(",").map(|x| x.parse().unwrap()).collect());
    println!("Product is {}", knot_hash.list[0] * knot_hash.list[1]);
}

pub fn run_problem2()
{
    let mut knot_hash = KnotHash::new_full();
    let v = knot_hash.hash_code("102,255,99,252,200,24,219,57,103,2,226,254,1,0,69,216");
    println!("Hash code: {}",KnotHash::hash_to_string(v));
}