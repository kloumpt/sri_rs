pub mod image_indexing;
pub mod sound_indexing;
pub mod text_indexing;


use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


use includes::image_types::ImageDescriptor;
use includes::sound_types::SoundDescriptor;
use includes::text_types::TextDescriptor;

use includes::context_types::ContextObject;
impl ContextObject{
	pub fn start_indexing(&mut self, documents_list_file: &str){

		match File::open(documents_list_file){
			Ok(file)=>{
				let file = BufReader::new(&file);
				for line in file.lines() {
					let l = line.unwrap();
					match l.rsplit('.').next().unwrap(){
						"txt" | "TXT"=>{
							// Image file
							println!("Indexing image: {}", l);

							match File::open(l.clone()){
								Ok(image_file)=>{
									match ImageDescriptor::from_image_file(self, image_file){
										Ok(descriptor)=>{
											self.add_image_association(String::from(descriptor.get_id()), l);
											self.add_image_descriptor(descriptor);

										},
										Err(e)=>println!("{:?}", e)
									}
								},
								Err(e)=>println!("{} {}", e, l)
							}
							()
						},
						"bin" | "BIN"=>{
							// Sound file
							println!("Indexing sound: {}", l);

							match File::open(l.clone()){
								Ok(image_file)=>{
									match SoundDescriptor::from_sound_file(self, image_file){
										Ok(descriptor)=>{
											self.add_sound_association(String::from(descriptor.get_id()), l);
											self.add_sound_descriptor(descriptor);

										},
										Err(e)=>println!("{:?}", e)
									}
								},
								Err(e)=>println!("{} {}", e, l)
							}
							()
						},
						"xml" | "XML"=>{
							// Text file
							println!("Indexing text: {}", l);

							match File::open(l.clone()){
								Ok(image_file)=>{
									match TextDescriptor::from_text_file(self, image_file){
										Ok(descriptor)=>{
											self.add_text_association(String::from(descriptor.get_id()), l);
											self.add_text_descriptor(descriptor);

										},
										Err(e)=>println!("{:?}", e)
									}
								},
								Err(e)=>println!("{} {}", e, l)
							}
							()
						},
						_=>()//println!("---{}---", l)
					}
				}
			},
			Err(e)=>panic!("{}", e)
		}
	}
}
