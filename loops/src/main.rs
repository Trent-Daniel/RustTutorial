fn main() {
    let mut count = 0;

    /*
     * This loop will run infinitely
    loop {
        count += 1;
        println!("count is {}", count);
    }
    */

    loop {
        if count == 10 {
            break;
        }
        count += 1;
        println!("count is {}", count);
    }

    println!("After the loop");

    // This kind of loop is useful if we want to try doing something repeatedly that might not
    // always be possible or correct
    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);

    let mut count = 20;
    // If the count is greater than 10 before the first iteration of the loop, the loop won't occur
    while count < 10 {
        count += 1;
        println!("count is {}", count);
    }

    let mut arr_count = 0;
    let letters = ['a', 'b', 'c'];

    while arr_count < letters.len() {
        println!("letter is {}", letters[arr_count]);
        arr_count += 1;
    }

    let message = ['h', 'e', 'l', 'l', 'o'];

    // Under the hood, Rust turns the message into an interator
    // An iterator implements the logic necessary to iterate over each item in a collection; has a
    // method called 'next' which calls the next entry in the collection
    for item in message {
        println!("item is {}", item);
    }

    // You can also use an iterator directly
    for item in message.iter() {
        println!("item is {}", item);
    }

    for (index, item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
    }

    // The & is required to refer to the char during conditional evaluation
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }

    // Nested loops
    let mut matrix = [[1, 2, 3],
                  [4, 5, 6],
                  [7, 8, 9]];

    for row in matrix.iter() {
        for num in row.iter() {
            print!("{}\t", num);
        }
        println!();
    }

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            // iter_mut passes references to the array entry, so we need to dereference num to
            // change the value
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }

    /* CHALLENGE */
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    let mut sum: i32 = 0;
    max = numbers[0];
    min = numbers[0];

    for entry in numbers {
        sum += entry;
        if entry > max {
            max = entry;
        }
        if entry < min {
            min = entry;
        }

    }

    mean = sum as f64 / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
