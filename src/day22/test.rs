use super::{Status,VirusCarrier};

#[test]
fn test_from_map() {
    let input = "...#..
    ......
    ##....
    .....#
    .#..#.
    ..#...";
    let vc = VirusCarrier::from_map(input,false);

    assert_eq!(vc.infection_map.get(&(3,0)),Some(&Status::Infected));
    assert_eq!(vc.infection_map.get(&(1,-3)),Some(&Status::Infected));
    assert_eq!(vc.infection_map.get(&(1,-2)),Some(&Status::Infected));
    assert_eq!(vc.infection_map.get(&(0,2)),Some(&Status::Infected));
    assert_eq!(vc.infection_map.get(&(-1,-2)),Some(&Status::Infected));
    assert_eq!(vc.infection_map.get(&(-1,1)),Some(&Status::Infected));
    assert_eq!(vc.infection_map.get(&(-2,-1)),Some(&Status::Infected));

    assert_eq!(vc.infection_map.get(&(3,-3)),None);
    assert_eq!(vc.infection_map.get(&(3,2)),None);
    assert_eq!(vc.infection_map.get(&(2,-3)),None);
    assert_eq!(vc.infection_map.get(&(2,-1)),None);
    assert_eq!(vc.infection_map.get(&(0,0)),None);
    assert_eq!(vc.infection_map.get(&(-2,2)),None);
}

#[test]
fn test_burst() {
    let input = "..#
    #..
    ...";
    let mut vc = VirusCarrier::from_map(input,false);
    for _ in 0..7
    {
        vc.burst();
    }
    assert_eq!(vc.infection_count,5);
    for _ in 7..70
    {
        vc.burst();
    }
    assert_eq!(vc.infection_count,41);
    for _ in 70..10000
    {
        vc.burst();
    }
    assert_eq!(vc.infection_count,5587);
}

#[test]
fn test_burst_evolved() {
    let input = "..#
    #..
    ...";
    let mut vc = VirusCarrier::from_map(input,true);

    for _ in 0..100
    {
        vc.burst();
    }
    assert_eq!(vc.infection_count,26);

    let mut vc = VirusCarrier::from_map(input,true);

    for _ in 0..10000000
    {
        vc.burst();
    }
    assert_eq!(vc.infection_count,2511944);
    
  
}