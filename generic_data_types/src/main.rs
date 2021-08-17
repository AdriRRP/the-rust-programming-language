fn main() {
    /*
     * GENERIC DATA TYPES
     *
     * We can use generics to create definitions for items like function
     * signatures or structs, which we can then use with many different
     * concrete data types.
     *
     */


    /*
     * IN FUNCTION DEFINITIONS
     *
     * We place the generics in the signature of the function where we would
     * usually specify the data types of the parameters and return value.
     *
     */

    fn largest_i32 (list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char (list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    
    /*
     * Let's generalize the functions above with generics types.
     *
     * Yo parameterize the types in a new function we need to name the type
     * parameter. You can use any identifier as a type parameter name.
     *
     * But we'll use T because, by convention, parameter names in Rust are
     * short, often just a letter, and Rust's type-naming convention is
     * CamelCase. Short for "Type", T, is the default choice.
     *
     * When we use a parameter in the body of the function, we have to declare
     * the parameter name in the signature so the compiler knows what that name
     * means. Similarly, when we use a type parameter in a function signature,
     * we have to declare the type parameter name before we use it:
     *
     * fn largest<T>(list: &[T]) -> T { ... }
     *
     */

    /*
     * First attempt
     *
     * fn largest<T>(list: &[T]) -> T {
     *     let mut largest = list [0];
     *
     *     for &item in list {
     *         if item > largest {
     *             largest = item;
     *         }
     *     }
     *
     *     largest
     * }
     *
     * let number_list = vec![34, 50, 25, 100, 65];
     * 
     * let result = largest(&number_list);
     * println!("The largest number is {}", result);
     * 
     * let char_list = vec!['y', 'm', 'a', 'q'];
     * 
     * let result = largest(&char_list);
     * println!("The largest char is {}", result);
     *
     * 
     * We get the following error:
     *
     * error[E0369]: binary operation `>` cannot be applied to type `T`
     *
     * The note mentions std::cmp::PartialOrd, which is a trait. We’ll talk
     * about traits in the next section.
     *
     *
     * For now, this error states that the body of largest won't work for all
     * possible types that T could be. We can only use types whose values can
     * be ordered.
     *
     */


    /*
     * USE GENERICS IN STRUCT DEFINITION
     *
     */
    
     struct Point<T> {
         x: T,
         y: T,
     }
    
     let integer = Point { x: 5, y: 10 };
     let float = Point { x: 1.0, y: 4.0 };

    /*
     * It will be work if both x and y have the same type. For example, the
     * following code doesn't compiles:
     *
     * let wont_work = Point {x: 5, y: 4.0 };
     *
     * To define a Point struct where x and y are both generics but could have
     * different types, we can use multiple generic type parameters:
     *
     * struct Point <T, U> {
     *     x: T,
     *     y: U,
     * }
     *
     *
     * so we can now:
     *
     * let both_integer = Point { x: 5, y: 10 };
     * let both_float = Point { x: 1.0, y: 4.0 };
     * let integer_and_float = Point { x: 5, y: 4.0 };
     * 
     */

     /*
      * USE GENERICS IN ENUM DEFINITION
      *
      * We have already seen:
      *
      * enum Option<T> {
      *     Some(T),
      *     None,
      * }
      *
      * or
      *
      * enum Result<T, E> {
      *     Ok(T),
      *     Err(E),
      * }
      *
      */

     /*
      * USE GENERICS IN METHOD DEFINITIONS
      *
      */

     impl<T> Point<T> {
         fn x(&self) -> &T {
             &self.x
         }
     }

     let p = Point { x: 5, y: 10 };

     println!("p.x = {}", p.x());

     /*
      * Note that we have to declare T just after impl so we can use it to
      * specify what we're implementing methods on the type Point<T>. By
      * declaring T as a generiuc type after impl, Rust can identify that the
      * type in the angle brackets in Point is a generic type rather than a
      * concrete type.
      *
      * We could, for example, implement methods only on Point<f32> instances
      * rather than on Point<T> instances with any generic type:
      *
      */

     impl Point<f32> {
         fn distance_from_origin(&self) -> f32 {
             (self.x.powi(2) + self.y.powi(2)).sqrt()
         }
     }

     /*
      * Generic type parameters in a struct definition aren’t always the same
      * as those you use in that struct’s method signatures. We defines the
      * method mixup on the Point<T, U>. The method takes another Point as a
      * parameter, which might have different types from the self Point we’re
      * calling mixup on. The method creates a new Point instance with the x
      * value from the self Point (of type T) and the y value from the
      * passed-in Point (of type W).
      *
      */
     struct Point2<T, U> {
         x: T,
         y: U,
     }
     
     impl<T, U> Point2<T, U> {
         fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
             Point2 {
                 x: self.x,
                 y: other.y,
             }
         }
     }
     
     let p1 = Point2 { x: 5, y: 10.4 };
     let p2 = Point2 { x: "Hello", y: 'c' };
     
     let p3 = p1.mixup(p2);
     
     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

     /*
      * Here we can see a situation in which some generic parameters are
      * declared with impl and some are declared with the method definition.
      *
      * Here, the generic parameters T and U are declared after imple, because
      * the go with the struct definition. The generic parameters V and W are
      * declared after fn mixup, because they're only relevant to the method.
      *
      */
     

     /*
      * PERFORMANCE OF CODE USING GENERICS
      *
      * You might be wondering whether there is a runtime cost when you’re
      * using generic type parameters. The good news is that Rust implements
      * generics in such a way that your code doesn’t run any slower using
      * generic types than it would with concrete types.
      *
      * Rust accomplishes this by performing monomorphization of the code that
      * is using generics at compile time. Monomorphization is the process of
      * turning generic code into specific code by filling in the concrete
      * types that are used when compiled.
      *
      * Let's look at how works Option<T> enum:
      *
      * let integer = Some(5);
      * let float = Some(5.0);
      *
      * When Rust compiles this code, it performs monomorphization. During that
      * process, the compiler reads the values that have been used in Option<T>
      * instances and identifies two kinds of Option<T>: one is i32 and the
      * other is f64. As such, it expands the generic definition of Option<T>
      * into Option_i32 and Option_f64, thereby replacing the generic
      * definition with the specific ones.
      *
      * enum Option_i32 {
      *     Some(i32),
      *     None,
      * }
      * 
      * enum Option_f64 {
      *     Some(f64),
      *     None,
      * }
      * 
      * fn main() {
      *     let integer = Option_i32::Some(5);
      *     let float = Option_f64::Some(5.0);
      * }
      */

     


}
