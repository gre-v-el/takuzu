use std::time::{SystemTime, UNIX_EPOCH};
use egui_macroquad::macroquad::{self, prelude::*};
use takuzu::{state::State, Assets};

/*
	TODO:
		add some nice background (shaders?) and glossy effect for tiles (maybe, test it in shadertoy)
		some high scores?
		the timer shouldn't twitch
		come up with a difficulty metric and decide what is better: PURGE vs DEGENERATE + PURGE
		improve solving with the last rule
*/

fn window_config() -> Conf {
    Conf {
        window_title: "Takuzu".into(),
        fullscreen: false,
		window_width: 500,
		window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64);

	let mut state = State::MainMenu;
	let assets = Assets {
		font: load_ttf_font_from_bytes(takuzu::FONT).unwrap(),
		gradient: Texture2D::from_file_with_format(takuzu::GRADIENT, None),
	};

    loop {
		if let Some(s) = state.update(&assets) {
			state = s;
		}

        next_frame().await
    }
}