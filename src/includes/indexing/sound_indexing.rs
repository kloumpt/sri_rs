
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::SeekFrom;
use byteorder::{ReadBytesExt, LittleEndian, BigEndian};

use includes::sound_types::*;
use super::super::context_types::ContextObject;

impl SoundDescriptor{
	pub fn from_sound_file(context: &mut ContextObject, image_file: File) ->Result<SoundDescriptor, String>{
		let histograms_size;
		let histograms_levels;
		match context.get_param("window_size"){
			Some(value)=>match value.parse::<usize>(){
					Ok(i)=> histograms_size=i,
					Err(e)=>return Err(String::from(format!("Invalid value for parameter window_size ({})", e)))
			},
			None=>panic!("Can't find parameter 'window_size' in config")
		}
		match context.get_param("window_levels"){
			Some(value)=>match value.parse::<usize>(){
					Ok(i)=> histograms_levels=i,
					Err(e)=>return Err(String::from(format!("Invalid value for parameter histograms_levels ({})", e)))
			},
			None=>panic!("Can't find parameter 'window_levels' in config")
		}

		let mut current_level;
		let mut position_in_window = 0;
		let mut window = 0;
		let mut histograms: Vec<Vec<u32>>=Vec::new();

		let read_f32 = |file_reader: &mut BufReader<&File>, little_endian| {
		    if little_endian{
				file_reader.read_f64::<LittleEndian>()
			} else {
				file_reader.read_f64::<BigEndian>()
			}
		};

		let mut file_reader = BufReader::new(&image_file);
		let mut little_endian =true;

		{
			let mut counts=0;
			while let Ok(value) =  read_f32(&mut file_reader, true){
				if counts>=histograms_size {
					break;
				}
				if value < -1. || value > 1.{
					little_endian=false;
				}
				counts+=1;

			}
		}

		file_reader.seek(SeekFrom::Start(0)).unwrap();
		while let Ok(mut current_value) =  read_f32(&mut file_reader, little_endian){
			if position_in_window==0{
				histograms.insert(window, Vec::new());
				histograms[window].resize(histograms_levels, 0);
			}

			if current_value < -1f64 || current_value > 1f64{
				return Err(String::from(format!("Wrong value in sound file ({})", current_value)));
			}

			current_value= current_value*0.5+0.5;
			current_level=current_value*(histograms_levels as f64);

			histograms[window][current_level as usize]+=1;

			position_in_window+=1;

			if position_in_window>=histograms_size {
				window+=1;
				position_in_window=0;
			}

		}


		Ok(SoundDescriptor::from_histogram(context.gen_id(String::from("snd")), histograms_size, histograms_levels, histograms))
	}
}
