pub mod util;

pub mod image_types;
pub mod sound_types;
pub mod text_types;

pub mod indexing;
pub mod querying;

pub mod context_types;

extern crate time;

pub fn get_time_millis()-> i64{
	let timespec = time::get_time();
	timespec.sec + timespec.nsec as i64 / 1000 / 1000
}
