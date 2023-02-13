use crate::{board::Board, utils::{rect_circumscribed_on_rect, button}};
use egui_macroquad::{egui, macroquad::prelude::*};

pub enum State {
	MainMenu,
	Game(Board, Option<u32>),
	Settings,
}



impl State {
	pub fn update(&mut self, font: Font) -> Option<State> {
		match self {
			Self::MainMenu => {
				let mut ret = None;

				let display_area = Rect {x: 0.0, y: 0.0, w: 1.0, h: 1.0};

				let cam = Camera2D::from_display_rect(rect_circumscribed_on_rect(display_area, screen_width()/screen_height()));
				set_camera(&cam);

				clear_background(BLACK);

				if button(&Rect{x: 0.3, y: 0.2, w: 0.4, h: 0.1}, GRAY, "SANDBOX", &cam, font) {
					ret = Some(State::Game(Board::new(20), None));
				}
				if button(&Rect{x: 0.3, y: 0.35, w: 0.4, h: 0.1}, GRAY, "LEARN", &cam, font) {
					ret = Some(State::Game(Board::new(8), None));
				}
				if button(&Rect{x: 0.3, y: 0.5, w: 0.4, h: 0.1}, GRAY, "SERIOUS", &cam, font) {
					ret = Some(State::Game(Board::new(8), None));
				}

				ret
			}
			Self::Game(board, tries) => {
				clear_background(
					if board.is_won {GREEN} else if board.is_valid {BLACK} else {RED}
				);
		
				let camera = board.camera();
				set_camera(&camera);
		
				board.handle_mouse();
				board.draw();
		
				egui_macroquad::ui(|ctx| {
					egui::Window::new("Controls").show(ctx, |ui| {
						
						if let Some(t) = tries {
							ui.label(format!("tries: {}", t));
						}
						else {
							ui.label("");
						}
		
						if ui.button("generate").clicked() {
							*tries = Some(board.generate_valid());
						}
						if ui.button("degenerate").clicked() {
							board.degenerate();
							board.verify_board();
						}
						if ui.button("purge").clicked() {
							board.purge_redundancies();
						}
		
						ui.add_space(20.0);
		
						if ui.button("clear").clicked() {
							board.reset();
							board.verify_board();
						}
		
						ui.add_space(20.0);
		
						if ui.button("surround").clicked() {
							board.surround_doubles();
							board.verify_board();
						}
						if ui.button("fill").clicked() {
							board.fill_row();
							board.verify_board();
						}
						if ui.button("separate").clicked() {
							board.separate_triples();
							board.verify_board();
						}
		
						ui.add_space(20.0);
		
						if ui.button("desurround").clicked() {
							board.desurround_doubles(0.3);
						}
						if ui.button("defill").clicked() {
							board.defill_row(0.3);
						}
						if ui.button("deseparate").clicked() {
							board.deseparate_triples(0.3);
						}
		
					});
				});
		
				egui_macroquad::draw();

				None
			}
			_ => None
		}
	}
}