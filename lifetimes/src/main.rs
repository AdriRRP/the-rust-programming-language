fn main() {

    // The problem
    {
        let r; // valueless declaration only works if after assigned

        {
            let x = 5;
            r = &x;
        }

        /*
         * The following doesn't works because it's a potentially dangling
         * reference.
         *
         */
        // println!(r: {}", r);
    }

    /*
     * Borrow checker with lifetimes
     *
     * 'a is the lifetime of r
     * 'b is the lifetime of x
     *
     * It doesn't compile becaus 'a is longer than 'b
     *
     */
        
    {
        let r;                  // ---------+-- 'a
                                //          |
        {                       //          |
            let x = 5;          // -+-- 'b  |
            r = &x;             //  |       |
        }                       // -+       |
                                //          |
        // println!(r: {}", r); // ---------+
    }

    /*
     * Fixing previous code
     *
     * Now 'b is longer than 'a, so its works
     *
     */
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    /*
     * Lifetime syntax
     *
     * The names of lifetime parameters must start with an apostrophe (') and
     * are usually all lowercase and very short, like generic types. Most
     * people use the name 'a. We place lifetime parameter annotations after
     * the & of a reference, using a space to separate the annotation from the
     * reference’s type.
     *
     * &i32        // a reference
     * &'a i32     // a reference with an explicit lifetime
     * &'a mut i32 // a mutable reference with an explicit lifetime
     *
     */

    /*
     * Generic lifetimes in functions
     *
     * Using largest function
     *
     */

    {
        /*
         * Without lifetime: compiler error
         *
         * Rust doesn't knows how much time the return value live
         *
         * fn longest(x: &str, y: &str) -> &str {
         *     if x.len() > y.len() {
         *         x
         *     } else {
         *         y
         *     }
         * }
         *
         */

        /*
         * With lifetimes: it works!
         *
         * Now Rust's compiler knows that return value's lifetime is equal to
         * SHORTEST lifetime between x and y.
         *
         */

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);

        /*
         * But the fowolling doesn't works:
         *
         * let string1 = String::from("long string is long");
         *     let result;
         *     {
         *         let string2 = String::from("xyz");
         *         result = longest(string1.as_str(), string2.as_str());
         *     }
         *     println!("The longest string is {}", result);
         * }
         * 
         * because `string2` does not live long enought
         *
         */
    }
    
    {
        // The following also works: lifetime of y does not have any
        // relationship with the lifetime of x or the return value

        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }

        /*
         * But the following fails because we create a dangling reference
         *
         * fn longest<'a>(x: &str, y: &str) -> &'a str {
         *     let result = String::from("really long string");
         *     result.as_str()
         * }
         *
         */

    }

    {
        // Lifetimes in struct definitions
        
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    {
        /*
         * LIFETIME ELISION: set of particular cases that the compiler will
         * consider, and if your code fits these cases, you don’t need to
         * write the lifetimes explicitly.
         *
         * Lifetimes on function or method parameters are called input
         * lifetimes, and lifetimes on return values are called output
         * lifetimes.
         *
         * 1) The first rule is that each parameter that is a reference gets
         * its own lifetime parameter. In other words, a function with one
         * parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a
         * function with two parameters gets two separate lifetime parameters:
         * fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
         * 
         * 2) The second rule is if there is exactly one input lifetime
         * parameter, that lifetime is assigned to all output lifetime
         * parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
         * 
         * 3) The third rule is if there are multiple input lifetime parameters,
         * but one of them is &self or &mut self because this is a method, the
         * lifetime of self is assigned to all output lifetime parameters.
         * This third rule makes methods much nicer to read and write because
         * fewer symbols are necessary.
         * 
         */

        /*
         * Example with first_word function signature:
         *
         *     fn first_word(s: &str) -> &str {
         *
         * After apply 1):
         *
         *     fn first_word<'a>(s: &'a str) -> &str {
         *
         * After apply 2):
         *
         *     fn first_word<'a>(s: &'a str) -> &'a str {
         *
         */

        /*
         * Example with longest function without parameters (fail):
         *
         *     fn longest(x: &str, y: &str) -> &str {
         *
         * After apply 1):
         *
         *     fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
         *
         * The 2) rule doesn't apply because is more than one input lifetime.
         *
         */

        {

            // Lifetime annotation in method definition

            struct ImportantExcerpt<'a> {
                part: &'a str,
            }
            
            /*
             * The lifetime parameter declaration after impl and its use after
             * the type name are required, but we’re not required to annotate
             * the lifetime of the reference to self because of the first
             * elision rule:
             *
             */

            impl <'a> ImportantExcerpt<'a> {
                fn level(&self) -> i32 {
                    3
                }
            }

            /*
             * Example of 3) rule
             *
             * There are two input lifetimes, so Rust applies the first
             * lifetime elision rule and gives both &self and announcement
             * their own lifetimes. Then, because one of the parameters is
             * &self, the return type gets the lifetime of &self, and all
             * lifetimes have been accounted for:
             *
             */

            impl<'a> ImportantExcerpt<'a> {
                fn announce_and_return_part(&self, announcement: &str) -> &str {
                    println!("Attention please: {}", announcement);
                    self.part
                }
            }

        }


    }

    {
        // The static lifetime

        let s: &'static str = "I have a static lifetime.";

        /*
         * The text of this string is stored directly in the program’s binary,
         * which is always available. Therefore, the lifetime of all string
         * literals is 'static.
         *
         * You might see suggestions to use the 'static lifetime in error
         * messages. But before specifying 'static as the lifetime for a
         * reference, think about whether the reference you have actually lives
         * the entire lifetime of your program or not. You might consider
         * whether you want it to live that long, even if it could. Most of the
         * time, the problem results from attempting to create a dangling
         * reference or a mismatch of the available lifetimes. In such cases,
         * the solution is fixing those problems, not specifying the 'static
         * lifetime.
         *
         */
    }

    {
        // Generic Type Parameters, Trait Bounds, and Lifetimes Together
        use std::fmt::Display;
        
        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }

}
