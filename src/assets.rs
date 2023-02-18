use std::{fs::File, io::{Read, Write}};

use macroquad::{text::{Font, load_ttf_font_from_bytes}, texture::Texture2D, prelude::{GRAY, Color, Material, load_material, MaterialParams, UniformType}};
use nanoserde::{DeBin, SerBin};


#[derive(Clone)]
pub struct Assets {
	pub font: Font,
	pub gradient: Texture2D,
	pub persistance: Persistance,
	pub material: Material,
}

impl Assets {
	pub fn get() -> Self {
		Assets {
			font: load_ttf_font_from_bytes(crate::FONT).unwrap(),
			gradient: Texture2D::from_file_with_format(crate::GRADIENT, None),
			persistance: Persistance::load(),
			material: load_material(include_str!("vertex.vert"), include_str!("shaders/whisper.frag"), MaterialParams {
				uniforms: vec![
					("time".to_string(), UniformType::Float1),
					("resolution".to_string(), UniformType::Float2),
				],
				..Default::default()
			}).unwrap()
		}
	}
}

#[derive(Clone, SerBin, DeBin)]
pub struct Persistance {
	pub highscores: [Option<f32>; 10], // [map size/2 - 1] = time (2 - 20) -> (0 - 9)
	pub color0: [f32; 4],
	pub color1: [f32; 4],
	pub color2: [f32; 4],
	pub game_size: usize,
}

impl Persistance {
	// bool - is highscore, option - previous highscore
	pub fn insert_highscore(&mut self, size: usize, time: f32) -> (bool, Option<f32>) {
		if size > 20 {
			return (false, None);
		}

		if let Some(prev) = self.highscores[size/2 - 1] {
			if time < prev {
				self.highscores[size/2 - 1] = Some(time);
				self.save();
				return (true, Some(prev));
			}
			return (false, None);
		}
		else {
			self.highscores[size/2 - 1] = Some(time);
			self.save();
			return (true, None);
		}
	}

	pub fn load() -> Self {
		let mut file = File::open("save");
		match &mut file {
			Ok(f) => {
				let mut vec = Vec::new();
				f.read_to_end(&mut vec).unwrap();
				Self::deserialize_bin(&vec).unwrap()
			},
			Err(_) => {
				Persistance {
					highscores: [None; 10],
					color0: GRAY.into(),
					color1: Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }.into(),
					color2: Color { r: 0.0, g: 0.5, b: 1.0, a: 1.0 }.into(),
					game_size: 4,
				}
			}
		}
	}

	pub fn save(&self) {
		let mut file = File::create("save").unwrap();
		file.write_all(self.serialize_bin().as_slice()).unwrap();
	}
}