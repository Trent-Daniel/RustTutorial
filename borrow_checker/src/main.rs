fn main() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        // This is a good reference and will not throw an error
        println!("propellant is {}", propellant);
    }
    // This is a bad reference, and will throw an error
    //println!("propellant is {}", propellant);
    //
    //This is important to note, because we're not just talking about variable scope. We are also
    //talking about the borrow checker observing the lifetime of references. In the second println,
    //propellant is still in scope, but the rp1 variable to which it refers is not in scope.
    //Therefore, rp1 was freed after going out of scope, but propellant is still referring to it,
    //meaning that propellant is a stale or incorrect reference. Comparing the scope of references
    //and the lifetime of references is at least in part how the borrow checker in the compiler
    //helps to make sure our code is safe
}
