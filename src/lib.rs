use macroquad::prelude::Color;

pub mod board;
pub mod cell_state;
pub mod state;
pub mod ui;
pub mod assets;

const FONT: &[u8] = include_bytes!("../assets/Jellee-Bold.ttf");
const GRADIENT: &[u8] = include_bytes!("../assets/gradient.png");
const LOCK: &[u8] = include_bytes!("../assets/lock.png");

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
const SFX_VOLUMES: [f32; 7] = [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

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