use super::{Network, Line,Direction};

#[test]
fn test_find_start() {
    let input = include_str!("input_test.txt");
    let n = Network::from_str(input);
    assert_eq!(n.curr_pos,(0,5));
}

#[test]
fn test_next() {
    let input = include_str!("input_test.txt");
    let mut n = Network::from_str(input);
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::Letter('A')));
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::Plus));
    assert_eq!(n.curr_dir,Direction::Right);
    assert_eq!(n.next(),Some(Line::Letter('B')));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Plus));
    assert_eq!(n.curr_dir,Direction::Up);
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.curr_dir,Direction::Up);
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::Plus));
    assert_eq!(n.curr_dir,Direction::Right);
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Plus));
    assert_eq!(n.curr_dir,Direction::Down);
    assert_eq!(n.next(),Some(Line::Letter('C')));
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::Plus));
    assert_eq!(n.curr_dir,Direction::Right);
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Plus));
    assert_eq!(n.curr_dir,Direction::Up);
    assert_eq!(n.next(),Some(Line::Letter('D')));
    assert_eq!(n.next(),Some(Line::Plus));
    assert_eq!(n.curr_dir,Direction::Left);
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::Letter('E')));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::VerticalBar));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Minus));
    assert_eq!(n.next(),Some(Line::Letter('F')));
    assert_eq!(n.next(),None);
    assert_eq!(n.next(),None);
    assert_eq!(n.next(),None);
}


#[test]
fn test_path() {
    let input = include_str!("input_test.txt");
    let (out,count) = super::path(input);
    assert_eq!("ABCDEF", out);
    assert_eq!(count,38);
}
