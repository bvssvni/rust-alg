#[link(name="alg")];
#[pkgid = "alg#0.1"];
#[crate_type="rlib"];

#[deny(non_camel_case_types)];
#[deny(missing_doc)];
#[feature(managed_boxes)];

//! Library for Algebra.

//			Add	Sub	Mul	Div	Neg
//	Dual2		x	x	x	x	x
//	Complex		x	x	x	x	x
//	Quaternion	x	x	x	x	x
//	Matrix4		x	x	x	x	x
//	Vector		x	x	x	x	x

//			Det	Inv	NormSq
//	Dual2		x		x
//	Complex				x
//	Quaternion			x
//	Matrix4		x	x	-
//	Vector				x

//			Eq	Zero	One
//	Dual2		x	x	x
//	Complex		x	x	x
//	Quaternion	x	x	x
//	Matrix4		x	x	x
//	Vector		x	-	-

//			Eps	Scale
//	Dual2
//	Complex
//	Quaternion
//	Matrix4		x
//	Vector
//	f32		x
//	f64		x

use std::num::{Zero, One};

/// Computes the square of length of algebraic type.
pub trait NormSq<Result> {
	/// Computes the square of the norm/length.
	fn norm_sq(&self) -> Result;
}

/// Is implemented on structures that has a determinant.
pub trait Det<Result> {
	/// Calculates the determinant of the structure.
	fn det(&self) -> Result;
}

/// Implemented on structures that can be inverted.
pub trait Inv<Result> {
	/// Creates an inverted version of the structure.
	fn inv(&self) -> Result;
}

/// Eps creates a value from f64 number.
/// This can be used to check if two numbers are closer than the eps.
pub trait Eps {
	/// Returns the value used for evaluating approximate equivalence.
	/// Creates an epsilon from a f64 number.
	fn eps(eps: f64) -> Self;

	/// Checks for equality with a custom approximate epsilon.
	fn close_eps(&self, other: &Self, eps: &Self) -> bool;
}

/// Implemented on structures that can scale under multiplication.
pub trait Scale {
	/// Creates a scaling type that scales up under multiplication.
	fn scale(&self, factor: f64) -> Self;
}

/// A Dual type is commonly used for automatic differentiation.
#[deriving(Eq, Zero)]
pub struct Dual2<T> {
	/// The real part of Dual number.
	x0 : T,
	/// The dual part of Dual number.
	x1 : T,
}

impl<T> Dual2<T> {
	/// Constructs a new dual number.
	pub fn new(x0: T, x1: T) -> Dual2<T> {
		Dual2 { x0: x0, x1: x1 }
	}
}

impl<T: Add<T, T>>
Add<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn add(&self, rhs: &Dual2<T>) -> Dual2<T> {
		Dual2 { x0: self.x0 + rhs.x0, x1: self.x1 + rhs.x1 }
	}
}

impl<T: Sub<T, T>>
Sub<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn sub(&self, rhs: &Dual2<T>) -> Dual2<T> {
		Dual2 { x0: self.x0 - rhs.x0, x1: self.x1 - rhs.x1 }
	}
}

impl<T: Mul<T, T> + Add<T, T>>
Mul<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn mul(&self, rhs: &Dual2<T>) -> Dual2<T> {
		Dual2 { x0: self.x0 * rhs.x0,
			x1: self.x0 * rhs.x1 + self.x1 * rhs.x0 }
	}
}

impl<T: Div<T, T> + Sub<T, T> + Mul<T, T>>
Div<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn div(&self, rhs: &Dual2<T>) -> Dual2<T> {
		let b2 = rhs.x0 * rhs.x0;
		Dual2 { x0: self.x0 / rhs.x0,
			x1: (self.x1 * rhs.x0 - self.x0 * rhs.x1) / b2 }
	}
}

impl<T: Neg<T>>
Neg<Dual2<T>> for Dual2<T> {
	fn neg(&self) -> Dual2<T> {
		Dual2 { x0: -self.x0, x1: -self.x1 }
	}
}

impl<T: Mul<T, T>>
NormSq<T> for Dual2<T> {
	fn norm_sq(&self) -> T {
		self.x0 * self.x0
	}
}

impl<T: Zero>
Det<bool> for Dual2<T> {
	fn det(&self) -> bool {
		!self.x0.is_zero()
	}
}

impl<T: One + Zero>
One for Dual2<T> {
	fn one() -> Dual2<T> {
		Dual2 { x0: One::one(), x1: Zero::zero() }
	}
}

/// A Complex number is commonly used for rotations in 2D.
#[deriving(Eq, Zero)]
pub struct Complex<T> {
	/// The real dimension of the complex number.
	x0: T,
	/// The imaginary dimension of the complex number.
	x1: T,
}

impl<T> Complex<T> {
	/// Creates a new Complex number.
	pub fn new(x0: T, x1: T) -> Complex<T> {
		Complex { x0: x0, x1: x1 }
	}
}

impl<T: Add<T, T>> Add<Complex<T>, Complex<T>> for Complex<T> {
	fn add(&self, rhs: &Complex<T>) -> Complex<T> {
		Complex { x0: self.x0 + rhs.x0,
			x1: self.x1 + rhs.x1 }
	}
}

impl<T: Sub<T, T>> Sub<Complex<T>, Complex<T>> for Complex<T> {
	fn sub(&self, rhs: &Complex<T>) -> Complex<T> {
		Complex { x0: self.x0 - rhs.x0,
			x1: self.x1 - rhs.x1 }
	}
}

impl<T: Mul<T, T> + Add<T, T> + Sub<T, T>>
Mul<Complex<T>, Complex<T>> for Complex<T> {
	fn mul(&self, rhs: &Complex<T>) -> Complex<T> {
		Complex { x0: self.x0 * rhs.x0 - self.x1 * rhs.x1,
			x1: self.x0 * rhs.x1 + self.x1 * rhs.x0 }
	}
}

impl<T: Div<T, T> + Add<T, T> + Sub<T, T> + Mul<T, T>>
Div<Complex<T>, Complex<T>> for Complex<T> {
	fn div(&self, rhs: &Complex<T>) -> Complex<T> {
		let len2 = rhs.x0 * rhs.x0 + rhs.x1 * rhs.x1;
		Complex { x0: (self.x0 * rhs.x0 + self.x1 * rhs.x1) / len2,
			x1: (self.x1 * rhs.x0 - self.x0 * rhs.x1) / len2 }
	}
}

impl<T: Neg<T>>
Neg<Complex<T>> for Complex<T> {
	fn neg(&self) -> Complex<T> {
		Complex { x0: -self.x0, x1: -self.x1 }
	}
}

impl<T: Mul<T, T> + Add<T, T>>
NormSq<T> for Complex<T> {
	fn norm_sq(&self) -> T {
		self.x0 * self.x0 + self.x1 * self.x1
	}
}

impl<T: One + Zero>
One for Complex<T> {
	fn one() -> Complex<T> {
		Complex { x0: One::one(), x1: Zero::zero() }
	}
}

/// A Quaternion type is commonly used for rotations in 3D.
#[deriving(Eq, Zero)]
pub struct Quaternion<T> {
	/// The x-dimension of the quaternion.
	x: T,
	/// The y-dimension of the quaternion.
	y: T,
	/// The z-dimension of the quaternion.
	z: T,
	/// The scalar component of the quaternion.
	w: T,
}

impl<T> Quaternion<T> {
	/// Creates a new Quaternion.
	/// Notice that the scalar component is after the vector.
	pub fn new(x: T, y: T, z: T, w: T) -> Quaternion<T> {
		Quaternion { x: x, y: y, z: z, w: w }
	}
}

impl<T: Add<T, T>>
Add<Quaternion<T>, Quaternion<T>> for Quaternion<T> {
	fn add(&self, rhs: &Quaternion<T>) -> Quaternion<T> {
		Quaternion { x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}

impl<T: Sub<T, T>>
Sub<Quaternion<T>, Quaternion<T>> for Quaternion<T> {
	fn sub(&self, rhs: &Quaternion<T>) -> Quaternion<T> {
		Quaternion { x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w
		}
	}
}

impl<T: Mul<T, T> + Sub<T, T> + Add<T, T>>
Mul<Quaternion<T>, Quaternion<T>> for Quaternion<T> {
	fn mul(&self, rhs: &Quaternion<T>) -> Quaternion<T> {
		Quaternion {
			x: 	self.w*rhs.x
				-self.z*rhs.y
				+self.y*rhs.z
				+self.x*rhs.w,
			y: 	self.z*rhs.x
				+self.w*rhs.y
				-self.x*rhs.z
				+self.y*rhs.w,
			z: 	self.x*rhs.y
				-self.y*rhs.x
				+self.w*rhs.z
				+self.z*rhs.w,
			w: 	self.w*rhs.w
				-self.x*rhs.x
				-self.y*rhs.y
				-self.z*rhs.z,
		}
	}
}

impl<T: Mul<T, T> + Sub<T, T> + Add<T, T> + Div<T, T>>
Div<Quaternion<T>, Quaternion<T>> for Quaternion<T> {
	fn div(&self, rhs: &Quaternion<T>) -> Quaternion<T> {
		let len2 = rhs.x * rhs.x 
			+ rhs.y * rhs.y 
			+ rhs.z * rhs.z
			+ rhs.w * rhs.w;
		Quaternion {
			x: 	(self.z*rhs.y
				-self.w*rhs.x
				-self.y*rhs.z
				+self.x*rhs.w) / len2,
			y: 	(self.x*rhs.z	
				-self.z*rhs.x
				-self.w*rhs.y
				+self.y*rhs.w) / len2,
			z: 	(self.y*rhs.x
				-self.x*rhs.y
				-self.w*rhs.z
				+self.z*rhs.w) / len2,
			w: 	(self.w*rhs.w
				+self.x*rhs.x
				+self.y*rhs.y
				+self.z*rhs.z) / len2,
		}
	}
}

impl<T: Neg<T>>
Neg<Quaternion<T>> for Quaternion<T> {
	fn neg(&self) -> Quaternion<T> {
		Quaternion { x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w }
	}
}

impl<T: Add<T, T> + Mul<T, T>>
NormSq<T> for Quaternion<T> {
	fn norm_sq(&self) -> T {
		self.x * self.x
		+ self.y * self.y
		+ self.z * self.z
		+ self.w * self.w
	}
}

impl<T: One + Zero>
One for Quaternion<T> {
	fn one() -> Quaternion<T> {
		Quaternion { x: Zero::zero(),
			y: Zero::zero(),
			z: Zero::zero(),
			w: One::one() }
	}
}

/// A Matrix4 is commonly used for linear transformations in 3D space.
#[deriving(Eq, Zero)]
pub struct Matrix4<T> {
	/// Element at first row and first column.
	m11: T, 
	/// Element at first row and second column.
	m12: T, 
	/// Element at first row and third column.
	m13: T, 
	/// Element at first row and fourth column.
	m14: T,
	
	/// Element at second row and first column.
	m21: T, 
	/// Element at second row and second column.
	m22: T, 
	/// Element at second row and third column.
	m23: T, 
	/// Element at second row and fourth column.
	m24: T,
	
	/// Element at third row and first column.
	m31: T, 
	/// Element at third row and second column.
	m32: T, 
	/// Element at third row and third column.
	m33: T, 
	/// Element at third row and fourth column.
	m34: T,
	
	/// Element at fourth row and first column.
	m41: T, 
	/// Element at fourth row and second column.
	m42: T, 
	/// Element at fourt row and third column.
	m43: T, 
	/// Element at fourth row and fourth column.
	m44: T
}

impl<T> Matrix4<T> {
	/// Creates a new matrix with elements.
	pub fn new(
		m11: T, m12: T, m13: T, m14: T,
		m21: T, m22: T, m23: T, m24: T,
		m31: T, m32: T, m33: T, m34: T,
		m41: T, m42: T, m43: T, m44: T
	) -> Matrix4<T> {
		Matrix4 {
			m11: m11, m12: m12, m13: m13, m14: m14,
			m21: m21, m22: m22, m23: m23, m24: m24,
			m31: m31, m32: m32, m33: m33, m34: m34,
			m41: m41, m42: m42, m43: m43, m44: m44
		}
	}
}

impl<T: Add<T, T>>
Add<Matrix4<T>, Matrix4<T>> for Matrix4<T> {
	fn add(&self, rhs: &Matrix4<T>) -> Matrix4<T> {
		Matrix4 {
			m11: self.m11 + rhs.m11,
			m12: self.m12 + rhs.m12,
			m13: self.m13 + rhs.m13,
			m14: self.m14 + rhs.m14,

			m21: self.m21 + rhs.m21,
			m22: self.m22 + rhs.m22,
			m23: self.m23 + rhs.m23,
			m24: self.m24 + rhs.m24,

			m31: self.m31 + rhs.m31,
			m32: self.m32 + rhs.m32,
			m33: self.m33 + rhs.m33,
			m34: self.m34 + rhs.m34,

			m41: self.m41 + rhs.m41,
			m42: self.m42 + rhs.m42,
			m43: self.m43 + rhs.m43,
			m44: self.m44 + rhs.m44
		}
	}
}

impl<T: Sub<T, T>>
Sub<Matrix4<T>, Matrix4<T>> for Matrix4<T> {
	fn sub(&self, rhs: &Matrix4<T>) -> Matrix4<T> {
		Matrix4 {
			m11: self.m11 - rhs.m11,
			m12: self.m12 - rhs.m12,
			m13: self.m13 - rhs.m13,
			m14: self.m14 - rhs.m14,

			m21: self.m21 - rhs.m21,
			m22: self.m22 - rhs.m22,
			m23: self.m23 - rhs.m23,
			m24: self.m24 - rhs.m24,

			m31: self.m31 - rhs.m31,
			m32: self.m32 - rhs.m32,
			m33: self.m33 - rhs.m33,
			m34: self.m34 - rhs.m34,

			m41: self.m41 - rhs.m41,
			m42: self.m42 - rhs.m42,
			m43: self.m43 - rhs.m43,
			m44: self.m44 - rhs.m44
		}
	}
}

impl<T: Mul<T, T> + Add<T, T>>
Mul<Matrix4<T>, Matrix4<T>> for Matrix4<T> {
	fn mul(&self, rhs: &Matrix4<T>) -> Matrix4<T> {
		Matrix4 {
			m11: self.m11*rhs.m11
				+self.m12*rhs.m21
				+self.m13*rhs.m31
				+self.m14*rhs.m41,
			m12: self.m11*rhs.m12
				+self.m12*rhs.m22
				+self.m13*rhs.m32
				+self.m14*rhs.m42,
			m13: self.m11*rhs.m13
				+self.m12*rhs.m23
				+self.m13*rhs.m33
				+self.m14*rhs.m43,
			m14: self.m11*rhs.m14
				+self.m12*rhs.m24
				+self.m13*rhs.m34
				+self.m14*rhs.m44,

			m21: self.m21*rhs.m11
				+self.m22*rhs.m21
				+self.m23*rhs.m31
				+self.m24*rhs.m41,
			m22: self.m21*rhs.m12
				+self.m22*rhs.m22
				+self.m23*rhs.m32
				+self.m24*rhs.m42,
			m23: self.m21*rhs.m13
				+self.m22*rhs.m23
				+self.m23*rhs.m33
				+self.m24*rhs.m43,
			m24: self.m21*rhs.m14
				+self.m22*rhs.m24
				+self.m23*rhs.m34
				+self.m24*rhs.m44,
		
			m31: self.m31*rhs.m11
				+self.m32*rhs.m21
				+self.m33*rhs.m31
				+self.m34*rhs.m41,
			m32: self.m31*rhs.m12
				+self.m32*rhs.m22
				+self.m33*rhs.m32
				+self.m34*rhs.m42,
			m33: self.m31*rhs.m13
				+self.m32*rhs.m23
				+self.m33*rhs.m33
				+self.m34*rhs.m43,
			m34: self.m31*rhs.m14
				+self.m32*rhs.m24
				+self.m33*rhs.m34
				+self.m34*rhs.m44,

			m41: self.m41*rhs.m11
				+self.m42*rhs.m21
				+self.m43*rhs.m31
				+self.m44*rhs.m41,
			m42: self.m41*rhs.m12
				+self.m42*rhs.m22
				+self.m43*rhs.m32
				+self.m44*rhs.m42,
			m43: self.m41*rhs.m13
				+self.m42*rhs.m23
				+self.m43*rhs.m33
				+self.m44*rhs.m43,
			m44: self.m41*rhs.m14
				+self.m42*rhs.m24
				+self.m43*rhs.m34
				+self.m44*rhs.m44
		}
	}
}

impl<T: Neg<T>>
Neg<Matrix4<T>> for Matrix4<T> {
	fn neg(&self) -> Matrix4<T> {
		Matrix4 {
			m11: -self.m11, m12: -self.m12, m13: -self.m13, m14: -self.m14,
			m21: -self.m21, m22: -self.m22, m23: -self.m23, m24: -self.m24,
			m31: -self.m31, m32: -self.m32, m33: -self.m33, m34: -self.m34,
			m41: -self.m41, m42: -self.m42, m43: -self.m43, m44: -self.m44
		}
	}
}

impl<T: Add<T, T> + Mul<T, T> + Sub<T, T>>
Det<T> for Matrix4<T> {
	fn det(&self) -> T {
		self.m11*self.m22*self.m33*self.m44
		+self.m11*self.m23*self.m34*self.m42
		+self.m11*self.m24*self.m32*self.m43

		+self.m12*self.m21*self.m34*self.m43
		+self.m12*self.m23*self.m31*self.m44
		+self.m12*self.m24*self.m33*self.m41

		+self.m13*self.m21*self.m32*self.m44
		+self.m13*self.m22*self.m34*self.m41
		+self.m13*self.m24*self.m31*self.m42

		+self.m14*self.m21*self.m33*self.m42
		+self.m14*self.m22*self.m31*self.m43
		+self.m14*self.m23*self.m32*self.m41

		-self.m11*self.m22*self.m34*self.m43
		-self.m11*self.m23*self.m32*self.m44
		-self.m11*self.m24*self.m33*self.m42

		-self.m12*self.m21*self.m33*self.m44
		-self.m12*self.m23*self.m34*self.m41
		-self.m12*self.m24*self.m31*self.m43

		-self.m13*self.m21*self.m34*self.m42
		-self.m13*self.m22*self.m31*self.m44
		-self.m13*self.m24*self.m32*self.m41

		-self.m14*self.m21*self.m32*self.m43
		-self.m14*self.m22*self.m33*self.m41
		-self.m14*self.m23*self.m31*self.m42
	}
}

impl<T: Add<T, T> + Mul<T, T> + Sub<T, T> + Div<T, T>>
Inv<Matrix4<T>> for Matrix4<T> {
	fn inv(&self) -> Matrix4<T> {
		let det = self.det();
		Matrix4 {
			m11: (self.m22*self.m33*self.m44
				+self.m23*self.m34*self.m42
				+self.m24*self.m32*self.m43
				-self.m22*self.m34*self.m43
				-self.m23*self.m32*self.m44
				-self.m24*self.m33*self.m42) / det,
			m12: (self.m12*self.m34*self.m43
				+self.m13*self.m32*self.m44
				+self.m14*self.m33*self.m42
				-self.m12*self.m33*self.m44
				-self.m13*self.m34*self.m42
				-self.m14*self.m32*self.m43) / det,
			m13: (self.m12*self.m23*self.m44
				+self.m13*self.m24*self.m42
				+self.m14*self.m22*self.m43
				-self.m12*self.m24*self.m43
				-self.m13*self.m22*self.m44
				-self.m14*self.m23*self.m42) / det,
			m14: (self.m12*self.m24*self.m33
				+self.m13*self.m22*self.m34
				+self.m14*self.m23*self.m32
				-self.m12*self.m23*self.m34
				-self.m13*self.m24*self.m32
				-self.m14*self.m22*self.m33) / det,

			m21: (self.m21*self.m34*self.m43
				+self.m23*self.m31*self.m44
				+self.m24*self.m33*self.m41
				-self.m21*self.m33*self.m44
				-self.m23*self.m34*self.m41
				-self.m24*self.m31*self.m43) / det,
			m22: (self.m11*self.m33*self.m44
				+self.m13*self.m34*self.m41
				+self.m14*self.m31*self.m43
				-self.m11*self.m34*self.m43
				-self.m13*self.m31*self.m44
				-self.m14*self.m33*self.m41) / det,
			m23: (self.m11*self.m24*self.m43
				+self.m13*self.m21*self.m44
				+self.m14*self.m23*self.m41
				-self.m11*self.m23*self.m44
				-self.m13*self.m24*self.m41
				-self.m14*self.m21*self.m43) / det,
			m24: (self.m11*self.m23*self.m34
				+self.m13*self.m24*self.m31
				+self.m14*self.m21*self.m33
				-self.m11*self.m24*self.m33
				-self.m13*self.m21*self.m34
				-self.m14*self.m23*self.m31) / det,

			m31: (self.m21*self.m32*self.m44
				+self.m22*self.m34*self.m41
				+self.m24*self.m31*self.m42
				-self.m21*self.m34*self.m42
				-self.m22*self.m31*self.m44
				-self.m24*self.m32*self.m41) / det,
			m32: (self.m11*self.m34*self.m42
				+self.m12*self.m31*self.m44
				+self.m14*self.m32*self.m41
				-self.m11*self.m32*self.m44
				-self.m12*self.m34*self.m41
				-self.m14*self.m31*self.m42) / det,
			m33: (self.m11*self.m22*self.m44
				+self.m12*self.m24*self.m41
				+self.m14*self.m21*self.m42
				-self.m11*self.m24*self.m42
				-self.m12*self.m21*self.m44
				-self.m14*self.m22*self.m41) / det,
			m34: (self.m11*self.m24*self.m32
				+self.m12*self.m21*self.m34
				+self.m14*self.m22*self.m31
				-self.m11*self.m22*self.m34
				-self.m12*self.m24*self.m31
				-self.m14*self.m21*self.m32) / det,

			m41: (self.m21*self.m33*self.m42
				+self.m22*self.m31*self.m43
				+self.m23*self.m32*self.m41
				-self.m21*self.m32*self.m43
				-self.m22*self.m33*self.m41
				-self.m23*self.m31*self.m42) / det,
			m42: (self.m11*self.m32*self.m43
				+self.m12*self.m33*self.m41
				+self.m13*self.m31*self.m42
				-self.m11*self.m33*self.m42
				-self.m12*self.m31*self.m43
				-self.m13*self.m32*self.m41) / det,
			m43: (self.m11*self.m23*self.m42
				+self.m12*self.m21*self.m43
				+self.m13*self.m22*self.m41
				-self.m11*self.m22*self.m43
				-self.m12*self.m23*self.m41
				-self.m13*self.m21*self.m42) / det,
			m44: (self.m11*self.m22*self.m33
				+self.m12*self.m23*self.m31
				+self.m13*self.m21*self.m32
				-self.m11*self.m23*self.m32
				-self.m12*self.m21*self.m33
				-self.m13*self.m22*self.m31) / det,
		}
	}
}

impl<T: Add<T, T> + Mul<T, T> + Sub<T, T> + Div<T, T>>
Div<Matrix4<T>, Matrix4<T>> for Matrix4<T> {
	fn div(&self, rhs: &Matrix4<T>) -> Matrix4<T> {
		let inv = rhs.inv();
		self * inv
	}
}

impl<T: Eps>
Eps for Matrix4<T> {
	fn eps(eps: f64) -> Matrix4<T> {
		Matrix4 {
			m11: Eps::eps(eps),
			m12: Eps::eps(eps),
			m13: Eps::eps(eps),
			m14: Eps::eps(eps),

			m21: Eps::eps(eps),
			m22: Eps::eps(eps),
			m23: Eps::eps(eps),
			m24: Eps::eps(eps),

			m31: Eps::eps(eps),
			m32: Eps::eps(eps),
			m33: Eps::eps(eps),
			m34: Eps::eps(eps),

			m41: Eps::eps(eps),
			m42: Eps::eps(eps),
			m43: Eps::eps(eps),
			m44: Eps::eps(eps),
		}
	}

	fn close_eps(&self, other: &Matrix4<T>, eps: &Matrix4<T>) -> bool {
		self.m11.close_eps(&other.m11, &eps.m11)
		&& self.m12.close_eps(&other.m12, &eps.m12)
		&& self.m13.close_eps(&other.m13, &eps.m13)
		&& self.m14.close_eps(&other.m14, &eps.m14)

		&& self.m21.close_eps(&other.m21, &eps.m21)
		&& self.m22.close_eps(&other.m22, &eps.m22)
		&& self.m23.close_eps(&other.m23, &eps.m23)
		&& self.m24.close_eps(&other.m24, &eps.m24)

		&& self.m31.close_eps(&other.m31, &eps.m31)
		&& self.m32.close_eps(&other.m32, &eps.m32)
		&& self.m33.close_eps(&other.m33, &eps.m33)
		&& self.m34.close_eps(&other.m34, &eps.m34)

		&& self.m41.close_eps(&other.m41, &eps.m41)
		&& self.m42.close_eps(&other.m42, &eps.m42)
		&& self.m43.close_eps(&other.m43, &eps.m43)
		&& self.m44.close_eps(&other.m44, &eps.m44)
	}
}

impl<T: Zero + One>
One for Matrix4<T> {
	fn one() -> Matrix4<T> {
		Matrix4 {
			m11: One::one(),
			m12: Zero::zero(),
			m13: Zero::zero(),
			m14: Zero::zero(),

			m21: Zero::zero(),
			m22: One::one(),
			m23: Zero::zero(),
			m24: Zero::zero(),

			m31: Zero::zero(),
			m32: Zero::zero(),
			m33: One::one(),
			m34: Zero::zero(),

			m41: Zero::zero(),
			m42: Zero::zero(),
			m43: Zero::zero(),
			m44: One::one()
		}
	}
}

/// A Vector type contains a list of values.
/// It is commonly used for list operations.
#[deriving(Eq)]
pub struct Vector<T> {
	/// Contains the items in the vector.
	x: ~[T],
}

impl<T> Vector<T> {
	/// Creates a new vector from a list of values.
	pub fn new(x: ~[T]) -> Vector<T> {
		Vector { x: x }
	}
}

impl<T: Add<T, T>>
Add<Vector<T>, Vector<T>> for Vector<T> {
	fn add(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { x: ~[] }
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] + rhs.x[i]);
				i += 1;
			}

			Vector { x: res }
		}
	}
}

impl<T: Sub<T, T>>
Sub<Vector<T>, Vector<T>> for Vector<T> {
	fn sub(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { x: ~[] }
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] - rhs.x[i]);
				i += 1;
			}

			Vector { x: res }
		}
	}
}

impl<T: Mul<T, T>>
Mul<Vector<T>, Vector<T>> for Vector<T> {
	fn mul(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { x: ~[] }
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] * rhs.x[i]);
				i += 1;
			}

			Vector { x: res }
		}
	}
}

impl<T: Div<T, T>>
Div<Vector<T>, Vector<T>> for Vector<T> {
	fn div(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { x: ~[] }
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] / rhs.x[i]);
				i += 1;
			}

			Vector { x: res }
		}
	}
}

impl<T: Neg<T>>
Neg<Vector<T>> for Vector<T> {
	fn neg(&self) -> Vector<T> {
		let mut res: ~[T] = ~[];
		for x in self.x.iter() {
			res.push(-x);
		}

		Vector { x: res }
	}
}

impl<T: Add<T, T> + Mul<T, T> + Clone>
NormSq<T> for Vector<T> {
	fn norm_sq(&self) -> T {
		let mut res = self.x[0].clone();
		for x in self.x.iter().skip(1) {
			res = res + *x * *x;
		}

		res
	}
}

impl Eps for f64 {
	fn eps(eps: f64) -> f64 {
		eps
	}

	fn close_eps(&self, other: &f64, eps: &f64) -> bool {
		let a = *self - *other;
		let b = if a < 0f64 {
				-a
			} else {
				a
			};
		b <= *eps
	}
}

impl Eps for f32 {
	fn eps(eps: f64) -> f32 {
		eps as f32
	}

	fn close_eps(&self, other: &f32, eps: &f32) -> bool {
		let a = *self - *other;
		let b = if a < 0f32 {
				-a
			} else {
				a
			};
		b <= *eps
	}
}

