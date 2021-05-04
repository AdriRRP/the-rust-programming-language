fn main() {
	// Storing Keys with Associated Values in Hash Maps

	// The type HashMap<K, V> stores a mapping of keys of type K to values of type V

	// It does this via a hashing function, which determines how it places these keys and values into memory

	// Hash maps are useful when you want to look up data not by using an index but by using a key that can
	// be of any type
	
	
	// Creating a New Hash Map

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	// Of our three common collections, this one is the least often used, so it’s not included in the features
	// brought into scope automatically in the prelude.

	// hash maps store their data on the heap
	
	// This HashMap has keys of type String and values of type i32
	// Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values
	// must have the same type.

	// Constructing a hash map is by using iterators and the collect method on a vector of tuples

	let teams = vec![String::from("Yellow"), String::from("Blue")];	
	let initial_score = vec![50, 10];

	let mut score: HashMap<_, _> =
		teams.into_iter().zip(initial_score.into_iter()).collect();
	
	// The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data
	// structures and Rust doesn’t know which you want unless you specify
	
	// For the parameters for the key and value types, however, we use underscores, and Rust can infer the types
	// that the hash map contains based on the types of the data in the vectors


	// Hash Maps and Ownership

	// For types that implement the Copy trait, like i32, the values are copied into the hash map
	// For owned values like String, the values will be moved and the hash map will be the owner of those values


	let field_name = String::from("Fauvorite Colour");
	let field_value = String::from("Blue");

	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	
	// field_name and field_value are invalid at this point


	// Accessing Values in a Hash Map
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let team_name = String::from("Blue");
	let score = scores.get(&team_name);
	
	// Score will have the value that’s associated with the Blue team, and the result will be Some(&10)
	// if there’s no value for that key in the hash map, get will return None

	// We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors

	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	// Updating a Hash Map

	// Overwriting a Value

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);

	println!("{:?}", scores);
	
	// Only Inserting a Value If the Key Has No Value
	
	// Hash maps have a special API for this called entry that takes the key you want to check as a
	// parameter

	// The return value of the entry method is an enum called Entry that represents a value that might or
	// might not exist
	
	scores.insert(String::from("Blue"), 10);

	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

	println!("{:?}", scores);
	
	// The or_insert method on Entry is defined to return a mutable reference to the value for the
	// corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for
	// this key and returns a mutable reference to the new value

	// This technique is much cleaner than writing the logic ourselves and, in addition, plays more
	// nicely with the borrow checker.

	// Updating a Value Based on the Old Value

	// Another common use case for hash maps is to look up a key’s value and then update it based on the
	// old value.
	
	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {

		// The or_insert method actually returns a mutable reference (&mut V) to the value for this key
		let count = map.entry(word).or_insert(0);

		// In order to assign to that value, we must first dereference count using the asterisk (*)
		*count += 1;

	}
	
	/*
	Hashing Functions

	By default, HashMap uses a “cryptographically strong” hashing function that can provide resistance to Denial
	of Service (DoS) attacks.

	This is not the fastest hashing algorithm available, but the trade-off for better security that comes with
	the drop in performance is worth it.

	If you profile your code and find that the default hash function is too slow for your purposes, you can switch
	to another function by specifying a different hasher.

	A hasher is a type that implements the BuildHasher trait.

	*/

	println!("{:?}", map);
}
