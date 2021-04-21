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

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }
}
