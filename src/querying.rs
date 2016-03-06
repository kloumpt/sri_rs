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
use std::fmt::Write;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::process::Stdio;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::AsRawFd;

use java_properties::PropertiesIter;

use includes::context_types::ContextObject;

use std::process::Command;


fn sort_results(result_filename: &str, result_limit: usize) {

	let mut awk_script = String::new();
	awk_script.push_str("BEGIN { first=true; MAX_RESULTS=");
	write!(awk_script, "{}", result_limit).unwrap();
	awk_script.push_str(";}{ if(first || $1!=last_query){first=false; occurence=0; last_query=$1;} occurence += 1; if(occurence<MAX_RESULTS){$4=occurence; print $0;}}");

	let sort_command = Command::new("sort")
		                   .env("LC_ALL", "C")
		                   .arg("-k1,1n")
		                   .arg("-k5,5nr")
		                   .arg(&result_filename)
		                   .stdout(Stdio::piped())
		                   .spawn()
		                   .unwrap();



	let awk_command = Command::new("awk")
		                  .arg(awk_script)
		                  .stdin(unsafe { Stdio::from_raw_fd(sort_command.stdout.unwrap().as_raw_fd()) })
		                  .output()
		                  .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

	match File::create(Path::new(result_filename)) {
		Ok(mut file) => {
			use std::io::Write;
			writeln!(file, "{}", &String::from_utf8_lossy(&awk_command.stdout)).unwrap()
		},
		Err(e) => panic!(e),
	}


}



fn main() {
	let config_filename = env::args().nth(1);
	let queries_list_filename = env::args().nth(2);
	let context = RefCell::new(ContextObject::new());


	println!("Loading config...");
	match config_filename {
		Some(filename) => {
			match File::open(&filename) {
				Ok(config_file) => {
					match PropertiesIter::new(BufReader::new(config_file)).read_into(|k, v| {
						context.borrow_mut().set_param(k, v);
					}) {
						Ok(_) => println!("Config loaded!"),
						Err(_) => panic!("Eror while loading config!"),
					}
				},
				Err(_) => panic!("Error, could not open config file '{}'", filename),
			}
		},
		None => panic!("No config file!"),
	}

	context.borrow_mut().complete_config();



	println!("Config: ");
	for (property, value) in context.borrow_mut().get_config() {
		println!("{}=>{}", property, value);
	}
	println!("");


	println!("Loading index...");
	context.borrow_mut().load_index();
	println!("");

	context.borrow().index_details(false);

	println!("Starting querying...");
	let result_folder = "results";
	fs::create_dir_all(result_folder).unwrap_or_else(|e| {
		panic!(e);
	});

	let mut result_filename = String::new();
	write!(&mut result_filename, "{}/{}.res", result_folder, includes::get_time_millis()).unwrap();
	match queries_list_filename {
		Some(queries_list_filename) => context.borrow_mut().start_querying(&queries_list_filename, &result_filename),
		None => panic!("No query file provided!"),
	}


	println!("{}", result_filename);
	sort_results(&result_filename, 10);
}
