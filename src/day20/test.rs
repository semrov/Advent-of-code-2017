use super::{Particle,V3};

#[test]
fn test_from_line_str() {
    let p = Particle::from_line_str("p=<2,-3,5>, v=<-6,9,-1>, a=<-10,8,-10>");
    assert_eq!(p, Particle
        {
            position: V3(2,-3,5),
            velocity: V3(-6,9,-1),
            acceleration: V3(-10,8,-10)
        });
}