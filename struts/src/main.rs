fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Updating values (needs to be mutable, obviously)

    let mut user = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user.email = String::from("anotheremail@example.com");

    // IMPORTANT! The entire instance must be mutable

    // As with any expression, we can construct a new instance of the struct as
    // the last expression in a function body to implicitly return new instance

    {
        fn build_user(email: String, username: String) -> User {
            User {
                email: email,
                username: username,
                active: true,
                sign_in_count: 1,
            }
        }
    }

    // With init shorthand we can simplify value assignment as follows:
    fn build_user(email: String, username: String) -> User {
        User {
            email, // init shorthand
            username, // init shorthand
            active: true,
            sign_in_count: 1,
        }
    }

    // Struct update syntax 
    let user2 = User {
        email: String::from("anothe@example.com"),
        username: String::from("anotherusername567"),
        ..user // Take the rest of parameters from user
    };

    // Tuple struct

    // Fields without names
    // We can access by the index: .1 , .2 or .3
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // black and origin are different because they have different types

    // Unit-Like Structs
    // Structs without fields
    // Can be useful in situations in which you need to implement a trait 
    // on some type but don't have any data that you want to store in the 
    // type itself.
    struct blank();

    // Ownership of Struct Data
    // We use owned String type rather than &str in User definition.
    // This is because we want instances of this struct to own all of
    // its data and for that data to be valid for as long as the entire
    // struct is valid.

    // It's possible for structos to store references to data owned by 
    // something else, but to do so requires the use of lifetimes.

    // The following code won't work:

    /*

    struct User {
        username: &str,
        email: &str,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: "someone@example.com",
        username:someusername123",
        active: true,
        sign_in_count: 1,
    };

    */

    // Example program using structs

    // First idea (without structs)
    {
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} saquare pixels.",
            area(width1, height1)
        );
    }

    // Refactoring with tuples
    // A bit more structured
    {
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} saquare pixels.",
            area(rect1)
        );
    }

    // Refactoring with structs
    // adding more meaning
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        
        println!(
            "The area of the rectangle is {} saquare pixels.",
            area(&rect1)
        );
    }

    // Adding useful functionality with derived traits 
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        
        // This throw an error because rust doesn't know how to 
        // represent a Rectangle
        // println!("rect1 is {}", rect1);
    }

    // Adding basic representation deriving Debug trait 
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        
        println!("rect1 is {:?}", rect1); // Requieres :? inside curly brackets
        println!("rect1 is {:#?}", rect1);  // Use #:? inside curly brackets to 
                                            // get prettier multiline representation

        // derive annotation can add useful behavior to our cutom types.

    }



}
