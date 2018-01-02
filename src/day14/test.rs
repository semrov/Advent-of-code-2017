use day10::KnotHash;

#[test]
fn test_used_squares() 
{
    let used_squares = super::used_squares("flqrgnkx");
    assert_eq!(used_squares,8108 );
}

#[test]
fn test_num_regions() {
    let regions = super::num_of_regions("flqrgnkx");
    assert_eq!(regions,1242 );
}