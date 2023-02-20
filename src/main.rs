use std::{time::{SystemTime, UNIX_EPOCH}};
use macroquad::{self, prelude::*};
use takuzu::{state::State, assets::Assets};

/*
	TODO:
		improve solving with the last rule

		add generating board screen (when the generation lasts more than 3 seconds, add confirmation (if it's serious))
		tutorial
		difficulty choice (how many tiles)

		some info (what's the difference between modes, what are the rules)

		logo
*/

fn window_config() -> Conf {
    Conf {
        window_title: "Takuzu".into(),
        fullscreen: false,
		window_width: 500,
		window_height: 500,
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

        next_frame().await
    }
}