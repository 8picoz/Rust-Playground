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
fn nondebug_test() {
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

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn reference_test() {
    let x = 5;
    let y = MyBox::new(x);
    let z = &x;
    let &w = z;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, w);
}

fn reference_test2() {
    let x = MyBox::new(MyBox::new(1));

    //Error: move occurs
    //let y = *x;

    //let y = &*x;
    //Error: move occurs
    //let &z = y;

    println!("{:?}", x)
}

fn main() {

}