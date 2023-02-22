use std::{fs::{File, self}, io::{Read, Write}, thread, sync::mpsc::{Receiver, Sender, channel}, time::Instant};
use pollster::FutureExt;

use macroquad::{prelude::*, miniquad::{BlendState, Equation, BlendFactor, BlendValue}, audio::{Sound, load_sound_from_bytes, play_sound, PlaySoundParams, set_sound_volume, play_sound_once}};
use nanoserde::{DeBin, SerBin};

use crate::{MUSIC, SFX, SFX_VOLUMES, MUSIC_LENGTHS, cell_state::CellState, state::GameMode, board::Board};


pub struct Assets {
	pub font: Font,
	pub gradient: Texture2D,
	pub lock: Texture2D,
	pub banner: Texture2D,
	pub persistance: Persistance,
	pub materials: Vec<Material>,
	pub material: usize,
	pub secondary_material: Option<(usize, f32)>, // id, time
	pub next_music_play: f32,
	pub music: Option<Vec<Sound>>, // None if not yet loaded
	pub sfx: Option<Vec<Sound>>,
	pub assets_receiver: Receiver<(Vec<Sound>, Vec<Sound>)>,

	pub receiver: Receiver<(Vec<Vec<CellState>>, usize, f32)>, // map, id, time
	pub sender: Sender<(usize, GameMode, usize)>,

	pub next_board_id: usize,
}

impl Assets {
	pub fn get() -> Self {
		let persistance = Persistance::load();
		// persistance.highscores = [Some(0.38), Some(3.08), Some(21.09), Some(63.50), Some(235.66), None, None, None, None, Some(2965.86)];
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


		// map, board_id
		let (map_sender, map_receiver) = channel::<(Vec<Vec<CellState>>, usize, f32)>();

		// map_size, game_mode, board_id
		let (order_sender, order_receiver) = channel::<(usize, GameMode, usize)>();

		thread::spawn(move || {
			loop {
				let (size, mode, id) = order_receiver.recv().unwrap();
				let start = Instant::now();
				let board = match mode {
					GameMode::Sandbox => Board::new(size, 0, false),
					GameMode::Learn => Board::new_learn(size, 0),
					GameMode::Serious => Board::new_serious(size, 0),
				};
				map_sender.send((board.map, id, start.elapsed().as_secs_f32())).unwrap();
			}
		});


		// music, sfx
		let (assets_sender, assets_receiver) = channel::<(Vec<Sound>, Vec<Sound>)>();

		thread::spawn(move || {
			let music: Vec<Sound> = MUSIC.iter().map(|b| load_sound_from_bytes(b).block_on().unwrap()).collect();
			let sfx: Vec<Sound> = SFX.iter().map(|b| load_sound_from_bytes(b).block_on().unwrap()).collect();

			assets_sender.send((music, sfx)).unwrap();
		});

		Assets {
			font: load_ttf_font_from_bytes(crate::FONT).unwrap(),
			gradient: Texture2D::from_file_with_format(crate::GRADIENT, None),
			lock: Texture2D::from_file_with_format(crate::LOCK, None),
			banner: Texture2D::from_file_with_format(crate::BANNER, None),
			persistance,
			material: rand::gen_range(0, materials.len()),
			materials,
			secondary_material: None,
			music: None,
			sfx: None,
			next_music_play: 0.0,
			sender: order_sender,
			receiver: map_receiver,
			next_board_id: 1,
			assets_receiver,
		}
	}

	pub fn try_play_music(&mut self) {
		if let Some(music) = &self.music {
			if get_time() as f32 > self.next_music_play {
				let index = rand::gen_range(0, music.len());
				play_sound_once(music[index]);
				set_sound_volume(music[index], self.persistance.music_volume);
				self.next_music_play = get_time() as f32 + MUSIC_LENGTHS[index];
			}
		}
		else {
			if let Ok((music, sfx)) = self.assets_receiver.try_recv() {
				// let index = rand::gen_range(0, music.len());
				// play_sound_once(music[index]);
				// set_sound_volume(music[index], self.persistance.master_volume);

				self.music = Some(music);
				self.sfx = Some(sfx);
			}

		}
	}

	pub fn play_sound(&self, id: usize) {
		if let Some(sfx) = &self.sfx {
			play_sound(sfx[id], PlaySoundParams { looped: false, volume: SFX_VOLUMES[id] * self.persistance.sfx_volume });
		}
	}

	pub fn update_volume(&mut self) {
		if let Some(music) = &mut self.music {
			for m in music.iter_mut() {
				set_sound_volume(*m, self.persistance.music_volume);
			}
		}
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
	pub music_volume: f32,
	pub sfx_volume: f32,
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
				match Self::deserialize_bin(&vec) {
					Ok(data) => data,
					Err(_) => Persistance {
						highscores: [None; 10],
						color0: DARKGRAY.into(),
						color1: Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }.into(),
						color2: Color { r: 0.0, g: 0.5, b: 1.0, a: 1.0 }.into(),
						game_size: 4,
						music_volume: 1.0,
						sfx_volume: 1.0,
					}
				}
			},
			Err(_) => {
				Persistance {
					highscores: [None; 10],
					color0: DARKGRAY.into(),
					color1: Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }.into(),
					color2: Color { r: 0.0, g: 0.5, b: 1.0, a: 1.0 }.into(),
					game_size: 4,
					music_volume: 1.0,
					sfx_volume: 1.0,
				}
			}
		}
	}

	pub fn save(&self) {
		let mut file = File::create("save").unwrap();
		file.write_all(self.serialize_bin().as_slice()).unwrap();
	}
}