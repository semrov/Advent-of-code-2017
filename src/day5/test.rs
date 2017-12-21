use super::{CPU};

#[test]
fn test_execute_jump_problem1() 
{
    let mut cpu = CPU::new(vec![0,3,0,1,-3],false);

    let mut old_pc = cpu.pc;
    cpu.execute_jump();
    assert_eq!(cpu.pc,0);
    assert_eq!(cpu.jump_list[old_pc as usize],1);
    old_pc = cpu.pc;

    cpu.execute_jump();
    assert_eq!(cpu.pc,1);
    assert_eq!(cpu.jump_list[old_pc as usize],2);
    old_pc = cpu.pc;

    cpu.execute_jump();
    assert_eq!(cpu.pc,4);
    assert_eq!(cpu.jump_list[old_pc as usize],4);
    old_pc = cpu.pc;

    cpu.execute_jump();
    assert_eq!(cpu.pc,1);
    assert_eq!(cpu.jump_list[old_pc as usize],-2);
    old_pc = cpu.pc;

    cpu.execute_jump();
    assert_eq!(cpu.pc, 5);
}

#[test]
fn test_execute_problem1()
{
    let mut cpu = CPU::new(vec![0,3,0,1,-3],false);
    assert_eq!(cpu.execute(),5); 
}

#[test]
fn test_execute_problem2()
{
    let mut cpu = CPU::new(vec![0,3,0,1,-3], true);
    assert_eq!(cpu.execute(),10); 
    assert_eq!(cpu.jump_list,vec![2,3,2,3,-1] );
}