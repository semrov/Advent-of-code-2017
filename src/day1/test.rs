#[test]
fn test_sum_part1() {
    assert_eq!(super::sum_part1("1111"),4);
    assert_eq!(super::sum_part1("1122"),3);
    assert_eq!(super::sum_part1("1234"),0);
    assert_eq!(super::sum_part1("91212129"),9);
}

#[test]
fn test_sum_part2() {
    assert_eq!(super::sum_part2("1212"),6);
    assert_eq!(super::sum_part2("1221"),0);
    assert_eq!(super::sum_part2("123425"),4);
    assert_eq!(super::sum_part2("123123"),12);
    assert_eq!(super::sum_part2("12131415"),4);
}