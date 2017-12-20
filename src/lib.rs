pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests_day1 {
    use day1;

    #[test]
    fn test_sum_part1() {
        assert_eq!(day1::sum_part1("1111"),4);
        assert_eq!(day1::sum_part1("1122"),3);
        assert_eq!(day1::sum_part1("1234"),0);
        assert_eq!(day1::sum_part1("91212129"),9);
    }

    #[test]
    fn test_sum_part2() {
        assert_eq!(day1::sum_part2("1212"),6);
        assert_eq!(day1::sum_part2("1221"),0);
        assert_eq!(day1::sum_part2("123425"),4);
        assert_eq!(day1::sum_part2("123123"),12);
        assert_eq!(day1::sum_part2("12131415"),4);
    }


}

#[cfg(test)]
mod tests_day2
{
    use day2::calculate_checksum;
    #[test]
    fn test()
    {
        let test_string = "5 1 9 5
        7 5 3
        2 4 6 8".trim();
        assert_eq!(calculate_checksum(test_string),18);
    }
}
