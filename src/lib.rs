use macroquad::prelude::Color;

pub mod board;
pub mod cell_state;
pub mod state;
pub mod ui;
pub mod assets;

pub const FONT: &[u8] = include_bytes!("../assets/Jellee-Bold.ttf");
pub const GRADIENT: &[u8] = include_bytes!("../assets/gradient.png");

const PRI_BUTTON_COL: Color = Color {r: 0.00, g: 0.38, b: 0.91, a: 0.5};
const SEC_BUTTON_COL: Color = Color {r: 0.48, g: 0.54, b: 0.68, a: 0.5};

const SLIDER_COL: Color = Color {r: 0.48, g: 0.54, b: 0.68, a: 0.7};

const POPUP_EDGE_COL: Color = Color {r: 0.1, g: 0.1, b: 0.1, a: 1.0};
const POPUP_COL: Color = Color {r: 0.3, g: 0.3, b: 0.3, a: 1.0};