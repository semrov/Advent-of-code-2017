use super::Program;

#[test]
pub fn test_regex_extract_problem1()
{
    //assert_eq!(format!("{:?}",Program::new("fwft (72) -> ktlj, cntj, xhth")),
    //        "Program { name: \"fwft\", children: {\"cntj\": None, \"xhth\": None, \"ktlj\": None}, weight: 72 }");
    let program = Program::new("fwft (72) -> ktlj, cntj, xhth");
    assert_eq!(program.name,"fwft");  
    assert_eq!(program.weight,72);
    assert_eq!(program.children.len(),3);
    assert!(program.children.contains_key("ktlj"));
    assert!(program.children.contains_key("cntj"));   
    assert!(program.children.contains_key("xhth")); 
    assert!(!program.children.contains_key("abcd")); 
    assert!(program.children.get("ktlj").unwrap().is_none());


    let program = Program::new("ktlj (134)"); 
    assert_eq!(program.name,"ktlj");  
    assert_eq!(program.weight,134);
    assert_eq!(program.children.len(),0);   
    assert!(!program.children.contains_key("xhth"));
}

#[test]
pub fn test_find_root_problem1()
{
        let input = "pbga (66)
        xhth (57)
        ebii (61)
        havc (66)
        ktlj (57)
        fwft (72) -> ktlj, cntj, xhth
        qoyq (66)
        padx (45) -> pbga, havc, qoyq
        tknk (41) -> ugml, padx, fwft
        jptl (61)
        ugml (68) -> gyxo, ebii, jptl
        gyxo (61)
        cntj (57)".trim();

        assert_eq!(Program::find_root(input),Some(String::from("tknk")));
}