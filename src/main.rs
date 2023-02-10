use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::prelude::*;
use takuzu::game_state::GameState;


#[macroquad::main("binary sudoku")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64);

	let mut state = GameState::new(8);


    loop {
		clear_background(
			if state.is_won {GREEN} else if state.is_valid {BLACK} else {RED}
		);

		if is_key_pressed(KeyCode::G) {
			state.generate_valid();
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