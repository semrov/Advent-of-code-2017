#[test]
fn test_largest_value() {
    let input = "b inc 5 if a > 1
    a inc 1 if b < 5
    c dec -10 if a >= 1
    c inc -20 if c == 10".trim();
    assert_eq!(super::largest_value(input),1);
}

#[test]
fn test_largest_value_ever() {
    let input = "b inc 5 if a > 1
    a inc 1 if b < 5
    c dec -10 if a >= 1
    c inc -20 if c == 10".trim();
    assert_eq!(super::largest_value_ever(input),10);
}