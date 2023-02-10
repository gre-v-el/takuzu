use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::prelude::*;
use takuzu::{game_state::GameState, cell_state::CellState};


#[macroquad::main("binary sudoku")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64);

	let mut state = GameState::new(6);

	let mut i = 1;
	state.generate();
	while !state.is_valid {
		state.reset();
		state.generate();
		i += 1;
		if i % 100 == 0{
			println!("{}", i);
		}
	}
	println!("{}", i);


    loop {
		clear_background(
			if state.is_won {GREEN} else if state.is_valid {BLACK} else {RED}
		);

		if is_key_pressed(KeyCode::G) {
			state.generate();
		}
		if is_key_pressed(KeyCode::U) {
			state.surround_doubles();
		}
		if is_key_pressed(KeyCode::E) {
			state.separate_triples();
		}
		if is_key_pressed(KeyCode::I) {
			state.insert_random();
		}
		if is_key_pressed(KeyCode::R) {
			state.reset();
		}

		let camera = state.camera();
		set_camera(&camera);

		state.handle_mouse();
		state.draw();

		

        next_frame().await
    }
}