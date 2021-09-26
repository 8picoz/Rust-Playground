use std::any::type_name;
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

use std::num::ParseIntError;
use std::ops::Deref;
use std::panic::catch_unwind;
use std::panic::resume_unwind;

use adqselect::nth_element;
use rust_playground::asynchronous::asyncronous;

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

//#[tokio::main]
//async fn main() {
//    asyncronous();
//    println!("hello");
//}

use rand::Rng;

fn main() {
    let p = 0.99;
    let mut result = 0.0;
    let mut rng = rand::thread_rng();

    let mut i = 0;
    loop {
        let rnd_value = rng.gen_range(0.0..=1.0);
        
        if rnd_value >= p {
            break;
        }

        result += (0.99_f32.powi(i)) / (0.99_f32).powi(i + 1) as f32;

        i += 1;
    }

    println!("{}", result);
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn unwind() {
    let result = catch_unwind(|| {
        panic!("panic!");
    });

    println!("catched panic");

    if let Err(e) = result {
        println!("Error occur");
        resume_unwind(e);
    }

    println!("end");
}

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError)
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
struct EmptyVec;

fn multiple_error(vec: Vec<&str>) -> Result<i32>{
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}