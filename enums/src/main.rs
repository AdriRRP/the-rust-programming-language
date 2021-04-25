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
}
