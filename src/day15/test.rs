use super::{BasicGenerator,UpgradedGenerator,Judge,Generate};

#[test]
fn test_generator_next() {
    let mut ga = Box::new(BasicGenerator::new(65,16807));
    let mut gb = Box::new(BasicGenerator::new(8921,48271));
    assert_eq!(ga.next(),1092455);
    assert_eq!(gb.next(),430625591);

    assert_eq!(ga.next(),1181022009);
    assert_eq!(gb.next(),1233683848);

    assert_eq!(ga.next(),245556042);
    assert_eq!(gb.next(),1431495498);

    assert_eq!(ga.next(),1744312007);
    assert_eq!(gb.next(),137874439);

    assert_eq!(ga.next(),1352636452);
    assert_eq!(gb.next(),285222916);
}

#[test]
fn test_judge() {
    let ga = Box::new(BasicGenerator::new(65,16807));
    let gb = Box::new(BasicGenerator::new(8921,48271));
    let mut judge = Judge::new(ga,gb);
    assert!(!judge.judge_next_values());
    assert!(!judge.judge_next_values());
    assert!(judge.judge_next_values());
    assert!(!judge.judge_next_values());
    assert!(!judge.judge_next_values());
}

#[test]
fn test_judge_40mio_times() {
    let ga = Box::new(BasicGenerator::new(65,16807));
    let gb = Box::new(BasicGenerator::new(8921,48271));
    let mut judge = Judge::new(ga,gb);
    let matches = judge.judge_n_times(super::JUDGE_SAMPLE1);
    assert_eq!(matches,588);
}


#[test]
fn test_upgraded_generator_next() {
    let mut ga = Box::new(UpgradedGenerator::new(65,16807,4));
    let mut gb = Box::new(UpgradedGenerator::new(8921,48271,8));
    assert_eq!(ga.next(),1352636452);
    assert_eq!(gb.next(),1233683848);

    assert_eq!(ga.next(),1992081072);
    assert_eq!(gb.next(),862516352);

    assert_eq!(ga.next(),530830436);
    assert_eq!(gb.next(),1159784568);

    assert_eq!(ga.next(),1980017072);
    assert_eq!(gb.next(),1616057672);

    assert_eq!(ga.next(),740335192);
    assert_eq!(gb.next(),412269392);
}

#[test]
fn test_judge_upgraded_generator() {
    let ga = Box::new(UpgradedGenerator::new(65,16807,4));
    let gb = Box::new(UpgradedGenerator::new(8921,48271,8));
    let mut judge = Judge::new(ga,gb);
    for _ in 0..1055
    {
        assert!(!judge.judge_next_values());
    }
    assert!(judge.judge_next_values());
}


#[test]
fn test_judge_upgraded_generator_5mio_times() {
    let ga = Box::new(UpgradedGenerator::new(65,16807,4));
    let gb = Box::new(UpgradedGenerator::new(8921,48271,8));
    let mut judge = Judge::new(ga,gb);
    let matches = judge.judge_n_times(super::JUDGE_SAMPLE2);
    assert_eq!(matches,309);
}
