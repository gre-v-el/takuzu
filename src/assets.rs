use std::{fs::{File, self}, io::{Read, Write}};
use pollster::FutureExt;

// use macroquad::{text::{Font, load_ttf_font_from_bytes}, texture::Texture2D, prelude::{Color, Material, load_material, MaterialParams, UniformType, DARKGRAY}, rand, time::get_time, window::{screen_width, screen_height}};
use macroquad::{prelude::*, miniquad::{BlendState, Equation, BlendFactor, BlendValue}, audio::{Sound, load_sound_from_bytes, play_sound, PlaySoundParams}};
use nanoserde::{DeBin, SerBin};

use crate::{MUSIC, SFX};


#[derive(Clone)]
pub struct Assets {
	pub font: Font,
	pub gradient: Texture2D,
	pub persistance: Persistance,
	pub materials: Vec<Material>,
	pub material: usize,
	pub secondary_material: Option<(usize, f32)>, // id, time
	pub music: Vec<Sound>,
	pub sfx: Vec<Sound>,
	pub sfx_volumes: Vec<f32>,
}

impl Assets {
	pub fn get() -> Self {
		let mut materials = Vec::new();
		let paths = fs::read_dir("src/shaders").unwrap();

		for path in paths {
			match path {
				Ok(entry) => {
					let frag = fs::read_to_string(entry.path()).unwrap();
					materials.push(load_material(
						include_str!("vertex.vert"), 
						frag.as_str(), 
						MaterialParams {
							uniforms: vec![
								("time".to_string(), UniformType::Float1),
								("resolution".to_string(), UniformType::Float2),
								("alpha".to_string(), UniformType::Float1),
							],
							pipeline_params: PipelineParams {
								alpha_blend: Some(BlendState::new(
									Equation::Add,
									BlendFactor::Value(BlendValue::SourceAlpha),
									BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
								)),
								color_blend: Some(BlendState::new(
									Equation::Add,
									BlendFactor::Value(BlendValue::SourceAlpha),
									BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
								)),
								..Default::default()
							},
							..Default::default()
						}).unwrap())
				}
				Err(_) => {}
			}
		}

		let music: Vec<Sound> = MUSIC.iter().map(|b| load_sound_from_bytes(b).block_on().unwrap()).collect();
		let sfx: Vec<Sound> = SFX.iter().map(|b| load_sound_from_bytes(b).block_on().unwrap()).collect();

		play_sound(music[0], PlaySoundParams { looped: false, volume: 1.0 });

		Assets {
			font: load_ttf_font_from_bytes(crate::FONT).unwrap(),
			gradient: Texture2D::from_file_with_format(crate::GRADIENT, None),
			persistance: Persistance::load(),
			material: rand::gen_range(0, materials.len()),
			materials,
			secondary_material: None,
			music,
			sfx,
			sfx_volumes: vec![0.5],
		}
	}

	pub fn play_sound(&self, id: usize) {
		play_sound(self.sfx[id], PlaySoundParams { looped: false, volume: self.sfx_volumes[id] });
	}

	pub fn material(&self) -> &Material {
		&self.materials[self.material]
	}

	pub fn change_material(&mut self) {
		if let Option::None = self.secondary_material {
			self.secondary_material = Some((self.material, get_time() as f32));
			self.material = rand::gen_range(0, self.materials.len());
		}
	}

	pub fn draw_material(&mut self) {

		set_camera(&Camera2D::from_display_rect(Rect{x: 0.0, y: 0.0, w: 1.0, h: 1.0}));

		self.material().set_uniform("resolution", (screen_width(), screen_height()));
		self.material().set_uniform("time", get_time() as f32);
		self.material().set_uniform("alpha", 1.0f32);

		gl_use_material(*self.material());
		draw_rectangle(0.0, 0.0, 1.0, 1.0, WHITE);

		
		if let Some((id, time)) = self.secondary_material {
			let a = 1.0 - (get_time() as f32 - time) / 5.0;
			self.materials[id].set_uniform("resolution", (screen_width(), screen_height()));
			self.materials[id].set_uniform("time", get_time() as f32);
			self.materials[id].set_uniform("alpha", a);


			gl_use_material(self.materials[id]);
			draw_rectangle(0.0, 0.0, 1.0, 1.0, WHITE);

			if a < 0.0 {
				self.secondary_material = None;
			}
		}

		
		gl_use_default_material();
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
					color0: DARKGRAY.into(),
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