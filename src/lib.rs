use std::collections::BTreeMap;

use macroquad::{text::Font, texture::Texture2D, prelude::Color};

pub mod board;
pub mod cell_state;
pub mod state;
pub mod utils;

pub const FONT: &[u8] = include_bytes!("../assets/Jellee-Bold.ttf");
pub const GRADIENT: &[u8] = include_bytes!("../assets/gradient.png");

#[derive(Clone)]
pub struct Assets {
	pub font: Font,
	pub gradient: Texture2D,
	pub persistance: Persistance,
}

#[derive(Clone)]
pub struct Persistance {
	pub highscores: BTreeMap<usize, f32>, // map size, time
	pub color0: Color,
	pub color1: Color,
	pub color2: Color,
}

impl Persistance {
	// bool - is highscore, option - previous highscore
	pub fn insert_highscore(&mut self, size: usize, time: f32) -> (bool, Option<f32>) {
		if let Some(prev) = self.highscores.get(&size) {
			if time < *prev {
				return (true, self.highscores.insert(size, time));
			}
			return (false, None);
		}
		else {
			self.highscores.insert(size, time);
			return (true, None);
		}
	}
}