fn main() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}", astronauts);

    let last = astronauts.pop();
    println!("last is {:?}", last);

    // This is not super safe because index two might be out of bounds
    //let third = &astronauts[2];
    //
    // An alternate and safer way to access vector elements is through the get() method, which
    // returns an Option enum containing a reference to the specified index
    let third = astronauts.get(2);
    println!("third is {:?}", third);

    let countdown = vec![5, 4, 3, 2, 1];
}
