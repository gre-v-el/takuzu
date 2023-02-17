use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::{self, prelude::*};
use takuzu::{state::State, assets::Assets};

/*
	TODO:
		write some background shaders
		improve solving with the last rule

		add generating board screen (when the generation lasts more than 3 seconds, add confirmation (if it's serious))
		tutorial
		show error after some delay (animation of fading and pulsing)
		difficulty choice (how many tiles)
		show locked more intuitively

		some info (what's the difference between modes, what are the rules)

		sound design and music
		logo
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
	let mut assets = Assets::get();

	let mut aspect = screen_width()/screen_height();
	assets.material.set_uniform("aspect", aspect);
    loop {
		if screen_width()/screen_height() != aspect {
			aspect = screen_width()/screen_height();
			assets.material.set_uniform("aspect", aspect);
		}
		assets.material.set_uniform("time", get_time() as f32);
		set_camera(&Camera2D::from_display_rect(Rect{x: 0.0, y: 0.0, w: 1.0, h: 1.0}));
		gl_use_material(assets.material);
		draw_rectangle(0.0, 0.0, 1.0, 1.0, WHITE);
		gl_use_default_material();
		// if let Some(s) = state.update(&mut assets, true) {
		// 	state = s;
		// }

        next_frame().await
    }
}