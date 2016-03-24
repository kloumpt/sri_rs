


use std::collections::HashMap;

use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::cmp;

use includes::image_types::ImageDescriptor;
use includes::sound_types::SoundDescriptor;
use includes::text_types::TextDescriptor;

static IMAGES_BASE_FILENAME: &'static str = "img_base.txt";
static SOUNDS_BASE_FILENAME: &'static str = "snd_base.txt";
static TEXTS_BASE_FILENAME: &'static str = "txt_base.txt";

static IMAGES_ASSOCIATION_FILENAME: &'static str = "img_association.txt";
static SOUNDS_ASSOCIATION_FILENAME: &'static str = "snd_association.txt";
static TEXTS_ASSOCIATION_FILENAME: &'static str = "txt_association.txt";

pub struct ContextObject {
	max_doc_id: u64,
	config: HashMap<String, String>,
	images_associations: HashMap<String, String>,
	sounds_associations: HashMap<String, String>,
	texts_associations: HashMap<String, String>,
	images_base: Vec<ImageDescriptor>,
	sounds_base: Vec<SoundDescriptor>,
	texts_base: Vec<TextDescriptor>,
}

impl ContextObject {
	pub fn new() -> ContextObject { ContextObject { max_doc_id: 0, config: HashMap::new(), images_associations: HashMap::new(), sounds_associations: HashMap::new(), texts_associations: HashMap::new(), images_base: vec![], sounds_base: vec![], texts_base: vec![] } }

	pub fn gen_id(&mut self, prefix: String) -> String {
		let id = String::from(format!("{}{}", prefix, self.max_doc_id));
		self.max_doc_id += 1;
		id
	}


	pub fn add_image_association(&mut self, id: String, filename: String) { self.images_associations.insert(id, filename); }
	pub fn add_sound_association(&mut self, id: String, filename: String) { self.sounds_associations.insert(id, filename); }
	pub fn add_text_association(&mut self, id: String, filename: String) { self.texts_associations.insert(id, filename); }


	pub fn add_image_descriptor(&mut self, descriptor: ImageDescriptor) { self.images_base.push(descriptor); }
	pub fn add_sound_descriptor(&mut self, descriptor: SoundDescriptor) { self.sounds_base.push(descriptor); }
	pub fn add_text_descriptor(&mut self, descriptor: TextDescriptor) { self.texts_base.push(descriptor); }



	pub fn get_images_associations(&self) -> &HashMap<String, String> { &self.images_associations }
	pub fn get_sounds_associations(&self) -> &HashMap<String, String> { &self.sounds_associations }
	pub fn get_texts_associations(&self) -> &HashMap<String, String> { &self.texts_associations }





	pub fn get_images_base(&self) -> &Vec<ImageDescriptor> { &self.images_base }

	pub fn get_sounds_base(&self) -> &Vec<SoundDescriptor> { &self.sounds_base }

	pub fn get_texts_base(&self) -> &Vec<TextDescriptor> { &self.texts_base }




	pub fn get_config(&self) -> &HashMap<String, String> { &self.config }
	pub fn get_param(&self, param: &str) -> Option<&String> { self.config.get(param) }
	pub fn set_param(&mut self, param: String, value: String) { self.config.insert(param, value); }


	pub fn complete_config(&mut self) {
		match self.get_param("images_base_filename") {
			Some(_) => (),
			None => {
				self.set_param(String::from("images_base_filename"), String::from(IMAGES_BASE_FILENAME));
				()
			},
		}
		match self.get_param("sounds_base_filename") {
			Some(_) => (),
			None => {
				self.set_param(String::from("sounds_base_filename"), String::from(SOUNDS_BASE_FILENAME));
				()
			},
		}
		match self.get_param("texts_base_filename") {
			Some(_) => (),
			None => {
				self.set_param(String::from("texts_base_filename"), String::from(TEXTS_BASE_FILENAME));
				()
			},
		}

		match self.get_param("images_associations_filename") {
			Some(_) => (),
			None => {
				self.set_param(String::from("images_associations_filename"), String::from(IMAGES_ASSOCIATION_FILENAME));
				()
			},
		}
		match self.get_param("sounds_associations_filename") {
			Some(_) => (),
			None => {
				self.set_param(String::from("sounds_associations_filename"), String::from(SOUNDS_ASSOCIATION_FILENAME));
				()
			},
		}
		match self.get_param("texts_associations_filename") {
			Some(_) => (),
			None => {
				self.set_param(String::from("texts_associations_filename"), String::from(TEXTS_ASSOCIATION_FILENAME));
				()
			},
		}

	}

	pub fn load_index(&mut self) {
		let index_path;
		match self.get_param("index") {
			Some(value) => index_path = value.clone(),
			None => panic!("Can't find parameter 'index' in config"),
		}


		let mut associations_reader = match self.get_param("images_associations_filename") {
			Some(value) => {
				match File::open(format!("{}/{}", index_path, value)) {
					Ok(file) => BufReader::new(file),
					Err(e) => panic!("{}", e),

				}
			},
			None => panic!("Can't find parameter 'images_associations_filename' in config"),
		};
		let mut file_reader = match self.get_param("images_base_filename") {
			Some(value) => {
				match File::open(format!("{}/{}", index_path, value)) {
					Ok(file) => BufReader::new(file),
					Err(e) => panic!("{}", e),

				}
			},
			None => panic!("Can't find parameter 'images_base_filename' in config"),
		};

		for line in associations_reader.lines() {
			match line {
				Ok(association) => {
					let mut fields = association.split("=");
					let id = fields.next().unwrap();
					let filename = fields.next().unwrap();
					self.add_image_association(String::from(id), String::from(filename));

					let string_number: String = id.chars().skip(3).collect();
					match string_number.parse::<u64>() {
						Ok(value) => self.max_doc_id = cmp::max(self.max_doc_id, value + 1),
						Err(_) => println!("Warning: wrong id for image association '{}' in saved index", association),
					}
				},
				Err(e) => panic!(e),
			}
		}


		loop {
			match ImageDescriptor::from_file(&mut file_reader) {
				Ok(value) => {
					match value {
						Some(image_descriptor) => self.add_image_descriptor(image_descriptor),
						None => {
							break;
						},
					}
				},
				Err(e) => {
					println!("{}", e);
				},
			}
		}




		associations_reader = match self.get_param("sounds_associations_filename") {
			Some(value) => {
				match File::open(format!("{}/{}", index_path, value)) {
					Ok(file) => BufReader::new(file),
					Err(e) => panic!("{}", e),

				}
			},
			None => panic!("Can't find parameter 'sounds_associations_filename' in config"),
		};
		file_reader = match self.get_param("sounds_base_filename") {
			Some(value) => {
				match File::open(format!("{}/{}", index_path, value)) {
					Ok(file) => BufReader::new(file),
					Err(e) => panic!("{}", e),

				}
			},
			None => panic!("Can't find parameter 'sounds_base_filename' in config"),
		};

		for line in associations_reader.lines() {
			match line {
				Ok(association) => {
					let mut fields = association.split("=");
					let id = fields.next().unwrap();
					let filename = fields.next().unwrap();
					self.add_sound_association(String::from(id), String::from(filename));

					let string_number: String = id.chars().skip(3).collect();
					match string_number.parse::<u64>() {
						Ok(value) => self.max_doc_id = cmp::max(self.max_doc_id, value + 1),
						Err(_) => println!("Warning: wrong id for sound association '{}' in saved index", association),
					}
				},
				Err(e) => panic!(e),
			}
		}

		loop {
			let result = match SoundDescriptor::from_file(&mut file_reader) {
				Ok(value) => {
					match value {
						Some(sound_descriptor) => Ok(sound_descriptor),
						None => {
							break;
						},
					}
				},
				Err(e) => Err(e),
			};
			match result {
				Ok(sound_descriptor) => self.add_sound_descriptor(sound_descriptor),
				Err(e) => {
					println!("{}", e);
					for line in (&mut file_reader).lines() {
						match line {
							Ok(line) => {
								match line.trim() {
									"" => break,
									_ => (),
								}
							},
							_ => (),
						}
					}
				},
			}
		}


		associations_reader = match self.get_param("texts_associations_filename") {
			Some(value) => {
				match File::open(format!("{}/{}", index_path, value)) {
					Ok(file) => BufReader::new(file),
					Err(e) => panic!("{}", e),
				}
			},
			None => panic!("Can't find parameter 'texts_associations_filename' in config"),
		};

		file_reader = match self.get_param("texts_base_filename") {
			Some(value) => {
				match File::open(format!("{}/{}", index_path, value)) {
					Ok(file) => BufReader::new(file),
					Err(e) => panic!("{}", e),

				}
			},
			None => panic!("Can't find parameter 'texts_base_filename' in config"),
		};

		for line in associations_reader.lines() {
			match line {
				Ok(association) => {
					let mut fields = association.split("=");
					let id = fields.next().unwrap();
					let filename = fields.next().unwrap();
					self.add_text_association(String::from(id), String::from(filename));


					let string_number: String = id.chars().skip(3).collect();
					match string_number.parse::<u64>() {
						Ok(value) => self.max_doc_id = cmp::max(self.max_doc_id, value + 1),
						Err(_) => println!("Warning: wrong id for text association '{}' in saved index", association),
					}
				},
				Err(e) => panic!(e),
			}
		}

		loop {
			let result = match TextDescriptor::from_file(&mut file_reader) {
				Ok(value) => {
					match value {
						Some(text_descriptor) => Ok(text_descriptor),
						None => {
							break;
						},
					}
				},
				Err(e) => Err(e),
			};
			match result {
				Ok(text_descriptor) => self.add_text_descriptor(text_descriptor),
				Err(e) => {
					println!("{}", e);
					for line in (&mut file_reader).lines() {
						match line {
							Ok(line) => {
								match line.trim() {
									"" => break,
									_ => (),
								}
							},
							_ => (),
						}
					}
				},
			}
		}
	}

	pub fn save_index(&self) {

		let index_path;
		match self.get_param("index") {
			Some(value) => index_path = value.clone(),
			None => panic!("Can't find parameter 'index' in config"),
		}


		let mut associations_writer = match self.get_param("images_associations_filename") {
			Some(value) => {
				match File::create(format!("{}/{}", index_path, value.clone())) {
					Ok(file) => file,
					Err(e) => panic!("{}", e),
				}
			},
			None => panic!("Can't find parameter 'images_associations_filename' in config"),
		};

		let mut base_writer = match self.get_param("images_base_filename") {
			Some(value) => {
				match File::create(format!("{}/{}", index_path, value.clone())) {
					Ok(file) => file,
					Err(e) => panic!("{}", e),
				}
			},
			None => panic!("Can't find parameter 'images_base_filename' in config"),
		};

		for (id, filename) in &self.images_associations {
			write!(associations_writer, "{}={}\n", id, filename).unwrap();
		}
		for image_descriptor in &self.images_base {
			image_descriptor.to_file(&mut base_writer);
		}





		associations_writer = match self.get_param("sounds_associations_filename") {
			Some(value) => {
				match File::create(format!("{}/{}", index_path, value.clone())) {
					Ok(file) => file,
					Err(e) => panic!("{}", e),
				}
			},
			None => panic!("Can't find parameter 'sounds_associations_filename' in config"),
		};

		base_writer = match self.get_param("sounds_base_filename") {
			Some(value) => {
				match File::create(format!("{}/{}", index_path, value.clone())) {
					Ok(file) => file,
					Err(e) => panic!("{}", e),

				}
			},
			None => panic!("Can't find parameter 'sounds_base_filename' in config"),
		};

		for (id, filename) in &self.sounds_associations {
			write!(associations_writer, "{}={}\n", id, filename).unwrap();
		}
		for sound_descriptor in &self.sounds_base {
			sound_descriptor.to_file(&mut base_writer);
		}



		associations_writer = match self.get_param("texts_associations_filename") {
			Some(value) => {
				match File::create(format!("{}/{}", index_path, value.clone())) {
					Ok(file) => file,
					Err(e) => panic!("{}", e),
				}
			},
			None => panic!("Can't find parameter 'texts_associations_filename' in config"),
		};

		base_writer = match self.get_param("texts_base_filename") {
			Some(value) => {
				match File::create(format!("{}/{}", index_path, value.clone())) {
					Ok(file) => file,
					Err(e) => panic!("{}", e),

				}
			},
			None => panic!("Can't find parameter 'texts_base_filename' in config"),
		};

		for (id, filename) in &self.texts_associations {
			write!(associations_writer, "{}={}\n", id, filename).unwrap();
		}
		for text_descriptor in &self.texts_base {
			text_descriptor.to_file(&mut base_writer);
		}
	}


	pub fn index_details(&self, verbose: bool) {
		println!("Indexed images: ");
		for descriptor in self.get_images_base() {
			match self.get_images_associations().get(descriptor.get_id()) {
				Some(images_base_filename) => {
					println!("{}=>{}", descriptor.get_id(), images_base_filename);
					if verbose {
						println!("{} {} {}", descriptor.get_id(), 64, descriptor.get_pixels_amount());
						for i in 0..descriptor.get_histogram().len() {
							print!("{}", descriptor.get_histogram()[i]);
						}
					}
				},
				None => (),
			}
		}
		println!("");

		println!("Indexed sounds: ");
		for descriptor in self.get_sounds_base() {
			match self.get_sounds_associations().get(descriptor.get_id()) {
				Some(sounds_base_filename) => {
					println!("{}=>{}", descriptor.get_id(), sounds_base_filename);
					if verbose {
						println!("{} {} {} {}", descriptor.get_id(), descriptor.get_histograms_size(), descriptor.get_histograms_levels(), descriptor.get_histograms().len());
						for i in 0..descriptor.get_histograms().len() {
							for j in 0..descriptor.get_histograms()[i].len() {
								print!("{} ", descriptor.get_histograms()[i][j]);
							}
							println!("");
						}
					}
				},
				None => (),
			}
		}
		println!("");


		println!("Indexed texts: ");
		for descriptor in self.get_texts_base() {
			match self.get_texts_associations().get(descriptor.get_id()) {
				Some(texts_base_filename) => {
					println!("{}=>{}", descriptor.get_id(), texts_base_filename);
					if verbose {
						println!("{} {} {}", descriptor.get_id(), descriptor.get_word_file(), descriptor.get_word_numb());
						for (word, occurence) in descriptor.get_sorted_occurences() {
							print!("{} {} ", word, occurence);
						}
						println!("");
					}
				},
				None => (),
			}
		}
		println!("");
	}
}
