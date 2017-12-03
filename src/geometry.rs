use std::f32;

#[derive(Copy, Clone)]
pub struct Circle {
	pub x: i32,
	pub y: i32,
	pub radius: f32
}

impl Circle {
	pub fn corner(&self)->(i32, i32) {
		(self.x-self.radius as i32/2, self.y-self.radius as i32/2)
	}
	pub fn center(&self)->(i32, i32) {
		(self.x, self.y)
	}
}

pub const PI:f32=3.14159265358;
pub fn split(lenght: u32, width: u32, index: u32)->u32 {
    index*lenght/width
}
pub fn to_cartesian(lenght: f32, angle: f32)->(i32, i32) {
    ((lenght*angle.cos()) as i32, (lenght*angle.sin()) as i32)
}

pub fn distance(a: (i32, i32), b:(i32, i32))->f32 {
    (((a.0-b.0)*(a.0-b.0)+(a.1-b.1)*(a.1-b.1)) as f32).sqrt()
}

pub fn horizontal_bounce(angle: f32)->f32 {
    if angle<=PI {PI-angle} else {PI-angle+2.0*PI}
}
pub fn vertical_bounce(angle: f32)->f32 {
    2.0*PI-angle
}
pub fn bounce(move_angle: f32, kolmice: f32)->f32 {
    if kolmice-(move_angle-kolmice)<=PI {kolmice-(move_angle-kolmice)+PI} else {kolmice-(move_angle-kolmice)-PI}
}

fn shift_angle_float(shift: (f32, f32))->f32 {
	if shift.1.atan2(shift.0)<0.0 {2.0*PI+shift.1.atan2(shift.0)} else {shift.1.atan2(shift.0)}
}

fn shift_angle(shift: (i32, i32))->f32 {
	shift_angle_float((shift.0 as f32, shift.1 as f32))
}

pub fn line_angle(begin: (i32, i32), end: (i32, i32))->f32 {
	shift_angle((end.0-begin.0, end.1-begin.1))
}


fn eq_float(a: f32, b: f32)->bool {
	(a-b).abs()<0.001
}

#[test]
fn test_horizontal_bounce() {
	assert!(eq_float(horizontal_bounce(PI/4.0), PI*3.0/4.0));
	assert!(eq_float(horizontal_bounce(PI*7.0/4.0), PI*5.0/4.0));
	
	assert!(eq_float(horizontal_bounce(PI*3.0/4.0), PI/4.0));
	assert!(eq_float(horizontal_bounce(PI*5.0/4.0), PI*7.0/4.0));
}

#[test]
fn test_vertical_bounce() {
	assert!(eq_float(vertical_bounce(PI/4.0), PI*7.0/4.0));
	assert!(eq_float(vertical_bounce(PI*7.0/4.0), PI/4.0));
	
	assert!(eq_float(vertical_bounce(PI*3.0/4.0), PI*5.0/4.0));
	assert!(eq_float(vertical_bounce(PI*5.0/4.0), PI*3.0/4.0));
}

#[test]
fn test_line_angle() {
	assert!(eq_float(line_angle((1, 1), (0, 0)), PI*5.0/4.0));
	assert!(eq_float(line_angle((0, 0), (1, 1)), PI/4.0));
	assert!(eq_float(line_angle((0, 0), (0, 1)), PI/2.0));
	assert!(eq_float(line_angle((0, 0), (1, 0)), 0.0));
}

#[test]
fn test_bounce() {
	assert!(eq_float(bounce(PI, 0.0), 0.0));
	assert!(eq_float(bounce(0.0, PI*3.0/4.0), PI/2.0));
	assert!(eq_float(bounce(PI*3.0/2.0, PI/4.0), 0.0));
	assert!(eq_float(bounce(PI*3.0/4.0, 0.0), PI/4.0));
	assert!(eq_float(bounce(PI*5.0/4.0, PI/2.0), PI*3.0/4.0));
}