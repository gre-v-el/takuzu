use std::f32::consts::PI;

use crate::{board::Board, ui::{rect_circumscribed_on_rect, button, draw_centered_text_stable, draw_round_rect, draw_centered_text, draw_centered_text_color, slider}, assets::Assets, PRI_BUTTON_COL, SEC_BUTTON_COL, SLIDER_COL, POPUP_COL, POPUP_EDGE_COL};
use macroquad::prelude::*;

#[derive(Clone)]
pub enum NextState {
	Sandbox,
	Learn,
	Serious,
}

#[derive(Clone)]
pub enum State {
	MainMenu,
	Sandbox(Board),
	Learn(Board),
	Serious(Board, f32, Option<f32>), // start time, finished time
	EndScreen(Box<State>, Option<(f32, Option<f32>)>), // is highscore - new time, previous time (if any)
	ExitConfirmation(Box<State>),
	Highscores,
	Settings(Board),
	DifficultyChoice(Board, NextState, usize), 
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
				
				
				if button(&Rect{x: 0.3, y: 0.28, w: 0.4, h: 0.1}, PRI_BUTTON_COL, "SANDBOX", &cam, font, 0.06) && handle_mouse {
					let mut board = Board::new(assets.persistance.game_size);
					board.generate_fraction(0.6);
					ret = Some(Self::DifficultyChoice(board, NextState::Sandbox, assets.persistance.game_size));
				}
				if button(&Rect{x: 0.3, y: 0.39, w: 0.4, h: 0.1}, PRI_BUTTON_COL, "LEARN", &cam, font, 0.06) && handle_mouse {
					let mut board = Board::new(assets.persistance.game_size);
					board.generate_fraction(0.6);
					ret = Some(Self::DifficultyChoice(board, NextState::Learn, assets.persistance.game_size));
				}
				if button(&Rect{x: 0.3, y: 0.5, w: 0.4, h: 0.1}, PRI_BUTTON_COL, "SERIOUS", &cam, font, 0.06) && handle_mouse {
					let mut board = Board::new(assets.persistance.game_size);
					board.generate_fraction(0.6);
					ret = Some(Self::DifficultyChoice(board, NextState::Serious, assets.persistance.game_size));
				}

				if button(&Rect{x: 0.3, y: 0.7, w: 0.4, h: 0.1}, SEC_BUTTON_COL, "HIGHSCORES", &cam, font, 0.05) && handle_mouse {
					ret = Some(State::Highscores);
				}

				if button(&Rect{x: 0.3, y: 0.81, w: 0.4, h: 0.1}, SEC_BUTTON_COL, "SETTINGS", &cam, font, 0.05) && handle_mouse {
					use crate::cell_state::CellState::*;
					let board = Board { 
						is_won: false, 
						is_valid: true, 
						size: 4, 
						map: vec![
							vec![True(false), True(false), None, None],
							vec![None, False(false), False(false), False(false)],
							vec![True(false), None, True(false), None],
							vec![False(false), False(false), None, True(false)],
						], 
						error: [Option::Some((1, 1, 3, 1)), Option::None],
						hint: Some((2, 3)),
						show_locked: Option::None
					}; 
					ret = Some(State::Settings(board));
				}
			}
			Self::DifficultyChoice(board, next, size) => {

				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.7, y: -0.2, w: 2.4, h: 2.6 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				board.draw(assets);


				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				draw_centered_text(vec2(0.5, 0.5), "board size:", font, 0.1);

				let old_size = *size;
				let mut val = (*size as f32 - 2.0)/18.0;
				slider(&mut val, 0.0, 1.0, vec2(0.2, 0.65), 0.6, SLIDER_COL, &camera);
				*size = ((val*18.0 + 2.0)/2.0).round() as usize * 2;
				
				draw_centered_text(vec2(0.5, 0.8), format!("{size}").as_str(), font, 0.1);

				if old_size != *size {
					*board = Board::new(*size);
					board.generate_fraction(0.6);
				}

				if button(&Rect{x: 0.35, y: 0.95, w: 0.3, h: 0.1}, PRI_BUTTON_COL, "PLAY", &camera, font, 0.08) {
					assets.persistance.game_size = *size;
					assets.persistance.save();
					ret = Some(
						match next {
							NextState::Sandbox => State::Sandbox(Board::new(*size)),
							NextState::Learn => State::Learn(Board::new_learn(*size)),
							NextState::Serious => State::Serious(Board::new_serious(*size), get_time() as f32 + 1.5, None),
						}
					);
				}

				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					ret = Some(State::MainMenu);
					assets.persistance.game_size = *size;
					assets.persistance.save();
				}
			}
			Self::Sandbox(board) => {
				
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
				board.draw(&assets);
				
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
				
				if button(&buttons[0], PRI_BUTTON_COL, "Generate", &camera, font, scale) && handle_mouse {
					board.generate_valid();
				}
				if button(&buttons[1], PRI_BUTTON_COL, "Purge some", &camera, font, scale) && handle_mouse {
					board.degenerate();
					board.verify_board();
				}
				if button(&buttons[2], PRI_BUTTON_COL, "Purge all", &camera, font, scale) && handle_mouse {
					board.purge_redundancies();
					board.verify_board();
				}
				if button(&buttons[3], PRI_BUTTON_COL, "Clear", &camera, font, scale) && handle_mouse {
					board.reset();
				}
				if button(&buttons[4], PRI_BUTTON_COL, "Surround", &camera, font, scale) && handle_mouse {
					board.surround_doubles();
					board.verify_board();
				}
				if button(&buttons[5], PRI_BUTTON_COL, "De-Surround", &camera, font, scale) && handle_mouse {
					board.desurround_doubles(1.0);
					board.verify_board();
				}
				if button(&buttons[6], PRI_BUTTON_COL, "Separate", &camera, font, scale) && handle_mouse {
					board.separate_triples();
					board.verify_board();
				}
				if button(&buttons[7], PRI_BUTTON_COL, "De-Separate", &camera, font, scale) && handle_mouse {
					board.deseparate_triples(1.0);
					board.verify_board();
				}
				if button(&buttons[8], PRI_BUTTON_COL, "Fill", &camera, font, scale) && handle_mouse {
					board.fill_rows();
					board.verify_board();
				}
				if button(&buttons[9], PRI_BUTTON_COL, "De-Fill", &camera, font, scale) && handle_mouse {
					board.defill_rows(1.0);
					board.verify_board();
				}
				
				if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Hint", &camera, font, 0.06) && handle_mouse {
					board.generate_hint();
				}
				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Exit", &camera, font, 0.06) && handle_mouse {
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
				board.draw(&assets);
				
				
				if handle_mouse {
					if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Hint", &camera, font, 0.06) && handle_mouse {
						board.generate_hint();
					}
					if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Exit", &camera, font, 0.06) && handle_mouse {
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
				if get_time() as f32 > *start_time {
					let passed = if let Some(t) = *finished_time {t} else {get_time() as f32 - *start_time};
					let mut str = format!("0{:.2}s", passed);
					if passed >= 10.0 {str = str[1..].to_owned();}
					
					for (i, c) in str.chars().enumerate() {
						draw_centered_text_stable(vec2(i as f32 * 0.07 + 0.04, -0.1), [c].iter().collect::<String>().as_str(), "0", font, 0.1);
					}
				}
				
				if handle_mouse && get_time() as f32 > *start_time {
					board.handle_mouse(&camera);
				}
				board.draw(&assets);
				
				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Exit", &camera, font, 0.06) && handle_mouse && get_time() as f32 > *start_time {
					ret = Some(State::ExitConfirmation(Box::new(State::Serious(board.clone(), *start_time, *finished_time))))
				}

				if *start_time > get_time() as f32 {
					let countdown = ((*start_time - get_time() as f32) / 1.5 * 4.0).floor();
					let t = 1.0 - ((*start_time - get_time() as f32) / 1.5 * 4.0).fract();
					draw_rectangle(display_rect.x, display_rect.y, display_rect.w, display_rect.h, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.8 });
					let col = Color {r: 1.0, g: 1.0, b: 1.0, a: (t*t*3.0).min(1.0)};
					if countdown > 0.0 {
						draw_centered_text_color(display_rect.center(), format!("{countdown}").as_str(), font, 0.4-t*0.2, col);
					}
					if countdown < 3.0 {
						draw_centered_text(display_rect.center(), format!("{}", countdown+1.0).as_str(), font, 0.2-t*0.2);
					}
				
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
				draw_round_rect(0.2-m, 0.3-m, 0.6+2.0*m, 0.4+2.0*m, 0.05+m, POPUP_EDGE_COL);
				draw_round_rect(0.2, 0.3, 0.6, 0.4, 0.05, POPUP_COL);
				
				draw_centered_text(allocated_rect.center() - vec2(0.0, 0.1), "Exit?", font, 0.1);
				
				if button(&Rect { x: 0.25, y: 0.55, w: 0.2, h: 0.1 }, PRI_BUTTON_COL, "Yes", &cam, font, 0.07) {
					ret = Some(State::MainMenu);
				}
				if button(&Rect { x: 0.55, y: 0.55, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "No", &cam, font, 0.07) {
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
				draw_round_rect(0.2-m, 0.1-m, 0.6+2.0*m, 0.8+2.0*m, 0.05+m, POPUP_EDGE_COL);
				draw_round_rect(0.2, 0.1, 0.6, 0.8, 0.05, POPUP_COL);
				
				draw_centered_text(allocated_rect.center() - vec2(0.0, 0.3), "Finished!", font, 0.1);
				
				match highscore {
					None => {
						match &**inner_state {
							State::Learn(_) => {
								draw_centered_text_color(allocated_rect.center(), "(No scores in Learn mode)", font, 0.03, GRAY);
							}
							State::Serious(board, _, time) => {
								draw_centered_text_color(allocated_rect.center() - vec2(0.0, 0.1), format!("time: {:.2}s", time.unwrap()).as_str(), font, 0.08, WHITE);
								draw_centered_text_color(allocated_rect.center(), format!("highscore: {:.2}s", assets.persistance.highscores[board.size/2 - 1].unwrap()).as_str(), font, 0.05, ORANGE);
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
				
				if button(&Rect { x: 0.25, y: 0.6, w: 0.5, h: 0.1 }, PRI_BUTTON_COL, "Play Again", &cam, font, 0.07) {
					match &**inner_state {
						State::Serious(b, _, _) => {
							ret = Some(State::Serious(Board::new_serious(b.size), get_time() as f32 + 1.5, None));
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
				if button(&Rect { x: 0.25, y: 0.75, w: 0.5, h: 0.1 }, SEC_BUTTON_COL, "Back", &cam, font, 0.07) {
					ret = Some(State::MainMenu);
				}
			}
			Self::Highscores => {
				
				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				if button(&Rect { x: 0.8, y: -0.1, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					ret = Some(State::MainMenu);
				}

				let mut y = 0.0;
				for (i, time) in assets.persistance.highscores.iter().enumerate() {
					if let Some(t) = time {
						let size = 2*(1+i);
						let extra_space = if size >= 20 {""} else if size >= 10 {" "} else {"   "};
						draw_centered_text_stable(vec2(0.5, y), format!("{extra_space}{}: {:.2}s", size, t).as_str(), "0: 000.00", font, 0.09);
						y += 0.1;
					}
				}
			}
			Self::Settings(board) => {

				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.7, y: -1.5, w: 2.4, h: 2.6 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				board.draw_errors();
				board.draw_hint();
				board.draw(assets);


				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				if button(&Rect { x: 0.8, y: -0.1, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					ret = Some(State::MainMenu);
					assets.persistance.save();
				}
				if button(&Rect { x: 0.0, y: -0.1, w: 0.2, h: 0.1 }, PRI_BUTTON_COL, "Reset", &camera, font, 0.06) {
					assets.persistance.color0 = DARKGRAY.into();
					assets.persistance.color1 = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }.into();
					assets.persistance.color2 = Color { r: 0.0, g: 0.5, b: 1.0, a: 1.0 }.into();
				}

				slider(&mut assets.persistance.color0[0], 0.0, 1.0, vec2(-0.05, 0.1), 0.3, color_u8!(255, 0, 0, 255), &camera);
				slider(&mut assets.persistance.color0[1], 0.0, 1.0, vec2(-0.05, 0.2), 0.3, color_u8!(0, 255, 0, 255), &camera);
				slider(&mut assets.persistance.color0[2], 0.0, 1.0, vec2(-0.05, 0.3), 0.3, color_u8!(0, 0, 255, 255), &camera);

				slider(&mut assets.persistance.color1[0], 0.0, 1.0, vec2(0.35, 0.1), 0.3, color_u8!(255, 0, 0, 255), &camera);
				slider(&mut assets.persistance.color1[1], 0.0, 1.0, vec2(0.35, 0.2), 0.3, color_u8!(0, 255, 0, 255), &camera);
				slider(&mut assets.persistance.color1[2], 0.0, 1.0, vec2(0.35, 0.3), 0.3, color_u8!(0, 0, 255, 255), &camera);

				slider(&mut assets.persistance.color2[0], 0.0, 1.0, vec2(0.75, 0.1), 0.3, color_u8!(255, 0, 0, 255), &camera);
				slider(&mut assets.persistance.color2[1], 0.0, 1.0, vec2(0.75, 0.2), 0.3, color_u8!(0, 255, 0, 255), &camera);
				slider(&mut assets.persistance.color2[2], 0.0, 1.0, vec2(0.75, 0.3), 0.3, color_u8!(0, 0, 255, 255), &camera);


			}
		}
		
		if let Some(Self::MainMenu) = ret {
			assets.change_material();
		}

		ret
	}
}