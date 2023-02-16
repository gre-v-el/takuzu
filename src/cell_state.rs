use macroquad::prelude::*;

use crate::Assets;

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
			CellState::False(_) => CellState::True(false),
			CellState::True(_) => CellState::None,
		}
	}

	pub fn prev(&self) -> Self {
		match self {
			CellState::None => CellState::True(false),
			CellState::False(_) => CellState::None,
			CellState::True(_) => CellState::False(false),
		}
	}

	pub fn inverse(&self) -> Self {
		match self {
			CellState::None => CellState::None,
			CellState::False(_) => CellState::True(false),
			CellState::True(_) => CellState::False(false),
		}
	}

	pub fn col(&self, assets: &Assets) -> Color {
		match self {
			CellState::None => assets.persistance.color0.into(),
			CellState::True(_) => assets.persistance.color1.into(),
			CellState::False(_) => assets.persistance.color2.into(),
		}
	}

	pub fn is_locked(&self) -> bool {
		match self {
			Self::None => false,
			Self::False(b) => *b,
			Self::True(b) => *b,
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