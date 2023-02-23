use std::f32::consts::PI;

use assets::Assets;
use macroquad::{prelude::Color, time::get_time};

pub mod board;
pub mod cell_state;
pub mod state;
pub mod ui;
pub mod assets;

const BACKGROUND_FACTOR: f32 = 5.0;

const FONT: &[u8] = include_bytes!("../assets/Jellee-Bold.ttf");
const GRADIENT: &[u8] = include_bytes!("../assets/gradient.png");
const LOCK: &[u8] = include_bytes!("../assets/lock.png");
const BANNER: &[u8] = include_bytes!("../assets/banner.png");

const MUSIC: [&[u8]; 4] = [
	include_bytes!("../assets/music/abstract-world-127012.ogg"),
	include_bytes!("../assets/music/password-infinity-123276.ogg"),
	include_bytes!("../assets/music/lofi-study-112191.ogg"),
	include_bytes!("../assets/music/unpacking-loop-ycle-138250.ogg"),
];
const MUSIC_LENGTHS: [f32; 4] = [186.0, 145.0, 147.0, 125.0];

const SFX: [&[u8]; 7] = [
	include_bytes!("../assets/sfx/pop.ogg"),
	include_bytes!("../assets/sfx/forward.ogg"),
	include_bytes!("../assets/sfx/backward.ogg"),
	include_bytes!("../assets/sfx/locked.ogg"),
	include_bytes!("../assets/sfx/error.ogg"),
	include_bytes!("../assets/sfx/hint.ogg"),
	include_bytes!("../assets/sfx/tick.ogg"),
];
const SFX_VOLUMES: [f32; 7] = [0.6, 1.0, 1.0, 1.0, 3.0, 1.0, 2.0];

const POP: usize = 0;
const FORWARD: usize = 1;
const BACKWARD: usize = 2;
const LOCKED: usize = 3;
const ERROR: usize = 4;
const HINT: usize = 5;
const TICK: usize = 6;

const PRI_BUTTON_COL: Color = Color {r: 0.00, g: 0.38, b: 0.91, a: 0.5};
const SEC_BUTTON_COL: Color = Color {r: 0.48, g: 0.54, b: 0.68, a: 0.5};

const SLIDER_COL: Color = Color {r: 0.48, g: 0.54, b: 0.68, a: 0.7};

const POPUP_EDGE_COL: Color = Color {r: 0.1, g: 0.1, b: 0.1, a: 1.0};
const POPUP_COL: Color = Color {r: 0.3, g: 0.3, b: 0.3, a: 1.0};

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
	t * b + a * (1.0-t)
}

pub fn col_lerp(a: Color, b: Color, t: f32) -> Color {
	Color { 
		r: lerp(a.r, b.r, t), 
		g: lerp(a.g, b.g, t), 
		b: lerp(a.b, b.b, t), 
		a: lerp(a.a, b.a, t) 
	}
}

pub fn generation_animation_cell_col(x: f32, y: f32, size: f32, assets: &Assets) -> Color {
	let angle = (y - size / 2.0).atan2(x - size / 2.0);
	let col = (angle / 2.0 / PI + get_time() as f32 * 0.3) % 1.0;

	let mut col = col_lerp(assets.persistance.color2.into(), assets.persistance.color1.into(), col);
	col.a = 0.3;

	col
}