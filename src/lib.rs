pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests_day1 {
    use day1::sum;

    #[test]
    fn test_sum1() {
        assert_eq!(sum("1111"),4);
    }

    #[test]
    fn test_sum2() {
        assert_eq!(sum("1122"),3);
    }

    #[test]
    fn test_sum3() {
        assert_eq!(sum("1234"),0);
    }

    #[test]
    fn test_sum4() {
        assert_eq!(sum("91212129"),9);
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
