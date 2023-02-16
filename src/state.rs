use std::f32::consts::PI;

use crate::{board::Board, utils::{rect_circumscribed_on_rect, button, draw_centered_text_stable, draw_round_rect, draw_centered_text, draw_centered_text_color}, Assets};
use macroquad::prelude::*;

#[derive(Clone)]
pub enum State {
	MainMenu,
	Sandbox(Board, Option<u32>),
	Learn(Board),
	Serious(Board, f32, Option<f32>), // start time, finished time
	EndScreen(Box<State>, Option<(f32, Option<f32>)>), // is highscore - new time, previous time (if any)
	ExitConfirmation(Box<State>),
	// Settings,
	// Highscores,
	// DifficultyChoice, 
}

impl State {
	pub fn update(&mut self, assets: &mut Assets, handle_mouse: bool) -> Option<State> {
		let font = assets.font;
		let mut ret = None;
		match self {
			Self::MainMenu => {
				
				let display_area = Rect {x: 0.0, y: 0.0, w: 1.0, h: 1.0};
				
				let cam = Camera2D::from_display_rect(rect_circumscribed_on_rect(display_area, screen_width()/screen_height()));
				set_camera(&cam);
				
				clear_background(BLACK);
				
				if button(&Rect{x: 0.3, y: 0.2, w: 0.4, h: 0.1}, GRAY, "SANDBOX", &cam, font, 0.06) && handle_mouse {
					ret = Some(State::Sandbox(Board::new(4), None));
				}
				if button(&Rect{x: 0.3, y: 0.35, w: 0.4, h: 0.1}, GRAY, "LEARN", &cam, font, 0.06) && handle_mouse {
					ret = Some(State::Learn(Board::new_learn(4)));
				}
				if button(&Rect{x: 0.3, y: 0.5, w: 0.4, h: 0.1}, GRAY, "SERIOUS", &cam, font, 0.06) && handle_mouse {
					ret = Some(State::Serious(Board::new_serious(4), get_time() as f32, None));
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
				
				if handle_mouse {
					board.handle_mouse(&camera);
				}
				board.draw_errors();
				board.draw_hint();
				board.draw();
				
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
				
				if button(&buttons[0], GRAY, "Generate", &camera, font, scale) && handle_mouse {
					*tries = Some(board.generate_valid());
				}
				if button(&buttons[1], GRAY, "Purge some", &camera, font, scale) && handle_mouse {
					board.degenerate();
					board.verify_board();
				}
				if button(&buttons[2], GRAY, "Purge all", &camera, font, scale) && handle_mouse {
					board.purge_redundancies();
					board.verify_board();
				}
				if button(&buttons[3], GRAY, "Clear", &camera, font, scale) && handle_mouse {
					board.reset();
				}
				if button(&buttons[4], GRAY, "Surround", &camera, font, scale) && handle_mouse {
					board.surround_doubles();
					board.verify_board();
				}
				if button(&buttons[5], GRAY, "De-Surround", &camera, font, scale) && handle_mouse {
					board.desurround_doubles(1.0);
					board.verify_board();
				}
				if button(&buttons[6], GRAY, "Separate", &camera, font, scale) && handle_mouse {
					board.separate_triples();
					board.verify_board();
				}
				if button(&buttons[7], GRAY, "De-Separate", &camera, font, scale) && handle_mouse {
					board.deseparate_triples(1.0);
					board.verify_board();
				}
				if button(&buttons[8], GRAY, "Fill", &camera, font, scale) && handle_mouse {
					board.fill_rows();
					board.verify_board();
				}
				if button(&buttons[9], GRAY, "De-Fill", &camera, font, scale) && handle_mouse {
					board.defill_rows(1.0);
					board.verify_board();
				}
				
				if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Hint", &camera, font, 0.06) && handle_mouse {
					board.generate_hint();
				}
				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Exit", &camera, font, 0.06) && handle_mouse {
					ret = Some(State::ExitConfirmation(Box::new(self.clone())));
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
				
				if handle_mouse {
					board.handle_mouse(&camera);
				}				
				board.draw_errors();
				board.draw_hint();
				board.draw();
				
				
				if handle_mouse {
					if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Hint", &camera, font, 0.06) && handle_mouse {
						board.generate_hint();
					}
					if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Exit", &camera, font, 0.06) && handle_mouse {
						ret = Some(State::ExitConfirmation(Box::new(State::Learn(board.clone()))));
					}
				
					if board.is_won {
						ret = Some(State::EndScreen(Box::new(State::Learn(board.clone())), None));
					}
				}
			}
			Self::Serious(board, start_time, finished_time) => {
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
				
				if board.is_won && finished_time.is_none() {
					let time = get_time() as f32 - *start_time;
					*finished_time = Some(time);
					let (is_highscore, prev_time) = assets.persistance.insert_highscore(board.size, time);
					ret = Some(State::EndScreen(Box::new(State::Serious(board.clone(), *start_time, Some(time))), if is_highscore {Some((time, prev_time))} else {None}));
				}
				let passed = if let Some(t) = *finished_time {t} else {get_time() as f32 - *start_time};
				let mut str = format!("0{:.2}s", passed);
				if passed >= 10.0 {str = str[1..].to_owned();}
				
				for (i, c) in str.chars().enumerate() {
					draw_centered_text_stable(vec2(i as f32 * 0.07 + 0.04, -0.1), [c].iter().collect::<String>().as_str(), "0", font, 0.1);
				}
				
				if handle_mouse {
					board.handle_mouse(&camera);
				}
				board.draw();
				
				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, GRAY, "Exit", &camera, font, 0.06) && handle_mouse {
					ret = Some(State::ExitConfirmation(Box::new(self.clone())));
				}
				
			}
			Self::ExitConfirmation(inner_state) => {
				inner_state.update(assets, false);
				
				let allocated_rect = Rect {x: 0.0, y: 0.0, w: 1.0, h: 1.0};
				let display_rect = rect_circumscribed_on_rect(allocated_rect, screen_width()/screen_height());
				
				let cam = Camera2D::from_display_rect(display_rect);
				set_camera(&cam);
				
				draw_rectangle(display_rect.x, display_rect.y, display_rect.w, display_rect.h, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.8 });
				
				let m = 0.01;
				draw_round_rect(0.2-m, 0.3-m, 0.6+2.0*m, 0.4+2.0*m, 0.05+m, BLACK);
				draw_round_rect(0.2, 0.3, 0.6, 0.4, 0.05, DARKGRAY);
				
				draw_centered_text(allocated_rect.center() - vec2(0.0, 0.1), "Exit?", font, 0.1);
				
				if button(&Rect { x: 0.25, y: 0.55, w: 0.2, h: 0.1 }, GRAY, "Yes", &cam, font, 0.07) {
					ret = Some(State::MainMenu);
				}
				if button(&Rect { x: 0.55, y: 0.55, w: 0.2, h: 0.1 }, GRAY, "No", &cam, font, 0.07) {
					ret = Some((**inner_state).clone());
				}
			}
			Self::EndScreen(inner_state, highscore) => {
				// When uncommented, breaks the font
				inner_state.update(assets, false); 
				
				let allocated_rect = Rect {x: 0.0, y: 0.0, w: 1.0, h: 1.0};
				let display_rect = rect_circumscribed_on_rect(allocated_rect, screen_width()/screen_height());
				
				let cam = Camera2D::from_display_rect(display_rect);
				set_camera(&cam);
				
				draw_rectangle(display_rect.x, display_rect.y, display_rect.w, display_rect.h, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.8 });
				
				let m = 0.01;
				draw_round_rect(0.2-m, 0.1-m, 0.6+2.0*m, 0.8+2.0*m, 0.05+m, BLACK);
				draw_round_rect(0.2, 0.1, 0.6, 0.8, 0.05, DARKGRAY);
				
				draw_centered_text(allocated_rect.center() - vec2(0.0, 0.3), "Finished!", font, 0.1);
				
				match highscore {
					None => {
						match &**inner_state {
							State::Learn(_) => {
								draw_centered_text_color(allocated_rect.center(), "(No scores in Learn mode)", font, 0.03, GRAY);
							}
							State::Serious(_, _, time) => {
								draw_centered_text_color(allocated_rect.center(), format!("time: {:.2}s", time.unwrap()).as_str(), font, 0.08, WHITE);
							}
							_ => {}
						}
					}
					Some((new, previous)) => {
						draw_centered_text_color(allocated_rect.center() - vec2(0.0, 0.2), "High Score!", font, 0.09, ORANGE);
						draw_centered_text_color(allocated_rect.center() - vec2(0.0, 0.07), format!("{:.2}s", new).as_str(), font, 0.08, ORANGE);
						if let Some(previous) = previous {
							draw_centered_text_color(allocated_rect.center() + vec2(0.0, 0.0), format!("{:.2}s", previous).as_str(), font, 0.05, WHITE);
						}
						
					}
				}
				
				if button(&Rect { x: 0.25, y: 0.6, w: 0.5, h: 0.1 }, GRAY, "Play Again", &cam, font, 0.07) {
					match &**inner_state {
						State::Serious(b, _, _) => {
							ret = Some(State::Serious(Board::new_serious(b.size), get_time() as f32, None));
						}
						State::Learn(b) => {
							let board = Board::new_learn(b.size);
							ret = Some(State::Learn(board));
						}
						_ => {
							ret = Some(State::Learn(Board::new_learn(6)));
						}
					}
				}
				if button(&Rect { x: 0.25, y: 0.75, w: 0.5, h: 0.1 }, GRAY, "Back", &cam, font, 0.07) {
					ret = Some(State::MainMenu);
				}
			}
		}
		
		ret
	}
}