use std::time::{SystemTime, UNIX_EPOCH};
use egui_macroquad::{macroquad, egui};
use macroquad::prelude::*;
use takuzu::{board::Board, state::State};

/*
	TODO:
		add tile locking (the user cannot change a generated tile)
		let the user know which tiles are locked
		make some menus, separate for "serious play", separate for sandbox and separate for learning
		some high scores?
		possibly switch to macroquad's ui so it can be compiled for android
		add timing
		come up with a difficulty metric and decide what is better: PURGE vs DEGENERATE + PURGE
		improve solving with the last rule
*/

#[macroquad::main("binary sudoku")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64);

	// let mut state = State::Game(Board::new(8), None);
	let mut state = State::MainMenu;

    loop {
		
		if let Some(s) = state.update() {
			state = s;
		}

        next_frame().await
    }
}