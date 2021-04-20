fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The lengrh of '{}' is {}.", s1, len);

    // let s = String::from("hello");
    // change(&s); WONT'T WORK!

    let mut s = String::from("hello");
    change(&mut s);
    /*
    Mutable references have one big restriction: you can have only one mutable
    reference to a particular piece of data in a particular scope.
    This FAIL:

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    */
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    /*
    This code results in an error:

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    We also cannot have a mutable reference while we have an inmutable one.
    */

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    /*
    Dangling references (will fail)
    let reference_to_nothing = dangle();
    */

    // THE RULES OF REFERENCES
    //  - At any given time, you can have either one mutable reference or
    //  any number of inmutable references.
    //  - References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

/*
WON'T COMPILE! Because can't modify borrowed variable
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
Dangling reference NOT COMPILE!

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello");  // s is a new String

    &s  // we return a reference to the String, s
}   // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!
*/
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
