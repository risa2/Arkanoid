use std::f32;

pub const PI:f32=3.14159265358;

pub fn to_rad(deg: f32)->f32 {
	deg/180.0*PI
}

pub fn split(lenght: i32, width: i32, index: i32)->i32 {
    index*lenght/width
}
pub fn to_cartesian(lenght: f32, angle: f32)->(f32, f32) {
    (lenght*angle.cos(), lenght*angle.sin())
}

fn relative_distance(dist: (i32, i32))->f32 {
    ((dist.0*dist.0+dist.1*dist.1) as f32).sqrt()
}

pub fn distance(a: (i32, i32), b:(i32, i32))->f32 {
	relative_distance((a.0-b.0, a.1-b.1))
}

fn check(angle: f32)->f32 {
	if angle>PI*2.0 {angle-2.0*PI} else {angle}
}

pub fn bounce(move_angle: f32, kolmice: f32)->f32 {
    check(kolmice-(move_angle-kolmice)+PI)
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

#[macro_export]
macro_rules! assert_eq_float {
	($a:expr, $b:expr) => (assert!((a-b).abs()<0.0001))
}

#[test]
fn test_horizontal_bounce() {
	assert_eq_float!(horizontal_bounce(PI/4.0), PI*3.0/4.0);
	assert_eq_float!(horizontal_bounce(PI*7.0/4.0), PI*5.0/4.0);
	
	assert_eq_float!(horizontal_bounce(PI*3.0/4.0), PI/4.0);
	assert_eq_float!(horizontal_bounce(PI*5.0/4.0), PI*7.0/4.0);
}

#[test]
fn test_vertical_bounce() {
	assert_eq_float!(vertical_bounce(PI/4.0), PI*7.0/4.0);
	assert_eq_float!(vertical_bounce(PI*7.0/4.0), PI/4.0);
	
	assert_eq_float!(vertical_bounce(PI*3.0/4.0), PI*5.0/4.0);
	assert_eq_float!(vertical_bounce(PI*5.0/4.0), PI*3.0/4.0);
}

#[test]
fn test_line_angle() {
	assert_eq_float!(line_angle((1, 1), (0, 0)), PI*5.0/4.0);
	assert_eq_float!(line_angle((0, 0), (1, 1)), PI/4.0);
	assert_eq_float!(line_angle((0, 0), (0, 1)), PI/2.0);
	assert_eq_float!(line_angle((0, 0), (1, 0)), 0.0);
}

#[test]
fn test_bounce() {
	assert_eq_float!(bounce(PI, 0.0), 0.0);
	assert_eq_float!(bounce(PI/4.0, PI), PI*3.0/4.0);
	assert_eq_float!(bounce(0.0, PI*3.0/4.0), PI/2.0);
	assert_eq_float!(bounce(PI*3.0/2.0, PI/4.0), 0.0);
	assert_eq_float!(bounce(PI*3.0/4.0, 0.0), PI/4.0);
	assert_eq_float!(bounce(PI*5.0/4.0, PI/2.0), PI*3.0/4.0);
}