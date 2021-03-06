#[crate_id = "alg#0.1"];
#[crate_type="rlib"];

#[deny(non_camel_case_types)];
#[deny(missing_doc)];
#[feature(managed_boxes)];

//! Library for Algebra.

//			Add	Sub	Mul	Div	Neg	Inv
//	Dual2		x	x	x	x	x	x
//	Complex		x	x	x	x	x	x
//	Quaternion	x	x	x	x	x	x
//	Matrix4		x	x	x	x	x	x
//	Vector		x	x	x	x	x	x
//	f32		std	std	std	std	std	x
//	f64		std	std	std	std	std	x

//			Det	NormSq
//	Dual2		x	x
//	Complex		x	x
//	Quaternion	-	x
//	Matrix4		x	-
//	Vector		-	x

//			Eq	Zero	One
//	Dual2		x	x	x
//	Complex		x	x	x
//	Quaternion	x	x	x
//	Matrix4		x	x	x
//	Vector		x	-	-

//			Eps	Scale
//	Dual2		x	x
//	Complex		x	x
//	Quaternion	x	x
//	Matrix4		x	x
//	Vector		x	-
//	f32		x	-
//	f64		x	-

/// Computes the square of the norm/length.
#[inline(always)]
pub fn norm_sq<T: NormSq<U>, U>(a: T) -> U {a.norm_sq()} 

/// Computes the square of length of algebraic type.
pub trait NormSq<Result> {
	/// Computes the square of the norm/length.
	fn norm_sq(&self) -> Result;
}

/// Calculates the determinant of the structure.
#[inline(always)]
pub fn det<T: Det<U>, U>(a: T) -> U {a.det()}

/// Is implemented on structures that has a determinant.
/// A determinant is a value that tells whether the structure is invertible.
/// If the determinant is zero the structure is not invertible.
pub trait Det<Result> {
	/// Calculates the determinant of the structure.
	fn det(&self) -> Result;
}

/// Creates an inverted version of the structure.
#[inline(always)]
pub fn inv<T: Inv<U>, U>(a: T) -> U {a.inv()}

/// Implemented on structures that can be inverted.
pub trait Inv<Result> {
	/// Creates an inverted version of the structure.
	fn inv(&self) -> Result;
}

/// Checks for equality with a custom approximate epsilon.
#[inline(always)]
pub fn close_eps<T: Eps>(a: &T, other: &T, eps: f64) -> bool {
	a.close_eps(other, eps)
}

/// Eps creates a value from f64 number.
/// This can be used to check if two numbers are closer than the eps.
pub trait Eps {
	/// Checks for equality with a custom approximate epsilon.
	fn close_eps(&self, other: &Self, eps: f64) -> bool;
}

/// Creates a scaling type that scales up under multiplication.
#[inline(always)]
pub fn scale<T: Scale<U>, U>(a: U) -> T {
	Scale::scale(a)
}

/// Implemented on structures that can scale under multiplication.
pub trait Scale<T> {
	/// Creates a scaling type that scales up under multiplication.
	fn scale(factor: T) -> Self;
}

/// A Dual type is commonly used for automatic differentiation.
#[deriving(Eq, Zero)]
pub struct Dual2<T> {
	/// The real part of Dual number.
	x0 : T,
	/// The dual part of Dual number.
	x1 : T,
}

/// Constructs a new dual number.
#[inline(always)]
pub fn dual2<T>(x0: T, x1: T) -> Dual2<T> {
	Dual2::new(x0, x1)
}

impl<T>
Dual2<T> {
	/// Constructs a new dual number.
	pub fn new(x0: T, x1: T) -> Dual2<T> {
		Dual2 {
			x0: x0,
			x1: x1
		}
	}
}

impl<
	T: std::num::Zero
>
Scale<T> for Dual2<T> {
	fn scale(factor: T) -> Dual2<T> {
		Dual2 {
			x0: factor, 
			x1: std::num::zero()
		}
	}
}

impl<
	T: Add<T, T>
>
Add<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn add(&self, rhs: &Dual2<T>) -> Dual2<T> {
		Dual2 {
			x0: self.x0 + rhs.x0, 
			x1: self.x1 + rhs.x1
		}
	}
}

impl<
	T: Sub<T, T>
>
Sub<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn sub(&self, rhs: &Dual2<T>) -> Dual2<T> {
		Dual2 {
			x0: self.x0 - rhs.x0, 
			x1: self.x1 - rhs.x1
		}
	}
}

impl<
	T: Mul<T, T> + Add<T, T>
>
Mul<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn mul(&self, rhs: &Dual2<T>) -> Dual2<T> {
		Dual2 {
			x0: self.x0 * rhs.x0,
			x1: self.x0 * rhs.x1 + self.x1 * rhs.x0
		}
	}
}

impl<
	T: Div<T, T> + Sub<T, T> + Mul<T, T>
>
Div<Dual2<T>, Dual2<T>> for Dual2<T> {
	fn div(&self, rhs: &Dual2<T>) -> Dual2<T> {
		let b2 = rhs.x0 * rhs.x0;
		Dual2 {
			x0: self.x0 / rhs.x0,
			x1: (self.x1 * rhs.x0 - self.x0 * rhs.x1) / b2
		}
	}
}

impl<
	T: Div<T, T> + Neg<T> + Mul<T, T> + Inv<T>
>
Inv<Dual2<T>> for Dual2<T> {
	fn inv(&self) -> Dual2<T> {
		let b2 = self.x0 * self.x0;
		Dual2 {
			x0: self.x0.inv(),
			x1: -self.x1 / b2
		}
	}
}

impl<
	T: Neg<T>
>
Neg<Dual2<T>> for Dual2<T> {
	fn neg(&self) -> Dual2<T> {
		Dual2 {
			x0: -self.x0,
			x1: -self.x1
		}
	}
}

impl<
	T: Mul<T, T>
>
NormSq<T> for Dual2<T> {
	fn norm_sq(&self) -> T {
		self.x0 * self.x0
	}
}

impl<
	T: Mul<T, T>
>
Det<T> for Dual2<T> {
	fn det(&self) -> T {
		self.x0 * self.x0
	}
}

impl<
	T: std::num::One + std::num::Zero
>
std::num::One for Dual2<T> {
	fn one() -> Dual2<T> {
		Dual2 {
			x0: std::num::one(),
			x1: std::num::zero()
		}
	}
}

impl<
	T: Eps
>
Eps for Dual2<T> {
	fn close_eps(&self, other: &Dual2<T>, eps: f64) -> bool {
		if !self.x0.close_eps(&other.x0, eps) {
			false
		} else if !self.x1.close_eps(&other.x1, eps) {
			false
		}
		else {
			true
		}
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

/// Creates a new Complex number.
#[inline(always)]
pub fn complex<T>(x0: T, x1: T) -> Complex<T> {
	Complex::new(x0, x1)
}

impl<T> 
Complex<T> {
	/// Creates a new Complex number.
	pub fn new(x0: T, x1: T) -> Complex<T> {
		Complex {
			x0: x0, 
			x1: x1
		}
	}
}

impl<
	T: std::num::Zero
>
Scale<T> for Complex<T> {
	fn scale(factor: T) -> Complex<T> {
		Complex {
			x0: factor,
			x1: std::num::zero()
		}
	}
}

impl<
	T: Add<T, T>
>
Add<Complex<T>, Complex<T>> for Complex<T> {
	fn add(&self, rhs: &Complex<T>) -> Complex<T> {
		Complex {
			x0: self.x0 + rhs.x0,
			x1: self.x1 + rhs.x1
		}
	}
}

impl<
	T: Sub<T, T>
> 
Sub<Complex<T>, Complex<T>> for Complex<T> {
	fn sub(&self, rhs: &Complex<T>) -> Complex<T> {
		Complex {
			x0: self.x0 - rhs.x0,
			x1: self.x1 - rhs.x1
		}
	}
}

impl<
	T: Mul<T, T> + Add<T, T> + Sub<T, T>
>
Mul<Complex<T>, Complex<T>> for Complex<T> {
	fn mul(&self, rhs: &Complex<T>) -> Complex<T> {
		Complex { 
			x0: self.x0 * rhs.x0 - self.x1 * rhs.x1,
			x1: self.x0 * rhs.x1 + self.x1 * rhs.x0 
		}
	}
}

impl<
	T: Div<T, T> + Add<T, T> + Sub<T, T> + Mul<T, T>
>
Div<Complex<T>, Complex<T>> for Complex<T> {
	fn div(&self, rhs: &Complex<T>) -> Complex<T> {
		let len2 = rhs.x0 * rhs.x0 + rhs.x1 * rhs.x1;
		Complex {
			x0: (self.x0 * rhs.x0 + self.x1 * rhs.x1) / len2,
			x1: (self.x1 * rhs.x0 - self.x0 * rhs.x1) / len2
		}
	}
}

impl<
	T: Add<T, T> + Mul<T, T> + Div<T, T> + Neg<T>
>
Inv<Complex<T>> for Complex<T> {
	fn inv(&self) -> Complex<T> {
		let len2 = self.x0 * self.x0 + self.x1 * self.x1;
		Complex {
			x0: self.x0 / len2,
			x1: -self.x1 / len2
		}
	}
}

impl<
	T: Neg<T>
>
Neg<Complex<T>> for Complex<T> {
	fn neg(&self) -> Complex<T> {
		Complex {
			x0: -self.x0, 
			x1: -self.x1
		}
	}
}

impl<
	T: Mul<T, T> + Add<T, T>
>
NormSq<T> for Complex<T> {
	fn norm_sq(&self) -> T {
		self.x0 * self.x0 + self.x1 * self.x1
	}
}

impl<
	T: std::num::One + std::num::Zero
>
std::num::One for Complex<T> {
	fn one() -> Complex<T> {
		Complex {
			x0: std::num::one(),
			x1: std::num::zero()
		}
	}
}

impl<
	T: Eps
>
Eps for Complex<T> {
	fn close_eps(&self, other: &Complex<T>, eps: f64) -> bool {
		if !self.x0.close_eps(&other.x0, eps) { 
			false 
		} else if !self.x1.close_eps(&other.x1, eps) { 
			false 
		} else { 
			true 
		}
	}
}

impl<
	T: Mul<T, T> + Add<T, T>
>
Det<T> 
for Complex<T> {
	fn det(&self) -> T {
		self.x0 * self.x0 + self.x1 * self.x1
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

/// Creates a new Quaternion.
/// Notice that the scalar component is after the vector.
#[inline(always)]
pub fn quaternion<T>(x: T, y: T, z: T, w: T) -> Quaternion<T> {
	Quaternion::new(x, y, z, w)
}

impl<T> 
Quaternion<T> {
	/// Creates a new Quaternion.
	/// Notice that the scalar component is after the vector.
	pub fn new(x: T, y: T, z: T, w: T) -> Quaternion<T> {
		Quaternion {
			x: x, 
			y: y, 
			z: z, 
			w: w 
		}
	}
}

impl<
	T: std::num::Zero
>
Scale<T> 
for Quaternion<T> {
	fn scale(factor: T) -> Quaternion<T> {
		Quaternion { 
			x: std::num::zero(),
			y: std::num::zero(),
			z: std::num::zero(),
			w: factor 
		}
	}
}

impl<
	T: Add<T, T>
>
Add<Quaternion<T>, Quaternion<T>> 
for Quaternion<T> {
	fn add(&self, rhs: &Quaternion<T>) -> Quaternion<T> {
		Quaternion { 
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}

impl<
	T: Sub<T, T>
>
Sub<Quaternion<T>, Quaternion<T>> 
for Quaternion<T> {
	fn sub(&self, rhs: &Quaternion<T>) -> Quaternion<T> {
		Quaternion { 
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w
		}
	}
}

impl<
	T: Mul<T, T> + Sub<T, T> + Add<T, T>
>
Mul<Quaternion<T>, Quaternion<T>> 
for Quaternion<T> {
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

impl<
	T: Mul<T, T> + Sub<T, T> + Add<T, T> + Div<T, T>
>
Div<Quaternion<T>, Quaternion<T>> 
for Quaternion<T> {
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

impl<
	T: Mul<T, T> + Add<T, T> + Div<T, T> + Neg<T>
>
Inv<Quaternion<T>> 
for Quaternion<T> {
	fn inv(&self) -> Quaternion<T> {
		let len2 = self.x * self.x 
			+ self.y * self.y 
			+ self.z * self.z
			+ self.w * self.w;
		Quaternion {
			x: 	-self.x / len2,
			y: 	-self.y / len2,
			z: 	-self.z / len2,
			w: 	self.w / len2,
		}
	}
}

impl<
	T: Neg<T>
>
Neg<Quaternion<T>> 
for Quaternion<T> {
	fn neg(&self) -> Quaternion<T> {
		Quaternion { 
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w 
		}
	}
}

impl<
	T: Add<T, T> + Mul<T, T>
>
NormSq<T> 
for Quaternion<T> {
	fn norm_sq(&self) -> T {
		self.x * self.x
		+ self.y * self.y
		+ self.z * self.z
		+ self.w * self.w
	}
}

impl<
	T: std::num::One + std::num::Zero
>
std::num::One 
for Quaternion<T> {
	fn one() -> Quaternion<T> {
		Quaternion { 
			x: std::num::zero(),
			y: std::num::zero(),
			z: std::num::zero(),
			w: std::num::one() 
		}
	}
}

impl<
	T: Eps
>
Eps 
for Quaternion<T> {
	fn close_eps(&self, other: &Quaternion<T>, eps: f64) -> bool {
		if !self.x.close_eps(&other.x, eps) { 
			false 
		} else if !self.y.close_eps(&other.y, eps) { 
			false 
		} else if !self.z.close_eps(&other.z, eps) { 
			false 
		} else if !self.w.close_eps(&other.w, eps) { 
			false 
		} else { 
			true 
		}
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

/// Creates a new matrix with elements.
#[inline(always)]
pub fn matrix4<T>(
	m11: T, m12: T, m13: T, m14: T,
	m21: T, m22: T, m23: T, m24: T,
	m31: T, m32: T, m33: T, m34: T,
	m41: T, m42: T, m43: T, m44: T
) -> Matrix4<T> {
	Matrix4::new(
		m11, m12, m13, m14,
		m21, m22, m23, m24,
		m31, m32, m33, m34,
		m41, m42, m43, m44
	)
}

impl<T> 
Matrix4<T> {
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

impl<
	T: std::num::Zero + Clone
>
Scale<T> 
for Matrix4<T> {
	fn scale(factor: T) -> Matrix4<T> {
		Matrix4 {
			m11: factor.clone(),
			m12: std::num::zero(),
			m13: std::num::zero(),
			m14: std::num::zero(),

			m21: std::num::zero(),
			m22: factor.clone(),
			m23: std::num::zero(),
			m24: std::num::zero(),

			m31: std::num::zero(),
			m32: std::num::zero(),
			m33: factor.clone(),
			m34: std::num::zero(),

			m41: std::num::zero(),
			m42: std::num::zero(),
			m43: std::num::zero(),
			m44: factor.clone()
		}
	}
}

impl<
	T: Add<T, T>
>
Add<Matrix4<T>, Matrix4<T>> 
for Matrix4<T> {
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

impl<
	T: Sub<T, T>
>
Sub<Matrix4<T>, Matrix4<T>> 
for Matrix4<T> {
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

impl<
	T: Mul<T, T> + Add<T, T>
>
Mul<Matrix4<T>, Matrix4<T>> 
for Matrix4<T> {
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

impl<
	T: Neg<T>
>
Neg<Matrix4<T>> 
for Matrix4<T> {
	fn neg(&self) -> Matrix4<T> {
		Matrix4 {
			m11: -self.m11, 
			m12: -self.m12, 
			m13: -self.m13, 
			m14: -self.m14,

			m21: -self.m21, 
			m22: -self.m22, 
			m23: -self.m23, 
			m24: -self.m24,
			
			m31: -self.m31, 
			m32: -self.m32, 
			m33: -self.m33, 
			m34: -self.m34,

			m41: -self.m41, 
			m42: -self.m42, 
			m43: -self.m43, 
			m44: -self.m44
		}
	}
}

impl<
	T: Add<T, T> + Mul<T, T> + Sub<T, T>
>
Det<T> 
for Matrix4<T> {
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

impl<
	T: Add<T, T> + Mul<T, T> + Sub<T, T> + Div<T, T>
>
Inv<Matrix4<T>> 
for Matrix4<T> {
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

impl<
	T: Add<T, T> + Mul<T, T> + Sub<T, T> + Div<T, T>
>
Div<Matrix4<T>, Matrix4<T>> 
for Matrix4<T> {
	fn div(&self, rhs: &Matrix4<T>) -> Matrix4<T> {
		let inv = rhs.inv();
		self * inv
	}
}

impl<
	T: Eps
>
Eps 
for Matrix4<T> {
	fn close_eps(&self, other: &Matrix4<T>, eps: f64) -> bool {
		self.m11.close_eps(&other.m11, eps)
		&& self.m12.close_eps(&other.m12, eps)
		&& self.m13.close_eps(&other.m13, eps)
		&& self.m14.close_eps(&other.m14, eps)

		&& self.m21.close_eps(&other.m21, eps)
		&& self.m22.close_eps(&other.m22, eps)
		&& self.m23.close_eps(&other.m23, eps)
		&& self.m24.close_eps(&other.m24, eps)

		&& self.m31.close_eps(&other.m31, eps)
		&& self.m32.close_eps(&other.m32, eps)
		&& self.m33.close_eps(&other.m33, eps)
		&& self.m34.close_eps(&other.m34, eps)

		&& self.m41.close_eps(&other.m41, eps)
		&& self.m42.close_eps(&other.m42, eps)
		&& self.m43.close_eps(&other.m43, eps)
		&& self.m44.close_eps(&other.m44, eps)
	}
}

impl<
	T: std::num::Zero + std::num::One
>
std::num::One 
for Matrix4<T> {
	fn one() -> Matrix4<T> {
		Matrix4 {
			m11: std::num::one(),
			m12: std::num::zero(),
			m13: std::num::zero(),
			m14: std::num::zero(),

			m21: std::num::zero(),
			m22: std::num::one(),
			m23: std::num::zero(),
			m24: std::num::zero(),

			m31: std::num::zero(),
			m32: std::num::zero(),
			m33: std::num::one(),
			m34: std::num::zero(),

			m41: std::num::zero(),
			m42: std::num::zero(),
			m43: std::num::zero(),
			m44: std::num::one()
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

/// Creates a new vector from a list of values.
#[inline(always)]
pub fn vector<T>(x: ~[T]) -> Vector<T> {
	Vector::new(x)
}

impl<T> 
Vector<T> {
	/// Creates a new vector from a list of values.
	pub fn new(x: ~[T]) -> Vector<T> {
		Vector {
			x: x 
		}
	}
}

impl<
	T: Add<T, T>
>
Add<Vector<T>, Vector<T>> 
for Vector<T> {
	fn add(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { 
				x: ~[] 
			}
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] + rhs.x[i]);
				i += 1;
			}

			Vector { 
				x: res 
			}
		}
	}
}

impl<
	T: Sub<T, T>
>
Sub<Vector<T>, Vector<T>> 
for Vector<T> {
	fn sub(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { 
				x: ~[] 
			}
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] - rhs.x[i]);
				i += 1;
			}

			Vector { 
				x: res 
			}
		}
	}
}

impl<
	T: Mul<T, T>
>
Mul<Vector<T>, Vector<T>> 
for Vector<T> {
	fn mul(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { 
				x: ~[] 
			}
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] * rhs.x[i]);
				i += 1;
			}

			Vector { 
				x: res 
			}
		}
	}
}

impl<
	T: Div<T, T>
>
Div<Vector<T>, Vector<T>> 
for Vector<T> {
	fn div(&self, rhs: &Vector<T>) -> Vector<T> {
		if self.x.len() != rhs.x.len() {
			Vector { 
				x: ~[] 
			}
		} else {
			let mut res: ~[T] = ~[];
			let mut i = 0;
			let n = self.x.len();
			while i < n {
				res.push(self.x[i] / rhs.x[i]);
				i += 1;
			}

			Vector { 
				x: res 
			}
		}
	}
}

impl<
	T: Inv<T>
>
Inv<Vector<T>> 
for Vector<T> {
	fn inv(&self) -> Vector<T> {
		let mut res: ~[T] = ~[];
		for i in range(0, self.x.len()) {
			res.push(self.x[i].inv());
		}

		Vector { 
			x: res 
		}
	}
}

impl<
	T: Neg<T>
>
Neg<Vector<T>> 
for Vector<T> {
	fn neg(&self) -> Vector<T> {
		let mut res: ~[T] = ~[];
		for x in self.x.iter() {
			res.push(-x);
		}

		Vector { 
			x: res 
		}
	}
}

impl<
	T: Add<T, T> + Mul<T, T> + Clone
>
NormSq<T> 
for Vector<T> {
	fn norm_sq(&self) -> T {
		let mut res = self.x[0].clone();
		for x in self.x.iter().skip(1) {
			res = res + *x * *x;
		}

		res
	}
}

impl<
	T: Eps
> 
Eps 
for Vector<T> {
	fn close_eps(&self, other: &Vector<T>, eps: f64) -> bool {
		let n = self.x.len();
		let m = other.x.len();
		if n != m { fail!("Vectors not of same length"); }

		if range(0, n).any(
			|i| !self.x[i].close_eps(&other.x[i], eps)
		) {
			false
		} else {
			true
		}
	}
}

impl 
Inv<f64> 
for f64 {
	fn inv(&self) -> f64 {
		1f64 / *self
	}
}

impl 
Eps 
for f64 {
	fn close_eps(&self, other: &f64, eps: f64) -> bool {
		let a = *self - *other;
		let b = if a < 0f64 {
				-a
			} else {
				a
			};
		b <= eps
	}
}

impl 
Inv<f32> 
for f32 {
	fn inv(&self) -> f32 {
		1f32 / *self
	}
}

impl 
Eps 
for f32 {
	fn close_eps(&self, other: &f32, eps: f64) -> bool {
		let a = *self - *other;
		let b = if a < 0f32 {
				-a
			} else {
				a
			};
		b <= eps as f32
	}
}

