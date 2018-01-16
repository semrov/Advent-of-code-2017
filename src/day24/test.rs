#[test]
fn test_max_strength() {
    let input = "0/2
    2/2
    2/3
    3/4
    3/5
    0/1
    10/1
    9/10";
    let (max_strength, max_strength_of_longest)= super::process(input);
    assert_eq!(max_strength, 31);
    assert_eq!(max_strength_of_longest,19);
}