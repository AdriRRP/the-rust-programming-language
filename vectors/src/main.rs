fn main() {
	// VECTORS

	// Vectors allow you to store more than one value in a single data structure that
	// puts all the values next to each other in memory.

	// Vectors can only store values of the same type.

	// They are useful when you have a list of items, such as the lines of text in a file
	// or the prices of items in a shopping cart.

	let v: Vec<i32> = Vec::new();

	// Rust doesn’t know what kind of elements we intend to store	

	// Vectors are implemented using generics: Vec<T>
	
	// Rust can often infer the type of value you want to store once you insert values
	// so you rarely need to do this type annotation. 

	// It’s more common to create a Vec<T> that has initial values, and Rust provides the
	// vec! macro for convenience.
	
	let v = vec![1, 2, 3];
	
	// Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t
	// necessary
	
	// Updating a Vector

	// If we want to be able to change its value, we need to make it mutable

	let mut v = Vec::new();

	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);


	// Dropping a Vector Drops Its Elements
	{
		let v = vec![1, 2, 3, 4];

		// do stuff with v
	}	// <- v goes out of scope and is freed here
	
	// When the vector gets dropped, all of its contents are also dropped, meaning those integers
	// it holds will be cleaned up.

	// This may seem like a straightforward point but can get a bit more complicated when you
	// start to introduce references to the elements of the vector.

	// Reading Elements of Vectors

	let v = vec![1, 2, 3, 4, 5];
	
	// There are two ways to reference a value stored in a vector

	let third: &i32 = &v[2];
	println!("The third element is {}", third);

	// We take the control over non exist index
	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element"),
	}
	
	/*
	The [] method to accesing items causes the program panic because program refers to inexistent index
	
	let does_not_exists = &v[100];
	
	This method is preferred if you want the program crash on an invalid input

	You can control this situation if you use the get method (That returns Option<i32>) and manage the
	None return value, that is more user friendly if an human introduces index value
	*/

	let does_not_exists = v.get(100); // Returns None value

	// When the program has a valid reference, the borrow checker enforces the ownership and borrowing
	// rules to ensure this reference and any other references to the contents of the vector remain valid

	// Recall the rule that states you can’t have mutable and immutable references in the same scope

	/*
	If we hold an immutable reference to the first element in a vector and try to add an element to the end,
	which won’t work if we also try to refer to that element later in the function

	let mut v = vec![1, 2, 3, 4, 5];

	let first = &v[0];
	
	v.push(6);

	println!("The first element is {}", first);
	
	Why should a reference to the first element care about what changes at the end of the vector?
	
	This error is due to the way vectors work: adding a new element onto the end of the vector might require
	allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all
	the elements next to each other where the vector currently is.

	In that case, the reference to the first element would be pointing to deallocated memory.
	The borrowing rules prevent programs from ending up in that situation.
	*/


	// Iterating over the Values in a Vector

	let v = vec![100, 32, 57];
	
	for i in &v {
		println!("{}", i);
	}

	// We can also iterate over mutable references to each element in a mutable vector in order to make
	// changes to all the elements

	let mut v = vec![100, 32, 57];

	for i in &mut v {
		*i += 50;
	}

	// To change the value that the mutable reference refers to, we have to use the dereference operator (*)
	// to get to the value in i before we can use the += operator. 


	// Using an Enum to Store Multiple Types

	enum SpreadSheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}
	
	let row = vec![
		SpreadSheetCell::Int(3),
		SpreadSheetCell::Text(String::from("blue")),
		SpreadSheetCell::Float(10.12),
	];
	

}
