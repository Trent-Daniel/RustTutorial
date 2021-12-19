fn main() {
    let x = 4;

    if x == 3 {
        println!("x is 3!");
    }

    /*
     * This will not work because Rust expects a boolean, not an i32. Regardless of the value of x,
     * if it is not boolean, then the conditional expression will not compile
    if x {
        println!("x is True!");
    }
    */

    let z = 3;
    let y = 5;

    if z > y {
        println!("z is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }

    if z > y {
        println!("z is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }

    
    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};
    
    // The following won't work because the compiler needs to know what the data type will be, so
    // you can't choose between an int and a float, but only between two i32 for example.
    //let x = if make_x_odd {1} else {2.0};

    /*
    // This commented block can be substituted by the previous let statement
    let x;

    if make_x_odd {
        x = 1;
    } else {
        // Even though this will never be executed, if we comment it out, the compilation will
        // fail, since the compiler will think that x could be uninitialized before being printed
        // later on
        x = 2;
    }
    */
    println!("x is {}", x);
}
