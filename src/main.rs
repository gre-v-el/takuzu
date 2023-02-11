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
				
				if let Some(t) = tries {
					ui.label(format!("tries: {}", t));
				}
				else {
					ui.label("");
				}

				if ui.button("generate").clicked() {
					tries = Some(state.generate_valid());
				}
				if ui.button("degenerate").clicked() {
					state.degenerate();
					state.verify_board();
				}

				ui.add_space(20.0);

				if ui.button("clear").clicked() {
					state.reset();
					state.verify_board();
				}

				ui.add_space(20.0);

				if ui.button("surround").clicked() {
					state.surround_doubles();
					state.verify_board();
				}
				if ui.button("fill").clicked() {
					state.fill_row();
					state.verify_board();
				}
				if ui.button("separate").clicked() {
					state.separate_triples();
					state.verify_board();
				}

				ui.add_space(20.0);

				if ui.button("desurround").clicked() {
					state.desurround_doubles(0.3);
				}
				if ui.button("defill").clicked() {
					state.defill_row(0.3);
				}
				if ui.button("deseparate").clicked() {
					state.deseparate_triples(0.3);
				}

			});
		});

		egui_macroquad::draw();
		

        next_frame().await
    }
}