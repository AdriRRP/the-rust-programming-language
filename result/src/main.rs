use std::fs::File;

fn main() {
    /*
     * REVOCERABLE ERRORS WITH RESULT
     *
     * Most errors aren't serious enough to require the program to stop
     * entirely. If you try to open a file and fails because it doesn't exists,
     * you might want to create the file instead of terminating the process.
     *
     * Result enum is defined as having two variants, Ok and Err:
     *
     * enum Result<T, E> {
     *  Ok(T),
     *  Err(E),
     * }
     *
     * T and E are generic type parameters. T is the type of value returned in
     * a success case within the Ok variant, and E the same in a failure case
     * for the Err variant.
     *
     */

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    /*
     * MATCHING ON DIFFERENT ERRORS
     *
     * We can select the behaviour in different kind of errors:
     *
     */

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?]", e)
            },
            other_error => {
                panic!("Problem opening rhe file: {:?}", other_error)
            }
        },
    };

    /* 
     * There are a lot of match! A more seasoned Rustacean might write:
     *
     */

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    /*
     * SHORTCUTS FOR PANIC ON ERROR: unwrap and expect
     *
     * unwrap method is a shortcut implemented like the match expression, if
     * the Result is the Ok variant, will return the value, if is the Err
     * variant, call the panic! macro for us.
     *
     */

    let f = File::open("hello.txt").unwrap();

    /*
     * expect method is similar to unwarap, but it receives a message to show
     * on panic! instead of the default one.
     *
     */

    let f = File::open("hello.txt").expect("Failed to open hello.txt");


}
