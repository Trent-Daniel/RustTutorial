fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    // .3 is for decimal precision printing
    // : starts print formatter
    // 8 pads 8 characters in front. This means that there will be a total of 8 characters
    // 0 after the colon changes the padding charachters from spaces to seros
    // 0 before the colon specifies which variable we want to use at that point in the string.
    println!("c is {0:08.3}\na is {1}\nonce again, c is{0}", c, a);
}
