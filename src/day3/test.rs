/*
#[test]
fn test_part1() {
   assert_eq!(super::num_of_steps(1),0);
   assert_eq!(super::num_of_steps(12),3);
   assert_eq!(super::num_of_steps(23),2);
   assert_eq!(super::num_of_steps(1024),31);
}
*/

#[test]
fn test_manhattan_distance()
{
    assert_eq!(super::manhattan_distance((1,1),(1,1)),0);
    assert_eq!(super::manhattan_distance((3,4),(5,2)),4);
    assert_eq!(super::manhattan_distance((3,0),(1,1)),3);
}