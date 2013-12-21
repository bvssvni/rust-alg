#[link(name="alg")];
#[pkgid = "alg#0.1"];
#[crate_type="rlib"];

#[deny(non_camel_case_types)];
// #[deny(missing_doc)];
#[feature(managed_boxes)];

//! Library for Algebra.

//			Add	Sub	Mul	Div	Neg	NormSq
//	Dual2		x	x	x	x	x	x
//	Complex		x	x	x	x	x	x
//	Quaternion	x	x	x	x	x	x

/// Computes the square of length of algebraic type.
pub trait NormSq<Result> {
	fn norm_sq(&self) -> Result;
}

#[deriving(Eq)]
pub struct Dual2<T> {
	x0 : T,
	x1 : T,
}

impl<T> Dual2<T> {
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

#[deriving(Eq)]
pub struct Complex<T> {
	x0: T,
	x1: T,
}

impl<T> Complex<T> {
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

#[deriving(Eq)]
pub struct Quaternion<T> {
	x: T,
	y: T,
	z: T,
	w: T,
}

impl<T> Quaternion<T> {
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


