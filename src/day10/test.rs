use super::KnotHash;

#[test]
fn test_hash() {
    let mut knot_hash = KnotHash::new(5);
    knot_hash.hash(&"3,4,1,5".split(",").map(|x| x.parse().unwrap()).collect());
    assert_eq!(knot_hash.list,vec![3,4,2,1,0]);
}


#[test]
fn test_hash_code()
{
    let mut knot_hash = KnotHash::new_full();
    let v = knot_hash.hash_code("");
    assert_eq!(KnotHash::hash_to_string(v),"a2582a3a0e66e6e86e3812dcb672a272");
    let mut knot_hash = KnotHash::new_full();
    let v = knot_hash.hash_code("AoC 2017");
    assert_eq!(KnotHash::hash_to_string(v),"33efeb34ea91902bb2f59c9920caa6cd");
    let mut knot_hash = KnotHash::new_full();
    let v = knot_hash.hash_code("1,2,3");
    assert_eq!(KnotHash::hash_to_string(v),"3efbe78a8d82f29979031a4aa0b16a9d");
}