use std::fmt::Display;


#[derive(PartialEq, Debug)]
struct NonDebug {
    value: i32,
}

impl NonDebug {
    fn new(value: i32) -> NonDebug {
        NonDebug { value }
    }
}

impl Display for NonDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "NonDebug")
    }
}

#[allow(dead_code)]
fn NonDebug_test() {
    let a = NonDebug::new(1);
    let b = NonDebug::new(2);

    assert_eq!(a, b);
}

//owning_ref
/*
fn invalid() -> ([i32; 3], &[i32]) {
    let v = [1, 2, 3];
    let s = &v[1..2];
    (v, s)
}
*/

fn main() {
    
}