pub mod day1;


#[cfg(test)]
mod tests {
    use day1::test;

    #[test]
    fn it_works() {
        assert_eq!(test(),5);
        assert_eq!(2 + 2, 4);
    }
}
