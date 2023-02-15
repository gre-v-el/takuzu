use std::collections::HashMap;

use egui_macroquad::macroquad::{text::Font, texture::Texture2D, prelude::Color};

pub mod board;
pub mod cell_state;
pub mod state;
pub mod utils;

pub const FONT: &[u8] = include_bytes!("../assets/Jellee-Bold.ttf");
pub const GRADIENT: &[u8] = include_bytes!("../assets/gradient.png");


pub struct Assets {
	pub font: Font,
	pub gradient: Texture2D,
	pub persistance: Persistance,
}

pub struct Persistance {
	pub highscores: HashMap<usize, f32>, // map size, time
	pub color0: Color,
	pub color1: Color,
}

impl Persistance {
	pub fn insert_highscore(&mut self, size: usize, time: f32) -> bool {
		let mut r = false;

		if let Some(prev) = self.highscores.get(&size) {
			if time < *prev {
				r = true;
				self.highscores.insert(size, time);
			}
		}

		r
	}
}