use super::{DanceMove,Dance};
use std::convert::From;

#[test]
fn test_parse_steps() {
    let v = Dance::str_to_dance_moves("s1,x3/4,pe/b");
    assert_eq!(v,vec![DanceMove::Spin(1),DanceMove::Exchange((3,4)),DanceMove::Partner(('e','b'))]);
}

#[test]
fn test_simple_dance() {
    let mut dance = Dance::from_vec(From::from(vec!['a','b','c','d','e']));
    dance.dance(&Dance::str_to_dance_moves("s1,x3/4,pe/b"));
    assert_eq!(dance.list,vec!['b','a','e','d','c']);
}