use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Initializing...");
	let mut filename_path:String = String::from("");
	let mut search_name:String = String::from("");
	let line_number: usize;
	let found_name: String;
	
	(filename_path, search_name) = get_environment_variables();
	
	if filename_path != "" && search_name != "" {
		println!("Filename is {}", filename_path);
		println!("Search name is {}", search_name);
		
		println!("Opening file...");
		
		// Open the file in read-only mode (ignoring errors).
		let file = File::open(filename_path).unwrap();
		let reader = BufReader::new(file);
		
		println!("Searching for {}....", search_name);
		(line_number, found_name) = search_file(reader, search_name);
		
		println!("Shutting down...");
	}		
}

//Searches for a name in the file
fn search_file(reader:BufReader<File>, search_name:String) -> (usize, String)
{
	// Read the file line by line using the lines() iterator from std::io::BufRead.
	for (index, line) in reader.lines().enumerate() {
		let line = line.unwrap(); // Ignore errors.
		
		if line == search_name {
			// Show the line and its number.
			println!("You found them: {}: {}", index + 1, line);
			return (index + 1, line);
		}
	}
	
	(usize::MAX, "".to_string())
}

//Validates and extracts the erguments passed to the program and returns them
fn get_environment_variables() -> (String, String)
{
	let mut filename_path:String = String::from("");
	let mut search_name:String = String::from("");
	
	
	if env::args().len() < 2 {
		print_argument_error();
		//println!("Argument length is {}", env::args().len());
	} else {
		for (index, argument) in env::args().enumerate() {
			if argument == "--filename" {
				filename_path = env::args().nth(index + 1).unwrap();
			} else if argument == "--search_name" {
				search_name = env::args().nth(index + 1).unwrap();
			}
			//println!("argument: {} is {}", index, argument);
		}
	}
	
	(filename_path, search_name)
}

//Prints usage errors to the screen
fn print_argument_error()
{
	println!("Wrong arguments supplied");
	println!("Required arguments:");
	println!("\t--filename <path.txt> \tPath to the list of names (including the file name)");
	println!("\t--search_name <Name> \tThe name to search for");
}
