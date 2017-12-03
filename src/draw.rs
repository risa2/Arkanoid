extern crate sdl2;

use geometry;

use sdl2::rect::Point;
use std::f32;
use std::result::Result;

pub fn circle<T: sdl2::render::RenderTarget>(out: &mut sdl2::render::Canvas<T>, circle: geometry::Circle, detail: f32){
	let mut angle: f32=0.0;
	let mut res: Result<(), String> =Result::Ok(());
	while angle<2.0*geometry::PI {
		let (dx, dy)=geometry::to_cartesian(circle.radius, angle);
		let err=out.draw_point(Point::new(circle.x+dx as i32, circle.y+dy as i32));
		res=res.and(err);
		angle+=detail;
	}
}