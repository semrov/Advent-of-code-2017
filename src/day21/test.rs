use ndarray::arr2;

#[test]
fn test_parse() {
    let array = super::parse(".#./..#/###");
    assert!(array == arr2(&[[false,true,false],[false,false,true],[true,true,true]]));
}