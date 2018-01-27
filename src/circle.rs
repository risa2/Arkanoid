extern crate sdl2;

use geometry;
use sdl2::gfx::primitives::DrawRenderer;

#[derive(Debug)]
pub enum Collision {
	None,
	At(i32, i32)
}

#[derive(Copy, Clone)]
pub struct Circle {
	pub x: f32,
	pub y: f32,
	pub radius: f32
}

impl Circle {
	pub fn corner(&self)->(i32, i32) {
		(self.x as i32-self.radius as i32, self.y as i32-self.radius as i32)
	}
	pub fn center(&self)->(i32, i32) {
		(self.x as i32, self.y as i32)
	}
	pub fn to_point(&self)->sdl2::rect::Point {
		sdl2::rect::Point::new(self.x as i32, self.y as i32)
	}
	pub fn to_rect(&self)->sdl2::rect::Rect {
		sdl2::rect::Rect::new(self.corner().0, self.corner().1, self.radius as u32*2, self.radius as u32*2)
	}
	pub fn collision(&self, rect: sdl2::rect::Rect)->Collision {
		if self.to_rect().has_intersection(rect) {
			let (left, up)=(rect.x, rect.y);
			let (right, down)=(rect.x+rect.w as i32, rect.y+rect.h as i32);
			let size=self.radius as i32*2;

			let left_up=sdl2::rect::Rect::new(left-size, up-size, size as u32, size as u32);
			let right_up=sdl2::rect::Rect::new(right, up-size, size as u32, size as u32);
			let left_down=sdl2::rect::Rect::new(left-size, down, size as u32, size as u32);
			let right_down=sdl2::rect::Rect::new(right, down, size as u32, size as u32);

			let left_r=sdl2::rect::Rect::new(left-size, up, size as u32, rect.h as u32);
			let up_r=sdl2::rect::Rect::new(left, up-size, rect.w as u32, size as u32);
			let right_r=sdl2::rect::Rect::new(right, up, size as u32, rect.h as u32);
			let down_r=sdl2::rect::Rect::new(left, down, rect.w as u32, size as u32);

			if left_up.contains_point(self.to_point()) {
				if geometry::distance(self.center(), (left, up))<=self.radius {Collision::At(left, up)}
				else {Collision::None}
			}
			else if right_up.contains_point(self.to_point()) {
				if geometry::distance(self.center(), (right, up))<=self.radius {Collision::At(right, up)}
				else {Collision::None}
			}
			else if left_down.contains_point(self.to_point()) {
				if geometry::distance(self.center(), (left, down))<=self.radius {Collision::At(left, down)}
				else {Collision::None}
			}
			else if right_down.contains_point(self.to_point()) {
				if geometry::distance(self.center(), (right, down))<=self.radius {Collision::At(right, down)}
				else {Collision::None}
			}
			else if left_r.contains_point(self.to_point())	{Collision::At(left, self.y as i32)}
			else if right_r.contains_point(self.to_point())	{Collision::At(right, self.y as i32)}
			else if up_r.contains_point(self.to_point())	{Collision::At(self.x as i32, up)}
			else if down_r.contains_point(self.to_point())	{Collision::At(self.x as i32, down)}
			else {Collision::None}
		}
		else {Collision::None}
	}
	pub fn render(&self, renderer: &mut sdl2::render::WindowCanvas, col: sdl2::pixels::Color) {
		renderer.filled_circle(self.x as i16, self.y as i16, self.radius as i16, col).unwrap();
		renderer.aa_circle(self.x as i16, self.y as i16, self.radius as i16, col).unwrap();
	}
}

#[test]
fn test_collision() {
	let palka=sdl2::rect::Rect::new(100, 100, 120, 10);
	let col_a=Circle{x: 140, y: 105, radius: 10.0}.collision(palka);
	if let Collision::At(x, y)=col_a {
		assert_eq!(x, 140);
	}
	else {
		assert!(false);
	}
}