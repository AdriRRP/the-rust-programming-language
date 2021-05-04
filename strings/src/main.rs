fn main() {
	// Creating a new empty String
	let mut s = String::new();

	// Creating String from string slices
	let s = "initial content";
	
	let s = s.to_string();

	// We can call to_string() method on literals
	let s = "initial content".to_string();
	
	// We can also use the String's from function
	let s = String::from("initial content");
	
	// Strings can store all types of UTF-8 characters
	let hello = String::from("السلام عليكم");
	let hello = String::from("Dobrý den");
	let hello = String::from("Hello");
	let hello = String::from("שָׁלוֹם");
	let hello = String::from("नमस्ते");
	let hello = String::from("こんにちは");
	let hello = String::from("안녕하세요");
	let hello = String::from("你好");
	let hello = String::from("Olá");
	let hello = String::from("Здравствуйте");
	let hello = String::from("Hola");
	
	// Updating Strings
	
	// Appending to a String with push_str and push
	let mut s = String::from("foo");
	
	s.push_str("bar");
	
	// push_str takes string slice because we don’t necessarily want to take ownership of the parameter
	
	// It would be unfortunate if we weren’t able to use s2 after appending its contents to s1

	let mut s1 = String::from("foo");

	let s2 = "bar";

	s1.push_str(s2);

	println!("s2 is {}", s2);
	
	// push method allow to push a single character to String
	
	let mut s = String::from("lo");

	s.push('l');
	
	// Concatenation with the + Operator or the format! Macro

	let s1 = String::from("Hello, ");

	let s2 = String::from("world!");

	let s3 = s1 + &s2;
	
	// The reason s1 is no longer valid after the addition and the reason we used a reference to s2 has to do
	// with the signature of the method that gets called when we use the + operator 

	// println!("{}", s1); // will fail
	
	// The + operator uses the add method, whose signature looks something like this
	
	// fn add(self, s: &str) -> String {
	
	// The type of s1 is String and the type of s2 is &str, why it works?

	// Compiler can coerce the &String argument into a &str

	// When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
	
	// Because add does not take ownership of the s parameter, s2 will still be a valid String after this
	// operation
	
	// Signature that add takes ownership of self, because self does not have an &
	
	// s1 will be moved into the add call and no longer be valid after that
	
	// If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");
	
	let s = s1 + "-" +  &s2 + "-" +  &s3;
	
	// With all of the + and " characters, it’s difficult to see what’s going on

	// For more complicated string combining, we can use the format! macro
	
	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");
	
	let s = format!("{}-{}-{}", s1, s2, s3);
	
	// The format! macro works in the same way as println!, but instead of printing the output to
	// the screen, it returns a String with the contents

	// The version of the code using format! is much easier to read and doesn’t take ownership of any
	// of its parameters
	

	// Indexing into Strings
	
	/*
	In many other programming languages, accessing individual characters in a string by referencing
	them by index is a valid and common operation
	
	if you try to access parts of a String using indexing syntax in Rust, you’ll get an error

	let s1 = String::from("hello");
   	let h = s1[0];
	
	The error and the note tell the story: Rust strings don’t support indexing

	But why not? To answer that question, we need to discuss how Rust stores strings in memory
	*/

	// Internal Representation

	// A String is a wrapper over a Vec<u8>
	
	let hello = String::from("Hola");

	// In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long
	// Each of these letters takes 1 byte when encoded in UTF-8
	// But what about the following line? 

	let hello = String::from("Здравствуйте");

	// Asked how long the string is, you might say 12
	// However, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8
	// because each Unicode scalar value in that string takes 2 bytes of storage

	// Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value

	/*
	To demonstrate, consider this invalid Rust code:


	let hello = "Здравствуйте";
	let answer = &hello[0];

	What should the value of answer be? Should it be З, the first letter? 

	When encoded in UTF-8, the first byte of З is 208 and the second is 151
	so answer should in fact be 208, but 208 is not a valid character on its own

	Returning 208 is likely not what a user would want if they asked for the first letter of this string
	however, that’s the only data that Rust has at byte index 0

	Users generally don’t want the byte value returned, even if the string contains only Latin letters:

	if &"hello"[0] were valid code that returned the byte value, it would return 104, not h

	To avoid returning an unexpected value and causing bugs that might not be discovered immediately
	Rust doesn’t compile this code at all and prevents misunderstandings early in the development process
	*/
	
	// Bytes and Scalar Values and Grapheme Clusters! Oh My!
	
	/*
	Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s
	perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters)

	Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks likes
	this:
	
	[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
	224, 165, 135]
	
	That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values,
	which are what Rust’s char type is, those bytes look like this:
	
	['न', 'म', 'स', '्', 'त', 'े']

	There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make
	sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four
	letters that make up the Hindi word:
	
	["न", "म", "स्", "ते"]

	Rust provides different ways of interpreting the raw string data that computers store so that each program can
	choose the interpretation it needs, no matter what human language the data is in.
	
	A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are
	expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String,
	because Rust would have to walk through the contents from the beginning to the index to determine how many valid
	characters there were.
	*/
	
	// Slicing Strings

	// Rust asks you to be more specific if you really need to use indices to create string slices.
	// To be more specific in your indexing and indicate that you want a string slice, rather than indexing using []
	// with a single number, you can use [] with a range to create a string slice containing particular bytes:

	let hello = "Здравствуйте";

	let s = &hello[0..4];

	// Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that each of these
	// characters was 2 bytes, which means s will be Зд.


	// What would happen if we used &hello[0..1]? The answer: Rust would panic at runtime in the same way as if an
	// invalid index were accessed in a vector:
	
	// thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
	
	// You should use ranges to create string slices with caution, because doing so can crash your program.


	let s = &hello[0..2]; // This works
	// let s = &hello[0..1]; // This compiles but panicked after
	
	// Methods for Iterating Over Strings

	// If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the
	// chars method
	
	for c in "नमस्ते".chars() {
	    println!("{}", c);
	}

	// The bytes method returns each raw byte, which might be appropriate for your domain:

	for b in "नमस्ते".bytes() {
	    println!("{}", b);
	}
	
	// Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library.
	// Crates are available on crates.io if this is the functionality you need.


	
}
