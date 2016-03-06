pub mod image_querying;
pub mod sound_querying;
pub mod text_querying;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;

use includes::image_types::ImageDescriptor;
use includes::sound_types::SoundDescriptor;
use includes::text_types::TextDescriptor;

use includes::context_types::ContextObject;

impl ContextObject {
	pub fn start_querying(&mut self, queries_list_filename: &str, result_filename: &str) {


		let queries_reader = match File::open(&queries_list_filename) {
			Ok(query_file) => BufReader::new(query_file),
			Err(_) => panic!("Error, could not open config file '{}'", queries_list_filename),
		};

		let mut result_writer = match File::create(&result_filename) {
			Ok(result_file) => result_file,
			Err(_) => panic!("Error, could not open result_file file '{}'", result_filename),
		};

		for line in queries_reader.lines() {
			match line {
				Ok(query) => {
					match query.trim() {
						"" => (),
						_ => {

							let mut fields = query.split(":");
							let query_id = fields.next().unwrap();
							let query_text = fields.next().unwrap();
							let query_type = match fields.next() {
								Some(query_type) => query_type.to_lowercase(),
								None => String::from("example"),
							};

							match query_type.as_ref() {
								"example" => {
									match query_text.rsplit('.').next().unwrap() {
										"txt" | "TXT" => {
											// Image file
											println!("Searching a picture similar to {}", query_text);

											let descriptor = match File::open(query_text.clone()) {
												Ok(image_file) => {
													match ImageDescriptor::from_image_file(self, image_file) {
														Ok(descriptor) => Some(descriptor),
														Err(e) => {
															println!("{:?}", e);
															None
														},
													}
												},
												Err(e) => {
													println!("{} {}", e, query);
													None
												},
											};


											match descriptor {
												Some(descriptor) => {
													for descriptor_from_index in self.get_images_base() {
														writeln!(result_writer, "{} 0 {} 0 {} sri-pfr", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();

													}
												},
												None => println!("Error: can't create a descriptor for the given file ({})", query_text),
											}
										},
										"bin" | "BIN" => {
											// Sound file
											println!("Searching a sound similar to {}", query_text);

											let descriptor = match File::open(query_text.clone()) {
												Ok(image_file) => {
													match SoundDescriptor::from_sound_file(self, image_file) {
														Ok(descriptor) => Some(descriptor),
														Err(e) => {
															println!("{:?}", e);
															None
														},
													}
												},
												Err(e) => {
													println!("{} {}", e, query);
													None
												},
											};

											match descriptor {
												Some(descriptor) => {
													for descriptor_from_index in self.get_sounds_base() {
														writeln!(result_writer, "{} 0 {} 0 {} sri-pfr", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();
													}
												},
												None => println!("Error: can't create a descriptor for the given file ({})", query_text),
											}
										},
										"xml" | "XML" => {
											// Text file
											println!("Searching a text similar to {}", query_text);

											let descriptor = match File::open(query_text.clone()) {
												Ok(image_file) => {
													match TextDescriptor::from_text_file(self, image_file) {
														Ok(descriptor) => Some(descriptor),
														Err(e) => {
															println!("{:?}", e);
															None
														},
													}
												},
												Err(e) => {
													println!("{} {}", e, query);
													None
												},
											};

											match descriptor {
												Some(descriptor) => {
													for descriptor_from_index in self.get_texts_base() {
														writeln!(result_writer, "{} 0 {} 0 {} sri-pfr", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();

													}
												},
												None => println!("Error: can't create a descriptor for the given file ({})", query_text),
											}
										},
										_ => println!("Error: invalid file extension for query {}", query_id),
									}
								},
								"criterion" => {
									let media_type = fields.next().unwrap();
									match media_type {
										"img" => {
											// Image file
											println!("Searching an image using the following color : {}", query_text);

											let descriptor = match ImageDescriptor::from_criterions(self, query_text) {
												Ok(descriptor) => Some(descriptor),
												Err(e) => {
													println!("{:?}", e);
													None
												},
											};

											match descriptor {
												Some(descriptor) => {
													for descriptor_from_index in self.get_images_base() {
														writeln!(result_writer, "{} 0 {} 0 {} sri-pfr", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();
													}
												},
												None => println!("Error: can't create a descriptor for the given criterions ({})", query_text),
											}


										},
										"txt" => {
											// Text file
											println!("Searching an image containing the following words : {}", query_text);


											let descriptor = match TextDescriptor::from_criterions(self, query_text) {
												Ok(descriptor) => Some(descriptor),
												Err(e) => {
													println!("{:?}", e);
													None
												},
											};

											match descriptor {
												Some(descriptor) => {
													for descriptor_from_index in self.get_texts_base() {
														writeln!(result_writer, "{} 0 {} 0 {} sri-pfr", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();

													}
												},
												None => println!("Error: can't create a descriptor for the given criterions ({})", query_text),
											}
										},
										_ => println!("Error, invalid media type : {} for query {}", media_type, query_id),
									}
								},
								_ => println!("Error, invalid query type : {} for query {}", query_type, query_id),
							}
						},
					}
				},
				Err(e) => panic!(e),
			}
		}
	}
}
