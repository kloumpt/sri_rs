use includes::image_types::*;

use std::io::BufReader;
use std::io::BufRead;

use std::fs::File;
use std::path::Path;

use image;
use image::Pixel;
use image::GenericImage;

use super::super::context_types::ContextObject;

impl ImageDescriptor {
	pub fn from_image_file(context: &mut ContextObject, image_filename: &str) -> Result<ImageDescriptor, String> {


		let mut pixels_amount = 0;
		let mut histogram: [i32; IMAGE_QUANT_LVL] = [0; IMAGE_QUANT_LVL];


		let img = image::open(&Path::new(image_filename)).unwrap();

		for (_x, _y, pixel) in img.pixels() {
			// let (r_int, g_int, b_int) = match pixel {
			// image::color::Rgba | image::color::Rgb => {
			// (pixel.data[0], pixel.data[1], pixel.data[2])
			// }
			// image::color::Luma => (pixel.data[0], pixel.data[0], pixel.data[0]),
			// _ => (0, 0, 0),
			//
			// };
			let pixel = pixel.to_rgb();
			match quantification_image(pixel.data[0] as u32, pixel.data[1] as u32, pixel.data[2] as u32) {
				Ok(quantification) => {
					histogram[quantification as usize] += 1;
					pixels_amount += 1;
				},
				Err(e) => panic!(e),
			}
		}

		Ok(ImageDescriptor::from_histogram(context.gen_id(String::from("img")), pixels_amount, histogram))
	}
	pub fn from_plain_text_file(context: &mut ContextObject, image_file: File) -> Result<ImageDescriptor, String> {
		let l_int;
		let h_int;
		let nbcomp_int;

		let mut pixels_amount = 0;
		let mut histogram: [i32; IMAGE_QUANT_LVL] = [0; IMAGE_QUANT_LVL];

		let mut header = String::new();

		let mut file_reader = BufReader::new(&image_file);
		match file_reader.read_line(&mut header) {
			Ok(_) => {
				let mut header_fields = header.split_whitespace();
				match header_fields.next() {
					Some(value) => {
						match value.parse::<usize>() {
							Ok(i) => l_int = i,
							Err(e) => return Err(String::from(format!("Invalid value for header field 1 ({})", e))),
						}
					},
					None => return Err(format!("Invalid header ({})", header)),
				}
				match header_fields.next() {
					Some(value) => {
						match value.parse::<usize>() {
							Ok(i) => h_int = i,
							Err(e) => return Err(String::from(format!("Invalid value for header field 2 ({})", e))),
						}
					},
					None => return Err(format!("Invalid header ({})", header)),
				}
				match header_fields.next() {
					Some(value) => {
						match value.parse::<usize>() {
							Ok(i) => nbcomp_int = i,
							Err(e) => return Err(String::from(format!("Invalid value for header field 3 ({})", e))),
						}
					},
					None => return Err(format!("Invalid header ({})", header)),
				}
			},
			Err(e) => return Err(format!("{}", e)),
		}

		if nbcomp_int == 3 {
			let mut r_values = String::new();
			let mut g_values = String::new();
			let mut b_values = String::new();

			for (line_number, line) in file_reader.lines().enumerate() {
				match line {
					Ok(line) => {
						match line_number / (h_int as usize) {
							0 => r_values.push_str(&line),
							1 => g_values.push_str(&line),
							2 => b_values.push_str(&line),
							_ => {
								break;
							},
						}
					},
					Err(e) => return Err(format!("{}", e)),
				}
			}

			for (cpt_int, ((r_str, g_str), b_str)) in r_values.split_whitespace().zip(g_values.split_whitespace()).zip(b_values.split_whitespace()).enumerate() {
				if cpt_int > l_int * h_int {
					break;
				}
				let r_int = match r_str.parse::<u32>() {
					Ok(i) => i,
					Err(e) => return Err(String::from(format!("Corrupted image file ({})", e))),
				};

				let g_int = match g_str.parse::<u32>() {
					Ok(i) => i,
					Err(e) => return Err(String::from(format!("Corrupted image file ({})", e))),
				};

				let b_int = match b_str.parse::<u32>() {
					Ok(i) => i,
					Err(e) => return Err(String::from(format!("Corrupted image file ({})", e))),
				};
				match quantification_image(r_int, g_int, b_int) {
					Ok(quantification) => {
						histogram[quantification as usize] += 1;
						pixels_amount += 1;
					},
					Err(e) => panic!(e),
				}
			}
		} else if nbcomp_int == 1 {
			let mut current_int;

			let mut cpt_int = 0;
			for line in file_reader.lines() {

				if cpt_int > l_int * h_int {
					break;
				}

				match line {
					Ok(line) => {
						for value in line.split_whitespace() {
							match value.parse::<u32>() {
								Ok(i) => current_int = i,
								Err(e) => return Err(String::from(format!("Corrupted image file ({})", e))),
							}

							match quantification_image(current_int, current_int, current_int) {
								Ok(quantification) => {
									histogram[quantification as usize] += 1;
									pixels_amount += 1;
								},
								Err(e) => panic!(e),
							}
							cpt_int += 1;
						}
					},
					Err(e) => return Err(format!("{}", e)),
				}
			}
		}

		Ok(ImageDescriptor::from_histogram(context.gen_id(String::from("img")), pixels_amount, histogram))
	}
}
