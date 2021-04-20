fn main() {
    {
        // first approach
        fn first_word(s: &String) -> usize {
            // Because we need to go through the String element by element and check whether
            // a value is a space, we'll convert our String to an array of bytes
            let bytes = s.as_bytes();

            // Create a iterator over the array of bytes
            // enumerate wraps the result of iter and return each element as a part of a
            // tuple where 1st element is the index and the 2nd is a reference to element
            // We can use patterns to destructure the tuple (like everywhere in Rust)
            for (i, &item) in bytes.iter().enumerate() {
                // If we find a space, we return the position
                if item == b' ' {
                    return i;
                }
            }

            // Otherwise we return the length of the String
            s.len()
        }
        
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""

        // word still has the value 5 here, but there's no more string that
        // we colud maningfully use the value 5 with. word is now totally invalid!

        // With a second_word function, we need starting and ending index, and
        // we have even more values that were calculated fron data in a particular
        // state but aren't tied to that state at all.
        
        // fn second_word(s: &String) -> (usize, usize) { ...
    }
    {
        // A string slice is a reference to a part of a String, and it looks like this:

        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        
        // Slices are references to a portion of the String
        // We can create slices using a range within brackets by specifying
        // [starting_index..ending_index] (ending index is one more than the last position
        // in the slice.
        // starting_index can be ommited if it is 0
        // ending_indez can be ommited if it is the length of the string

        // IMPORTANT: String slice range indices must occur at valid UTF-8 character
        // boundaries. If you attempr to create a string slice in the middle of a multibyte
        // character, your program will exit with an error.

        // first_word rewritten
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }

            &s[..]
        }

        // Now when we call first_word, we get back a single value that is tied to the
        // underlying data.

        // Returning a slice wold also work for a second_word function

        // fn second_word(s: &String) -> &str {

        // We now have a strightforward API that's much harder to mess up, because the 
        // compiler will wnsure the references into the String remain valid
    }

    {
        // IMPORTANT: String literals are slices
        let s = "Hello, world!"; // The type of s is &str: a slice pointing to that specific
        // point of the binary. This is also why string literals are inmutable; &str is an 
        // inmutable reference

        // Knowing that you can take slices of literals and String values leads us to one more
        // improvement on first_word: change the signature
        
         fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }

            &s[..]
        }

       
        // If we have a string slice, we can pass that directly. If we have s String, we can 
        // pass a slice of the entire String. This makes our API more general
         let my_string = String::from("hello world");
         
         // first_word works on slices of `String`s
         let word = first_word(&my_string[..]);

         let my_string_literal = "hello world";

         // first_word works on slices of string literals
         let word = first_word(&my_string_literal[..]);

         // Because string literals *are* string slices already,
         // this works too, without the slice syntax!
         let word = first_word(my_string_literal);

    }

    // OTHER SLICES

    let a = [1, 2, 3, 4, 5];

    // Just as we might want to refer ro a part of a string, we might want to refere to
    // part of an array

    let slice = &a[1..3];

    // This slice has the type &[i32]. It works for some other types.


}
