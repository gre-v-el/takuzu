pub mod board;
pub mod cell_state;
pub mod state;
pub mod ui;
pub mod assets;

pub const FONT: &[u8] = include_bytes!("../assets/Jellee-Bold.ttf");
pub const GRADIENT: &[u8] = include_bytes!("../assets/gradient.png");