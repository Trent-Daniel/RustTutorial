fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5]; // Create array of size 5 of type i32
    numbers = [0; 5]; // Assign five zeroes to array
    println!("last number is {}", numbers[4]);
    // usize is based on number of bytes needed to reference memory. e.g. on 64-bit, usize is 64;
    // on 32-bit, usize is 32
    /*
    // The following will compile but will fail at runtime because the length is 5, but rust is
    // 0-indexed, so index five is the sixth element, which is out of bounds
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index]);
    */

    let parking_lot = [[1,2,3],
                       [4,5,6]];

    /*
    // The following will fail to compile because 2D arrays in rust must be square
    let parking_lot = [[1,2,3],
                       [4,5,6]];

    */
    let number = parking_lot[0][1];
    println!("number is {}", number);

    /*
    // Declare
    let garage: [[[i32: 100]; 20]; 5]';
    // Instantiate with zeroes
    let garage: [[[0: 100]; 20]; 5]';
    */
}
