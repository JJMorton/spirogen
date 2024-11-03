use std::f64::consts::PI;

use crate::{maths::Transform2D, shapes::ParametricShape};

/// Compute transform to apply to a shape when using as a wheel attached to a guide
pub fn transform_for_wheel(
	wheel: &dyn ParametricShape,
	guide: &dyn ParametricShape,
	inside: bool,
	s: f64
) -> Transform2D {

	// If the wheel is on the outside, it will rotate the opposite way around
	let s_wheel = (if inside {1.0} else {-1.0}) * s;

	// Compute the normal to the surface at each shape's contact point
	let norm_guide = guide.normal_at(s);
	let norm_wheel = wheel.normal_at(s_wheel);

	// The rotation of this shape to make contact with the other
	let theta =
		norm_guide.heading() - norm_wheel.heading()
		+ (if inside {0.0} else {PI});

	// Now construct the transform...
	let mut t = Transform2D::identity();

	// Rotate this shape to align the normals
	t = Transform2D::rotation_xy(theta) * t;

	// Move the centre of this shape around the guide's perimeter
	t = Transform2D::translation(guide.parametric(s)) * t;

	// Move this shape outwards/inwards to align the edges
	// 1. Find the spoke from the wheel's centre to its contact point
	// 2. Rotate this vector to align to the guide's normal
	// 3. Flip it (rotation by pi)
	// 4. Translate this shape by the resulting vector
	t = Transform2D::translation(
		Transform2D::rotation_xy(PI + theta) * wheel.parametric(s_wheel)
	) * t;

	t
}


pub fn transform_for_pen(
	wheel: &dyn ParametricShape,
	theta: f64,
	radius: f64,
) -> Transform2D {
	Transform2D::translation(
		wheel.parametric(0.5 * theta / PI * wheel.perimeter()) * radius.clamp(0.0, 1.0)
	)
}
