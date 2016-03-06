
use std::collections::HashMap;

use std::io::BufReader;
use std::fs::File;


use xml::reader::{EventReader, XmlEvent};

use stemmer::Stemmer;

use includes::text_types::*;
use super::super::context_types::ContextObject;
// Extrait de http://snowball.tartarus.org/algorithms/french/stop.txt
static STOP_WORDS: &'static [&'static str] = &["au", "aux", "avec", "ce", "ces", "dans", "de", "des", "du", "elle", "en", "et", "eux", "il", "je", "la", "le", "leur", "lui", "ma", "mais", "me", "même", "mes", "moi", "mon", "ne", "nos", "notre", "nous", "on", "ou", "par", "pas", "pour", "qu", "que", "qui", "sa", "se", "ses", "son", "sur", "ta", "te", "tes", "toi", "ton", "tu", "un", "une", "vos", "votre", "vous", "c", "d", "j", "l", "à", "m", "n", "s", "t", "y", "été", "étée", "étées", "étés", "étant", "suis", "es", "est", "sommes", "êtes", "sont", "serai", "seras", "sera", "serons", "serez", "seront", "serais", "serait", "serions", "seriez", "seraient", "étais", "était", "étions", "étiez", "étaient", "fus", "fut", "fûmes", "fûtes", "furent", "sois", "soit", "soyons", "soyez", "soient", "fusse", "fusses", "fût", "fussions", "fussiez", "fussent", "ayant", "eu", "eue", "eues", "eus", "ai", "as", "avons", "avez", "ont", "aurai", "auras", "aura", "aurons", "aurez", "auront", "aurais", "aurait", "aurions", "auriez", "auraient", "avais", "avait", "avions", "aviez", "avaient", "eut", "eûmes", "eûtes", "eurent", "aie", "aies", "ait", "ayons", "ayez", "aient", "eusse", "eusses", "eût", "eussions", "eussiez", "eussent", "ceci", "cela", "celà", "cet", "cette", "ici", "ils", "les", "leurs", "quel", "quels", "quelle", "quelles", "sans", "soi"];

impl TextDescriptor {
	pub fn from_text_file(context: &mut ContextObject, text_file: File) -> Result<TextDescriptor, String> {
		let mut stemmer = Stemmer::new("french").unwrap();

		let mut occurences: HashMap<String, i32> = HashMap::new();


		let file_reader = BufReader::new(&text_file);

		let mut word_in_file = 0;
		let mut word_filtered = 0;
		let mut word_in_descriptor = 0;


		let parser = EventReader::new(file_reader);
		let mut in_phrase = false;
		for e in parser {
			match e {
				Ok(event) => {
					match event {
						XmlEvent::StartDocument {version, encoding, standalone} => {
							if encoding.to_lowercase().contains("iso") {
								println!("Warning : encoding not supported, trying to index anyway ({})", encoding);
							}
						},
						XmlEvent::StartElement { name, .. } => {
							if name.local_name == "phrase" {
								in_phrase = true;
							}
						},
						XmlEvent::EndElement { name } => {
							if name.local_name == "phrase" {
								in_phrase = false;
							}
						},
						XmlEvent::Characters(phrase) => {
							if in_phrase {
								for word in clean_string(phrase.to_lowercase()).split_whitespace() {
									if word.len() > SIZE_WORD_RM {
										let mut use_word = true;
										for stop_word in STOP_WORDS {
											if &word == stop_word {
												use_word = false;
												break;
											}
										}
										if use_word {
											let word_stemmed = stemmer.stem(word);
											let word_occurence = occurences.entry(word_stemmed).or_insert(0);

											*word_occurence += 1;
										}
									}
									word_in_file += 1;
								}
							}
						},
						_ => {},
					}
				},
				Err(e) => {
					return Err(format!("Error in parsing xml text {}", e));
				},
			}
		}


		let mut values: Vec<(String, i32)> = occurences.into_iter().collect();
		values.sort_by(|&(_, v_a), &(_, v_b)| v_b.cmp(&v_a));
		occurences = HashMap::new();

		for (word, occurence) in values {
			if word_in_descriptor < NB_TERM_MAX {
				occurences.insert(word, occurence);
				word_filtered += occurence;
				word_in_descriptor += 1;

			} else {
				break;
			}
		}


		Ok(TextDescriptor::from_occurences(context.gen_id(String::from("txt")), word_in_file, word_filtered as usize, occurences))
	}
}


fn clean_string(value: String) -> String {
	value.replace(".", " ")
	     .replace(",", " ")
	     .replace("/", " ")
	     .replace("#", " ")
	     .replace("!", " ")
	     .replace("$", " ")
	     .replace("%", " ")
	     .replace("\\", " ")
	     .replace("^", " ")
	     .replace("&", " ")
	     .replace("*", " ")
	     .replace(";", " ")
	     .replace(":", " ")
	     .replace("{", " ")
	     .replace("}", " ")
	     .replace("=", " ")
	     .replace("_", " ")
	     .replace("'", " ")
	     .replace("\"", " ")
	     .replace("~", " ")
	     .replace("(", " ")
	     .replace(")", " ")
	     .replace("-", " ")

}
