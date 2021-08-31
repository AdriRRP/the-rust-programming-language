use std::fmt::Display;

fn main() {
    /*
     * TRAITS: DEFINING SHARED BEHAVIOR
     * 
     * A trait tells the Rust compiler about functionality a partiruclar type has and can 
     * share with other types. We can use traits to define shared behavior in an abstract 
     * way. We can use trait bounds to specify that a generic can be any type that has
     * certain behavior.
     * 
     * NOTE: Traits are similar to a feature often called interfaces in other languages, 
     * although with some differences. 
     * 
     */ 

    /*
     * DEFINING A TRAIT
     * 
     * A type's behavior consist of the methods we can call on that type. Different types
     * share the same behavior if we can call the same methods on all of those types.
     * 
     * Trait definitons are a way to group methods signatures together to define a set of
     * behaviors neccesary to accomplish som purpose.
     * 
     */ 

    /*
     * Example:
     * 
     * Tweets and News articles must be stored in media aggregator.
     * We can summarize it with a trait.
     * 
     */ 

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    /*
     * Implementing a Trait on a Type
     * 
     */ 
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    /*
     * One restriction to note with trait implementations is that we can implement a trait
     * on a type only if either the trait or the type is local to our crate. For example,
     * we can implement standard library traits like Display on a custom type like Tweet
     * as part of our aggregator crate functionality, because the type Tweet is local to
     * our aggregator crate. We can also implement Summary on Vec<T> in our aggregator
     * crate, because the trait Summary is local to our aggregator crate.
     * 
     * But we can’t implement external traits on external types. For example, we can’t
     * implement the Display trait on Vec<T> within our aggregator crate, because
     * Display and Vec<T> are defined in the standard library and aren’t local to our
     * aggregator crate. This restriction is part of a property of programs called
     * coherence, and more specifically the orphan rule, so named because the parent type
     * is not present. This rule ensures that other people’s code can’t break your code
     * and vice versa. Without the rule, two crates could implement the same trait for
     * the same type, and Rust wouldn’t know which implementation to use.
     * 
     */ 

    /*
     * DEFAULT IMPLEMENTATION
     * 
     * Sometimes it's useful to have a default behavior.
     * 
     *    pub trait Summary {
     *        fn summarize(&self) -> String {
     *            String::from("(Read more...)")
     *        }
     *    }
     *
     */

    /* 
     * We specify an empty impl block with impl Summary for NewsArticle {}.
     * 
     * Even though we’re no longer defining the summarize method on NewsArticle directly,
     * we’ve provided a default implementation and specified that NewsArticle implements
     * the Summary
     * 
     *     let article = NewsArticle {
     *         headline: String::from("Penguins win the Stanley Cup Championship!"),
     *         location: String::from("Pittsburgh, PA, USA"),
     *         author: String::from("Iceburgh"),
     *         content: String::from(
     *             "The Pittsburgh Penguins once again are the best \
     *              hockey team in the NHL.",
     *         ),
     *     };
     * 
     *     println!("New article available! {}", article.summarize());
     *
     */

    /*
     * Default implementations can call other methods in the same trait, even
     * if those other methods don’t have a default implementation. In this way,
     * a trait can provide a lot of useful functionality and only require
     * implementors to specify a small part of it. For example, we could
     * define the Summary trait to have a summarize_author method whose
     * implementation is required, and then define a summarize method that
     * has a default implementation that calls the summarize_author method:
     * 
     * 
     * pub trait Summary {
     *     fn summarize_author(&self) -> String;
     * 
     *     fn summarize(&self) -> String {
     *         format!("(Read more from {}...)", self.summarize_author())
     *     }
     * }
     * 
     * To use this version of Summary, we only need to define summarize_author
     * when we implement the trait on a type:
     * 
     * 
     * impl Summary for Tweet {
     *     fn summarize_author(&self) -> String {
     *         format!("@{}", self.username)
     *     }
     * }
     * 
     * After we define summarize_author, we can call summarize on instances
     * of the Tweet struct, and the default implementation of summarize will
     * call the definition of summarize_author that we’ve provided. Because
     * we’ve implemented summarize_author, the Summary trait has given us the
     * behavior of the summarize method without requiring us to write any
     * more code.
     * 
     * 
     * let tweet = Tweet {
     *     username: String::from("horse_ebooks"),
     *     content: String::from(
     *         "of course, as you probably already know, people",
     *     ),
     *     reply: false,
     *     retweet: false,
     * };
     * 
     * println!("1 new tweet: {}", tweet.summarize());
     *  
     * 
     */ 

    /*
     * TRAITS AS PARAMETERS
     */ 

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    /*
     * TRAITS BOUND SYNTAX
     * 
     *   pub fn notify<T: Summary>(item: &T) {
     *       println!("Breaking news! {}", item.summarize());
     *   }
     *   
     * The impl Trait syntax is convenient and makes for more concise code in simple cases
     * 
     *   pub fn notify(item1: &impl Summary, item2: &impl Summary) {
     *   
     * can be written as
     *   
     *   pub fn notify<T: Summary>(item1: &T, item2: &T) {
     *   
     */ 

    /*
     * SPECIFYING MULTIPLE TRAIT BOUNDS WITH THE + SYNTAX
     * 
     * 
     *   pub fn notify(item: &(impl Summary + Display)) {
     * 
     * The + syntax is also valid with trait bounds on generic types:
     * 
     *   pub fn notify<T: Summary + Display>(item: &T) {
     * 
     */ 

    /*
     * CLEARER TRAIT BOUNDS WITH WHERE CLAUSES
     * 
     * Using too many trait bounds has its downsides. Rust has alternate syntax
     * for specifying trait bounds inside a where clause after the function signature.
     *
     * So instead of writing this:
     *
     *   fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
     *
     * we can use where clause, like this:
     *   
     *   fn some_function<T, U>(t: &T, u: &U) -> i32
     *       where T: Display + Clone,
     *             U: Clone + Debug
     *   {      
     *
     */ 

    /*
     * RETURNING TYPES THAT IMPLEMENTS TRAITS
     *
     * fn returns_summarizable() -> impl Summary {
     *     Tweet {
     *         username: String::from("horse_ebooks"),
     *         content: String::from(
     *             "of course, as you probably already know, people",
     *         ),
     *         reply: false,
     *         retweet: false,
     *     }
     * }
     * You can only use impl Trait if you’re returning a single type. For example,
     * this code that returns either a NewsArticle or a Tweet with the return type
     * specified as impl Summary wouldn’t work:
     * 
     * This code does not compile!
     * 
     * fn returns_summarizable(switch: bool) -> impl Summary {
     *     if switch {
     *         NewsArticle {
     *             headline: String::from(
     *                 "Penguins win the Stanley Cup Championship!",
     *             ),
     *             location: String::from("Pittsburgh, PA, USA"),
     *             author: String::from("Iceburgh"),
     *             content: String::from(
     *                 "The Pittsburgh Penguins once again are the best \
     *                  hockey team in the NHL.",
     *             ),
     *         }
     *     } else {
     *         Tweet {
     *             username: String::from("horse_ebooks"),
     *             content: String::from(
     *                 "of course, as you probably already know, people",
     *             ),
     *             reply: false,
     *             retweet: false,
     *         }
     *     }
     * }
     * 
     * Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions
     * around how the impl Trait syntax is implemented in the compiler.
     * 
     */ 

    /* 
     * FIXING THE 'largest' FUNCTION WITH TRAIT BOUNDS 
     * 
     * Remember the problen we've previously when try to generalize largest function: 
     * 
     *     help: consider restricting type parameter `T`
     *      |
     *    1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
     *      |   
     * 
     * We can change the signature of the function to:
     * 
     *     fn largest<T: PartialOrd>(list: &[T]) -> T { 
     * 
     * The trait PartialOrd is included in the prelude, we don't need to import 
     * 
     * This time when we compile the code, we get a different set of errors:
     * 
     *     error[E0508]: cannot move out of type `[T]`, a non-copy slice 
     *     error[E0507]: cannot move out of a shared reference
     * 
     * When we made the largest function generic, it became possible for the list parameter to
     * have types in it that don’t implement the Copy trait. Consequently, we wouldn’t be able
     * to move the value out of list[0] and into the largest variable, resulting in this error.
     * 
     * To call this code with only those types that implement the Copy trait, we can add Copy
     * to the trait bounds of T!
     * 
     */ 

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        
        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    /* 
     * If we don’t want to restrict the largest function to the types that implement the Copy
     * trait, we could specify that T has the trait bound Clone instead of Copy. Then we could
     * clone each value in the slice when we want the largest function to have ownership.
     * 
     * Using the clone function means we’re potentially making more heap allocations in the
     * case of types that own heap data like String, and heap allocations can be slow if
     * we’re working with large amounts of data.
     * 
     */ 


    fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = list[0].clone();
        
        for &item in list.clone() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_clone(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_clone(&char_list);
    println!("The largest char is {}", result);


    /* 
     * Another way we could implement largest is for the function to return a reference
     * to a T value in the slice. If we change the return type to &T instead of T, thereby
     * changing the body of the function to return a reference, we wouldn’t need the Clone
     * or Copy trait bounds and we could avoid heap allocations.
     *  
     */ 

    fn largest_ref<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = list[0].clone();
        
        for &item in list.clone() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);


    /* 
     * USING TRAIT BOUNDS TO CONDITIONALLY IMPLEMENT METHODS
     * 
     * By using a trait bound with an impl block that uses generic type parameters, we
     * can implement methods conditionally for types that implement the specified traits.
     * 
     */ 
     
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else { 
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    /* 
     * We can also conditionally implement a trait for any type that implements another trait.
     * Implementations of a trait on any type that satisfies the trait bounds are called blanket
     * implementations and are extensively used in the Rust standard library. 
     *
     */

}
