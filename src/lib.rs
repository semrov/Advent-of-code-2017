#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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
    use day2;
    #[test]
    fn test_part1()
    {
        let test_string = "5 1 9 5
        7 5 3
        2 4 6 8".trim();
        assert_eq!(day2::calculate_checksum(test_string,day2::get_row_diff),18);
    }

    #[test]
    fn test_part2()
    {
        let test_string = "5 9 2 8
        9 4 7 3
        3 8 6 5".trim();
        assert_eq!(day2::calculate_checksum(test_string,day2::get_row_dividable),9);
    }
}
