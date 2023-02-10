use std::time::{SystemTime, UNIX_EPOCH};
use egui_macroquad::{macroquad, egui};
use macroquad::prelude::*;
use takuzu::game_state::GameState;


#[macroquad::main("binary sudoku")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64);

	let mut state = GameState::new(8);
	let mut tries = None;

    loop {
		clear_background(
			if state.is_won {GREEN} else if state.is_valid {BLACK} else {RED}
		);

		// if is_key_pressed(KeyCode::G) {
		// 	state.generate_valid();
		// }
		// if is_key_pressed(KeyCode::U) {
		// 	state.surround_doubles();
		// }
		// if is_key_pressed(KeyCode::E) {
		// 	state.separate_triples();
		// }
		// if is_key_pressed(KeyCode::I) {
		// 	state.insert_random();
		// }
		// if is_key_pressed(KeyCode::R) {
		// 	state.reset();
		// }

		let camera = state.camera();
		set_camera(&camera);

		state.handle_mouse();
		state.draw();

		egui_macroquad::ui(|ctx| {
			egui::Window::new("Controls").show(ctx, |ui| {
				if ui.button("generate").clicked() {
					tries = Some(state.generate_valid());
				}

				if let Some(t) = tries {
					ui.label(format!("tries: {}", t));
				}
				else {
					ui.label("");
				}

				if ui.button("clear").clicked() {
					state.reset();
				}
			});
		});

		egui_macroquad::draw();
		

        next_frame().await
    }
}