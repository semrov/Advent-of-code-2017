use super::{Memory};

#[test]
fn test_new_problem1() {
    let memory = Memory::new("0 2 7 0");
    assert_eq!(memory.banks, vec![0,2,7,0]);
}

#[test]
fn test_max_problem1() {
    let mut memory = Memory::new("0 2 7 0");
    assert_eq!(memory.max(), (2,7));
    assert_eq!(memory.banks[2],0);
    memory = Memory::new("0 4 2 7 1 5 7 0");
    assert_eq!(memory.max(), (3,7));
    assert_eq!(memory.banks[3],0);
}

#[test]
fn test_reallocate_problem1() {
    let mut memory = Memory::new("0 2 7 0");
    memory.reallocate();
    assert_eq!(memory.banks, vec![2,4,1,2]);
    memory.reallocate();
    assert_eq!(memory.banks, vec![3,1,2,3]);
    memory.reallocate();
    assert_eq!(memory.banks, vec![0,2,3,4]);
    memory.reallocate();
    assert_eq!(memory.banks, vec![1,3,4,1]);
    memory.reallocate();
    assert_eq!(memory.banks, vec![2,4,1,2]);
}

#[test]
fn test_how_many_cycles_problem1() {
    let mut memory = Memory::new("0 2 7 0");
    assert_eq!(memory.how_many_cycles(),5); 
}

#[test]
fn test_how_many_cycles_again_problem2()
{
    let mut memory = Memory::new("0 2 7 0");
    assert_eq!(memory.how_many_cycles_again(),4); 

}