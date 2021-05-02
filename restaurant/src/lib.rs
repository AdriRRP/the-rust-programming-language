/*
Initial representation of module

mod front_of_house {
	mod hosting {
		fn add_to_whitelist() {}
		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}
		
		fn serve_order() {}

		fn take_payment() {}
	}
}
*/

/*
This code doesn't compile!
Function add_to_whitelist() is not public, and can't be called 

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

*/

/*
This code doesn't compile!
Making the module public doesn't implies that its components are public

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/

// IMPORTANT! We can use front_of_house module inside eat_at_restaurant because
// they are SIBLINGS (are defined in the same module)

// Suppose that the chef fixes an incorrect order and personal bring it out the
// customers:
/*
fn serve_order() {}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order(); // Like ../ in paths
	}

	fn cook_order() {}
}
*/

// Making structs and enums public

// pub modifier makes a struct public, but its members still private
// We can make each field public or not on a case-by-case 
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// In contrast, if we make an enum public, all of its variants are then public. 

mod back_of_house_2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_2() {
    let order1 = back_of_house_2::Appetizer::Soup;
    let order2 = back_of_house_2::Appetizer::Salad;
}
