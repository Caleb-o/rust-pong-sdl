use sdl2::{rect::Rect, render::WindowCanvas};

const WIDTH: u32 = 16;
const HEIGHT: u32 = 160;
pub const SPEED: f32 = 800.0;

pub struct Paddle {
	y_offset: i32,
	pub pos_bounds: Rect,
}

impl Paddle {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			y_offset: 0,
			pos_bounds: Rect::new(x, y, WIDTH, HEIGHT),
		}
	}

	// Note: These numbers are not absolute positions, they're relative
	pub fn move_offset(&mut self, offset: i32) {
		self.y_offset = offset;
	}

	pub fn tick(&mut self, delta_time: f32, window_y: i32) {
		// Note: current_pos is used as an offset rather than absolute position
		let new_y = (self.y_offset as f32 * delta_time * SPEED) as i32;
		let pos_y = self.pos_bounds.y;

		// Don't allow the paddle offscreen
		if pos_y + new_y > 0 && new_y + pos_y < window_y - self.pos_bounds.height() as i32 {
			self.pos_bounds.offset(0, new_y);
		}
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		canvas.fill_rect(self.pos_bounds).unwrap();
	}
}