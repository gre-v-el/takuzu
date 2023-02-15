use std::f32::consts::PI;

use crate::{board::Board, utils::{rect_circumscribed_on_rect, button}, Assets};
use egui_macroquad::macroquad::prelude::*;

pub enum State {
	MainMenu,
	Sandbox(Board, Option<u32>),
	Learn(Board),
	Serious(Board),
	// Settings,
}

impl State {
	pub fn update(&mut self, assets: &Assets) -> Option<State> {
		let font = assets.font;
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
					let mut board = Board::new(8);
					board.generate_valid();
					board.degenerate();
					board.purge_redundancies();
					board.lock_tiles();
					ret = Some(State::Learn(board));
				}
				if button(&Rect{x: 0.3, y: 0.5, w: 0.4, h: 0.1}, GRAY, "SERIOUS", &cam, font, 0.06) {
					let mut board = Board::new(8);
					board.generate_valid();
					board.purge_redundancies();
					board.lock_tiles();
					ret = Some(State::Serious(board));
				}
			}
			Self::Sandbox(board, tries) => {
		
				// (0,0) to (1,1) is the board. Depending on the aspect ratio: vertical will have space at the bottom and horizontal will have space to the left for some ui. Also allocate space at the top for exit and timer
				let allocated_rect = if screen_width() / screen_height() > 1.0 {
					Rect{x: -0.8, y: -0.2, w: 1.9, h: 1.3}
				}
				else {
					Rect{x: -0.1, y: -0.2, w: 1.2, h: 1.6}
				};
				let display_rect = rect_circumscribed_on_rect(allocated_rect, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				let status_color = if board.is_won {Some(GREEN)} else if board.is_valid {None} else {Some(RED)};

				if let Some(c) = status_color {
					let w = 0.2;
					draw_texture_ex(assets.gradient, display_rect.left(), display_rect.top(), c, 
						DrawTextureParams { 
							source: Some(Rect{x: 0.5, y: 0.0, w: 1.0, h: 1.0}),
							dest_size: Some(vec2(w, display_rect.h)),
							pivot: Some(display_rect.center()),
							rotation: PI,
							..Default::default()
						});
					draw_texture_ex(assets.gradient, display_rect.left(), display_rect.top(), c, 
						DrawTextureParams { 
							source: Some(Rect{x: 0.5, y: 0.0, w: 1.0, h: 1.0}),
							dest_size: Some(vec2(w, display_rect.h)),
							..Default::default()
						});
				}

				board.handle_mouse(&camera);
				board.draw_errors();
				board.draw_hint();
				board.draw();
				

				// todo: show tries
				let scale = 0.04;
				let buttons = if screen_width() / screen_height() > 1.0 {
					let w = 0.32;
					let w2 = 0.65;
					let h = 0.14;
					[
						Rect{x: -0.7,  y: 0.0,  w: w2, h},
						Rect{x: -0.7,  y: 0.15, w,     h},
						Rect{x: -0.37, y: 0.15, w,     h},
						Rect{x: -0.7,  y: 0.35, w: w2, h},
						Rect{x: -0.7,  y: 0.55, w,     h},
						Rect{x: -0.37, y: 0.55, w,     h},
						Rect{x: -0.7,  y: 0.7,  w,     h},
						Rect{x: -0.37, y: 0.7,  w,     h},
						Rect{x: -0.7,  y: 0.85, w,     h},
						Rect{x: -0.37, y: 0.85, w,     h},
					]
				} 
				else {
					let w = 0.24;
					let w2 = 0.38;
					let w3 = 0.29;
					let h = 0.1;
					let c1 = 0.0;
					let c2 = 0.25;
					let c3 = 0.41;
					let c4 = 0.71;
					let r1 = 1.05;
					let r2 = 1.16;
					let r3 = 1.27;
					[
						Rect{x: c1, y: r1, w,     h},
						Rect{x: c1, y: r2, w: w2, h}, 
						Rect{x: c1, y: r3, w: w2, h},
						Rect{x: c2, y: r1, w: 0.13, h},
						Rect{x: c3, y: r1, w: w3, h},
						Rect{x: c4, y: r1, w: w3, h},
						Rect{x: c3, y: r2, w: w3, h},
						Rect{x: c4, y: r2, w: w3, h},
						Rect{x: c3, y: r3, w: w3, h},
						Rect{x: c4, y: r3, w: w3, h},
					]
				};
				
				if button(&buttons[0], GRAY, "Generate", &camera, font, scale) {
					*tries = Some(board.generate_valid());
				}
				if button(&buttons[1], GRAY, "Purge some", &camera, font, scale) {
					board.degenerate();
					board.verify_board();
				}
				if button(&buttons[2], GRAY, "Purge all", &camera, font, scale) {
					board.purge_redundancies();
					board.verify_board();
				}
				if button(&buttons[3], GRAY, "Clear", &camera, font, scale) {
					board.reset();
				}
				if button(&buttons[4], GRAY, "Surround", &camera, font, scale) {
					board.surround_doubles();
					board.verify_board();
				}
				if button(&buttons[5], GRAY, "De-Surround", &camera, font, scale) {
					board.desurround_doubles(1.0);
					board.verify_board();
				}
				if button(&buttons[6], GRAY, "Separate", &camera, font, scale) {
					board.separate_triples();
					board.verify_board();
				}
				if button(&buttons[7], GRAY, "De-Separate", &camera, font, scale) {
					board.deseparate_triples(1.0);
					board.verify_board();
				}
				if button(&buttons[8], GRAY, "Fill", &camera, font, scale) {
					board.fill_rows();
					board.verify_board();
				}
				if button(&buttons[9], GRAY, "De-Fill", &camera, font, scale) {
					board.defill_rows(1.0);
					board.verify_board();
				}

				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Exit", &camera, font, 0.06) {
					ret = Some(State::MainMenu);
				}
				if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Hint", &camera, font, 0.06) {
					board.generate_hint();
				}
			}
			Self::Learn(board) => {
				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				let status_color = if board.is_won {Some(GREEN)} else if board.is_valid {None} else {Some(RED)};

				if let Some(c) = status_color {
					let w = 0.2;
					draw_texture_ex(assets.gradient, display_rect.left(), display_rect.top(), c, 
						DrawTextureParams { 
							source: Some(Rect{x: 0.5, y: 0.0, w: 1.0, h: 1.0}),
							dest_size: Some(vec2(w, display_rect.h)),
							pivot: Some(display_rect.center()),
							rotation: PI,
							..Default::default()
						});
					draw_texture_ex(assets.gradient, display_rect.left(), display_rect.top(), c, 
						DrawTextureParams { 
							source: Some(Rect{x: 0.5, y: 0.0, w: 1.0, h: 1.0}),
							dest_size: Some(vec2(w, display_rect.h)),
							..Default::default()
						});
				}

				
				board.handle_mouse(&camera);
				board.draw_errors();
				board.draw_hint();
				board.draw();

				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Exit", &camera, font, 0.06) {
					ret = Some(State::MainMenu);
				}
				if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Hint", &camera, font, 0.06) {
					board.generate_hint();
				}


			}
			Self::Serious(board) => {
				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				let status_color = if board.is_won {Some(GREEN)} else if !board.has_nones() && !board.is_valid {Some(RED)} else {None};

				if let Some(c) = status_color {
					let w = 0.2;
					draw_texture_ex(assets.gradient, display_rect.left(), display_rect.top(), c, 
						DrawTextureParams { 
							source: Some(Rect{x: 0.5, y: 0.0, w: 1.0, h: 1.0}),
							dest_size: Some(vec2(w, display_rect.h)),
							pivot: Some(display_rect.center()),
							rotation: PI,
							..Default::default()
						});
					draw_texture_ex(assets.gradient, display_rect.left(), display_rect.top(), c, 
						DrawTextureParams { 
							source: Some(Rect{x: 0.5, y: 0.0, w: 1.0, h: 1.0}),
							dest_size: Some(vec2(w, display_rect.h)),
							..Default::default()
						});
				}

				
				board.handle_mouse(&camera);
				board.draw();

				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Exit", &camera, font, 0.06) {
					ret = Some(State::MainMenu);
				}

			}
		}

		ret
	}
}