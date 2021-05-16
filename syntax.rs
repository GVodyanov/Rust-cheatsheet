//RUST COMMANDS

	//install rust
		"curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh"

	//update rust
		"rustup update"

	//compile
		"rustc main.rs"

//CARGO COMMANDS

	//create new
		"rustc main.rs"

	//build
		"cargo build"

	//run
		"cargo run"

	//check (check it compiles but no executable)
		"cargo check"
		
//GENERAL SYNTAX

	//main function
		fn main() {

		}
	
	//include file
		//std = standard library, io = input / output
		use std::io;

	//variable declaration
		let x = 5;
	
	//mutable variable declaration
		//let = create variable, mut = mutable, name, 
		let mut variable_string = String::new();
	
	//changing unmutable variables by shadowing:
		let x = 5;

		let x = x + 1;
	
	//constant declaration
		//consts are always unmutable, have flexible scope, and can not be set to the result of a function
		//underscore just for readability
		const MAX_POINTS: u32 = 100_000;
	
	
	
//DATA TYPES
	//declaring
		let guess: u32 = 42;
	
	//integers
		8, 16, 32, 64, 128 //bit options
		//u in front for unsigned, i for signed
		//ie: i8, u64
	
		isize, usize //for arch (when u dont know cpu architecture)
	
		//best to use i32/u32, fastest on most systems
	
	//floats
		f64, f32
		f64 //is best
	
	//operators (pretty normal)
		+, -, *, /, % //(for remainder)
	
	//bool
		bool //is just bool

	//characters
		//its just 
		char
		//can also do emojis and funny symbols btw
		
	//tuples
		//declaration
		let tup: (i32, f64, u8) = (500, 6.4, 1);
		//destructuring
		let tup = (500, 6.4, 1);
		let (x, y, z) = tup;
		//referencing
		let one = x.2;

//FUNCTIONS
//print new line
println!("Hello, world!");
