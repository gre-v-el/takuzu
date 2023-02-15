use egui_macroquad::macroquad::prelude::*;

pub fn rect_circumscribed_on_rect(rect: Rect, aspect: f32) -> Rect {
	let rect_aspect = rect.w / rect.h;

	if aspect > rect_aspect {
		let center = rect.center();
		let new_w = rect.w / rect_aspect * aspect;
		Rect {
			x: center.x - new_w/2.0,
			y: rect.y,
			w: new_w,
			h: rect.h,
		}
	}
	else {
		let center = rect.center();
		let new_h = rect.h * rect_aspect / aspect;
		Rect {
			x: rect.x,
			y: center.y - new_h / 2.0,
			w: rect.w,
			h: new_h,
		}
	}
}

pub fn draw_round_rect(x: f32, y: f32, w: f32, h: f32, r: f32, col: Color) {
	draw_rectangle(x + r, y as f32, w - 2.0*r, h, col);
	draw_rectangle(x, y as f32 + r, w, h - 2.0*r, col);

	draw_circle(x + r, 		y + r, 		r, col);
	draw_circle(x + w - r, 	y + r, 		r, col);
	draw_circle(x + r,		y + h - r, 	r, col);
	draw_circle(x + w - r, 	y + h - r, 	r, col);
}


pub fn button(rect: &Rect, mut col: Color, text: &str, camera: &Camera2D, font: Font, scale: f32) -> bool {
	let mouse = camera.screen_to_world(mouse_position().into());
	if rect.contains(mouse) && is_mouse_button_down(MouseButton::Left) {
		col.r -= 0.1;
		col.g -= 0.1;
		col.b -= 0.1;
	}
	else if rect.contains(mouse) {
		col.r += 0.1;
		col.g += 0.1;
		col.b += 0.1;
	}
	draw_round_rect(rect.x, rect.y, rect.w, rect.h, 0.01, col);

	draw_centered_text(rect.center(), text, font, scale);

	return is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse);
}

pub fn draw_centered_text(center: Vec2, text: &str, font: Font, scale: f32) {
	let dims = measure_text(text, Some(font), 128, 1.0/128.0 * scale);

	draw_text_ex(text, center.x - dims.width/2.0, center.y + dims.height/2.0, TextParams { 
	   font: font, 
	   font_size: 128,
	   font_scale: 1.0/128.0 * scale, 
	   font_scale_aspect: 1.0, 
	   rotation: 0.0, 
	   color: WHITE 
   });
}

pub fn draw_centered_text_color(center: Vec2, text: &str, font: Font, scale: f32, col: Color) {
	let dims = measure_text(text, Some(font), 128, 1.0/128.0 * scale);

	draw_text_ex(text, center.x - dims.width/2.0, center.y + dims.height/2.0, TextParams { 
	   font: font, 
	   font_size: 128,
	   font_scale: 1.0/128.0 * scale, 
	   font_scale_aspect: 1.0, 
	   rotation: 0.0, 
	   color: col
   });
}

pub fn draw_centered_text_stable(center: Vec2, text: &str, format: &str, font: Font, scale: f32) {
	let dims = measure_text(format, Some(font), 128, 1.0/128.0 * scale);

	draw_text_ex(text, center.x - dims.width/2.0, center.y + dims.height/2.0, TextParams { 
	   font: font, 
	   font_size: 128,
	   font_scale: 1.0/128.0 * scale, 
	   font_scale_aspect: 1.0, 
	   rotation: 0.0, 
	   color: WHITE 
   });
}