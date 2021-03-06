extern crate java_properties;
extern crate byteorder;
extern crate xml;
extern crate stemmer;
extern crate regex;
extern crate image;
extern crate hound;
extern crate simplemad;
extern crate subrip;

pub mod includes;

use std::env;
use std::cell::RefCell;


use std::io::BufReader;
use std::fs;
use std::fs::File;


use java_properties::PropertiesIter;


use includes::context_types::ContextObject;




fn main() {
	let config_filename = env::args().nth(1);
	let documents_list_filename = env::args().nth(2);
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

	println!("Indexing documents...");
	match documents_list_filename {
		Some(filename) => context.borrow_mut().start_indexing(&filename),
		None => panic!("No documents list provided!"),
	}
	println!("Indexing finished!");
	println!("");

	println!("Saving index to disk...");
	match context.borrow().get_param("index") {
		Some(index_path) => fs::create_dir_all(index_path).unwrap_or_else(|e| panic!(e)),
		None => panic!("Can't find parameter 'images_associations_filename' in config"),
	};

	context.borrow().save_index();
	println!("Index saved!");
	println!("");

	context.borrow().index_details(false);

}
