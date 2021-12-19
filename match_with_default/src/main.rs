fn main() {

    // The following will throw a compiler error because u8 can range from 0..255, and this only
    // covers 0..3. The compiler requires that ALL possible cases be acconted for in the match
    // statement
    //let my_number = 1u8;
    //let result = match my_number {
    //    0 => "zero",
    //    1 => "one",
    //    2 => "two",
    //    3 => "three"
    //};

    // The following will compile because we have accounted for all other cases using the wildcard
    // symbol '_'. Try each of the following scenarios to see for yourself:
    //let my_number = 1u8;
    let my_number = 4u8;
    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("result is {}", result);
}

// Note, the wildcard has to go at the end of the match list, because the match will proceed with
// whichever case it matches first. The wildcard matches all cases, so if it is not last, then
// any cases written after the wildcard will never be reached.
