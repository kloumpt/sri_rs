
use std::io::BufRead;
use std::io::Write;
use std::fs::File;

pub struct SoundDescriptor {
	id: String,
	histograms_size: usize,
	histograms_levels: usize,
	histograms: Vec<Vec<i32>>,
}

impl SoundDescriptor {
	pub fn new(id: String) -> SoundDescriptor { SoundDescriptor { id: id, histograms_size: 0, histograms_levels: 0, histograms: Vec::new() } }


	pub fn from_histogram(id: String, histograms_size: usize, histograms_levels: usize, histograms: Vec<Vec<i32>>) -> SoundDescriptor { SoundDescriptor { id: id, histograms_size: histograms_size, histograms_levels: histograms_levels, histograms: histograms } }

	pub fn from_file(file_reader: &mut BufRead) -> Result<Option<SoundDescriptor>, String> {
		let id: String;
		let histograms_size: usize;
		let histograms_count: usize;
		let histograms_levels: usize;
		let mut histograms: Vec<Vec<i32>> = Vec::new();

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
									Ok(i) => histograms_size = i,
									Err(e) => return Err(String::from(format!("Invalid value for header field 2 ({})", e))),
								}
							},
							None => return Err(format!("Invalid header ({})", header)),
						}

						match header_fields.next() {
							Some(value) => {
								match value.parse::<usize>() {
									Ok(i) => histograms_levels = i,
									Err(e) => return Err(String::from(format!("Invalid value for header field 2 ({})", e))),
								}
							},
							None => return Err(format!("Invalid header ({})", header)),
						}

						match header_fields.next() {
							Some(value) => {
								match value.parse::<usize>() {
									Ok(i) => histograms_count = i,
									Err(e) => return Err(String::from(format!("Invalid value for header field 2 ({})", e))),
								}
							},
							None => return Err(format!("Invalid header ({})", header)),
						}
					},
				}
			},
			Err(e) => return Err(format!("{}", e)),
		}

		for (line_number, line) in file_reader.lines().enumerate() {
			if line_number >= histograms_count {
				break;
			}
			match line {
				Ok(histogram_line) => {
					match histogram_line.trim() {
						"" => break,
						_ => (),
					};

					let mut histogram = Vec::new();
					histogram.resize(histograms_levels * 2, 0);
					for (index, value) in histogram_line.split_whitespace().enumerate() {
						match value.parse::<i32>() {
							Ok(i) => histogram[index] = i,
							Err(e) => return Err(String::from(format!("Invalid value in histogram of {} ({})", id, e))),
						}
					}
					histograms.push(histogram);
				},
				Err(e) => return Err(format!("{}", e)),
			}

		}

		Ok(Some(SoundDescriptor::from_histogram(id, histograms_size, histograms_levels, histograms)))
	}
	pub fn get_id(&self) -> &str { &self.id }

	pub fn get_histograms_size(&self) -> usize { self.histograms_size }

	pub fn get_histograms_levels(&self) -> usize { self.histograms_levels }

	pub fn get_histograms(&self) -> &Vec<Vec<i32>> { &self.histograms }

	pub fn to_file(&self, file_writer: &mut File) {
		write!(file_writer, "{} {} {} {}\n", self.get_id(), self.get_histograms_size(), self.get_histograms_levels(), self.get_histograms().len()).unwrap();
		for histogram in self.get_histograms() {
			for value in histogram {
				write!(file_writer, "{} ", value).unwrap();
			}
			write!(file_writer, "\n").unwrap();
		}
		write!(file_writer, "\n").unwrap();
	}


	pub fn compare_to(&self, other: &SoundDescriptor) -> f64 {
		if self.get_histograms().len() < other.get_histograms().len() {
			return other.compare_to(self);
		}


		let histograms_amount_a = self.get_histograms().len();
		let histograms_amount_b = other.get_histograms().len();

		let window_levels = self.get_histograms_levels();


		let self_weight = self.get_histograms_size() as f64;
		let other_weight = other.get_histograms_size() as f64;

		let mut score: f64 = 0 as f64;
		for i in 0..(histograms_amount_a - histograms_amount_b + 1) {
			for j in 0..histograms_amount_b {
				let mut tmp_score: f64 = 0.0;

				for (index, (self_value, other_value)) in self.get_histograms()[i + j].iter().zip(other.get_histograms()[j].iter()).enumerate() {
					if index >= window_levels {
						break;
					}

					let self_value = *self_value as f64 / self_weight;
					let other_value = *other_value as f64 / other_weight;

					tmp_score += self_value * other_value;
				}

				score = score.max(tmp_score);
			}
		}
		return score;
	}
}
