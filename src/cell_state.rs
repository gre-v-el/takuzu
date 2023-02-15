use egui_macroquad::macroquad::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum CellState {
	None,
	True(bool),
	False(bool),
}

impl CellState {
	pub fn from_bool(b: bool) -> Self {
		if b {
			CellState::True(false)
		}
		else {
			CellState::False(false)
		}
	}
	pub fn next(&self) -> Self {
		match self {
			CellState::None => CellState::False(false),
			CellState::False(false) => CellState::True(false),
			CellState::True(false) => CellState::None,
			rest => *rest
		}
	}

	pub fn prev(&self) -> Self {
		match self {
			CellState::None => CellState::True(false),
			CellState::False(false) => CellState::None,
			CellState::True(false) => CellState::False(false),
			rest => *rest
		}
	}

	pub fn inverse(&self) -> Self {
		match self {
			CellState::None => CellState::None,
			CellState::False(false) => CellState::True(false),
			CellState::True(false) => CellState::False(false),
			rest => *rest
		}
	}

	pub fn col(&self) -> Color {
		match self {
			CellState::None => GRAY,
			CellState::False(false) => BLUE,
			CellState::True(false) => YELLOW,
			CellState::False(true) => DARKBLUE,
			CellState::True(true) => ORANGE,
		}
	}
}

// ignore the locked part
impl PartialEq for CellState {
	fn eq(&self, other: &Self) -> bool {
		match self {
			CellState::None => {
				match other {
					CellState::None => true,
					_ => false
				}
			},
			CellState::True(_) => {
				match other {
					CellState::True(_) => true,
					_ => false
				}
			},
			CellState::False(_) => {
				match other {
					CellState::False(_) => true,
					_ => false
				}
			}
		}
	}
}