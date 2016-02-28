extern crate java_properties;
extern crate byteorder;
extern crate xml;
extern crate stemmer;
extern crate regex;
extern crate time;

pub mod includes;

use std::env;

use std::cell::RefCell;


use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fmt::Write;
use std::process::Stdio;
use std::process::ChildStdout;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::AsRawFd;

use java_properties::PropertiesIter;

use includes::context_types::ContextObject;

use std::process::Command;






fn main() {
	let config_filename = env::args().nth(1);
	let queries_list_filename = env::args().nth(2);
	let context = RefCell::new(ContextObject::new());


	println!("Loading config...");
	match config_filename{
		Some(filename)=> match File::open(&filename){
			Ok(config_file)=>{
				match PropertiesIter::new(BufReader::new(config_file)).read_into(|k, v| { context.borrow_mut().set_param(k, v); }){
					Ok(_)=>println!("Config loaded!"),
					Err(_)=> panic!("Eror while loading config!")
				}
			},
			Err(_)=> panic!("Error, could not open config file '{}'", filename)
		},
		None=> panic!("No config file!")
	}

	context.borrow_mut().complete_config();



	println!("Config: ");
	for (property, value) in context.borrow_mut().get_config(){
		println!("{}=>{}", property, value);
	}
	println!("");


	println!("Loading index...");
	context.borrow_mut().load_index();
	println!("");

	context.borrow().index_details(false);

	println!("Starting querying...");
	let mut result_filename = String::new();
	write!(&mut result_filename, "results/{}.res", includes::get_time_millis());
	match queries_list_filename{
		Some(queries_list_filename)=>context.borrow_mut().start_querying(&queries_list_filename, &result_filename),
		None=> panic!("No query file provided!")
	}
	unsafe{

		let mut awk_command = String::new();
		awk_command.push_str("BEGIN { first=true; MAX_RESULTS=");
		write!(awk_command, "{}", 10);
		awk_command.push_str(";}{ if(first || $1!=last_query){first=false; occurence=0; last_query=$1;} occurence += 1; if(occurence<MAX_RESULTS){$4=occurence; print $0;}}");

		let cmd = Command::new("sort").arg("-k1,1n").arg("-k5,5nr").arg(&result_filename)
		.stdout(Stdio::piped())
		.spawn().unwrap();



		let cmd2 = Command::new("awk")
		.arg(awk_command)
		.stdin(Stdio::from_raw_fd(cmd.stdout.unwrap().as_raw_fd()))
		.output()
		.unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
		println!("");

		println!("{}", String::from_utf8_lossy(&cmd2.stdout));
		print!("\n");

		println!("{}", String::from_utf8_lossy(&cmd2.stderr));
		print!("\n");
	}

}
