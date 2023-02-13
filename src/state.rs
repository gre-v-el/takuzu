use crate::{board::Board, utils::{rect_circumscribed_on_rect, button}};
use egui_macroquad::macroquad::prelude::*;

pub enum State {
	MainMenu,
	Sandbox(Board, Option<u32>),
	Learn(Board),
	Serious(Board),
	// Settings,
}

impl State {
	pub fn update(&mut self, font: Font) -> Option<State> {
		let mut ret = None;
		match self {
			Self::MainMenu => {

				let display_area = Rect {x: 0.0, y: 0.0, w: 1.0, h: 1.0};

				let cam = Camera2D::from_display_rect(rect_circumscribed_on_rect(display_area, screen_width()/screen_height()));
				set_camera(&cam);

				clear_background(BLACK);

				if button(&Rect{x: 0.3, y: 0.2, w: 0.4, h: 0.1}, GRAY, "SANDBOX", &cam, font, 0.06) {
					ret = Some(State::Sandbox(Board::new(8), None));
				}
				if button(&Rect{x: 0.3, y: 0.35, w: 0.4, h: 0.1}, GRAY, "LEARN", &cam, font, 0.06) {
					ret = Some(State::Learn(Board::new(8)));
				}
				if button(&Rect{x: 0.3, y: 0.5, w: 0.4, h: 0.1}, GRAY, "SERIOUS", &cam, font, 0.06) {
					ret = Some(State::Serious(Board::new(8)));
				}
			}
			Self::Sandbox(board, tries) => {
				clear_background(
					if board.is_won {GREEN} else if board.is_valid {BLACK} else {RED}
				);
		
				// (0,0) to (1,1) is the board. Depending on the aspect ratio: vertical will have space at the bottom and horizontal will have space to the left for some ui. Also allocate space at the top for exit and timer
				let camera = 
				if screen_width() / screen_height() > 1.0 {
					Camera2D::from_display_rect(
						rect_circumscribed_on_rect(Rect{x: -0.8, y: -0.2, w: 1.9, h: 1.3}, screen_width()/screen_height())
					)
				}
				else {
					Camera2D::from_display_rect(
						rect_circumscribed_on_rect(Rect{x: -0.1, y: -0.2, w: 1.2, h: 1.8}, screen_width()/screen_height())
					)
				};
				set_camera(&camera);
		
				// draw_rectangle(-0.8, -0.2, 1.9, 1.3, DARKGRAY);
				board.handle_mouse(&camera);
				board.draw();
				

				// todo: show tries, show exit
				if screen_width() / screen_height() > 1.0 {
					let scale = 0.04;
					let x = -0.7;
					let w = 0.3;
					let h = 0.1;
					if button(&Rect { x, y: 0.0, w: 0.65, h }, GRAY, "Generate", &camera, font, scale) {
						*tries = Some(board.generate_valid());
					}
					if button(&Rect { x, y: 0.15, w, h }, GRAY, "Purge some", &camera, font, scale) {
						board.degenerate();
						board.verify_board();
					}
					if button(&Rect { x, y: 0.35, w: 0.65, h }, GRAY, "Clear", &camera, font, scale) {
						board.reset();
					}
					if button(&Rect { x, y: 0.55, w, h }, GRAY, "Surround", &camera, font, scale) {
						board.surround_doubles();
						board.verify_board();
					}
					if button(&Rect { x, y: 0.7, w, h }, GRAY, "Separate", &camera, font, scale) {
						board.separate_triples();
						board.verify_board();
					}
					if button(&Rect { x, y: 0.85, w, h }, GRAY, "Fill", &camera, font, scale) {
						board.fill_rows();
						board.verify_board();
					}

					let x = -0.35;
					if button(&Rect { x, y: 0.15, w, h }, GRAY, "Purge all", &camera, font, scale) {
						board.purge_redundancies();
						board.verify_board();
					}
					if button(&Rect { x, y: 0.55, w, h }, GRAY, "De-Surround", &camera, font, scale) {
						board.desurround_doubles(1.0);
						board.verify_board();
					}
					if button(&Rect { x, y: 0.7, w, h }, GRAY, "De-Separate", &camera, font, scale) {
						board.deseparate_triples(1.0);
						board.verify_board();
					}
					if button(&Rect { x, y: 0.85, w, h }, GRAY, "De-Fill", &camera, font, scale) {
						board.defill_rows(1.0);
						board.verify_board();
					}

					if button(&Rect { x: 0.8, y: -0.2, w: 0.2, h: 0.1 }, GRAY, "Exit", &camera, font, 0.06) {
						ret = Some(State::MainMenu);
					}
				}
			}
			_ => {}
		}

		ret
	}
}