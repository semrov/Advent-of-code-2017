mod test;

pub struct SpinLock
{
    circular_buffer : Vec<u32>,
    current_position : usize,
    steps : usize,
}

impl SpinLock {
    pub fn new(steps : usize) -> SpinLock
    {
        SpinLock
        { 
            circular_buffer : vec![0],
            current_position : 0,
            steps,
        }
    }

    pub fn get(&self, index : usize) -> u32 
    {
        *self.circular_buffer.get(self.calculate_index(index)).unwrap()
    }

    #[inline]
    fn calculate_index(&self, index : usize) -> usize
    {
        index % self.circular_buffer.len()
    }

    fn insert(&mut self, val : u32)
    {
        let insert_index = self.calculate_index(self.current_position + self.steps)+1;
        self.circular_buffer.insert(insert_index,val);
        self.current_position = insert_index;
    }


    // Since real inserting would take too long
    //and zero never moves (is always in postition 0)
    // we just need to monitor value at position 1
    pub fn simulate_insert_50_mio(&self, start : u32, end : u32) -> u32
    {
        let mut after_zero = self.circular_buffer[1];
        let mut position = self.current_position-1;
        for i in start..end
        {
            position = (position+1+self.steps) % (i as usize);    
            if position == 0
            {
                after_zero = i;
            }
        }
        after_zero
    }

    pub fn insert_range(&mut self, n : u32)
    {
        for i in 1..(n+1)
        {
            self.insert(i);
        }
    }
}

pub fn run_problem1() 
{
    let mut spinlock = SpinLock::new(343);
    spinlock.insert_range(2017);
    println!("Value after 2017: {}", spinlock.get(spinlock.current_position+1));
    
    //right answer 41797835
    println!("Value after 0 after 50mio iter: {}",spinlock.simulate_insert_50_mio(2018,50_000_000));

}

