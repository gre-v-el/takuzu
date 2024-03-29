use std::f32::consts::PI;

use crate::{board::Board, ui::{rect_circumscribed_on_rect, button, draw_centered_text_stable, draw_round_rect, draw_centered_text, draw_centered_text_color, slider}, assets::Assets, PRI_BUTTON_COL, SEC_BUTTON_COL, SLIDER_COL, POPUP_COL, POPUP_EDGE_COL, FORWARD, BACKWARD, TICK, cell_state::CellState};
use macroquad::prelude::*;

#[derive(Clone)]
pub enum GameMode {
	Sandbox,
	Learn,
	Serious,
}

impl GameMode {
	pub fn as_str(&self) -> &str {
		match self {
			Self::Sandbox => "Sandbox",
			Self::Learn => "Learn",
			Self::Serious => "Serious",
		}
	}

	pub fn info(&self) -> Vec<&str> {
		match self {
			Self::Sandbox => vec!["Here you can play", "around with different", "algorithms."],
			Self::Learn =>   vec!["Here you can play", "Takuzu with hints", "and error highlighting"],
			Self::Serious => vec!["Here you can play", "Takuzu without any", "hints or highlighting.", "Your best times will be", "saved as highscores."],
		}
	}
}

#[derive(Clone)]
pub enum State {
	MainMenu,
	Sandbox(Board),
	Learn(Board),
	Serious(Board, f32, Option<f32>, usize), // start time, finished time, completed tick sound plays
	EndScreen(Box<State>, Option<(f32, Option<f32>)>), // is highscore - new time, previous time (if any)
	ExitConfirmation(Box<State>),
	Highscores,
	Settings(Board),
	DifficultyChoice(Board, GameMode, usize), 
	Attribution,
	ModeInfo(GameMode)
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
				
				let w = 0.4;
				draw_texture_ex(assets.banner, 0.5-w/2.0, 0.1, WHITE, DrawTextureParams { 
					dest_size: Some(vec2(w, w/assets.banner.width()*assets.banner.height())), 
					source: None, 
					rotation: 0.0, 
					flip_x: false, 
					flip_y: false, 
					pivot: None 
				});
				
				if button(&Rect{x: 0.3, y: 0.28, w: 0.4, h: 0.1}, PRI_BUTTON_COL, "SANDBOX", &cam, font, 0.06) && handle_mouse {
					let mut board = Board::new(assets.persistance.game_size, 0, false);
					board.generate_fraction(0.6);
					ret = Some(Self::DifficultyChoice(board, GameMode::Sandbox, assets.persistance.game_size));
					assets.play_sound(FORWARD);
				}
				if button(&Rect{x: 0.3, y: 0.39, w: 0.4, h: 0.1}, PRI_BUTTON_COL, "LEARN", &cam, font, 0.06) && handle_mouse {
					let mut board = Board::new(assets.persistance.game_size, 0, false);
					board.generate_fraction(0.6);
					ret = Some(Self::DifficultyChoice(board, GameMode::Learn, assets.persistance.game_size));
					assets.play_sound(FORWARD);
				}
				if button(&Rect{x: 0.3, y: 0.5, w: 0.4, h: 0.1}, PRI_BUTTON_COL, "SERIOUS", &cam, font, 0.06) && handle_mouse {
					let mut board = Board::new(assets.persistance.game_size, 0, false);
					board.generate_fraction(0.6);
					ret = Some(Self::DifficultyChoice(board, GameMode::Serious, assets.persistance.game_size));
					assets.play_sound(FORWARD);
				}
				
				if button(&Rect{x: 0.2, y: 0.29, w: 0.08, h: 0.08}, SEC_BUTTON_COL, "?", &cam, font, 0.05) && handle_mouse {
					ret = Some(Self::ModeInfo(GameMode::Sandbox));
					assets.play_sound(FORWARD);
				}
				if button(&Rect{x: 0.2, y: 0.40, w: 0.08, h: 0.08}, SEC_BUTTON_COL, "?", &cam, font, 0.05) && handle_mouse {
					ret = Some(Self::ModeInfo(GameMode::Learn));
					assets.play_sound(FORWARD);
				}
				if button(&Rect{x: 0.2, y: 0.51, w: 0.08, h: 0.08}, SEC_BUTTON_COL, "?", &cam, font, 0.05) && handle_mouse {
					ret = Some(Self::ModeInfo(GameMode::Serious));
					assets.play_sound(FORWARD);
				}

				if button(&Rect{x: 0.3, y: 0.7, w: 0.4, h: 0.1}, SEC_BUTTON_COL, "HIGHSCORES", &cam, font, 0.05) && handle_mouse {
					ret = Some(State::Highscores);
					assets.play_sound(FORWARD);
				}

				if button(&Rect{x: 0.3, y: 0.81, w: 0.4, h: 0.1}, SEC_BUTTON_COL, "SETTINGS", &cam, font, 0.05) && handle_mouse {
					use crate::cell_state::CellState::*;
					let board = Board { 
						id: 0,
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
						error_time: 0.0,
						hint: Some((2, 3)),
						show_locked: Option::None,
						last_error_sound: -1.0,
						generation_end_time: -1.0,
						generation_duration: -1.0,
						is_generating: false,
					}; 
					ret = Some(State::Settings(board));
					assets.play_sound(FORWARD);
				}

				if button(&Rect { x: 0.76, y: 0.9, w: 0.21, h: 0.07 }, SEC_BUTTON_COL, "Attribution", &cam, font, 0.03) {
					ret = Some(Self::Attribution);
					assets.play_sound(FORWARD);
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
					*board = Board::new(*size, 0, false);
					board.generate_fraction(0.6);
				}

				if button(&Rect{x: 0.35, y: 0.95, w: 0.3, h: 0.1}, PRI_BUTTON_COL, "PLAY", &camera, font, 0.08) {
					assets.persistance.game_size = *size;
					assets.persistance.save();
					let id = assets.next_board_id;
					assets.next_board_id += 1;
					ret = Some(
						match next {
							GameMode::Sandbox => State::Sandbox(Board::new(*size, id, false)),
							GameMode::Learn => {
								assets.sender.send((*size, GameMode::Learn, id)).unwrap();
								State::Learn(Board::new(*size, id, true))
							},
							GameMode::Serious => {
								assets.sender.send((*size, GameMode::Serious, id)).unwrap();
								State::Serious(Board::new(*size, id, true), get_time() as f32 + 1.5, None, 0)
							}
						}
					);
					assets.play_sound(FORWARD);
				}

				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					ret = Some(State::MainMenu);
					assets.persistance.game_size = *size;
					assets.persistance.save();
					assets.play_sound(BACKWARD);
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
				
				if let Some(mut c) = status_color {
					c.a = board.get_error_alpha();
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
					board.handle_mouse(&camera, &assets);
				}
				board.draw_errors(Some(&assets));
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
					assets.play_sound(FORWARD);
					board.generate_valid();
				}
				if button(&buttons[1], PRI_BUTTON_COL, "Purge some", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.degenerate();
					board.verify_board();
				}
				if button(&buttons[2], PRI_BUTTON_COL, "Purge all", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.purge_redundancies();
					board.verify_board();
				}
				if button(&buttons[3], PRI_BUTTON_COL, "Clear", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.reset();
				}
				if button(&buttons[4], PRI_BUTTON_COL, "Surround", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.surround_doubles();
					board.verify_board();
				}
				if button(&buttons[5], PRI_BUTTON_COL, "De-Surround", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.desurround_doubles(1.0);
					board.verify_board();
				}
				if button(&buttons[6], PRI_BUTTON_COL, "Separate", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.separate_triples();
					board.verify_board();
				}
				if button(&buttons[7], PRI_BUTTON_COL, "De-Separate", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.deseparate_triples(1.0);
					board.verify_board();
				}
				if button(&buttons[8], PRI_BUTTON_COL, "Fill", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.fill_rows();
					board.verify_board();
				}
				if button(&buttons[9], PRI_BUTTON_COL, "De-Fill", &camera, font, scale) && handle_mouse {
					assets.play_sound(FORWARD);
					board.defill_rows(1.0);
					board.verify_board();
				}
				
				if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Hint", &camera, font, 0.06) && handle_mouse {
					board.generate_hint(&assets);
				}
				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Exit", &camera, font, 0.06) && handle_mouse {
					assets.play_sound(BACKWARD);
					ret = Some(State::ExitConfirmation(Box::new(self.clone())));
				}
			}
			Self::Learn(board) => {
				
				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);
				
				let status_color = if board.is_won {Some(GREEN)} else if board.is_valid {None} else {Some(RED)};
				
				if let Some(mut c) = status_color {
					c.a = board.get_error_alpha();
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
					board.handle_mouse(&camera, &assets);
				}				
				board.draw_errors(Some(&assets));
				board.draw_hint();
				board.draw(&assets);
				
				
				if handle_mouse {
					if button(&Rect { x: 0.0, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Hint", &camera, font, 0.06) && handle_mouse {
						board.generate_hint(&assets);
					}
					if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Exit", &camera, font, 0.06) && handle_mouse {
						ret = Some(State::ExitConfirmation(Box::new(State::Learn(board.clone()))));
						assets.play_sound(BACKWARD);
					}
				
					if board.is_won {
						ret = Some(State::EndScreen(Box::new(State::Learn(board.clone())), None));
					}
				}
			}
			Self::Serious(board, start_time, finished_time, sounds) => {
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
					ret = Some(State::EndScreen(Box::new(State::Serious(board.clone(), *start_time, Some(time), *sounds)), if is_highscore {Some((time, prev_time))} else {None}));
				}
				if get_time() as f32 > *start_time && !board.is_generating {
					let passed = if let Some(t) = *finished_time {t} else {get_time() as f32 - *start_time};
					let mut str = format!("0{:.2}s", passed);
					if passed >= 10.0 {str = str[1..].to_owned();}
					
					for (i, c) in str.chars().enumerate() {
						draw_centered_text_stable(vec2(i as f32 * 0.07 + 0.04, -0.1), [c].iter().collect::<String>().as_str(), "0", font, 0.1);
					}
				}
				
				if handle_mouse && get_time() as f32 > *start_time {
					board.handle_mouse(&camera, &assets);
				}
				board.draw(&assets);
				
				if button(&Rect { x: 0.8, y: -0.15, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Exit", &camera, font, 0.06) && handle_mouse && get_time() as f32 > *start_time {
					assets.play_sound(BACKWARD);
					ret = Some(State::ExitConfirmation(Box::new(State::Serious(board.clone(), *start_time, *finished_time, *sounds))))
				}

				if *start_time > get_time() as f32 && !board.is_generating {
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
					
					if countdown > 0.0 && 3.0 - (*sounds as f32) >= countdown {
						*sounds += 1;
						assets.play_sound(TICK);
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
					assets.play_sound(BACKWARD);
				}
				if button(&Rect { x: 0.55, y: 0.55, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "No", &cam, font, 0.07) {
					ret = Some((**inner_state).clone());
					assets.play_sound(FORWARD);
				}
			}
			Self::EndScreen(inner_state, highscore) => {
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
							State::Serious(board, _, time, _) => {
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
					assets.play_sound(FORWARD);
					let id = assets.next_board_id;
					assets.next_board_id += 1;
					match &**inner_state {
						State::Serious(b, _, _, _) => {
							assets.sender.send((b.size, GameMode::Serious, id)).unwrap();
							ret = Some(State::Serious(Board::new(b.size, id, true), get_time() as f32 + 1.5, None, 0));
						}
						State::Learn(b) => {
							assets.sender.send((b.size, GameMode::Learn, id)).unwrap();
							let board = Board::new(b.size, id, true);
							ret = Some(State::Learn(board));
						}
						_ => {
							ret = Some(State::Learn(Board::new(6, id, true)));
						}
					}
				}
				if button(&Rect { x: 0.25, y: 0.75, w: 0.5, h: 0.1 }, SEC_BUTTON_COL, "Back", &cam, font, 0.07) {
					assets.play_sound(BACKWARD);
					ret = Some(State::MainMenu);
				}
			}
			Self::Highscores => {
				
				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				if button(&Rect { x: 0.8, y: -0.1, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					assets.play_sound(BACKWARD);
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

				let display_rect = rect_circumscribed_on_rect(Rect { x: -1.1, y: -1.3, w: 2.4, h: 2.6 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				board.draw_errors(None);
				board.draw_hint();
				board.draw(assets);


				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				if button(&Rect { x: 0.8, y: -0.1, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					assets.play_sound(BACKWARD);
					ret = Some(State::MainMenu);
					assets.persistance.save();
				}
				if button(&Rect { x: 0.0, y: -0.1, w: 0.2, h: 0.1 }, PRI_BUTTON_COL, "Reset", &camera, font, 0.06) {
					assets.play_sound(FORWARD);
					assets.persistance.color0 = DARKGRAY.into();
					assets.persistance.color1 = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }.into();
					assets.persistance.color2 = Color { r: 0.0, g: 0.5, b: 1.0, a: 1.0 }.into();
				}

				let mut v = assets.persistance.music_volume;
				slider(&mut v, 0.0, 2.0, vec2(0.45, 0.15), 0.55, SLIDER_COL, &camera);
				draw_centered_text(vec2(0.725, 0.07), "Music Volume", font, 0.07);
				if v != assets.persistance.music_volume {
					assets.persistance.music_volume = v;
					assets.update_volume();
				}

				let mut v = assets.persistance.sfx_volume;
				slider(&mut v, 0.0, 2.0, vec2(0.45, 0.35), 0.55, SLIDER_COL, &camera);
				draw_centered_text(vec2(0.725, 0.27), "Sfx Volume", font, 0.07);
				if v != assets.persistance.sfx_volume {
					assets.persistance.sfx_volume = v;
				}

				slider(&mut assets.persistance.color0[0], 0.0, 1.0, vec2(-0.05, 0.15), 0.3, color_u8!(255, 0, 0, 255), &camera);
				slider(&mut assets.persistance.color0[1], 0.0, 1.0, vec2(-0.05, 0.24), 0.3, color_u8!(0, 255, 0, 255), &camera);
				slider(&mut assets.persistance.color0[2], 0.0, 1.0, vec2(-0.05, 0.33), 0.3, color_u8!(0, 0, 255, 255), &camera);

				slider(&mut assets.persistance.color1[0], 0.0, 1.0, vec2(-0.05, 0.46), 0.3, color_u8!(255, 0, 0, 255), &camera);
				slider(&mut assets.persistance.color1[1], 0.0, 1.0, vec2(-0.05, 0.55), 0.3, color_u8!(0, 255, 0, 255), &camera);
				slider(&mut assets.persistance.color1[2], 0.0, 1.0, vec2(-0.05, 0.64), 0.3, color_u8!(0, 0, 255, 255), &camera);

				slider(&mut assets.persistance.color2[0], 0.0, 1.0, vec2(-0.05, 0.77), 0.3, color_u8!(255, 0, 0, 255), &camera);
				slider(&mut assets.persistance.color2[1], 0.0, 1.0, vec2(-0.05, 0.86), 0.3, color_u8!(0, 255, 0, 255), &camera);
				slider(&mut assets.persistance.color2[2], 0.0, 1.0, vec2(-0.05, 0.95), 0.3, color_u8!(0, 0, 255, 255), &camera);
			}
			Self::Attribution => {
				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);

				if button(&Rect { x: 0.8, y: -0.1, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					assets.play_sound(BACKWARD);
					ret = Some(State::MainMenu);
				}

				draw_centered_text(vec2(0.5, 0.2), "Programming by gremble", font, 0.07);
				draw_centered_text(vec2(0.5, 0.4), "Backgrounds by vivavolt and", font, 0.07);
				draw_centered_text(vec2(0.5, 0.5), "mrange on shadertoy.com", font, 0.07);
				draw_centered_text(vec2(0.5, 0.7), "Music by FASSounds, AlexiAction", font, 0.07);
				draw_centered_text(vec2(0.5, 0.8), "and SoulProdMusic on pixabay.com", font, 0.07);
			}
			Self::ModeInfo(mode) => {
				Self::update(&mut Self::MainMenu, assets, false);

				let display_rect = rect_circumscribed_on_rect(Rect { x: -0.1, y: -0.2, w: 1.2, h: 1.3 }, screen_width()/screen_height());
				let camera = Camera2D::from_display_rect(display_rect);
				set_camera(&camera);


				draw_rectangle(display_rect.x, display_rect.y, display_rect.w, display_rect.h, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.8 });
				
				let m = 0.01;
				draw_round_rect(0.0-m, 0.1-m, 1.0+2.0*m, 0.8+2.0*m, 0.05+m, POPUP_EDGE_COL);
				draw_round_rect(0.0,   0.1,   1.0,       0.8,       0.05,   POPUP_COL);
				
				draw_centered_text(vec2(0.5, 0.2), mode.as_str(), font, 0.1);
				for (i, line) in mode.info().iter().enumerate() {
					draw_centered_text(vec2(0.5, 0.4+i as f32 * 0.1), *line, font, 0.08);
				}


				if button(&Rect { x: 0.8, y: -0.1, w: 0.2, h: 0.1 }, SEC_BUTTON_COL, "Back", &camera, font, 0.06) {
					assets.play_sound(BACKWARD);
					ret = Some(State::MainMenu);
				}

			}
		}
		
		if let Some(Self::MainMenu) = ret {
			assets.change_material();
		}

		ret
	}

	pub fn capture_generated_map(&mut self, map_size: usize, map: Vec<Vec<CellState>>, id: usize, time: f32) {
		match self {
			Self::Learn(board) => {
				if board.id != id || board.size != map_size || !board.is_generating { return; }
				board.map = map;
				board.is_generating = false;
				board.generation_end_time = get_time() as f32;
				board.generation_duration = time;
			}
			Self::Serious(board, start, _, _) => {
				if board.id != id || board.size != map_size || !board.is_generating { return; }
				board.map = map;
				board.is_generating = false;
				*start = get_time() as f32 + 1.5;
				board.generation_end_time = get_time() as f32;
				board.generation_duration = time;
			}
			_ => {}
		}
	}
}