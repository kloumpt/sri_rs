use includes::image_types::*;
use includes::image_types;
use includes::context_types::ContextObject;

impl ImageDescriptor {
	pub fn from_criteria(context: &mut ContextObject, criteria_as_str: &str) -> Result<ImageDescriptor, String> {

		let mut histogram: [i32; IMAGE_QUANT_LVL] = [0; IMAGE_QUANT_LVL];


		let mut color_components = criteria_as_str.split_whitespace();

		let r_int = match color_components.next().unwrap().parse::<u32>() {
			Ok(i) => i,
			Err(e) => return Err(String::from(format!("Corrupted image file ({})", e))),
		};

		let g_int = match color_components.next().unwrap().parse::<u32>() {
			Ok(i) => i,
			Err(e) => return Err(String::from(format!("Corrupted image file ({})", e))),
		};

		let b_int = match color_components.next().unwrap().parse::<u32>() {
			Ok(i) => i,
			Err(e) => return Err(String::from(format!("Corrupted image file ({})", e))),
		};

		match image_types::quantification_image(r_int, g_int, b_int) {
			Ok(quantification) => {
				histogram[quantification as usize] = 1;
			},
			Err(e) => panic!(e),
		}


		Ok(ImageDescriptor::from_histogram(context.gen_id(String::from("img")), 1, histogram))
	}
}
