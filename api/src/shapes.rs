use std::f64::{consts::PI, INFINITY};

use serde::{Deserialize, Serialize};

use crate::maths::{Coordinate, Linspace};


/// A shape defined by a parametric equation t -> (x, y)
pub trait ParametricShape {

	/// The parametric equation, s in [0, perimeter]
	fn parametric(&self, s: f64) -> Coordinate;

	/// Compute the total perimeter of the shape
	fn perimeter(&self) -> f64;

	/// Minumum radius of curvature of the shape
	fn min_radius(&self) -> f64;

	/// Maximum radius of curvature of the shape
	fn max_radius(&self) -> f64;

	/// Rasterise the shape, giving coordinates along the path
	fn rasterise(&self, resolution: usize) -> Vec<Coordinate> {
		Linspace::new(0.0, self.perimeter() * 0.95, resolution)
			.map(|t| self.parametric(t))
			.collect()
	}

	/// Compute the normal to the shape at distance `s`
	fn normal_at(&self, s: f64) -> Coordinate {
		let eps = 0.0001;
		(self.parametric(s) - self.parametric(s - eps))
			.rotated(-PI * 0.5)
			.normalised()
	}

}

/// A basic circle
#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Circle {
	/// Radius of the circle
	pub radius: f64,
}

/// A straight rod with rounded ends
#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Rod {
	/// Length from centre to cap
	pub major_radius: f64,

	/// Width to length ratio
	pub aspect_ratio: f64,
}


// ==================


impl Circle {
	pub fn new(radius: f64) -> Circle {
		Circle {radius}
	}
}

impl ParametricShape for Circle {

	fn perimeter(&self) -> f64 {
		2.0 * PI * self.radius
	}

	fn min_radius(&self) -> f64 { self.radius }

	fn max_radius(&self) -> f64 { self.radius }

	fn parametric(&self, s: f64) -> Coordinate {
		let mut t = (s / self.perimeter()) % 1.0;
		if t < 0.0 { t += 1.0; }
		// 0 <= t <= 1
	    Coordinate {
	    	x: self.radius * (2.0 * PI * t).cos(),
	    	y: self.radius * (2.0 * PI * t).sin()
	    }
	}
}

impl Rod {
	pub fn new(major_radius: f64, aspect_ratio: f64) -> Rod {
		Rod {major_radius, aspect_ratio}
	}

	fn side_length(&self) -> f64 {
		2.0 * self.major_radius * (1.0 - self.aspect_ratio)
	}

	fn cap_radius(&self) -> f64 {
		self.aspect_ratio * 2.0 * self.major_radius
	}
}

impl ParametricShape for Rod {

	fn perimeter(&self) -> f64 {
	    2.0 * PI * self.cap_radius() + 4.0 * self.side_length()
	}

	fn min_radius(&self) -> f64 { self.cap_radius() }

	fn max_radius(&self) -> f64 { INFINITY }

	fn parametric(&self, s: f64) -> Coordinate {
		let side_length = self.side_length();
		let cap_radius = self.cap_radius();
		let cap_length = PI * cap_radius;

		// Make t=0 correspond with the centre of a straight edge
		let perim = self.perimeter();
		let mut t = (perim + s - side_length) % perim;
		if t < 0.0 { t += perim; }
		// 0 <= t <= perimeter

		// Right circular cap
		if t < cap_length {
			let alpha = t / cap_radius;
			return Coordinate {
				x: -cap_radius * alpha.sin() - side_length,
				y: cap_radius * alpha.cos(),
			};
		}
		// Bottom straight edge
		else if t < cap_length + 2.0 * side_length {
			return Coordinate {
				x: -side_length + t - PI * cap_radius,
				y: -cap_radius,
			};
		}
		// Left circular cap
		else if t < 2.0 * cap_length + 2.0 * side_length {
			let alpha = (t - 2.0 * side_length) / cap_radius;
			return Coordinate {
				x: -cap_radius * alpha.sin() + side_length,
				y: cap_radius * alpha.cos(),
			};
		}
		// Top straight edge
		else {
			return Coordinate {
				x: 3.0 * side_length - t + 2.0 * PI * cap_radius,
				y: cap_radius,
			};
		}
	}
}
