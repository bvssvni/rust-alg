extern mod alg;

use alg::{ Dual2, Complex, Quaternion, Matrix4, Vector };
use alg::{ NormSq, Det, Inv, Eps, Scale };
use std::num::{One};

#[test]
fn test_dual_scale() {
	let a: Dual2<f64> = Scale::scale( 2_f64 );
	let b = Dual2::new( 2_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_dual_add() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = Dual2::new( 2_f64, 1_f64 );
	let c = a + b;
	let d = Dual2::new( 3_f64, 3_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_sub() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = Dual2::new( 2_f64, 1_f64 );
	let c = a - b;
	let d = Dual2::new( -1_f64, 1_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_mul() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = Dual2::new( 2_f64, 1_f64 );
	let c = a * b;
	let d = Dual2::new( 2_f64, 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_div() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = Dual2::new( 2_f64, 1_f64 );
	let c = a / b;
	let d = Dual2::new( 0.5_f64, 0.75_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_inv() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = a.inv();
	let c = Dual2::new( 1_f64, -2_f64 );
	assert_eq!( b, c );

	let a = Dual2::new( 1_f32, 2_f32 );
	let b = a.inv();
	let c = Dual2::new( 1_f32, -2_f32 );
	assert_eq!( b, c );
}

#[test]
fn test_dual_neg() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = -a;
	let c = Dual2::new( -1_f64, -2_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_dual_normsq() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = a.norm_sq();
	assert_eq!( b, 1_f64 );
}

#[test]
fn test_dual_det() {
	let a = Dual2::new( 0_f64, 1_f64 );
	let b = a.det();
	assert_eq!( b, 0_f64 );
}

#[test]
fn test_dual_one() {
	let a: Dual2<f64> = One::one();
	let b = Dual2::new( 1_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_dual_eps() {
	let a = Dual2::new( 1_f64, 2_f64 );
	let b = Dual2::new( 1_f64, 2_f64 );
	assert!( a.close_eps(&b, 0_f64) );
	let c = Dual2::new( 2_f64, 2_f64 );
	assert!( !a.close_eps(&c, 0_f64) );
	let d = Dual2::new( 1_f64, 0_f64 );
	assert!( !a.close_eps(&d, 0_f64) );
}

#[test]
fn test_complex_scale() {
	let a: Complex<f64> = Scale::scale(2_f64);
	let b = Complex::new( 2_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_complex_add() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = Complex::new( 2_f64, 1_f64 );
	let c = a + b;
	let d = Complex::new( 3_f64, 3_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_sub() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = Complex::new( 2_f64, 1_f64 );
	let c = a - b;
	let d = Complex::new( -1_f64, 1_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_mul() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = Complex::new( 2_f64, 1_f64 );
	let c = a * b;
	let d = Complex::new( 0_f64, 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_div() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = Complex::new( 2_f64, 1_f64 );
	let c = a / b;
	let d = Complex::new( 4_f64 / 5_f64, 3_f64 / 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_inv() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = a.inv();
	let c = Complex::new( 0.2_f64, -0.4_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_complex_neg() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = -a;
	let c = Complex::new( -1_f64, -2_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_complex_norm_sq() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = a.norm_sq();
	let c = 5_f64;
	assert_eq!( b, c );
}

#[test]
fn test_complex_one() {
	let a: Complex<f64> = One::one();
	let b = Complex::new( 1_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_complex_eps() {
	let a = Complex::new( 1_f64, 2_f64 );
	let b = Complex::new( 1_f64, 2_f64 );
	assert!( a.close_eps(&b, 0_f64) );
	let c = Complex::new( 2_f64, 2_f64 );
	assert!( !a.close_eps(&c, 0_f64) );
	let d = Complex::new( 1_f64, 0_f64 );
	assert!( !a.close_eps(&d, 0_f64) );
}

#[test]
fn test_complex_det() {
	let a = Complex::new( 0_f64, 0_f64 );
	let b = a.det();
	assert_eq!( b, 0_f64 );
}

#[test]
fn test_quaternion_scale() {
	let a: Quaternion<f64> = Scale::scale( 2_f64 );
	let b = Quaternion::new( 0_f64, 0_f64, 0_f64, 2_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_quaternion_add() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = Quaternion::new( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a + b;
	let d = Quaternion::new( 5_f64, 5_f64, 5_f64, 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_sub() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = Quaternion::new( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a - b;
	let d = Quaternion::new( -3_f64, -1_f64, 1_f64, 3_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_mul() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = Quaternion::new( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a * b;
	let d = Quaternion::new( 12_f64, 24_f64, 6_f64, -12_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_neg() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = -a;
	let c = Quaternion::new( -1_f64, -2_f64, -3_f64, -4_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_quaternion_norm_sq() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = a.norm_sq();
	let c = 30_f64;
	assert_eq!( b, c );
}

#[test]
fn test_quaternion_div() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = Quaternion::new( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a / b;
	let n = b.norm_sq();
	let d = Quaternion::new( -10_f64 / n, -20_f64 / n, 0_f64 / n, 20_f64 / n );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_inv() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = a.inv().inv();
	assert!( b.close_eps(&a, 0.00001_f64) );
}

#[test]
fn test_quaternion_one() {
	let a: Quaternion<f64> = One::one();
	let b = Quaternion::new( 0_f64, 0_f64, 0_f64, 1_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_quaternion_eps() {
	let a = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = Quaternion::new( 1_f64, 2_f64, 3_f64, 4_f64 );
	assert!( a.close_eps(&b, 0_f64) );
	let b = Quaternion::new( 0_f64, 2_f64, 3_f64, 4_f64 );
	assert!( !a.close_eps(&b, 0_f64) );
	let b = Quaternion::new( 1_f64, 0_f64, 3_f64, 4_f64 );
	assert!( !a.close_eps(&b, 0_f64) );
	let b = Quaternion::new( 1_f64, 2_f64, 0_f64, 4_f64 );
	assert!( !a.close_eps(&b, 0_f64) );	
	let b = Quaternion::new( 1_f64, 2_f64, 3_f64, 0_f64 );
	assert!( !a.close_eps(&b, 0_f64) );
}

#[test]
fn test_matrix4_scale() {
	let a: Matrix4<f64> = Scale::scale( 2_f64 );
	let b = Matrix4::new(
		2_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 2_f64
	);
	assert_eq!( a, b );
}

#[test]
fn test_matrix4_add() {
	let a = Matrix4::new(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = Matrix4::new(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 3_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let c = a + b;
	let d = Matrix4::new(
		5_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 5_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 5_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 5_f64
	);
	assert_eq!( c, d );
}

#[test]
fn test_matrix4_sub() {
	let a = Matrix4::new(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = Matrix4::new(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 3_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let c = a - b;
	let d = Matrix4::new(
		-3_f64, 0_f64, 0_f64, 0_f64,
		0_f64, -1_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 1_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 3_f64
	);
	assert_eq!( c, d );
}

#[test]
fn test_matrix4_mul() {
	let a = Matrix4::new(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = Matrix4::new(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 3_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let c = a * b;
	let d = Matrix4::new(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 6_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 6_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	assert_eq!( c, d );
}

#[test]
fn test_matrix4_neg() {
	let a = Matrix4::new(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = -a;
	let c = Matrix4::new(
		-1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, -2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, -3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, -4_f64
	);
	assert_eq!( b, c );
}

#[test]
fn test_matrix4_det() {
	let a = Matrix4::new(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = a.det();
	let c = 24_f64;
	assert_eq!( b, c );
}

#[test]
fn test_matrix4_inv() {
	let a = Matrix4::new(
		2_f64, 3_f64, 5_f64, 7_f64,
		11_f64, 13_f64, 17_f64, 19_f64,
		23_f64,	29_f64,	31_f64,	37_f64,
		41_f64,	43_f64,	47_f64,	51_f64
	);
	let b = a.inv();
	let c = b.inv();
	assert!( a.close_eps(&c, 0.0001_f64) );
}

#[test]
fn test_matrix4_div() {
	let a = Matrix4::new(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 1_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 1_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let b = a / a;
	assert!( b.close_eps(&a, 0.00001_f64) );
}

#[test]
fn test_matrix4_one() {
	let a: Matrix4<f64> = One::one();
	let b = Matrix4::new(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 1_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 1_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	assert_eq!( a, b );
}

#[test]
fn test_vector_add() {
	let a = Vector::new( ~[1, 2, 3] );
	let b = Vector::new( ~[3, 2, 1] );
	let c = a + b;
	let d = Vector::new( ~[4, 4, 4] );
	assert_eq!( c, d );
}

#[test]
fn test_vector_sub() {
	let a = Vector::new( ~[1, 2, 3] );
	let b = Vector::new( ~[3, 2, 1] );
	let c = a - b;
	let d = Vector::new( ~[-2, 0, 2] );
	assert_eq!( c, d );
}

#[test]
fn test_vector_mul() {
	let a = Vector::new( ~[1, 2, 3] );
	let b = Vector::new( ~[3, 2, 1] );
	let c = a * b;
	let d = Vector::new( ~[3, 4, 3] );
	assert_eq!( c, d );
}

#[test]
fn test_vector_div() {
	let a = Vector::new( ~[1_f64, 2_f64, 3_f64] );
	let b = Vector::new( ~[3_f64, 2_f64, 1_f64] );
	let c = a / b;
	let d = Vector::new( ~[1_f64 / 3_f64, 1_f64, 3_f64] );
	assert!( c.close_eps(&d, 0_f64) );
}

#[test]
fn test_vector_inv() {
	let a = Vector::new( ~[1_f64, 2_f64, 3_f64] );
	let b = a.inv();
	let c = Vector::new( ~[1_f64, 0.5_f64, 1_f64/3_f64] );
	assert!( b.close_eps(&c, 0.00001f64) );
}

#[test]
fn test_vector_neg() {
	let a = Vector::new( ~[1_f64, 2_f64, 3_f64] );
	let b = -a;
	let c = Vector::new( ~[-1_f64, -2_f64, -3_f64] );
	assert_eq!( b, c );
}

#[test]
fn test_vector_norm_sq() {
	let a = Vector::new( ~[1_f64, 2_f64, 3_f64] );
	let b = a.norm_sq();
	let c = 14_f64;
	assert_eq!( b, c );
}

#[test]
fn test_f64_eps() {
	let a = 1_f64;
	let b = 1_f64;
	assert!( a.close_eps(&b, 0_f64) );
}

#[test]
fn test_f32_eps() {
	let a = 1f32;
	let b = 1f32;
	assert!( a.close_eps(&b, 0_f64) );
}

#[test]
fn test_f32() {
	let a = 1f32;
	let b = 1f32;
	assert!( a.approx_eq_eps(&b, &0.01f32) );
}

