fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {

    let mut temp: i32;

    for i in (0..vec.len()).rev() {
        for j in 0..i {
            if vec[j] > vec[j + 1] {
                temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
            }
        }
    }

    vec
}

fn bubble_sort_func() {
    let vec = vec![1, 49, 5, 32, 2, 5, 5, 1, 89, 8, 7, 6];
    println!("{:?}", vec);

    let vec = bubble_sort(vec);
    println!("{:?}", vec);
}
