use super::{Computer,Instruction};
use std::collections::VecDeque;



#[test]
fn test_problem1() {
    let input = "set a 1
    add a 2
    mul a a
    mod a 5
    snd a
    set a 0
    rcv a
    jgz a -1
    set c 1
    jgz a -2";
    let instructions = Computer::parse_program(input);
    let mut comp = Computer::new(&instructions,0);
    assert_eq!(comp.pc,0);
    assert_eq!(comp.register[&'p'],0);
    comp.execute_instruction();
    assert_eq!(comp.register[&'a'],1);
    comp.execute_instruction();
    assert_eq!(comp.register[&'a'],3);
    comp.execute_instruction();
    assert_eq!(comp.register[&'a'],9);
    comp.execute_instruction();
    assert_eq!(comp.register[&'a'],4);
    assert_eq!(comp.execute_instruction(),Some(4));
    assert_eq!(comp.register[&'a'],4);
    comp.execute_instruction();
    assert_eq!(comp.register[&'a'],0);
    comp.execute_instruction();
    let pc = comp.pc;
    comp.execute_instruction();
    comp.execute_instruction();
    comp.execute_instruction();
    assert_eq!(pc, comp.pc);
    comp.add_to_queue(13);
    comp.execute_instruction();
    assert_eq!(pc+1,comp.pc);
    assert_eq!(comp.register[&'a'],13);
    comp.execute_instruction();
    comp.add_to_queue(-1);
    comp.execute_instruction();
    assert_eq!(comp.register[&'a'],-1);
    comp.execute_instruction();
    comp.execute_instruction();
    assert_eq!(comp.register[&'c'],1);
    comp.execute_instruction();
    comp.execute_instruction();
    assert!(!comp.is_running());
}


#[test]
fn test_problem2() {
    let input = "snd 1
    snd 2
    snd p
    rcv a
    rcv b
    rcv c
    rcv d";
    let instructions = Computer::parse_program(input);
    let mut comp0 = Computer::new(&instructions,0);
    let mut comp1 = Computer::new(&instructions,1);
    let stat = (comp0.execute_instruction(),comp1.execute_instruction());
    assert_eq!(stat,(Some(1),Some(1)));
    let (v0,v1) = (stat.0.unwrap(), stat.1.unwrap());
    comp0.add_to_queue(v1);
    comp1.add_to_queue(v0);
    assert_eq!(comp0.message_queue, VecDeque::from(vec![1]));
    assert_eq!(comp1.message_queue, VecDeque::from(vec![1]));

    let stat = (comp0.execute_instruction(),comp1.execute_instruction());
    assert_eq!(stat,(Some(2),Some(2)));
    let (v0,v1) = (stat.0.unwrap(), stat.1.unwrap());
    comp0.add_to_queue(v1);
    comp1.add_to_queue(v0);
    assert_eq!(comp0.message_queue, VecDeque::from(vec![1,2]));
    assert_eq!(comp1.message_queue, VecDeque::from(vec![1,2]));

    let stat = (comp0.execute_instruction(),comp1.execute_instruction());
    assert_eq!(stat,(Some(0),Some(1)));
    let (v0,v1) = (stat.0.unwrap(), stat.1.unwrap());
    comp0.add_to_queue(v1);
    comp1.add_to_queue(v0);
    assert_eq!(comp0.message_queue, VecDeque::from(vec![1,2,1]));
    assert_eq!(comp1.message_queue, VecDeque::from(vec![1,2,0]));


    assert_eq!(comp0.pc,3);
    let stat = (comp0.execute_instruction(),comp1.execute_instruction());
    assert_eq!(comp0.register.get(&'a'),Some(&1));
    assert_eq!(comp1.register.get(&'a'),Some(&1));

    let stat = (comp0.execute_instruction(),comp1.execute_instruction());
    assert_eq!(comp0.register.get(&'b'),Some(&2));
    assert_eq!(comp1.register.get(&'b'),Some(&2));

    let stat = (comp0.execute_instruction(),comp1.execute_instruction());
    assert_eq!(comp0.register.get(&'c'),Some(&1));
    assert_eq!(comp1.register.get(&'c'),Some(&0));

    let stat = (comp0.execute_instruction(),comp1.execute_instruction());
    assert_eq!(stat,(None,None) );
    assert_eq!(comp0.is_running(),false);
    assert_eq!(comp1.is_running(),false);
}

#[test]
fn test_problem2_waiting() {
    let input = "snd 1
    snd 2
    snd p
    rcv a
    rcv b
    rcv c
    rcv d";
    let instructions = Computer::parse_program(input);
    let mut comp0 = Computer::new(&instructions,0);
    let mut comp1 = Computer::new(&instructions,1);
    let v = comp0.execute_instruction().unwrap();
    comp1.add_to_queue(v);
    let v = comp0.execute_instruction().unwrap();
    comp1.add_to_queue(v);
    let v = comp0.execute_instruction().unwrap();
    comp1.add_to_queue(v);
    comp0.execute_instruction();
    assert!(!comp0.is_running());
    comp0.execute_instruction();
    assert!(!comp0.is_running());
    comp0.execute_instruction();
    assert!(!comp0.is_running());

    let v = comp1.execute_instruction().unwrap();
    comp0.add_to_queue(v);
    let v = comp1.execute_instruction().unwrap();
    comp0.add_to_queue(v);
    let v = comp1.execute_instruction().unwrap();
    comp0.add_to_queue(v);
    assert!(!comp0.is_running());
    assert!(comp1.is_running());

    comp1.execute_instruction();
    assert!(comp1.is_running());
    comp1.execute_instruction();
    assert!(comp1.is_running());
    comp1.execute_instruction();
    assert!(comp1.is_running());
    comp1.execute_instruction();
    assert!(!comp1.is_running());

    comp0.execute_instruction();
    assert!(comp0.is_running());
    comp0.execute_instruction();
    assert!(comp0.is_running());
    comp0.execute_instruction();
    assert!(comp0.is_running());
    comp0.execute_instruction();
    assert!(!comp1.is_running());



}