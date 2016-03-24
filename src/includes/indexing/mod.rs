pub mod image_indexing;
pub mod sound_indexing;
pub mod text_indexing;


use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


use includes::image_types::ImageDescriptor;
use includes::sound_types::SoundDescriptor;
use includes::text_types::TextDescriptor;

use includes::context_types::ContextObject;
impl ContextObject {
	pub fn start_indexing(&mut self, documents_list_file: &str) {

		match File::open(documents_list_file) {
			Ok(file) => {
				let file = BufReader::new(&file);
				for line in file.lines() {
					let l = line.unwrap();
					let mut document_already_indexed = false;
					for document_filename in self.get_images_associations().values().chain(self.get_texts_associations().values()).chain(self.get_sounds_associations().values()) {
						if document_filename == &l {
							document_already_indexed = true;
							break;
						}
					}
					if !document_already_indexed {
						match l.rsplit('.').next().unwrap().to_string().to_lowercase().as_ref() {
							"txt" => {
								// Image file
								println!("Indexing image: {}", l);

								match File::open(l.clone()) {
									Ok(image_file) => {
										match ImageDescriptor::from_plain_text_file(self, image_file) {
											Ok(descriptor) => {
												self.add_image_association(String::from(descriptor.get_id()), l);
												self.add_image_descriptor(descriptor);

											},
											Err(e) => println!("{:?}", e),
										}
									},
									Err(e) => println!("{} {}", e, l),
								}
								()
							},
							"png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "tiff" | "webp" => {
								match ImageDescriptor::from_image_file(self, l.as_ref()) {
									Ok(descriptor) => {
										self.add_image_association(String::from(descriptor.get_id()), l);
										self.add_image_descriptor(descriptor);

									},
									Err(e) => println!("{:?}", e),
								}
							},
							"bin" => {
								// Sound file
								println!("Indexing sound: {}", l);

								match File::open(l.clone()) {
									Ok(image_file) => {
										match SoundDescriptor::from_raw_file(self, image_file) {
											Ok(descriptor) => {
												self.add_sound_association(String::from(descriptor.get_id()), l);
												self.add_sound_descriptor(descriptor);

											},
											Err(e) => println!("{:?}", e),
										}
									},
									Err(e) => println!("{} {}", e, l),
								}
								()
							},
							"wav" => {
								// Sound file
								println!("Indexing sound: {}", l);

								match File::open(l.clone()) {
									Ok(image_file) => {
										match SoundDescriptor::from_wav_file(self, image_file) {
											Ok(descriptor) => {
												self.add_sound_association(String::from(descriptor.get_id()), l);
												self.add_sound_descriptor(descriptor);

											},
											Err(e) => println!("{:?}", e),
										}
									},
									Err(e) => println!("{} {}", e, l),
								}
								()
							},
							"mp3" | "mpeg3" => {
								// Sound file
								println!("Indexing sound: {}", l);

								match File::open(l.clone()) {
									Ok(image_file) => {
										match SoundDescriptor::from_mp3_file(self, image_file) {
											Ok(descriptor) => {
												self.add_sound_association(String::from(descriptor.get_id()), l);
												self.add_sound_descriptor(descriptor);

											},
											Err(e) => println!("{:?}", e),
										}
									},
									Err(e) => println!("{} {}", e, l),
								}
								()
							},
							"xml" => {
								// Text file
								println!("Indexing text: {}", l);

								match File::open(l.clone()) {
									Ok(image_file) => {
										match TextDescriptor::from_text_file(self, image_file) {
											Ok(descriptor) => {
												self.add_text_association(String::from(descriptor.get_id()), l);
												self.add_text_descriptor(descriptor);

											},
											Err(e) => println!("{:?}", e),
										}
									},
									Err(e) => println!("{} {}", e, l),
								}
								()
							},
							_ => (),
						}
					}
				}
			},
			Err(e) => panic!("{}", e),
		}
	}
}
