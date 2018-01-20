use super::SpinLock;

#[test]
fn test_insert() {
    let mut spinlock = SpinLock::new(3);
    assert_eq!(spinlock.current_position,0);
    assert_eq!(spinlock.circular_buffer,vec![0]);
    spinlock.insert(1);
    assert_eq!(spinlock.current_position,1);
    assert_eq!(spinlock.circular_buffer,vec![0,1]);
    spinlock.insert(2);
    assert_eq!(spinlock.current_position,1);
    assert_eq!(spinlock.circular_buffer,vec![0,2,1]);
    spinlock.insert(3);
    assert_eq!(spinlock.current_position,2);
    assert_eq!(spinlock.circular_buffer,vec![0,2,3,1]);
    spinlock.insert(4);
    assert_eq!(spinlock.current_position,2);
    assert_eq!(spinlock.circular_buffer,vec![0,2,4,3,1]);
    spinlock.insert(5);
    assert_eq!(spinlock.current_position,1);
    assert_eq!(spinlock.circular_buffer,vec![0,5,2,4,3,1]);
    spinlock.insert(6);
    assert_eq!(spinlock.current_position,5);
    assert_eq!(spinlock.circular_buffer,vec![0,5,2,4,3,6,1]);
    spinlock.insert(7);
    assert_eq!(spinlock.current_position,2);
    assert_eq!(spinlock.circular_buffer,vec![0,5,7,2,4,3,6,1]);
    spinlock.insert(8);
    assert_eq!(spinlock.current_position,6);
    assert_eq!(spinlock.circular_buffer,vec![0,5,7,2,4,3,8,6,1]);
    spinlock.insert(9);
    assert_eq!(spinlock.current_position,1);
    assert_eq!(spinlock.circular_buffer,vec![0,9,5,7,2,4,3,8,6,1]);
}

#[test]
fn test_after_2017_inserts() {
    let mut spinlock = SpinLock::new(3);
    spinlock.insert_range(2017);
    assert_eq!(spinlock.circular_buffer[spinlock.current_position-3],1512);
    assert_eq!(spinlock.circular_buffer[spinlock.current_position-2],1134);
    assert_eq!(spinlock.circular_buffer[spinlock.current_position-1],151);
    assert_eq!(spinlock.circular_buffer[spinlock.current_position],2017);
    assert_eq!(spinlock.circular_buffer[spinlock.current_position+1],638);
    assert_eq!(spinlock.circular_buffer[spinlock.current_position+2],1513);
    assert_eq!(spinlock.circular_buffer[spinlock.current_position+3],851);
}