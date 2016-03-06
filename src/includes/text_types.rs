
use std::collections::HashMap;

use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::collections::HashSet;

// pub const NB_TERM: usize = 64;
pub const NB_TERM_MAX: usize = 128;
pub const SIZE_WORD_RM: usize = 0;


pub struct TextDescriptor {
	id: String,
	word_file: usize,
	word_numb: usize,
	occurences: HashMap<String, i32>,
}


impl TextDescriptor {
	pub fn new(id: String) -> TextDescriptor { TextDescriptor { id: id, word_file: 0, word_numb: 0, occurences: HashMap::new() } }


	pub fn from_occurences(id: String, word_file: usize, word_numb: usize, occurences: HashMap<String, i32>) -> TextDescriptor { TextDescriptor { id: id, word_file: word_file, word_numb: word_numb, occurences: occurences } }


	pub fn from_file(file_reader: &mut BufRead) -> Result<Option<TextDescriptor>, String> {
		let id: String;
		let word_file: usize;
		let word_numb: usize;
		let mut occurences: HashMap<String, i32> = HashMap::new();

		let mut header = String::new();
		match file_reader.read_line(&mut header) {
			Ok(_) => {
				match header.trim() {
					"" => return Ok(None),
					_ => {
						let mut header_fields = header.split_whitespace();
						match header_fields.next() {
							Some(value) => id = String::from(value),
							None => return Err(format!("Invalid header ({})", header)),
						}
						match header_fields.next() {
							Some(value) => {
								match value.parse::<usize>() {
									Ok(i) => word_file = i,
									Err(e) => return Err(String::from(format!("Invalid value for header field 2 ({})", e))),
								}
							},
							None => return Err(format!("Invalid header ({})", header)),
						}
						match header_fields.next() {
							Some(value) => {
								match value.parse::<usize>() {
									Ok(i) => word_numb = i,
									Err(e) => return Err(String::from(format!("Invalid value for header field 3 ({})", e))),
								}
							},
							None => return Err(format!("Invalid header ({})", header)),
						}
					},
				}
			},
			Err(e) => return Err(format!("{}", e)),
		}

		let mut occurences_line = String::new();
		match file_reader.read_line(&mut occurences_line) {
			Ok(_) => {
				let mut line_iterator = occurences_line.split_whitespace();
				loop {
					let word = match line_iterator.next() {
						Some(value) => value,
						None => break,
					};

					let result = match line_iterator.next() {
						Some(value) => {
							match value.parse::<i32>() {
								Ok(i) => Ok(occurences.insert(String::from(word), i)),
								Err(e) => Err(String::from(format!("Can't parse occurence for '{}' ({})", word, e))),
							}
						},
						None => Err(String::from(format!("Can't find occurence for '{}'", word))),
					};

					match result {
						Err(e) => return Err(e),
						_ => (),
					}
				}
				()
			},
			Err(e) => return Err(format!("{}", e)),
		}

		Ok(Some(TextDescriptor::from_occurences(id, word_file, word_numb, occurences)))
	}

	pub fn get_id(&self) -> &str { &self.id }

	pub fn get_word_file(&self) -> usize { self.word_file }

	pub fn get_word_numb(&self) -> usize { self.word_numb }

	pub fn get_occurences(&self) -> &HashMap<String, i32> { &self.occurences }

	pub fn get_sorted_occurences(&self) -> Vec<(String, i32)> {
		let mut values: Vec<(String, i32)> = self.occurences.clone().into_iter().collect();
		values.sort_by(|&(_, v_a), &(_, v_b)| v_b.cmp(&v_a));
		values
	}

	pub fn to_file(&self, file_writer: &mut File) {
		write!(file_writer, "{} {} {}\n", self.get_id(), self.get_word_file(), self.get_word_numb()).unwrap();
		for (word, occurence) in self.get_sorted_occurences() {
			write!(file_writer, "{} {} ", word, occurence).unwrap();
		}
		write!(file_writer, "\n").unwrap();
	}

	pub fn compare_to(&self, other: &TextDescriptor) -> f64 {
		let self_keys: HashSet<&String> = self.get_occurences().keys().collect();
		let other_keys: HashSet<&String> = other.get_occurences().keys().collect();
		let common_keys = self_keys.intersection(&other_keys);

		let self_weight = self.get_word_numb() as f64;
		let other_weight = other.get_word_numb() as f64;

		let mut score: f64 = 0.0;
		for key in common_keys {

			let self_occ = *self.get_occurences().get(*key).unwrap() as f64 / self_weight;
			let other_occ = *other.get_occurences().get(*key).unwrap() as f64 / other_weight;
			score += self_occ * other_occ;

		}
		return score;
	}
}
