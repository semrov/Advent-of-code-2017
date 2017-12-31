
#[test]
fn test_problem1() {
    let data = "0 <-> 2
    1 <-> 1
    2 <-> 0, 3, 4
    3 <-> 2, 4
    4 <-> 2, 3, 6
    5 <-> 6
    6 <-> 4, 5";
    let graph = super::read(data);
    assert_eq!(super::visited(&graph,0).len(),6);
    
}

#[test]
fn test_problem2() {
    let data = "0 <-> 2
    1 <-> 1
    2 <-> 0, 3, 4
    3 <-> 2, 4
    4 <-> 2, 3, 6
    5 <-> 6
    6 <-> 4, 5";
    let graph = super::read(data);
    assert_eq!(super::groups(&graph),2);
    
}