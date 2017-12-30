use super::{StepTuple};

#[test]
fn test_sum_steptuples() {
    let input = "n,n,n,ne,ne,se,se,se,se,s,sw,sw,sw,nw,nw";
    assert_eq!(StepTuple::sum_steptuples(input),StepTuple(3,2,4,1,3,2));
    let input = "n,n,ne,ne,ne,se,se,se,se,s,s,s,s,s,sw,sw";
    assert_eq!(StepTuple::sum_steptuples(input),StepTuple(2,3,4,5,2,0));
}

#[test]
fn test_reduce()
{
    assert_eq!(StepTuple::sum_steptuples("ne,ne,ne").reduce(),3);
    assert_eq!(StepTuple::sum_steptuples("ne,ne,sw,sw").reduce(),0);
    assert_eq!(StepTuple::sum_steptuples("ne,ne,s,s").reduce(),2);
    assert_eq!(StepTuple::sum_steptuples("se,sw,se,sw,sw").reduce(),3);
}