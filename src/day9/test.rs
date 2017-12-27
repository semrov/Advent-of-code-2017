use super::Parser;

#[test]
fn test_total_score() {
    let mut parser = Parser::new("{}");
    parser.parse();
    assert_eq!(parser.total_score,1); 

    let mut parser = Parser::new("{{{}}}");
    parser.parse();
    assert_eq!(parser.total_score,6); 

    let mut parser = Parser::new("{{},{}}");
    parser.parse();
    assert_eq!(parser.total_score,5); 

    let mut parser = Parser::new("{{{},{},{{}}}}");
    parser.parse();
    assert_eq!(parser.total_score,16); 

    let mut parser = Parser::new("{<a>,<a>,<a>,<a>}");
    parser.parse();
    assert_eq!(parser.total_score,1); 

    let mut parser = Parser::new("{{<ab>},{<ab>},{<ab>},{<ab>}}");
    parser.parse();
    assert_eq!(parser.total_score,9); 

    let mut parser = Parser::new("{{<a!>},{<a!>},{<a!>},{<ab>}}");
    parser.parse();
    assert_eq!(parser.total_score,3); 

    let mut parser = Parser::new("{{<!!>},{<!!>},{<!!>},{<!!>}}");
    parser.parse();
    assert_eq!(parser.total_score,9); 
}

#[test]
fn test_remove_chars() 
{
    let mut parser = Parser::new("{}");
    parser.parse();
    assert_eq!(parser.removed_characters,0);

    let mut parser = Parser::new("<random characters>");
    parser.parse();
    assert_eq!(parser.removed_characters,17);

    let mut parser = Parser::new("<<<<>");
    parser.parse();
    assert_eq!(parser.removed_characters,3); 

    let mut parser = Parser::new("<{!>}>");
    parser.parse();
    assert_eq!(parser.removed_characters,2); 

    let mut parser = Parser::new("<!!>");
    parser.parse();
    assert_eq!(parser.removed_characters,0); 

    let mut parser = Parser::new("<!!!>>");
    parser.parse();
    assert_eq!(parser.removed_characters,0); 

    let mut parser = Parser::new("<{o\"i!a,<{i<a>");
    parser.parse();
    assert_eq!(parser.removed_characters,10);  
    
}