use egui_macroquad::macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CellState {
	None,
	True,
	False,
}

impl CellState {
	pub fn from_bool(b: bool) -> Self {
		if b {
			CellState::True
		}
		else {
			CellState::False
		}
	}
	pub fn next(&self) -> Self {
		match self {
			CellState::None => CellState::False,
			CellState::False => CellState::True,
			CellState::True => CellState::None,
		}
	}

	pub fn prev(&self) -> Self {
		match self {
			CellState::None => CellState::True,
			CellState::False => CellState::None,
			CellState::True => CellState::False,
		}
	}

	pub fn inverse(&self) -> Self {
		match self {
			CellState::None => CellState::None,
			CellState::False => CellState::True,
			CellState::True => CellState::False,
		}
	}

	pub fn col(&self) -> Color {
		match self {
			CellState::None => GRAY,
			CellState::False => BLUE,
			CellState::True => YELLOW,
		}
	}
}
