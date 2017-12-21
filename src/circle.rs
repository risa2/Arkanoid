extern crate sdl2;

use sdl2::rect::{Rect, Point};
use geometry;

pub enum Collision {
	None,
	At(i32, i32)
}

#[derive(Copy, Clone)]
pub struct Circle {
	pub x: i32,
	pub y: i32,
	pub radius: f32
}

impl Circle {
	pub fn corner(&self)->(i32, i32) {
		(self.x-self.radius as i32, self.y-self.radius as i32)
	}
	pub fn center(&self)->(i32, i32) {
		(self.x, self.y)
	}
	pub fn to_point(&self)->Point {
		Point::new(self.x, self.y)
	}
	pub fn to_rect(&self)->Rect {
		Rect::new(self.corner().0, self.corner().1, self.radius as u32*2, self.radius as u32*2)
	}
	pub fn collision(&self, rect: Rect)->Collision {
		if self.to_rect().has_intersection(rect) {
			let (left, up)=(rect.x, rect.y);
			let (right, down)=(rect.x+rect.w as i32, rect.y+rect.h as i32);
			let size=self.radius as i32*2;

			let left_up=Rect::new(left-size, up-size, size as u32, size as u32);
			let right_up=Rect::new(right, up-size, size as u32, size as u32);
			let left_down=Rect::new(left-size, down, size as u32, size as u32);
			let right_down=Rect::new(right, down, size as u32, size as u32);

			let left_r=Rect::new(left-size, up, size as u32, rect.h as u32);
			let up_r=Rect::new(left, up-size, rect.w as u32, size as u32);
			let right_r=Rect::new(right, up, size as u32, rect.h as u32);
			let down_r=Rect::new(left, down, rect.w as u32, size as u32);

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
			else if left_r.contains_point(self.to_point())	{Collision::At(left, self.y)}
			else if right_r.contains_point(self.to_point())	{Collision::At(right, self.y)}
			else if up_r.contains_point(self.to_point())	{Collision::At(self.x, up)}
			else if down_r.contains_point(self.to_point())	{Collision::At(self.x, down)}
			else {Collision::None}
		}
		else {Collision::None}
	}
}