fn main() {
	/*
	IP Address currently can be IPv4 and IPv6
	We can enumerate all its variants, so fits enum definition
	Any IP Address can be either a version four or a version six, but not both at same time
	Thats make enums appropiate for this problem
	*/
	{
		enum IpAddrKind {
			V4,
			V6,
		}
		
		// Instances

		/*
		Note that the variants of the enum are namespaced under its identifier, and we use a
		double colon to separate the two
		*/

		let four = IpAddrKind::V4;
		let six = IpAddrKind::V6;
		
		// We can now define a function that takes any IpAddrKind
		
		fn route(ip_kind: IpAddrKind) {}

		route(IpAddrKind::V4);

		route(IpAddrKind::V6);

	}
	// Enums can be fields of structs
	{
		enum IpAddrKind {
			V4,
			V6,
		}
		
		struct IpAddr {
			kind: IpAddrKind,
			address: String,
		}
		
		let home = IpAddr {
			kind: IpAddrKind::V4,
			address: String::from("127.0.0.1"),
		};
		
		let loopback = IpAddr {
			kind: IpAddrKind::V6,
			address: String::from("::1"),
		};

	}
	// Putting data on enum variants, simplifiying previous
	{
		enum IpAddr {
			V4(String),
			V6(String),
		}
		
		let home = IpAddr::V4(String::from("127.0.0.1"));

		let loopback = IpAddr::V6(String::from("::1"));
	}
	
	// Data put inside enum variant can be of different types for each variant
	{
		enum IpAddr {
			V4(u8, u8, u8, u8),
			V6(String),
		}
		
		let home = IpAddr::V4(127, 0, 0, 1);

		let loopback = IpAddr::V6(String::from("::1"));
	}
	// Variants can also use Stucts as variant value
	{
		struct Ipv4Addr {
			// -- snip --
		}

		struct Ipv6Addr {
			// -- snip --
		}
		
		enum IpAddr {
			V4(Ipv4Addr),
			V6(Ipv6Addr),
		}
	}
	// Another example: Message
	{
		enum Message {
			Quit, // No data associated
			Move { x: i32, y: i32 }, // Anonymout struct
			Write(String), // Include an associated String
			ChangeColor(i32, i32, i32), // Three i32 values
		}
	}
	// If we were using structs it was more dirty
	{
		struct QuitMessage; // Unit struct
		struct MoveMessage {
			x: i32,
			y: i32,
		}
		struct WriteMessage(String); // Tuple struct
		struct ChangeColorMessage(i32, i32, i32); // Tuple struct
	}
	// and also can't pass a generic Message as argument of a function

	// We can implement methods on an enum, like on structs
	{
		enum Message {
			Quit, // No data associated
			Move { x: i32, y: i32 }, // Anonymout struct
			Write(String), // Include an associated String
			ChangeColor(i32, i32, i32), // Three i32 values
		}
		
		impl Message {
			fn call(&self) {
				// Method body here
			}
		}
		
		let m = Message::Write(String::from("hello"));
		m.call();
	}

	/*
	Rust uses Option enum to encode null values
	
	enum Option<T> {
		Some(T),
		None,
	}

	You can use Some(T) or None withour Option:: prefix
	*/

	{
		let some_number = Some(5);
		let some_string = Some("a string");

		// If we use None rather than Some, we need to specify the type
		// because compiler can't infer it
		let absent_number : Option<i32> = None;
	}
		
	/*
	We can't combine T value with Some<T> value	
	The following code crashes

	let x: i8 = 5;
	let y: Option<i8> = Some(5);

	let sum = x + y;
	
	*/

	// MATCH EXPRESSION

	{
		enum Coin {
			Penny,
			Nickel,
			Dime,
			Quarter,
		}

		fn value_in_cents(coin: Coin) -> u8 {
			match coin {
				Coin::Penny => 1,
				Coin::Nickel => 5,
				Coin::Dime => 10,
				Coin::Quarter => 25,
			}
		}
	}

	// We can use multiples lines if we need, using {} 

	{
		enum Coin {
			Penny,
			Nickel,
			Dime,
			Quarter,
		}

		fn value_in_cents(coin: Coin) -> u8 {
			match coin {
				Coin::Penny => {
					println!("Lucky Penny");
					1
				},
				Coin::Nickel => 5,
				Coin::Dime => 10,
				Coin::Quarter => 25,
			}
		}
	}

	// Patterns that binds to value
	{
		#[derive(Debug)] // So we can inspect the state in a minute
		enum UsState {
			Alabama,
			Alaska,
			// --snip--
		}

		enum Coin {
			Penny,
			Nickel,
			Dime,
			Quarter(UsState),
		}

		fn value_in_cents(coin: Coin) -> u8 {
			match coin {
				Coin::Penny => 1,
				Coin::Nickel => 1,
				Coin::Dime => 1,
				Coin::Quarter(state) => {
					println!("State quarter from {:?}!", state);
					25
				},
			}
		}
	}

	// Matching with Option<T>

	{
		fn plus_one(x: Option<i32>) -> Option<i32> {
			match x {
				None => None,
				Some(i) => Some(i + 1),
			}
		}

		let five = Some(5);
		let six = plus_one(five);
		let none = plus_one(None);
	}
	
	// Matches are exhaustive
	// If we don't cover all posibile values, compiler throws an error

	/*
	This doesn't compiles

	fn plus_one(x: Option<i32>) -> Option<i32> {
		match x {
			Some(i) => Some(i + 1)
		}
	}

	*/

	// The placeholder

	// Use when we don't want to list all remaining possibilities

	{
		let some_value : u8 = 0u8;
		match some_value {
			1 => println!("one"),
			3 => println!("three"),
			5 => println!("five"),
			7 => println!("seven"),
			_ => ()
		}
	}
	
	// IF LET

	// We only worry about 1 pattern
	
	// Using match...
	{
		let some_u8_value = Some(0u8);
		match some_u8_value {
			Some(3) => println!("three"),
			_ => (),
		}
	}
	
	// if-let version
	{
		let some_u8_value = Some(0u8);
		if let Some(3) = some_u8_value {
			println!("three");
		}
	}

	// if-let else
	{
		#[derive(Debug)] // So we can inspect the state in a minute
		enum UsState {
			Alabama,
			Alaska,
			// --snip--
		}

		enum Coin {
			Penny,
			Nickel,
			Dime,
			Quarter(UsState),
		}
		
		let coin = Coin::Quarter(UsState::Alaska);

		// Match way
		let mut count = 0;
		match &coin {
        		Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        		_ => count += 1,
   		}
		
		// If-let way	
		let mut count = 0;
		if let Coin::Quarter(state) = coin {
			println!("State quarter from {:?}!", state);
	    	} else {
			count += 1;
	    	}
	}


}
