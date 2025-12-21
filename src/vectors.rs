
pub fn vectors_handler() {
    let mut v1 = vec![1,2,3]; // declare a vector
    let mut v1_iter = v1.iter_mut();
    while let Some(val) = v1_iter.next() { //  
        println!("{}", val);
    }
    iterators();
}

fn iterators(){
    let vec1 = vec![1,2,3,4,5];
    let vec1_iter = vec1.iter();
    let total: i32 = vec1_iter.sum(); // sum method takes ownership of vec1_iter, in the declaration sum<S>(self), it takes self as an input, hence takes ownership
    assert_eq!(total, 15);
    println!("{}", total);
}