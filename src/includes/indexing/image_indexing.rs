use includes::image_types::*;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use super::super::context_types::ContextObject;

impl ImageDescriptor{

	pub fn from_image_file(context: &mut ContextObject, image_file: File) ->Result<ImageDescriptor, String>{
		let mut l_int=0;
		let mut h_int=0;
		let mut nbcomp_int=0;

		let mut histogram: [u32; IMAGE_QUANT_LVL]=[0; IMAGE_QUANT_LVL];

		let mut header=String::new();

		let mut file_reader = BufReader::new(&image_file);
		match file_reader.read_line(&mut header){
			Ok(_)=>{
				let mut header_fields=header.split_whitespace();
				match header_fields.next(){
					Some(value)=>match value.parse::<usize>(){
							Ok(i)=> l_int=i,
							Err(e)=>return Err(String::from(format!("Invalid value for header field 1 ({})", e)))
					},
					None=>return Err(format!("Invalid header ({})", header))
				}
				match header_fields.next(){
					Some(value)=>match value.parse::<usize>(){
							Ok(i)=> h_int=i,
							Err(e)=>return Err(String::from(format!("Invalid value for header field 2 ({})", e)))
					},
					None=>return Err(format!("Invalid header ({})", header))
				}
				match header_fields.next(){
					Some(value)=>match value.parse::<usize>(){
							Ok(i)=> nbcomp_int=i,
							Err(e)=>return Err(String::from(format!("Invalid value for header field 3 ({})", e)))
					},
					None=>return Err(format!("Invalid header ({})", header))
				}
			},
			Err(e)=>println!("{}", e)
		}

		if nbcomp_int == 3 {
			let mut r_values = String::new();
			let mut g_values = String::new();
			let mut b_values = String::new();

			for (line_number, line) in file_reader.lines().enumerate(){
				match line {
					Ok(line)=>{
						match line_number/(h_int as usize){
							0=>r_values.push_str(&line),
							1=>g_values.push_str(&line),
							2=>b_values.push_str(&line),
							_=>{
								break;
							}
						}
					},
					Err(e)=>println!("{}", e)
				}
			}

			for (cpt_int, ((r_str, g_str), b_str)) in r_values.split_whitespace().zip(g_values.split_whitespace()).zip(b_values.split_whitespace()).enumerate(){
				if cpt_int > l_int*h_int{
					break;
				}
				let r_int = match r_str.parse::<u32>(){
					Ok(i)=> i,
					Err(e)=>return Err(String::from(format!("Corrupted image file ({})", e)))
				};

				let g_int = match g_str.parse::<u32>(){
					Ok(i)=> i,
					Err(e)=>return Err(String::from(format!("Corrupted image file ({})", e)))
				};

				let b_int = match b_str.parse::<u32>(){
					Ok(i)=> i,
					Err(e)=>return Err(String::from(format!("Corrupted image file ({})", e)))
				};
				match quantification_image(r_int, g_int, b_int){
					Ok(quantification)=>histogram[quantification as usize]+=1,
					Err(e)=>panic!(e)
				}
			}
		} else if nbcomp_int == 1 {
			let mut current_int;

			let mut cpt_int=0;
			for line in file_reader.lines(){

				if cpt_int > l_int*h_int{
					break;
				}

				match line {
					Ok(line)=>{
						for value in line.split_whitespace(){
							match value.parse::<u32>(){
									Ok(i)=> current_int=i,
									Err(e)=>return Err(String::from(format!("Corrupted image file ({})", e)))
							}

							match quantification_image(current_int, current_int, current_int){
								Ok(quantification)=>{histogram[quantification as usize]+=1;},
								Err(e)=>panic!(e)
							}
							cpt_int+=1;
						}
					},
					Err(e)=>println!("{}", e)
				}
			}
		}

		Ok(ImageDescriptor::from_histogram(context.gen_id(String::from("img")), histogram))
	}
}


fn quantification_image(r_int: u32, g_int: u32, b_int: u32)->Result<u32, String>{
	let r1_int;
	let r2_int;
	let g1_int;
	let g2_int;
	let b1_int;
	let b2_int;
	//Rouge
	if r_int > 127 {
			r1_int = 1;
		if (r_int - 128) > 63{
			r2_int = 1;
		}else{
			r2_int = 0;
		}
	}else{
		r1_int = 0;
		if r_int > 63{
			r2_int = 1;
		}else{
			r2_int = 0;
		}
	}


	//Vert
	if g_int > 127{
		g1_int = 1;
		if (g_int - 128) > 63{
			g2_int = 1;
		}else{
			g2_int = 0;
		}
	}else{
		g1_int = 0;
		if g_int > 63{
			g2_int = 1;
		}else{
			g2_int = 0;
		}
	}


	//Bleu
	if b_int > 127{
			b1_int = 1;
		if (b_int - 128) > 63{
			b2_int = 1;
		}else{
			b2_int = 0;
		}
	}else{
		b1_int = 0;
		if b_int > 63{
			b2_int = 1;
		}else{
			b2_int = 0;
		}
	}
	return Ok(r1_int*32 + r2_int*16 + g1_int*8 + g2_int*4 + b1_int*2 + b2_int);
}
