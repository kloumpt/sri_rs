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
use includes::Descriptor;

use includes::context_types::ContextObject;

impl ContextObject {
	pub fn start_querying(&mut self, queries_list_filename: &str, result_filename: &str) {


		let queries_reader = match File::open(&queries_list_filename) {
			Ok(query_file) => BufReader::new(query_file),
			Err(_) => panic!("Error, could not open query file '{}'", queries_list_filename),
		};

		let mut result_writer = match File::create(&result_filename) {
			Ok(result_file) => result_file,
			Err(_) => panic!("Error, could not create result_file file '{}'", result_filename),
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

							let descriptor = match query_type.as_ref() {
								"example" => {
									match query_text.rsplit('.').next().unwrap().to_string().to_lowercase().as_ref() {
										"txt" => {
											match File::open(query_text.clone()) {
												Ok(image_file) => {
													match ImageDescriptor::from_plain_text_file(self, image_file) {
														Ok(descriptor) => Some(Descriptor::ImageDescriptor(descriptor)),
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
											}
										},
										"png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "tiff" | "webp" => {
											match ImageDescriptor::from_image_file(self, query_text.as_ref()) {
												Ok(descriptor) => Some(Descriptor::ImageDescriptor(descriptor)),
												Err(e) => {
													println!("{:?}", e);
													None
												},
											}
										},
										"bin" => {
											match File::open(query_text.clone()) {
												Ok(image_file) => {
													match SoundDescriptor::from_raw_file(self, image_file) {
														Ok(descriptor) => Some(Descriptor::SoundDescriptor(descriptor)),
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
											}
										},
										"wav" => {
											match File::open(query_text.clone()) {
												Ok(image_file) => {
													match SoundDescriptor::from_wav_file(self, image_file) {
														Ok(descriptor) => Some(Descriptor::SoundDescriptor(descriptor)),
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
											}
										},
										"mp3" | "MPEG3" => {
											match File::open(query_text.clone()) {
												Ok(image_file) => {
													match SoundDescriptor::from_raw_file(self, image_file) {
														Ok(descriptor) => Some(Descriptor::SoundDescriptor(descriptor)),
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
											}
										},
										"xml" => {
											match File::open(query_text.clone()) {
												Ok(image_file) => {
													match TextDescriptor::from_text_file(self, image_file) {
														Ok(descriptor) => Some(Descriptor::TextDescriptor(descriptor)),
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
											}
										},
										"srt" => {
											match File::open(query_text.clone()) {
												Ok(image_file) => {
													match TextDescriptor::from_subrip_file(self, image_file) {
														Ok(descriptor) => Some(Descriptor::TextDescriptor(descriptor)),
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
											}
										},
										_ => {
											println!("Error: invalid file extension for query {}", query_id);
											None
										},

									}
								},
								"criterion" => {
									let media_type = fields.next().unwrap();
									match media_type {
										"img" => {
											match ImageDescriptor::from_criteria(self, query_text) {
												Ok(descriptor) => Some(Descriptor::ImageDescriptor(descriptor)),
												Err(e) => {
													println!("{:?}", e);
													None
												},
											}
										},
										"txt" => {
											match TextDescriptor::from_criteria(self, query_text) {
												Ok(descriptor) => Some(Descriptor::TextDescriptor(descriptor)),
												Err(e) => {
													println!("{:?}", e);
													None
												},
											}
										},
										_ => {
											println!("Error, invalid media type : {} for query {}", media_type, query_id);
											None
										},
									}
								},
								_ => {
									println!("Error, invalid query type : {} for query {}", query_type, query_id);
									None
								},
							};


							match descriptor {
								Some(descriptor) => {
									match descriptor {
										Descriptor::ImageDescriptor(descriptor) => {
											println!("Searching a picture similar to {}", query_text);
											for descriptor_from_index in self.get_images_base() {
												writeln!(result_writer, "{} 0 {} 0 {} sri_rs", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();

											}
										},
										Descriptor::SoundDescriptor(descriptor) => {
											println!("Searching a sound similar to {}", query_text);
											for descriptor_from_index in self.get_sounds_base() {
												writeln!(result_writer, "{} 0 {} 0 {} sri_rs", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();
											}
										},
										Descriptor::TextDescriptor(descriptor) => {
											println!("Searching a text similar to {}", query_text);
											for descriptor_from_index in self.get_texts_base() {
												writeln!(result_writer, "{} 0 {} 0 {} sri_rs", query_id, descriptor_from_index.get_id(), descriptor.compare_to(descriptor_from_index)).unwrap();

											}
										},
									}
								},
								None => println!("Error: could not create descriptor for query {}", query_id),
							}
						},
					}
				},
				Err(e) => panic!(e),
			}
		}
	}
}
