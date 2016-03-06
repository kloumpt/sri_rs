
use std::io::BufRead;
use std::io::Write;
use std::fs::File;


pub const IMAGE_QUANT_LVL: usize = 64;

pub struct ImageDescriptor {
	id: String,
	pixels_amount: usize,
	histogram: [i32; IMAGE_QUANT_LVL],
}

impl ImageDescriptor {
	pub fn new(id: String) -> ImageDescriptor { ImageDescriptor { id: id, pixels_amount: 0, histogram: [0; IMAGE_QUANT_LVL] } }


	pub fn from_histogram(id: String, length: usize, source_histogram: [i32; IMAGE_QUANT_LVL]) -> ImageDescriptor { ImageDescriptor { id: id, pixels_amount: length, histogram: source_histogram } }


	pub fn from_file(file_reader: &mut BufRead) -> Result<Option<ImageDescriptor>, String> {
		let id: String;
		let _quantification_levels: usize;
		let mut histogram = [0; IMAGE_QUANT_LVL];
		let pixels_amount;

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
									Ok(i) => _quantification_levels = i,
									Err(e) => return Err(String::from(format!("Invalid value for header field 2 ({})", e))),
								}
							},
							None => return Err(format!("Invalid header ({})", header)),
						}
						match header_fields.next() {
							Some(value) => {
								match value.parse::<usize>() {
									Ok(i) => pixels_amount = i,
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

		let mut histogram_line = String::new();
		match file_reader.read_line(&mut histogram_line) {
			Ok(_) => {
				for (index, value) in histogram_line.split_whitespace().enumerate() {
					match value.parse::<i32>() {
						Ok(i) => {
							histogram[index] = i;
						},
						Err(e) => return Err(String::from(format!("Invalid value in histogram of {} ({})", id, e))),
					}
				}
			},
			Err(e) => return Err(format!("{}", e)),
		}

		Ok(Some(ImageDescriptor::from_histogram(id, pixels_amount, histogram)))
	}

	pub fn get_id(&self) -> &str { &self.id }
	pub fn get_pixels_amount(&self) -> usize { self.pixels_amount }
	pub fn get_histogram(&self) -> &[i32; IMAGE_QUANT_LVL] { &self.histogram }

	pub fn to_file(&self, file_writer: &mut File) {
		write!(file_writer, "{} {} {}\n", self.get_id(), 64, self.get_pixels_amount()).unwrap();
		for value in self.get_histogram().into_iter() {
			write!(file_writer, "{} ", value).unwrap();
		}
		write!(file_writer, "\n").unwrap();
	}

	pub fn compare_to(&self, other: &ImageDescriptor) -> f64 {

		let self_weight = self.get_pixels_amount() as f64;
		let other_weight = self.get_pixels_amount() as f64;
		let mut score: f64 = 0.0;
		for (self_occ, other_occ) in self.histogram.iter().zip(other.get_histogram().iter()) {
			let self_occ = *self_occ as f64 / self_weight;
			let other_occ = *other_occ as f64 / other_weight;
			score += self_occ * other_occ;

		}

		return score;
	}
}

pub fn quantification_image(r_int: u32, g_int: u32, b_int: u32) -> Result<u32, String> {
	let r1_int;
	let r2_int;
	let g1_int;
	let g2_int;
	let b1_int;
	let b2_int;
	// Rouge
	if r_int > 127 {
		r1_int = 1;
		if (r_int - 128) > 63 {
			r2_int = 1;
		} else {
			r2_int = 0;
		}
	} else {
		r1_int = 0;
		if r_int > 63 {
			r2_int = 1;
		} else {
			r2_int = 0;
		}
	}


	// Vert
	if g_int > 127 {
		g1_int = 1;
		if (g_int - 128) > 63 {
			g2_int = 1;
		} else {
			g2_int = 0;
		}
	} else {
		g1_int = 0;
		if g_int > 63 {
			g2_int = 1;
		} else {
			g2_int = 0;
		}
	}


	// Bleu
	if b_int > 127 {
		b1_int = 1;
		if (b_int - 128) > 63 {
			b2_int = 1;
		} else {
			b2_int = 0;
		}
	} else {
		b1_int = 0;
		if b_int > 63 {
			b2_int = 1;
		} else {
			b2_int = 0;
		}
	}
	return Ok(r1_int * 32 + r2_int * 16 + g1_int * 8 + g2_int * 4 + b1_int * 2 + b2_int);
}
