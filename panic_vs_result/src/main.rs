fn main() {
    /*
     * TO panic! OR NOT TO panic!
     *
     * When you should call panic! and when you should return Result?
     *
     * When code panics there's no way to recover (you're making the decision
     * on behalf of the code calling your code.
     *
     * When you choose to return a Result value you give the calling code
     * options rather than making the decision for it.
     *
     * Therefore, returning Result is a good default choice when you're
     * defining a function that might fail.
     *
     * In rare situations it's more appropriate to write code that panics
     * instead of returning a Result.
     *
     * Let's explore why it's appropriate to panic in examples, prototype code
     * and tests.
     *
     */

    /*
     * EXAMPLES, PROTOTYPE CODE, AND TESTS
     *
     * In examples, having robust error-handling code can make the example less
     * clear. In examples it's understood that call methods like unwrap that
     * could panic! is meant as placeholder for the error handling.
     *
     * Similary, unwrap and expect methods are very handy when prototyping,
     * before you're reade to decide how to handle errors.
     *
     * If a method call fails in a test, you'd want the whole test to fail,
     * even if method isn't the functionality under test.
     *
     */

    /*
     * CASES IN WHICH YOU HAVE MORE INFORMATION THAN THE COMPILER
     *
     * Example: We're creating an IpAddr instance by parsing a hardcoded
     * string ("127.0.0.1"). We can see that is a valid IP address, so it's
     * acceptable to use unwrap.
     *
     * However, this fact doesn't change the return type of the parse method:
     * we still get a Result value, and the compiler will still make us handle
     * the Result as if the Err variant is a possibility.
     *
     * If the IP address come from an user rather than being hardcoded , we'd
     * definitely want to handle the Result in a more robust way instead.
     *
     */

    /*
     * GUIDELINES FOR ERROR HANDLING
     *
     * It's advisable that your code panic when it's possible that ir end up in
     * a bad state (some assumption, guarantee, contract, or invariant has been
     * broken) such as when invalid values, contradictory values or missing
     * values are passed to your code -- plus one or more of the following
     *
     *  - The bad state is not something that's expected to happen
     *    occasionally.
     *  - Your code after this point needs to rely on not being in this bad
     *    state.
     *  - There's not a good way to encode this information in the types you
     *    use.
     *
     */

    /*
     * CREATING CUSTOM TYPES FOR VALIDATION
     *
     * Let's take the idea of Rust's type system to ensure we have a valid
     * value one step further and look at creating a custom type for 
     * validation.
     *
     * For example, in guessing game we can ensure that input value must be
     * between 1 and 100:
     *
     */

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.",
                       value);

            }
                Guess { value }
        } 

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
