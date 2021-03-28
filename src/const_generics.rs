use std::usize;

fn array_len<const LENGTH: usize>() -> usize {
    [0; LENGTH].len()
}

pub fn const_generics() {
    let a = array_len::<10usize>();
    let b = array_len::<20usize>();

    println!("a: {}", a);
    println!("b: {}", b);
}
