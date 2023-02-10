use macroquad::prelude::*;
use crate::cell_state::CellState;

pub struct GameState {
	pub is_won: bool,
	pub is_valid: bool,
	pub size: usize,
	pub map: Vec<Vec<CellState>>
}

impl GameState {
	pub fn new(size: usize) -> Self {
		let s = Self {
			is_won: false,
			is_valid: true,
			size,
			map: vec![vec![CellState::None; size]; size]
		};

		// s.generate();

		s
	}

	pub fn camera(&self) -> Camera2D {
		Camera2D {
			target: Vec2::splat(self.size as f32 * 0.5),
			zoom: vec2(1.0 / self.size as f32, 1.0 / self.size as f32 * screen_width() / screen_height()),
			..Default::default()
		}
	}

	pub fn handle_mouse(&mut self) {
		if !is_mouse_button_pressed(MouseButton::Left) && !is_mouse_button_pressed(MouseButton::Right) {
			return;
		}

		let (x, y) = self.camera().screen_to_world(mouse_position().into()).into();

		if x < 0.0 || y < 0.0  || x >= self.size as f32 || y >= self.size as f32 {
			return;
		}

		let (x, y) = (x as usize, y as usize);

		if is_mouse_button_down(MouseButton::Left) {
			self.map[y][x] = self.map[y][x].next();
		}
		else {
			self.map[y][x] = self.map[y][x].prev();
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

	pub fn verify_board(&mut self) {
		self.is_valid = 
			self.verify_board_axis(|v, x, y| v[y][x]) &&
			self.verify_board_axis(|v, y, x| v[y][x]);
		self.is_won = !self.has_nones() && self.is_valid;
	}

	pub fn verify_board_axis<F : Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState>(&mut self, get: F) -> bool { 
		// check triplets and counts
		for c1 in 0..self.size {
			let mut trues = 0;
			let mut falses = 0;
			let mut nones = 0;

			let mut counter = 1;
			let mut state = CellState::None;
			for c2 in 0..self.size {

				match get(&self.map, c1, c2) {
					CellState::False => falses += 1,
					CellState::True => trues += 1,
					CellState::None => nones += 1,
				};

				if get(&self.map, c1, c2) == state {
					counter += 1;
				}
				else {
					counter = 1;
					state = get(&self.map, c1, c2);
				}

				if counter > 2 && state != CellState::None {
					return false;
				}
			}

			if nones == 0 && falses != trues {
				return false;
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
					return false;
				}
			}
		}

		return true;
	}

	pub fn draw(&self) {
		let m = 0.02;
		for (y, row) in self.map.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				draw_rectangle(x as f32 + m, y as f32 + m, 1.0 - 2.0 * m, 1.0 - 2.0 * m, cell.col());
			}
		}
	}

	pub fn generate(&mut self) {
		while self.has_nones() {
			while self.surround_doubles() | self.separate_triples() {}
			self.insert_random();
		}
		self.verify_board();
	}

	pub fn reset(&mut self) {
		self.map = vec![vec![CellState::None; self.size]; self.size];
	}

	pub fn insert_random(&mut self) {
		let mut nones = 0;
		for y in 0..self.size {
			for x in 0..self.size {
				if self.map[y][x] == CellState::None {
					nones += 1;
				}
			}
		}

		if nones == 0 {
			return;
		}

		let mut index = rand::gen_range(0, nones);
		let mut x = 0; 
		let mut y = 0;
		loop {
			if self.map[y][x] == CellState::None {
				index -= 1;
				if index < 0 {
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
		
		self.verify_board();

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
		
		self.verify_board();

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
				if last_last_state == get(&self.map, c1, c2)  && last_last_state != CellState::None && last_state == CellState::None {
					set(&mut self.map, c1, c2 - 1, last_last_state.inverse());
					changed = true;
				}
				last_last_state = last_state;
				last_state = get(&self.map, c1, c2);
			}
		}

		changed
	}

	

	pub fn fill_row(&mut self) -> bool {
		let r = self.fill_row_axis(
			|v, y, x| v[y][x], 
			|v, y, x, s| v[y][x] = s) |
		self.fill_row_axis(
			|v, x, y| v[y][x], 
			|v, x, y, s| v[y][x] = s);
		
		self.verify_board();

		r
	}

	pub fn fill_row_axis<
		F: Fn(&Vec<Vec<CellState>>, usize, usize) -> CellState,
		G: Fn(&mut Vec<Vec<CellState>>, usize, usize, CellState) -> ()
	>(&mut self, get: F, set: G) -> bool {

		let mut changed = false;

		for c1 in 0..self.size {
			let mut last_state = CellState::None;
			let mut last_last_state = CellState::None;

			for c2 in 0..self.size {
				if last_last_state == get(&self.map, c1, c2)  && last_last_state != CellState::None && last_state == CellState::None {
					set(&mut self.map, c1, c2 - 1, last_last_state.inverse());
					changed = true;
				}
				last_last_state = last_state;
				last_state = get(&self.map, c1, c2);
			}
		}

		changed
	}
}