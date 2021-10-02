//RUST COMMANDS

	//install rust
		"curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh"

	//update rust
		"rustup update"

	//compile
		"rustc main.rs"

//CARGO COMMANDS

	//create new
		"cargo new hello_cargo"

	//build
		"cargo build"

	//run
		"cargo run"

	//check (check it compiles but no executable)
		"cargo check"
		l
//GENERAL SYNTAX

	//main function
		fn main() {

		}
	
	//include file
		//std = standard library, io = input / output
		use std::io;
		let mut variable_string 
	//variable declaration
		let x = 5;
		let main d
	//mutable variable declaration
		//let = create variable, mut = mutable, name, 
		let mut variable_string = String::new();
		let mut var
		//changing unmutable variables by shadowing:
		let x = 5;

		let x = x + 1;

		let mut variable_string 
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
		//its just char
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

	//array
		//unlike tuple all elemnts have to be the same data type
		//fixed size, vector for not fixed
		//declaration
		let mut a: [i32; 5] = [1, 2, 3, 4, 5];
		//modifying
		let a = [3; 5];
		//referencing 
		let first = a[0];
		
//FUNCTIONS
	//declaration
		fn another_function(x: i32, y: i32) {
			println!("another function."); 
		}
	//with return values
	//just a value with no semicolon or anything, could be a variable
		fn return_function() -> i32 {
			5
		}
	//print new line
		println!("Hello, world!");
		
//CONTROL FLOW
	//if expression
		if number < 5 {
    		println!("smaller than 5");
		} if number > 5 { //could also use else if
			println!("larger than 5");
    	} else {
    		println!("equal to 5");
		}
		//1 does not work after the if, only bool
		//we can also do all sorts of funky things
		let number = if condition { 5 } else { 6 };

		//however data types must be matched, the following does not work
    	let number = if condition { 5 } else { "six" };

    //loop
    	loop {
    		println!("again!");
    	}

  		//here is how to also break out of a loop
		 let result = loop {
    		 counter += 1;

    		 if counter == 10 {
        		break counter * 2;
    		 }
		  };

		  //here are while loops
		  while number != 0 {
    		  println! ("{}", number);
    		  number -=1;
		  }

		  //here are for loops with an Iterator
		  let a = [10, 20, 30, 40, 50];

		  for element in a.iter() {
          	println!("the value is: {}", element);
		  }

		  //here are loops for a specified number, the = is to include the 4
		  for number in (1..=4).rev() {
    		  //:)
		  }

//OWNERSHIP
	//ik i said this would only be for syntax but this is useful stuff i promise
	//stack
		//basically a list in RAM
		//data is added and removed in order
		//all data has a known fixed size
		
	//heap
		//about as much organization as my home directory :)
		//unknown data type and data size at compile time
		//when requesting RAM on the heap, the allocator just finds somewhere that has space
		//allocator gives u a pointer to be able to access ur data

	//using the stack is obviously much more efficient

	//ownership rules
		//each value has a var that is its owner
		//there can only be 1 owner at a time
		//when the owner goes out of scope, value gets dropped

	//scope is generally determined by {}

	//complex data types
		//strings
		let s = String::from("hello");
		//this, unline string literals, can be mutated (with a mut in front natutally)
		//String::from is a request for memory

	//moving memory
		//memory on the stack is copied in this situation:
		let x = 5;
		let y = x;
		//whereas string data as in the previous example is directed to the same place
		let s1 = s;
		//in the ram, meaning the string is only stored once
		//the POINTER moved, this also means that u cannot use the og var => s is no longer
		//valid
		
	//cloning data
		let s1 = String::from("hello");
		let s2 = s1.clone();
		//the following data types copy the data by default:
		//all ints, bools, floats, chars, tuples (if all contents compatible)
		
	//references
		//basically when u want to have the value of a variable but do not want to 
		//take ownership; if u have a string s1 u can do this
		let len = calculate_length(&s1);
		println!("The length of '{}' is {}.", s1, len);
		//with this being calculate_length; remember to declare the references in the parameters
		fn calculate_length(s: &String) -> usize {
			s.len()
		}
		//calculate_length is NOT the owner of s1 in this operation
		//ATTENTION u cannot modify a borrowed value, unless specifically defined
		//here is how u mut one, using the same example
		let len = calculate_length(&mut s1);
		println!("The length of '{}' is {}.", s1, len);
		//function
		fn calculate_length(s: &mut String) -> usize {
			s.len()
		}
		//ATTENTION there can only be ONE mutable reference, as references cannot sync
		//in a given scope, same for references in general, or all of ur references are
		//mutable, or all are immutable
		//the following however works, because r1 and r2 are no longer being used
		let mut s = String::from("hello");
		
		let r1 = &s; // no problem
		let r2 = &s; // no problem
		println!("{} and {}", r1, r2);
		// r1 and r2 are no longer used after this point

		let r3 = &mut s; // no problem
		println!("{}", r3);

//SLICES 🍕
	//slices, like references do not have ownership, they simply point to 
	//a part in a sequence of elements
	//here is a string slice
		let s = String::from("hello world");
		let hello = &s[..5];
		let world = &s[6..];
		//here is an example of them in action
		fn first_word(s: &String) -> &str {
			let bytes = s.as_bytes();

			for (i, &item) in bytes.iter().enumerate() {
				if item == b' ' {
					return &s[0..i];
				}
			}

			&s[..]
		}

//STRUCTS
	//another data type, similar to data in an object
	//different parts of a struct can be different data types, they are called fields
		struct User {
			username: String,
			email: String,
			sign_in_count: u64,
			active: bool,
		}
		
	//to then use the struct we create an instance of it, just like objects
	//note that fields can be accessed in any order
		let user1 = User {
			email: String::from("someone@example.com"),
			username: String::from("someusername123"),
			active: true,
			sign_in_count: 1,
		};
		
	//here is how to make one mutable, whole thing has to be mutable
		let mut user1 = User { --- - - -
		//here is how to actually change a value, to reference a field dot notation is used
		user1.email = String::from("anotheremail@example.com");

	//shorthand to attribute parameters to fields
		fn build_user(email: String, username: String) -> User {
			User {
				email,
				username,
				active: true,
				sign_in_count: 1,
			}
		}
		//instead of
		fn build_user(email: String, username: String) -> User {
			User {
				email: email,
				username: username,
				active: true,
				sign_in_count: 1,
			}
		}
	
	//updating structs (by making a new version)
		//basically there is no problem in taking 
		    let user2 = User {
				email: String::from("another@example.com"),
				username: String::from("anotherusername567"),
				active: user1.active,
				sign_in_count: user1.sign_in_count,
			};
