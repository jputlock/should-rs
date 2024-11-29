use should::*;

#[test]
fn usability() {
    let x = 3;

    x.should_be_eq(&3);
    x.should_be_gt(&2);
    x.should_be_leq(&4);
}
