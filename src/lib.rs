pub mod day1;


#[cfg(test)]
mod tests {
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
