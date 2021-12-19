fn main() {
    let countdown = [5, 4, 3, 2, 1];
    // The following will throw a compiler error because the index is out of range
    //let number = countdown[5];

    // Instead, let's try using the get() method. The get method can be used on slices to return an
    // Option enum holding a reference to the value at the specified index
    // Because we are getting from index 5, this will return None, but will still compile
    let number = countdown.get(5);
    println!("number is {:?}", number);
    // Because we are getting from index 4, this will return Some(1)
    let number = countdown.get(4);
    println!("number is {:?}", number);
    // The following will not compile, because number is an Option enum referencing an int-type
    // value, and is not actually an integer
    //let number = number + 1;
    // This works, but calling unwrap() is discouraged because you can't unwrap None
    //let number = number.unwrap() + 1;
    // unwrap_or returns it's parameter if the instance it's being called from is None
    let number = number.unwrap_or(&0) + 1;
    println!("number is {:?}", number);
}
