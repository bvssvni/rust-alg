extern mod alg;

#[test]
fn test_dual_scale() {
	let a: alg::Dual2<f64> = alg::scale( 2_f64 );
	let b = alg::dual2( 2_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_dual_add() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = alg::dual2( 2_f64, 1_f64 );
	let c = a + b;
	let d = alg::dual2( 3_f64, 3_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_sub() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = alg::dual2( 2_f64, 1_f64 );
	let c = a - b;
	let d = alg::dual2( -1_f64, 1_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_mul() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = alg::dual2( 2_f64, 1_f64 );
	let c = a * b;
	let d = alg::dual2( 2_f64, 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_div() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = alg::dual2( 2_f64, 1_f64 );
	let c = a / b;
	let d = alg::dual2( 0.5_f64, 0.75_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_dual_inv() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = alg::inv(a);
	let c = alg::dual2( 1_f64, -2_f64 );
	assert_eq!( b, c );

	let a = alg::dual2( 1_f32, 2_f32 );
	let b = alg::inv(a);
	let c = alg::dual2( 1_f32, -2_f32 );
	assert_eq!( b, c );
}

#[test]
fn test_dual_neg() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = -a;
	let c = alg::dual2( -1_f64, -2_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_dual_normsq() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = alg::norm_sq(a);
	assert_eq!( b, 1_f64 );
}

#[test]
fn test_dual_det() {
	let a = alg::dual2( 0_f64, 1_f64 );
	let b = alg::det(a);
	assert_eq!( b, 0_f64 );
}

#[test]
fn test_dual_one() {
	let a: alg::Dual2<f64> = std::num::one();
	let b = alg::dual2( 1_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_dual_eps() {
	let a = alg::dual2( 1_f64, 2_f64 );
	let b = alg::dual2( 1_f64, 2_f64 );
	assert!( alg::close_eps(&a, &b, 0_f64) );
	let c = alg::dual2( 2_f64, 2_f64 );
	assert!( !alg::close_eps(&a, &c, 0_f64) );
	let d = alg::dual2( 1_f64, 0_f64 );
	assert!( !alg::close_eps(&a, &d, 0_f64) );
}

#[test]
fn test_complex_scale() {
	let a: alg::Complex<f64> = alg::scale(2_f64);
	let b = alg::complex( 2_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_complex_add() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = alg::complex( 2_f64, 1_f64 );
	let c = a + b;
	let d = alg::complex( 3_f64, 3_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_sub() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = alg::complex( 2_f64, 1_f64 );
	let c = a - b;
	let d = alg::complex( -1_f64, 1_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_mul() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = alg::complex( 2_f64, 1_f64 );
	let c = a * b;
	let d = alg::complex( 0_f64, 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_div() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = alg::complex( 2_f64, 1_f64 );
	let c = a / b;
	let d = alg::complex( 4_f64 / 5_f64, 3_f64 / 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_complex_inv() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = alg::inv(a);
	let c = alg::complex( 0.2_f64, -0.4_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_complex_neg() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = -a;
	let c = alg::complex( -1_f64, -2_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_complex_norm_sq() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = alg::norm_sq(a);
	let c = 5_f64;
	assert_eq!( b, c );
}

#[test]
fn test_complex_one() {
	let a: alg::Complex<f64> = std::num::one();
	let b = alg::complex( 1_f64, 0_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_complex_eps() {
	let a = alg::complex( 1_f64, 2_f64 );
	let b = alg::complex( 1_f64, 2_f64 );
	assert!( alg::close_eps(&a, &b, 0_f64) );
	let c = alg::complex( 2_f64, 2_f64 );
	assert!( !alg::close_eps(&a, &c, 0_f64) );
	let d = alg::complex( 1_f64, 0_f64 );
	assert!( !alg::close_eps(&a, &d, 0_f64) );
}

#[test]
fn test_complex_det() {
	let a = alg::complex( 0_f64, 0_f64 );
	let b = alg::det(a);
	assert_eq!( b, 0_f64 );
}

#[test]
fn test_quaternion_scale() {
	let a: alg::Quaternion<f64> = alg::scale( 2_f64 );
	let b = alg::quaternion( 0_f64, 0_f64, 0_f64, 2_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_quaternion_add() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = alg::quaternion( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a + b;
	let d = alg::quaternion( 5_f64, 5_f64, 5_f64, 5_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_sub() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = alg::quaternion( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a - b;
	let d = alg::quaternion( -3_f64, -1_f64, 1_f64, 3_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_mul() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = alg::quaternion( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a * b;
	let d = alg::quaternion( 12_f64, 24_f64, 6_f64, -12_f64 );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_neg() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = -a;
	let c = alg::quaternion( -1_f64, -2_f64, -3_f64, -4_f64 );
	assert_eq!( b, c );
}

#[test]
fn test_quaternion_norm_sq() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = alg::norm_sq(a);
	let c = 30_f64;
	assert_eq!( b, c );
}

#[test]
fn test_quaternion_div() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = alg::quaternion( 4_f64, 3_f64, 2_f64, 1_f64 );
	let c = a / b;
	let n = alg::norm_sq(b);
	let d = alg::quaternion( -10_f64 / n, -20_f64 / n, 0_f64 / n, 20_f64 / n );
	assert_eq!( c, d );
}

#[test]
fn test_quaternion_inv() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = alg::inv(alg::inv(a));
	assert!( alg::close_eps(&b, &a, 0.00001_f64) );
}

#[test]
fn test_quaternion_one() {
	let a: alg::Quaternion<f64> = std::num::one();
	let b = alg::quaternion( 0_f64, 0_f64, 0_f64, 1_f64 );
	assert_eq!( a, b );
}

#[test]
fn test_quaternion_eps() {
	let a = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	let b = alg::quaternion( 1_f64, 2_f64, 3_f64, 4_f64 );
	assert!( alg::close_eps(&a, &b, 0_f64) );
	let b = alg::quaternion( 0_f64, 2_f64, 3_f64, 4_f64 );
	assert!( !alg::close_eps(&a, &b, 0_f64) );
	let b = alg::quaternion( 1_f64, 0_f64, 3_f64, 4_f64 );
	assert!( !alg::close_eps(&a, &b, 0_f64) );
	let b = alg::quaternion( 1_f64, 2_f64, 0_f64, 4_f64 );
	assert!( !alg::close_eps(&a, &b, 0_f64) );	
	let b = alg::quaternion( 1_f64, 2_f64, 3_f64, 0_f64 );
	assert!( !alg::close_eps(&a, &b, 0_f64) );
}

#[test]
fn test_matrix4_scale() {
	let a: alg::Matrix4<f64> = alg::scale( 2_f64 );
	let b = alg::matrix4(
		2_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 2_f64
	);
	assert_eq!( a, b );
}

#[test]
fn test_matrix4_add() {
	let a = alg::matrix4(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = alg::matrix4(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 3_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let c = a + b;
	let d = alg::matrix4(
		5_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 5_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 5_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 5_f64
	);
	assert_eq!( c, d );
}

#[test]
fn test_matrix4_sub() {
	let a = alg::matrix4(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = alg::matrix4(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 3_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let c = a - b;
	let d = alg::matrix4(
		-3_f64, 0_f64, 0_f64, 0_f64,
		0_f64, -1_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 1_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 3_f64
	);
	assert_eq!( c, d );
}

#[test]
fn test_matrix4_mul() {
	let a = alg::matrix4(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = alg::matrix4(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 3_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 2_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let c = a * b;
	let d = alg::matrix4(
		4_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 6_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 6_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	assert_eq!( c, d );
}

#[test]
fn test_matrix4_neg() {
	let a = alg::matrix4(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = -a;
	let c = alg::matrix4(
		-1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, -2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, -3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, -4_f64
	);
	assert_eq!( b, c );
}

#[test]
fn test_matrix4_det() {
	let a = alg::matrix4(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 2_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 3_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 4_f64
	);
	let b = alg::det(a);
	let c = 24_f64;
	assert_eq!( b, c );
}

#[test]
fn test_matrix4_inv() {
	let a = alg::matrix4(
		2_f64, 3_f64, 5_f64, 7_f64,
		11_f64, 13_f64, 17_f64, 19_f64,
		23_f64,	29_f64,	31_f64,	37_f64,
		41_f64,	43_f64,	47_f64,	51_f64
	);
	let b = alg::inv(a);
	let c = alg::inv(b);
	assert!( alg::close_eps(&a, &c, 0.0001_f64) );
}

#[test]
fn test_matrix4_div() {
	let a = alg::matrix4(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 1_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 1_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	let b = a / a;
	assert!( alg::close_eps(&b, &a, 0.00001_f64) );
}

#[test]
fn test_matrix4_one() {
	let a: alg::Matrix4<f64> = std::num::one();
	let b = alg::matrix4(
		1_f64, 0_f64, 0_f64, 0_f64,
		0_f64, 1_f64, 0_f64, 0_f64,
		0_f64, 0_f64, 1_f64, 0_f64,
		0_f64, 0_f64, 0_f64, 1_f64
	);
	assert_eq!( a, b );
}

#[test]
fn test_vector_add() {
	let a = alg::vector( ~[1, 2, 3] );
	let b = alg::vector( ~[3, 2, 1] );
	let c = a + b;
	let d = alg::vector( ~[4, 4, 4] );
	assert_eq!( c, d );
}

#[test]
fn test_vector_sub() {
	let a = alg::vector( ~[1, 2, 3] );
	let b = alg::vector( ~[3, 2, 1] );
	let c = a - b;
	let d = alg::vector( ~[-2, 0, 2] );
	assert_eq!( c, d );
}

#[test]
fn test_vector_mul() {
	let a = alg::vector( ~[1, 2, 3] );
	let b = alg::vector( ~[3, 2, 1] );
	let c = a * b;
	let d = alg::vector( ~[3, 4, 3] );
	assert_eq!( c, d );
}

#[test]
fn test_vector_div() {
	let a = alg::vector( ~[1_f64, 2_f64, 3_f64] );
	let b = alg::vector( ~[3_f64, 2_f64, 1_f64] );
	let c = a / b;
	let d = alg::vector( ~[1_f64 / 3_f64, 1_f64, 3_f64] );
	assert!( alg::close_eps(&c, &d, 0_f64) );
}

#[test]
fn test_vector_inv() {
	let a = alg::vector( ~[1_f64, 2_f64, 3_f64] );
	let b = alg::inv(a);
	let c = alg::vector( ~[1_f64, 0.5_f64, 1_f64/3_f64] );
	assert!( alg::close_eps(&b, &c, 0.00001f64) );
}

#[test]
fn test_vector_neg() {
	let a = alg::vector( ~[1_f64, 2_f64, 3_f64] );
	let b = -a;
	let c = alg::vector( ~[-1_f64, -2_f64, -3_f64] );
	assert_eq!( b, c );
}

#[test]
fn test_vector_norm_sq() {
	let a = alg::vector( ~[1_f64, 2_f64, 3_f64] );
	let b = alg::norm_sq(a);
	let c = 14_f64;
	assert_eq!( b, c );
}

#[test]
fn test_f64_eps() {
	let a = 1_f64;
	let b = 1_f64;
	assert!( alg::close_eps(&a, &b, 0_f64) );
}

#[test]
fn test_f32_eps() {
	let a = 1f32;
	let b = 1f32;
	assert!( alg::close_eps(&a, &b, 0_f64) );
}

#[test]
fn test_f32() {
	let a = 1f32;
	let b = 1f32;
	assert!( a.approx_eq_eps(&b, &0.01f32) );
}

