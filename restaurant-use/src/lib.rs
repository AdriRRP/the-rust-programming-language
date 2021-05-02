// We can bring a path into a scope once and then call the items in that path as if 
// they’re local items with the use keyword.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // Similar to symlink in filesystem

// Relative path can be used
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// We can use the function directly, but is not recommended
// Functions without package are usually functions defined in the same file

use crate::front_of_house::hosting::add_to_waitlist; // The context of founction is lost 

pub fn eat_at_restaurant_2() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// IDIOMATIC PATHS are useful when bringing structs, enums and other items

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// But there is an exception using items: 2 items with the same name
/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
*/

// Providing New Names with the as Keyword
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
*/
// Re-exporting Names with pub use

// When we bring a name into scope with the use keyword, the name available in the new scope is private.
// To enable the code that calls our code to refer to that name as if it had been defined in that code’s
// scope, we can combine pub and use

mod front_of_house_2 {
    pub mod hosting_2 {
        pub fn add_to_waitlist_2() {}
    }
}

pub use crate::front_of_house_2::hosting_2;

pub fn eat_at_restaurant_3() {
    hosting_2::add_to_waitlist_2();
    hosting_2::add_to_waitlist_2();
    hosting_2::add_to_waitlist_2();
}

// Use external crates

/*
In cargo.toml, add:

[dependencies]
rand = "0.5.5"

Then, in code, as we see previously:

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

-- Bringing items:

use std::collections::HashMap;

*/

// Using Nested Paths to Clean Up Large use Lists

/*
use std::cmp::Ordering;
use std::io;

can be written as:
*/
use std::{cmp::Ordering, io};

// The Glob Operator
// bring all public items defined in a path into scope

use std::collections::*;

