fn main() {
    say_hello();
    say_hello();
    // We don't explicitly say the type of 'x' or 'y', but the compiler is smart enough to know
    // that we only use them in say_the_sum, which expects u8, so the compiler treats 'x' and 'y'
    // as u8 integers.
    let x = 1;
    let y = 2;
    say_the_sum(x,y);
    /*
     * The following fails because say_a_number expects i32 but receives u8 after x is passed to
     * say_the_sum. x is u8 instead of i32 because the compiler assumes the data type from the
     * first use of the variable
    say_a_number(x);
    */
    // The following will work because we are casting the variable while passing it to the function
    // expecting i32
    say_a_number(x as i32);

    let result = square(13);
    println!("result is {}", result);

    println!("result is {}", square2(13));
    /*
     * This will not work because the default formatter can't print a tuple
    println!("result is {}", square3(13));
    */
    // This will work because the debug formatter is used
    println!("result is {:?}", square3(13));

    test_challenge_code();
}

fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum);
}

// If the last line of a function is an expression, the expression result will be returned.
fn square(x: i32) -> i32 {
    println!("squaring {}", x);
    x * x
}

fn square2(x: i32) -> i32 {
    println!("squaring {}", x);
    return x * x;
    println!("End of function");
}

fn square3(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
    println!("end of function");
}

/* CHALLENGE */
/*
 * write a celsius to fahrenheit function
 * input parameter: temperature in Celsius
 * return value: temperature in Farenheit
 * f = (1.8 * c) + 32
 */

fn test_challenge_code() {
    // This acts as a sort of main method for the challenge that we can call from the actual main
    // method, so as to not clutter the main method any more than it already is
    let celsius_temp = 23.0;
    let farenheit_temp = celsius_to_farenheit(celsius_temp);

    assert_eq!(farenheit_temp, 73.4);
    println!("Test passed!");
}

fn celsius_to_farenheit(celsius_temp: f64) -> f64 {
    return (celsius_temp * 1.8) + 32 as f64;
}
