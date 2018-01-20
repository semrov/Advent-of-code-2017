#[test]
fn test_part1()
{
    let test_string = "5 1 9 5
    7 5 3
    2 4 6 8".trim();
    assert_eq!(super::calculate_checksum(test_string,super::get_row_diff),18);
}

#[test]
fn test_part2()
{
    let test_string = "5 9 2 8
    9 4 7 3
    3 8 6 5".trim();
    assert_eq!(super::calculate_checksum(test_string,super::get_row_dividable),9);
}