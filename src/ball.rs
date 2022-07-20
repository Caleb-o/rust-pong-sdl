use sdl2::{rect::{Rect, Point}, render::WindowCanvas};

use crate::paddle::Paddle;

const SIZE: u32 = 16;
pub const SPEED: f32 = 500.0;

pub enum Scorer {
	Noone, Ai, Player,
}

pub struct Ball {
	origin: Point,
	pos_offset: Point,
	pub pos_bounds: Rect,
}

impl Ball {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			origin: Point::new(x, y),
			pos_offset: Point::new(0, 0),
			pos_bounds: Rect::new(x, y, SIZE, SIZE),
		}
	}

	// Note: These numbers are not absolute positions, they're relative
	pub fn move_offset(&mut self, x: i32, y: i32) {
		self.pos_offset.x = x;
		self.pos_offset.y = y;
	}

	pub fn tick(&mut self, delta_time: f32, window_size: (i32, i32)) -> Scorer {
		// Note: current_pos is used as an offset rather than absolute position
		let new_x = (self.pos_offset.x as f32 * delta_time * SPEED) as i32;
		let mut new_y = (self.pos_offset.y as f32 * delta_time * SPEED) as i32;
		
		// let pos_x = self.pos_bounds.x;
		let pos_x = self.pos_bounds.x;
		let pos_y = self.pos_bounds.y;

		// Don't allow the ball offscreen, it needs to flip its position to "bounce"
		if pos_y + new_y <= 0 || pos_y + new_y >= window_size.1 - self.pos_bounds.height() as i32 {
			new_y = -new_y;
			// ;-;
			self.pos_offset.y *= -1;
		}

		self.pos_bounds.offset(new_x, new_y);
		
		// TEMP
		if pos_x + new_x <= 0 || pos_x + new_x >= window_size.0 - self.pos_bounds.width() as i32 {
			self.pos_offset.x *= -1;
			self.pos_offset.y *= -1;

			// Reset
			self.pos_bounds.set_x(self.origin.x);
			self.pos_bounds.set_y(self.origin.y);

			if pos_x + new_x <= 0 {
				return Scorer::Ai;
			} else {
				return Scorer::Player;
			}
		}

		Scorer::Noone
	}

	pub fn check_collision(&mut self, paddle: &Paddle) {
		if let Some(intersection) = self.pos_bounds.intersection(paddle.pos_bounds) {
			self.pos_offset.x *= -1;

			// Ball Bottom hit Top Paddle
			if intersection.y >= paddle.pos_bounds.y && intersection.y <= paddle.pos_bounds.y as i32 + 8 {
				self.pos_offset.y *= -1;
			}
			// Ball Top hit Bottom Paddle
			else if intersection.y >= (paddle.pos_bounds.y + paddle.pos_bounds.height() as i32) - 8 &&
			intersection.y <= paddle.pos_bounds.y + paddle.pos_bounds.height() as i32 {
				self.pos_offset.y *= -1;
			}
		}
	}

	pub fn render(&self, canvas: &mut WindowCanvas) {
		canvas.fill_rect(self.pos_bounds).unwrap();
	}
}