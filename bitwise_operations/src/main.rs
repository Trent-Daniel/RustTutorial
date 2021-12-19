fn main() {
    let mut value = 0b1111_0101u8; // default is 32 bit signed integer, but we can specify different type like u8
    // Rust doesn't care that we specified the value as a binary number, in the below println!, it
    // will be printed like an integer in decimal
    println!("value is {}", value);
    // The following is the print formatter for print an 8 bit binary number
    println!("value is {:08b}", value);

    // NOT
    value = !value;
    println!("value is {:08b}", value);

    // AND
    value = value & 0b1111_0111;
    println!("value is {:08b}", value);

    // OR
    value = value | 0b0100_0000;
    println!("value is {:08b}", value);

    // XOR
    value = value ^ 0b0101_0101;
    println!("value is {:08b}", value);

    // SHIFT LEFT
    value = value << 4;
    println!("value is {:08b}", value);

    // SHIFT RIGHT
    value = value >> 2;
    println!("value is {:08b}", value);

    // Boolean stuff
    let a = true;
    let b = false;

    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    // Short circuit logic
    // Only the left side here is evaluated because it is true
    let c = (a ^ b) || (a & b);
    println!("c is {}", c);
    
    // A panic macro causes the program to crash. But, it is not evaluated since the left side is
    // found to be true. Short circuit logic only works for boolan logic, and not bitwise logic. If
    // the panic macro was on the left side, the program would crash since the left side is
    // evaluated first.
    let d = (a ^ b) || panic!();
    println!("c is {}", c);

    let e = true;
    let f = false;
    println!("e is {} and f is {}", e, f);
    println!("e EQUAL to f is {}", e == f);
    println!("e NOT EQUAL to f is {}", e != f);
    println!("e GREATER THAN f is {}", e > f);
    println!("e GREATER THAN OR EQUAL TO f is {}", e >= f);
    println!("e LESS THAN f is {}", e < f);
    println!("e LESS THAN OR EQUAL TO f is {}", e <= f);

    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);

    // CHALLENGE STARTS HERE:
    // starting code:
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    /* YOUR CODE GOES HERE */
    let average = (((a as f32 + b as f32 + c ) / 3.0) * 10.0).round() / 10.0;

    /* TEST your code */
    assert_eq!(average, 45.1);
    println!("Test passed!");
}
