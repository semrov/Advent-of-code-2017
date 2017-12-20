#[test]
fn test_problem1() {
    use super::is_valid_passphrase;
    assert!(is_valid_passphrase("aa bb cc dd ee"));
    assert!(!is_valid_passphrase("aa bb cc dd aa"));
    assert!(is_valid_passphrase("aa bb cc dd aaa"));
}

#[test]
fn test_problem2() {
    use super::is_valid_passphrase_no_anagrams;
    assert!(is_valid_passphrase_no_anagrams("abcde fghij"));
    assert!(!is_valid_passphrase_no_anagrams("abcde xyz ecdab"));
    assert!(is_valid_passphrase_no_anagrams("a ab abc abd abf abj"));
    assert!(is_valid_passphrase_no_anagrams("iiii oiii ooii oooi oooo"));
    assert!(!is_valid_passphrase_no_anagrams("oiii ioii iioi iiio"));
}