use std::{time::{SystemTime, UNIX_EPOCH}};
use macroquad::{self, prelude::*, miniquad::conf::Icon};
use takuzu::{state::State, assets::Assets};

/*
	TODO:
		improve solving with the last rule

		tutorial
		difficulty choice (how many tiles)

		sandbox lock/unlock
		zoom :weary:

		sandbox algorithms multithreading

		adjust allocated spaces to really used ones
*/

fn window_config() -> Conf {
    Conf {
        window_title: "Takuzu".into(),
        fullscreen: false,
		window_width: 500,
		window_height: 500,
		icon: Some(Icon::miniquad_logo()),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64);

	let mut state = State::MainMenu;
	let mut assets = Assets::get();

    loop {
		assets.draw_material();
		assets.try_play_music();
		
		if let Some(s) = state.update(&mut assets, true) {
			state = s;
		}

		if let Ok((map, id, time)) = assets.receiver.try_recv() {
			state.capture_generated_map(map.len(), map, id, time);
		}

        next_frame().await
    }
}