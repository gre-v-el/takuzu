use std::{f32::consts::PI};

use macroquad::prelude::*;
use crate::{cell_state::CellState, ui::draw_round_rect, assets::Assets, POP, LOCKED, HINT, ERROR, col_lerp};

#[derive(Clone)]
pub struct Board {
	pub id: usize, // for determining if a board returned by the generator thred regards this exact board
	pub is_won: bool,
	pub is_valid: bool,
	pub is_generating: bool,
	pub size: usize,
	pub map: Vec<Vec<CellState>>,
	pub error: [Option<(usize, usize, usize, usize)>; 2], // up to two regions on the board
	pub error_time: f32,
	pub hint: Option<(usize, usize)>,
	pub show_locked: Option<f32>,
	pub last_error_sound: f32,
}

impl Board {
	pub fn new(size: usize, id: usize, will_generate: bool) -> Self {
		let s = Self {
			is_won: false,
			is_valid: true,
			is_generating: will_generate,
			size,
			map: vec![vec![CellState::None; size]; size],
			error: [None; 2],
			error_time: 0.0,
			hint: None,
			show_locked: None,
			last_error_sound: -1.0,
			id: id,
		};

		s
	}

	pub fn new_learn(size: usize, id: usize) -> Self {
		let mut board = Board::new(size, id, true);
		board.generate_valid();
		board.degenerate();
		board.purge_redundancies();
		board.lock_tiles();
		board
	}

	pub fn new_serious(size: usize, id: usize) -> Self {
		let mut board = Board::new(size, id, true);
		board.generate_valid();
		board.purge_redundancies();
		board.lock_tiles();
		board
	}

	pub fn handle_mouse(&mut self, camera: &Camera2D, assets: &Assets) {
		if !is_mouse_button_pressed(MouseButton::Left) && !is_mouse_button_pressed(MouseButton::Right) || self.is_generating {
			return;
		}

		self.hint = None;

		let (x, y) = (camera.screen_to_world(mouse_position().into()) * self.size as f32).into();

		if x < 0.0 || y < 0.0  || x >= self.size as f32 || y >= self.size as f32 {
			return;
		}

		let (x, y) = (x as usize, y as usize);

		if self.map[y][x].is_locked() {
			self.show_locked = Some(get_time() as f32);
			assets.play_sound(LOCKED);
			return;
		}

		if is_mouse_button_down(MouseButton::Left) {
			self.map[y][x] = self.map[y][x].next();
			assets.play_sound(POP);
		}
		else {
			self.map[y][x] = self.map[y][x].prev();
			assets.play_sound(POP);
		}

		self.verify_board();
	}

	pub fn has_nones(&self) -> bool {
		for row in &self.map {
			for cell in row {
				if *cell == CellState::None {
					return true;
				}
			}
		}
		false
	}

	pub fn count_nones(&self) -> u32 {
		let mut counter = 0;
		for row in &self.map {
			for cell in row {
				if *cell == CellState::None {
					counter += 1;
				}
			}
		}
		counter
	}

	pub fn generate_hint(&mut self, assets: &Assets) {

		let mut vec = Vec::new();
		let mut clone = self.clone();
		
		let mut actions: Vec<Box<dyn Fn(&mut Board) -> bool>> = vec![
			Box::new(|board| {board.surround_doubles_axis(|v, y, x| v[y][x], |v, y, x, s| v[y][x] = s)}),
			Box::new(|board| {board.surround_doubles_axis(|v, x, y| v[y][x], |v, x, y, s| v[y][x] = s)}),
			Box::new(|board| {board.separate_triples_axis(|v, y, x| v[y][x], |v, y, x, s| v[y][x] = s)}),
			Box::new(|board| {board.separate_triples_axis(|v, x, y| v[y][x], |v, x, y, s| v[y][x] = s)}),
			Box::new(|board| {board.fill_rows_axis(		  |v, y, x| v[y][x], |v, y, x, s| v[y][x] = s)}),
			Box::new(|board| {board.fill_rows_axis(		  |v, x, y| v[y][x], |v, x, y, s| v[y][x] = s)})
		];

		use rand::ChooseRandom;
		actions.shuffle();

		let mut completed = false;
		for action in actions.iter() {
			if action(&mut clone) {
				completed = true;
				break;
			}
		}

		if completed {
			for y in 0..self.size {
				for x in 0..self.size {
					if self.map[y][x] == CellState::None && clone.map[y][x] != CellState::None {
						vec.push((x, y));
					}
				}
			}
		}
		

		if vec.len() != 0 {
			self.hint = Some(*vec.choose().unwrap());
			assets.play_sound(HINT);
		}
	}

	pub fn lock_tiles(&mut self) {
		for y in 0..self.size {
			for x in 0..self.size {
				self.map[y][x] = match self.map[y][x] {
					CellState::False(false) => CellState::False(true),
					CellState::True(false) => CellState::True(true),
					rest => rest
				}
			}
		}
	}

	pub fn verify_board(&mut self) {
		let mut resp1 = self.get_errors_axis(|v, x, y| v[y][x]);
		let resp2 = self.get_errors_axis(|v, y, x| v[y][x]);

		self.is_valid = resp1[0].is_none() && resp2[0].is_none();

		if resp1[0].is_some() {
			let err = resp1[0].unwrap();
			let err = (err.1, err.0, err.3, err.2);
			resp1[0] = Some(err);

			if let Some(err) = resp1[1] {
				let err = (err.1, err.0, err.3, err.2);
				resp1[1] = Some(err);
			}

			self.error = resp1;
			self.error_time = get_time() as f32;
		}
		else if resp2[0].is_some() {
			self.error = resp2;
			self.error_time = get_time() as f32;
		}
		else {
			self.error = [None; 2];
		}

		self.is_won = !self.has_nones() && self.is_valid;
	}

	// returns up to two errors
	pub fn get_errors_axis<F : Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState>(&mut self, get: F) -> [Option<(usize, usize, usize, usize)>; 2] { 
		for c1 in 0..self.size {
			let mut trues = 0;
			let mut falses = 0;

			let mut counter = 1;
			let mut state = CellState::None;
			for c2 in 0..self.size {

				match get(&self.map, c1, c2) {
					CellState::False(_) => falses += 1,
					CellState::True(_) => trues += 1,
					CellState::None => {},
				};

				if get(&self.map, c1, c2) == state {
					counter += 1;
				}
				else {
					counter = 1;
					state = get(&self.map, c1, c2);
				}

				if counter > 2 && state != CellState::None {
					return [Some((c2-2, c1, 3, 1)), None];
				}
			}

			if falses > self.size/2 || trues > self.size/2 {
				return [Some((0, c1, self.size, 1)), None];
			}
		}

		for c1_1 in 0..self.size {
			for c1_2 in c1_1+1..self.size {
				let mut are_same = true;
				let mut any_nones = false;
				for c2 in 0..self.size {
					are_same &= get(&self.map, c1_1, c2) == get(&self.map, c1_2, c2);
					any_nones |= get(&self.map, c1_1, c2) == CellState::None || get(&self.map, c1_2, c2) == CellState::None;
				}

				if are_same && !any_nones {
					return [Some((0, c1_1, self.size, 1)), Some((0, c1_2, self.size, 1))];
				}
			}
		}

		return [None; 2];
	}

	pub fn draw(&mut self, assets: &Assets) {
		let m = 0.05 / self.size as f32;
		let b = 0.1 / self.size as f32;
		let w = 1.0 / self.size as f32;
		for (y, row) in self.map.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				let color = 
					if self.is_generating {
						let angle = (y as f32 - self.size as f32 / 2.0).atan2(x as f32 - self.size as f32 / 2.0);
						let col = (angle / 2.0 / PI + get_time() as f32 * 0.3) % 1.0;

						let col = col_lerp(assets.persistance.color2.into(), assets.persistance.color1.into(), col);

						col
					} 
					else {cell.col(assets)};
				let x = x as f32 / self.size as f32;
				let y = y as f32 / self.size as f32;
				draw_round_rect(x + m, y + m, w - 2.0*m, w - 2.0*m, b, color);
			}
		}
		
		if let Some(t) = self.show_locked {
			let passed = get_time() as f32 - t;
			if passed > 2.0 {
				self.show_locked = None;
			}

			for y in 0..self.size {
				for x in 0..self.size {
					if self.map[y][x].is_locked() {
						let col = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 - 0.5*(2.0*PI*passed).cos() };
						let x = x as f32 / self.size as f32;
						let y = y as f32 / self.size as f32;
						// draw_circle(x + w*0.5, y + w*0.5, w*0.2, col);
						draw_texture_ex(assets.lock, x+m, y+m, col, DrawTextureParams { 
							dest_size: Some(vec2(w-2.0*m, w-2.0*m)), 
							source: None, 
							rotation: 0.0, 
							flip_x: false, 
							flip_y: false, 
							pivot: None 
						});
					}
				}
			}
		}
	}

	pub fn draw_hint(&self) {
		if let Some((x, y)) = self.hint {
			let m = 0.05 / self.size as f32;
			let b = 0.13 / self.size as f32;
			draw_round_rect(
				x as f32 / self.size as f32 - m, 
				y as f32 / self.size as f32 - m, 
				1.0 / self.size as f32 + 2.0*m, 
				1.0 as f32 / self.size as f32 + 2.0*m, 
				b, 
				WHITE
			);
		}
	}

	pub fn draw_errors(&mut self, assets: Option<&Assets>) {
		if let Some(e) = self.error[0] {
			self.draw_error(&e, assets);
		}
		if let Some(e) = self.error[1] {
			self.draw_error(&e, assets);
		}
	}

	pub fn get_error_alpha(&self) -> f32 {
		let t = (get_time() as f32 - self.error_time - 0.3).max(0.0);
		1.0-((5.0*t).cos() * 0.5 + 0.5)
	}

	fn draw_error(&mut self, e: &(usize, usize, usize, usize), assets: Option<&Assets>) {
		let m = 0.05 / self.size as f32;
		let b = 0.13 / self.size as f32;
		let alpha = self.get_error_alpha();
		draw_round_rect(
			e.0 as f32 / self.size as f32 - m, 
			e.1 as f32 / self.size as f32 - m, 
			e.2 as f32 / self.size as f32 + 2.0*m, 
			e.3 as f32 / self.size as f32 + 2.0*m, 
			b, 
			Color { r: 1.0, g: 0.0, b: 0.0, a: alpha }
		);

		if let Some(assets) = assets {
			if alpha > 0.3 && get_time() as f32 - self.last_error_sound > 1.0 {
				self.last_error_sound = get_time() as f32;
				assets.play_sound(ERROR);
			}
		}
	}

	pub fn generate_valid(&mut self) -> u32 {
		let mut i = 1;
		self.generate();
		while !self.is_valid {
			self.reset();
			self.generate();
			i += 1;
		}
		
		i
	}

	pub fn generate(&mut self) {
		while self.has_nones() {
			while self.fill_rows() | self.surround_doubles() | self.separate_triples() {}
			self.insert_random();
		}
		self.verify_board();
	}

	pub fn generate_fraction(&mut self, fract: f32) {
		while (self.count_nones() as f32 / (self.size * self.size) as f32) > fract {
			while self.fill_rows() | self.surround_doubles() | self.separate_triples() {}
			self.insert_random();
		}
		self.verify_board();
	}

	pub fn reset(&mut self) {
		self.map = vec![vec![CellState::None; self.size]; self.size];
		self.verify_board();
	}

	pub fn insert_random(&mut self) {
		let nones = self.count_nones() as i32;

		if nones == 0 {
			return;
		}

		let mut index = rand::gen_range(0, nones);
		let mut x = 0; 
		let mut y = 0;
		loop {
			if self.map[y][x] == CellState::None {
				index -= 1;
				if index <= 0 {
					break;
				}
			}
			x += 1;
			if x >= self.size {
				x = 0;
				y += 1;
			}
			
		}

		self.map[y][x] = CellState::from_bool(rand::gen_range(0, 2) == 0);
	}

	pub fn surround_doubles(&mut self) -> bool {
		let r = self.surround_doubles_axis(
			|v, y, x| v[y][x], 
			|v, y, x, s| v[y][x] = s) |
		self.surround_doubles_axis(
			|v, x, y| v[y][x], 
			|v, x, y, s| v[y][x] = s);

		r
	}

	pub fn surround_doubles_axis<
		F: Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState,
		G: Fn(&mut Vec<Vec<CellState>>, usize, usize, CellState) -> ()
	>(&mut self, get: F, set: G) -> bool {
		let mut changed = false;

		for c1 in 0..self.size {
			let mut last_state = CellState::None;

			for c2 in 0..self.size {
				if last_state == get(&self.map, c1, c2) && last_state != CellState::None{
					if c2 as i32 - 2 >= 0 && get(&self.map, c1, c2 - 2) == CellState::None {
						set(&mut self.map, c1, c2 - 2, last_state.inverse());
						changed = true;
					}
					if c2 + 1 < self.size && get(&self.map, c1, c2 + 1) == CellState::None  {
						set(&mut self.map, c1, c2 + 1, last_state.inverse());
						changed = true;
					}
				}
				last_state = get(&self.map, c1, c2);
			}
		}

		changed
	}

	pub fn separate_triples(&mut self) -> bool {
		let r = self.separate_triples_axis(
			|v, y, x| v[y][x], 
			|v, y, x, s| v[y][x] = s) |
		self.separate_triples_axis(
			|v, x, y| v[y][x], 
			|v, x, y, s| v[y][x] = s);

		r
	}

	pub fn separate_triples_axis<
		F: Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState,
		G: Fn(&mut Vec<Vec<CellState>>, usize, usize, CellState) -> ()
	>(&mut self, get: F, set: G) -> bool {

		let mut changed = false;

		for c1 in 0..self.size {
			let mut last_state = CellState::None;
			let mut last_last_state = CellState::None;

			for c2 in 0..self.size {
				if last_last_state == get(&self.map, c1, c2) && last_last_state != CellState::None && last_state == CellState::None {
					set(&mut self.map, c1, c2 - 1, last_last_state.inverse());
					changed = true;
				}
				last_last_state = last_state;
				last_state = get(&self.map, c1, c2);
			}
		}

		changed
	}

	

	pub fn fill_rows(&mut self) -> bool {
		let r = self.fill_rows_axis(
			|v, y, x| v[y][x], 
			|v, y, x, s| v[y][x] = s) |
		self.fill_rows_axis(
			|v, x, y| v[y][x], 
			|v, x, y, s| v[y][x] = s);

		r
	}

	pub fn fill_rows_axis<
		F: Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState,
		G: Fn(&mut Vec<Vec<CellState>>, usize, usize, CellState) -> ()
	>(&mut self, get: F, set: G) -> bool {

		let mut changed = false;

		for c1 in 0..self.size {
			let mut nones = 0;
			let mut trues = 0;
			let mut falses = 0;

			for c2 in 0..self.size {
				match get(&self.map, c1, c2) {
					CellState::False(_) => falses += 1,
					CellState::True(_) => trues += 1,
					CellState::None => nones += 1,
				}
			}

			if trues == self.size/2 && nones != 0 {
				for c2 in 0..self.size {
					if get(&self.map, c1, c2) == CellState::None {
						set(&mut self.map, c1, c2, CellState::False(false));
					}
				}
				changed = true;
			}
			if falses == self.size/2 && nones != 0 {
				for c2 in 0..self.size {
					if get(&self.map, c1, c2) == CellState::None {
						set(&mut self.map, c1, c2, CellState::True(false));
					}
				}
				changed = true;
			}
		}

		changed
	}

	
	pub fn degenerate(&mut self) {
		self.deseparate_triples(0.2);
		self.desurround_doubles(0.2);
		self.defill_rows(0.2);
		self.defill_rows(1.0);
		self.desurround_doubles(0.4);
		self.deseparate_triples(0.5);
		self.desurround_doubles(1.0);
		self.deseparate_triples(1.0);


		self.verify_board();
	}

	pub fn is_solvable(&self) -> bool {
		let mut clone = self.clone();

		while clone.surround_doubles() | clone.separate_triples() | clone.fill_rows() {}

		clone.verify_board();

		clone.is_won
	}

	pub fn delete_percentage(&mut self, percentage: f32) {
		for y in 0..self.size {
			for x in 0..self.size {
				if rand::gen_range(0.0, 1.0) < percentage {
					self.map[y][x] = CellState::None;
				}
			}
		}
	}

	pub fn copy_nones(&mut self, other: &Self) {
		if other.size != self.size {return;}

		for y in 0..self.size {
			for x in 0..self.size {
				if other.map[y][x] == CellState::None {
					self.map[y][x] = CellState::None;
				}
			}
		}
	}

	pub fn deseparate_triples(&mut self, percentage: f32) -> bool {
		let r = self.deseparate_triples_axis(
			|v, y, x| v[y][x], 
			|v, y, x, s| v[y][x] = s, percentage) |
		self.deseparate_triples_axis(
			|v, x, y| v[y][x], 
			|v, x, y, s| v[y][x] = s, percentage);

		r
	}

	pub fn deseparate_triples_axis<
		F: Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState,
		G: Fn(&mut Vec<Vec<CellState>>, usize, usize, CellState) -> ()
	>(&mut self, get: F, set: G, percentage: f32) -> bool {
		let mut changed = false;

		for c1 in 0..self.size {
			let mut last_state = CellState::None;
			let mut last_last_state = CellState::None;

			for c2 in 0..self.size {
				if last_last_state == get(&self.map, c1, c2) && last_last_state != CellState::None && last_state != CellState::None && rand::gen_range(0.0, 1.0) < percentage {
					set(&mut self.map, c1, c2 - 1, CellState::None);
					last_state = CellState::None;
					changed = true;
				}
				last_last_state = last_state;
				last_state = get(&self.map, c1, c2);
			}
		}

		return changed;
	}


	
	pub fn defill_rows(&mut self, percentage: f32) -> bool {
		let r = self.defill_row_axis(
			|v, y, x| v[y][x], 
			|v, y, x, s| v[y][x] = s, percentage) |
		self.defill_row_axis(
			|v, x, y| v[y][x], 
			|v, x, y, s| v[y][x] = s, percentage);

		r
	}

	pub fn defill_row_axis<
		F: Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState,
		G: Fn(&mut Vec<Vec<CellState>>, usize, usize, CellState) -> ()
	>(&mut self, get: F, set: G, percentage: f32) -> bool {

		let mut changed = false;

		for c1 in 0..self.size {
			let mut trues = 0;
			let mut falses = 0;

			for c2 in 0..self.size {
				match get(&self.map, c1, c2) {
					CellState::False(_) => falses += 1,
					CellState::True(_) => trues += 1,
					CellState::None => {},
				}
			}

			if trues == self.size/2 && falses == self.size/2 && rand::gen_range(0.0, 1.0) < percentage {
				let to_delete = CellState::from_bool(rand::gen_range(0, 2) == 0);
				for c2 in 0..self.size {
					if get(&self.map, c1, c2) == to_delete {
						set(&mut self.map, c1, c2, CellState::None);
					}
				}
				changed = true;
			}
		}

		changed
	}

	pub fn desurround_doubles(&mut self, percentage: f32) -> bool {
		let r = self.desurround_doubles_axis(
			|v, y, x| v[y][x], 
			|v, y, x, s| v[y][x] = s, percentage) |
		self.desurround_doubles_axis(
			|v, x, y| v[y][x], 
			|v, x, y, s| v[y][x] = s, percentage);

		r
	}

	pub fn desurround_doubles_axis<
		F: Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState,
		G: Fn(&mut Vec<Vec<CellState>>, usize, usize, CellState) -> ()
	>(&mut self, get: F, set: G, percentage: f32) -> bool {
		let mut changed = false;

		for c1 in 0..self.size {
			let mut last_state = CellState::None;

			for c2 in 0..self.size {
				if last_state == get(&self.map, c1, c2) && last_state != CellState::None && rand::gen_range(0.0, 1.0) < percentage {
					if c2 as i32 - 2 >= 0 {
						set(&mut self.map, c1, c2 - 2, CellState::None);
						changed = true;
					}
					if c2 + 1 < self.size  {
						set(&mut self.map, c1, c2 + 1, CellState::None);
						changed = true;
					}
				}
				last_state = get(&self.map, c1, c2);
			}
		}

		changed
	}

	pub fn purge_redundancies(&mut self) {
		while self.delete_one() {}
		self.verify_board();
	}
	
	pub fn delete_one(&mut self) -> bool {
		let mut to_delete = Vec::new();

		for y in 0..self.size {
			for x in 0..self.size {
				if self.map[y][x] != CellState::None {
					let temp = self.map[y][x];

					self.map[y][x] = CellState::None;

					if self.is_solvable() {
						to_delete.push((x, y));
					}

					self.map[y][x] = temp;
				}
			}
		}

		if to_delete.len() == 0 {
			return false;
		}

		use macroquad::rand::ChooseRandom;
		let coords = to_delete.choose().unwrap();

		self.map[coords.1][coords.0] = CellState::None;

		true
	}
}
