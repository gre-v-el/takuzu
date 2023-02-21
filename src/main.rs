use std::{time::{SystemTime, UNIX_EPOCH}};
use macroquad::{self, prelude::*};
use takuzu::{state::State, assets::Assets};

/*
	TODO:
		improve solving with the last rule

		add generating board screen (when the generation lasts more than 3 seconds, add confirmation (if it's serious))
		tutorial
		difficulty choice (how many tiles)

		logo

		music/sfx volume
		sandbox lock/unlock
		zoom :weary:

		no countdown in serious mode when waiting for generation

		load music on a separate thread
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

		if let Ok((map, id)) = assets.receiver.try_recv() {
			state.capture_generated_map(map.len(), map, id);
		}

        next_frame().await
    }
}