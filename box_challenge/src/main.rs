// The std::ops::Add<Output = T> was recommended by the compiler and was added to make the file
// run, but traits like these will be discussed only in a later chapter
fn sum_boxes<T: std::ops::Add<Output = T>>(num1: Box<T>, num2: Box<T>) -> Box<T> {
    Box::new(*num1 + *num2)
}

fn main() {
    println!("beginning program");
    // Both of the following are equivalent
    let box1: Box<u8> = Box::new(5);
    let box2 = Box::<u8>::new(10);
    let sum = *sum_boxes(box1, box2);
    println!("sum of boxes is: {}", sum);
    assert_eq!(sum, 15);
    let box3 = Box::new(5.6);
    let box4 = Box::new(7.9);
    let sum = *sum_boxes(box3, box4);
    println!("sum of boxes is: {}", sum);
    assert_eq!(sum, 13.5);
    println!("ending program");
}
