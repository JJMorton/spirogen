use std::ops::{Add, Div, Mul, Sub};

use serde::{ser::SerializeTuple, Serialize};

/// Holds a 2D coordinate
#[derive(Copy, Clone)]
pub struct Coordinate {
	pub x: f64,
	pub y: f64,
}

/// Matrix transform for a 2D coordinate
#[derive(Copy, Clone)]
pub struct Transform2D {
	/// Matrix indexed by (row, col)
	pub matrix: [[f64; 3]; 3]
}

/// A domain from which equally spaced values are taken
#[derive(Copy, Clone)]
pub struct Linspace {
	pub lower: f64,
	pub upper: f64,
	pub count: usize,
	index: usize,
}


// ==================


impl Coordinate {
	/// The null vector
	pub fn null() -> Coordinate {
		Coordinate { x: 0.0, y: 0.0}
	}
	/// Rotate by the angle `theta`
	pub fn rotated(&self, theta: f64) -> Coordinate {
		Transform2D::rotation_xy(theta) * *self
	}
	/// Create new vector with a magnitude of unity
	pub fn normalised(&self) -> Coordinate {
		let mag = self.magnitude();
		Coordinate { x: self.x / mag, y: self.y / mag }
	}
	/// Magnitude of this vector
	pub fn magnitude(&self) -> f64 {
		(self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
	}
	/// Get angle of this vector
	pub fn heading(&self) -> f64 {
		self.y.atan2(self.x)
	}
}

impl Add for Coordinate {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
	    Coordinate { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

impl Mul<f64> for Coordinate {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
	    Coordinate { x: self.x * rhs, y: self.y * rhs }
	}
}

impl Div<f64> for Coordinate {
	type Output = Self;
	fn div(self, rhs: f64) -> Self::Output {
	    self * (1.0 / rhs)
	}
}

impl Sub for Coordinate {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
	    self + (rhs * -1.0)
	}
}

impl Serialize for Coordinate {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
	    let mut tuple = serializer.serialize_tuple(2)?;
	    tuple.serialize_element(&self.x)?;
	    tuple.serialize_element(&self.y)?;
	    tuple.end()
	}
}


impl Transform2D {
	/// The identity matrix
	pub fn identity() -> Transform2D {
		Transform2D {
			matrix: [
				[1.0, 0.0, 0.0],
				[0.0, 1.0, 0.0],
				[0.0, 0.0, 1.0],
			]
		}
	}
	/// A matrix full of zeroes
	pub fn null() -> Transform2D {
		Transform2D {
			matrix: [
				[0.0, 0.0, 0.0],
				[0.0, 0.0, 0.0],
				[0.0, 0.0, 0.0],
			]
		}
	}
	/// A rotation in the x-y plane
	pub fn rotation_xy(theta: f64) -> Transform2D {
		let cos = theta.cos();
		let sin = theta.sin();
		Transform2D {
			matrix: [
				[cos, -sin, 0.0],
				[sin,  cos, 0.0],
				[0.0,  0.0, 1.0],
			]
		}
	}
	/// A translation in the x-y plane
	pub fn translation(dr: Coordinate) -> Transform2D {
		Transform2D {
			matrix: [
				[1.0, 0.0, dr.x],
				[0.0, 1.0, dr.y],
				[0.0, 0.0, 1.0]
			]
		}
	}
}

/// Matrix-vector multiplication
impl Mul<Coordinate> for Transform2D {
	type Output = Coordinate;
	fn mul(self, rhs: Coordinate) -> Self::Output {
		let m = self.matrix;
		Coordinate {
			x: m[0][0] * rhs.x + m[0][1] * rhs.y + m[0][2],
			y: m[1][0] * rhs.x + m[1][1] * rhs.y + m[1][2],
		}
	}
}

/// Matrix-matrix multiplication
impl Mul<Transform2D> for Transform2D {
	type Output = Transform2D;
	fn mul(self, rhs: Transform2D) -> Self::Output {
		let mut t = Transform2D::null();
		for i in 0..3 {
			for j in 0..3 {
				for k in 0..3 {
					t.matrix[i][j] += self.matrix[i][k] * rhs.matrix[k][j]
				};
			}
		};
		t
	}
}

/// Matrix-vector multiplication to an array of vectors
impl Mul<Vec<Coordinate>> for Transform2D {
	type Output = Vec<Coordinate>;
	fn mul(self, rhs: Vec<Coordinate>) -> Self::Output {
        rhs.iter().map(|c| self * *c).collect()
	}
}


impl Linspace {
	pub fn new(lower: f64, upper: f64, count: usize) -> Linspace {
		Linspace {lower, upper, count, index: 0}
	}
}

impl Iterator for Linspace {
	type Item = f64;

	fn next(&mut self) -> Option<f64> {
		if self.index > self.count {
			return Option::None;
		}
		let v = self.lower + (self.upper - self.lower) * self.index as f64 / self.count as f64;
		self.index += 1;
		return Option::Some(v);
	}
}


