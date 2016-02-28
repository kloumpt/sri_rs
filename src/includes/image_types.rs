
use std::io::BufRead;
use std::io::Write;
use std::fs::File;


pub const IMAGE_QUANT_LVL: usize = 64;

pub struct ImageDescriptor {
	id: String,
	histogram: [u32; IMAGE_QUANT_LVL]
}

impl ImageDescriptor{
	pub fn new(id: String)->ImageDescriptor{
		ImageDescriptor{
			id: id,
			histogram: [0; IMAGE_QUANT_LVL]
		}
	}


	pub fn from_histogram(id: String, source_histogram: [u32; IMAGE_QUANT_LVL])->ImageDescriptor{
		ImageDescriptor{
			id: id,
			histogram: source_histogram
		}
	}


	pub fn from_file(file_reader: &mut BufRead)->Result<Option<ImageDescriptor>, String>{
		let mut id: String = String::new();
		let _quantification_levels: usize;
		let mut histogram = [0; IMAGE_QUANT_LVL];

		let mut header = String::new();
		match file_reader.read_line(&mut header){
			Ok(_)=>{
				match header.trim(){
					""=> return Ok(None),
					_=>{
						let mut header_fields=header.split_whitespace();
						match header_fields.next(){
							Some(value)=>id=String::from(value),
							None=>return Err(format!("Invalid header ({})", header))
						}
						match header_fields.next(){
							Some(value)=>match value.parse::<usize>(){
									Ok(i)=> _quantification_levels=i,
									Err(e)=>return Err(String::from(format!("Invalid value for header field 2 ({})", e)))
							},
							None=>return Err(format!("Invalid header ({})", header))
						}
					}
				}
			},
			Err(e)=>println!("{}", e)
		}

		let mut histogram_line = String::new();
		match file_reader.read_line(&mut histogram_line){
			Ok(_)=>
				for (index, value) in histogram_line.split_whitespace().enumerate(){
					match value.parse::<u32>(){
						Ok(i)=> histogram[index]=i,
						Err(e)=>return Err(String::from(format!("Invalid value in histogram of {} ({})", id, e)))
					}
				}
			,
			Err(e)=>println!("{}", e)
		}

		Ok(Some(ImageDescriptor::from_histogram(id, histogram)))
	}

	pub fn get_id(&self)->&str{
		&self.id
	}
	pub fn get_histogram(&self)->&[u32; IMAGE_QUANT_LVL]{
		&self.histogram
	}

	pub fn to_file(&self, file_writer: &mut File){
		write!(file_writer, "{} {}\n", self.get_id(), 64).unwrap();
		for value in self.get_histogram().into_iter(){
			write!(file_writer, "{} ", value).unwrap();
		}
		write!(file_writer, "\n").unwrap();
	}

	pub fn compare_to(&self, other: &ImageDescriptor) -> f64 {
		0.0f64
	}
}
