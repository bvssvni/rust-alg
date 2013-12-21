extern mod alg;

use alg::{Dual2, Complex, Quaternion, NormSq};

#[test]
fn test_dual_add() {
	let a = Dual2::new( 1f64, 2f64 );
	let b = Dual2::new( 2f64, 1f64 );
	let c = a + b;
	let d = Dual2::new( 3f64, 3f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_sub() {
	let a = Dual2::new( 1f64, 2f64 );
	let b = Dual2::new( 2f64, 1f64 );
	let c = a - b;
	let d = Dual2::new( -1f64, 1f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_mul() {
	let a = Dual2::new( 1f64, 2f64 );
	let b = Dual2::new( 2f64, 1f64 );
	let c = a * b;
	let d = Dual2::new( 2f64, 5f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_div() {
	let a = Dual2::new( 1f64, 2f64 );
	let b = Dual2::new( 2f64, 1f64 );
	let c = a / b;
	let d = Dual2::new( 0.5f64, 0.75f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_neg() {
	let a = Dual2::new( 1f64, 2f64 );
	let b = -a;
	let c = Dual2::new( -1f64, -2f64 );
	assert_eq!( b, c );
}

#[test]
fn test_dual_normsq() {
	let a = Dual2::new( 1f64, 2f64 );
	let b = a.norm_sq();
	assert_eq!( b, 1f64 );
}

#[test]
fn test_complex_add() {
	let a = Complex::new( 1f64, 2f64 );
	let b = Complex::new( 2f64, 1f64 );
	let c = a + b;
	let d = Complex::new( 3f64, 3f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_sub() {
	let a = Complex::new( 1f64, 2f64 );
	let b = Complex::new( 2f64, 1f64 );
	let c = a - b;
	let d = Complex::new( -1f64, 1f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_mul() {
	let a = Complex::new( 1f64, 2f64 );
	let b = Complex::new( 2f64, 1f64 );
	let c = a * b;
	let d = Complex::new( 0f64, 5f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_div() {
	let a = Complex::new( 1f64, 2f64 );
	let b = Complex::new( 2f64, 1f64 );
	let c = a / b;
	let d = Complex::new( 4f64/5f64, 3f64/5f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_neg() {
	let a = Complex::new( 1f64, 2f64 );
	let b = -a;
	let c = Complex::new( -1f64, -2f64 );
	assert_eq!( b, c );
}

#[test]
fn test_complex_norm_sq() {
	let a = Complex::new( 1f64, 2f64 );
	let b = a.norm_sq();
	let c = 5f64;
	assert_eq!( b, c );
}

#[test]
fn test_quaternion_add() {
	let a = Quaternion::new( 1f64, 2f64, 3f64, 4f64 );
	let b = Quaternion::new( 4f64, 3f64, 2f64, 1f64 );
	let c = a + b;
	let d = Quaternion::new( 5f64, 5f64, 5f64, 5f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_sub() {
	let a = Quaternion::new( 1f64, 2f64, 3f64, 4f64 );
	let b = Quaternion::new( 4f64, 3f64, 2f64, 1f64 );
	let c = a - b;
	let d = Quaternion::new( -3f64, -1f64, 1f64, 3f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_mul() {
	let a = Quaternion::new( 1f64, 2f64, 3f64, 4f64 );
	let b = Quaternion::new( 4f64, 3f64, 2f64, 1f64 );
	let c = a * b;
	let d = Quaternion::new( 12f64, 24f64, 6f64, -12f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_neg() {
	let a = Quaternion::new( 1f64, 2f64, 3f64, 4f64 );
	let b = -a;
	let c = Quaternion::new( -1f64, -2f64, -3f64, -4f64 );
	assert_eq!( b, c );
}

#[test]
fn test_quaternion_norm_sq() {
	let a = Quaternion::new( 1f64, 2f64, 3f64, 4f64 );
	let b = a.norm_sq();
	let c = 30f64;
	assert_eq!( b, c );
}

#[test]
fn test_quaternion_div() {
	let a = Quaternion::new( 1f64, 2f64, 3f64, 4f64 );
	let b = Quaternion::new( 4f64, 3f64, 2f64, 1f64 );
	let c = a / b;
	let n = b.norm_sq();
	let d = Quaternion::new( -10f64 / n, -20f64 / n, 0f64 / n, 20f64 / n );
	assert_eq!( c, d );
}


