use super::Firewall;

#[test]
fn test_severity_cost() {
    let input = "0: 3
    1: 2
    4: 4
    6: 4".trim();
    let firewall = Firewall::from_str(input);
    assert_eq!(firewall.severity(), 24);
}


#[test]
fn test_delay() {
    let input = "0: 3
    1: 2
    4: 4
    6: 4".trim();
    let firewall = Firewall::from_str(input);
    assert_eq!(firewall.get_delay(),10 );
}
