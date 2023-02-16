use std::{time::{SystemTime, UNIX_EPOCH}, collections::BTreeMap};
use macroquad::{self, prelude::*};
use takuzu::{state::State, Assets, Persistance};

/*
	TODO:
		add some nice background (shaders?) and glossy effect for tiles (maybe, test it in shadertoy)
		let the user choose board size
		come up with a difficulty metric and decide what is better: PURGE vs DEGENERATE + PURGE
		improve solving with the last rule
		change colors (error highlight consumes the red tiles)

		serde persistance
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
	let persistance = Persistance {
		highscores: BTreeMap::from([(2, 5.0), (4, 11.0), (6, 40.0), (8, 100.0), (10, 200.0), (12, 400.0), (14, 700.0), (16, 1000.0), (18, 1500.0), (20, 2500.0)]),
		color0: GRAY,
		color1: Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 },
		color2: Color { r: 0.0, g: 0.5, b: 1.0, a: 1.0 },
	};
	let mut assets = Assets {
		font: load_ttf_font_from_bytes(takuzu::FONT).unwrap(),
		gradient: Texture2D::from_file_with_format(takuzu::GRADIENT, None),
		persistance,
	};

    loop {
		if let Some(s) = state.update(&mut assets, true) {
			state = s;
		}

        next_frame().await
    }
}