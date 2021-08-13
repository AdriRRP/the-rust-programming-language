use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

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

    /*
     * PROPAGATING ERRORS
     *
     * Instead of handling errors, yo can return the error to calling code
     * so that it can decide what to do (propagating).
     *
     */

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    
    /*
     * A SHORTCUT FOR PROPAGATING ERRORS: ? Operator
     *
     */

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    /*
     * If the value of the Result is an Ok, the value inside the Ok will get
     * returned from this expression, and the program will continue.
     *
     * If the value is an Err, the Err will be returned from the whole function
     * as if we had used the return keyword so the error value gets propagated
     * to the calling code.
     *
     * There is a difference between the match expression and what the ?
     * operator does: error values that have the ? operator calls the from
     * function (defined in the From trait in std library) used to convert
     * errors from one type into another.
     *
     * When the ? operator calls from function, error type received is
     * converted into the one error type to represent all the ways a function
     * might fail, even if parts might fail for manyu different reason.
     *
     * We can do de following simplification:
     *
     */

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    /*
     * But... there's a way to make this even shorter:
     *
     */
    
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    /*
     * Because the operation is very common, Rust provides the convenient
     * fs::read_to_string function that opens the file, creates a new String,
     * reads the contents of the file, puts the contents into rhat String, and
     * return it.
     *
     */

    /* 
     * THE ? OPERATOR CAN BE USED IN FUNCTIONS THAT RETURN Result
     *
     * because it is defined to work in the same way as the match expression
     * we defined previously. The part of the match that requires a return 
     * type of Result is return Err<e>, so the return type of the function
     * has to be a Result to be compatible with this return.
     *
     */

    /*
     * Doing the following:
     *
     * use std:fs::File;
     *
     * fn main() {
     *     let f = File::open("hello.txt")?;
     * }
     *
     * causes a compilation error because the return type of main() isn't a
     * Result or Option or another type that implements std::ops::Try.
     *
     * You have 2 options: change the return type of the function to
     * Result<T, E> or using the match expression.
     *
     * The main function is special, and there are restrictions on what its
     * return type must be. One valid return type for main is (), and 
     * conveniently, another valid return type is Result<T, E>:
     *
     * use std::error::Error;
     * use std::fs::File;
     *
     * fn main() -> Ewaulr<(), Box<dyn Error>> {
     *     let f = File::open("hello.txt")?;
     *
     *     Ok(());
     * }
     *
     * The Box<dyn Error> type is called a trait object, that at the moment
     * has the mean "any kind of error.
     *
     */
     *
     */
    
     *
     */




}
