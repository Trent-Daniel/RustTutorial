fn main() {
    let x = 10;
    println!("x is {}", x);
    /*
     * The following will cause E0384 because variables by default are immutable, and we try to
     * change the value on the following line. To make a variable mutable, declare it using 'let mut'
     * instead of 'let'.
    x = 20;
    println!("x is {}", x)
    */
    let mut y = 10;
    println!("y is {}", y);
    y = 20;
    println!("y is {}", y);

    /*
    // This will work
    let z = -1000;
    
    // This will not work because we assign a negative number to an unsigned integer. This will
    // fail at compile time.
    let z: u8 = -1000;
    
    // This will not work because we assign a number outside the range ofi u8. This will fail at
    // compile time.
    let z: u8 = 1000;

    // This will not work because we try to overwrite the value of a u8 with a number outside the
    // allowed range. This will fail at runtime, but not at compile time.
    let mut z: u8 = 255;
    z = z + 1;
    */

    let xx: f32 = 10.563534634534636; // This will default to 64-bit float
    println!("xx is {}", xx);
    let yy = 10.456435634345543; // This will create a 32 float
    println!("yy is {}", yy);

    let a = 10;
    let b = 3;
    let c = a + b;
    println!("c is {}", c);
    let d = a - b;
    println!("d is {}", d);
    let e = a * b;
    println!("e is {}", e);
    let f = a / b;
    println!("f is {}", f);
    let g = a % b;
    println!("g is {}", g);

    /* This will not work because of ambiguity of data type
    let h = 10;
    let i = 3.0;
    let j = h / i;
    */
    let h = 10;
    let i = 3.0;
    let j = h as f64 / i;
    println!("j is {}", j);



}
