#[cfg (any (test, feature = "proptest"))]
use proptest::strategy::Strategy;

use crate::traits::*;
use crate::macros::impl_traits::*;

macro_rules! impl_traits_for_ty
{
	($T: tt, $Algebra: ty) =>
	{
		impl_unary_ops! ($T, Neg, neg, -, $Algebra);

		impl_unary_methods! ($T, Abs, abs, $Algebra);
		impl_unary_methods! ($T, Recip, recip, $Algebra);
		impl_unary_methods! ($T, Sqrt, sqrt, $Algebra);

		impl_unary_methods! ($T, Ln, ln, $Algebra);
		impl_unary_methods! ($T, Exp, exp, $Algebra);

		impl_unary_methods! ($T, Sin, sin, $Algebra);
		impl_unary_methods! ($T, Cos, cos, $Algebra);
		impl_unary_methods! ($T, Tan, tan, $Algebra);

		impl_sin_cos! ($T, $Algebra);

		impl_bin_ops! ($T, Add, add, +, $Algebra);
		impl_bin_ops! ($T, Sub, sub, -, $Algebra);
		impl_bin_ops! ($T, Mul, mul, *, $Algebra);
		impl_bin_ops! ($T, Div, div, /, $Algebra);

		impl_op_assigns! ($T, AddAssign, add_assign, +=, $Algebra);
		impl_op_assigns! ($T, SubAssign, sub_assign, -=, $Algebra);
		impl_op_assigns! ($T, MulAssign, mul_assign, *=, $Algebra);
		impl_op_assigns! ($T, DivAssign, div_assign, /=, $Algebra);

		impl AdditionIsCommutative <$T, $T> for $Algebra
		{
		}

		impl MultiplicationIsCommutative <$T, $T> for $Algebra
		{
		}

		impl_bin_methods! ($T, Log, log, log, $Algebra);
		impl_bin_methods! ($T, Pow, pow, powf, $Algebra);

		impl_method_assigns! ($T, LogAssign, log_assign, log, $Algebra);
		impl_method_assigns! ($T, PowAssign, pow_assign, powf, $Algebra);

		impl_value_partialeq! ($T, Zero, zero, is_zero, 0.0, $Algebra);
		impl_value_partialeq! ($T, One, one, is_one, 1.0, $Algebra);
		impl_value_partialeq! ($T, E, e, is_e, std::$T::consts::E, $Algebra);
		impl_value_partialeq! ($T, Pi, pi, is_pi, std::$T::consts::PI, $Algebra);
		impl_value_partialeq! ($T, Inf, inf, is_inf, $T::INFINITY, $Algebra);

		impl_value_predicate! ($T, NaN, nan, is_nan, $T::NAN, $Algebra);

		impl Accumulatable <$T> for $Algebra
		{
			type AccumulatorAlgebra = Self;
			type Accumulator = $T;

			fn accumulator (self) -> Self::AccumulatorAlgebra
			{
				self
			}
		}

		#[cfg (any (test, feature = "proptest"))]
		impl UnitRange <$T> for $Algebra
		{
			fn unit_range (self) -> impl Strategy <Value = $T>
			{
				let start: $T = 0.0;
				let end: $T = 1.0;

				start..end
			}
		}
	}
}

#[derive (Copy, Clone)]
pub struct F32Algebra;

impl_traits_for_ty! (f32, F32Algebra);

impl_convert! (u8, f32, F32Algebra);
impl_convert! (u16, f32, F32Algebra);
impl_convert! (i8, f32, F32Algebra);
impl_convert! (i16, f32, F32Algebra);
impl_convert! (f32, f32, F32Algebra);

impl_approx_convert! (u8, f32, F32Algebra);
impl_approx_convert! (u16, f32, F32Algebra);
impl_approx_convert! (u32, f32, F32Algebra);
impl_approx_convert! (u64, f32, F32Algebra);
impl_approx_convert! (usize, f32, F32Algebra);
impl_approx_convert! (i8, f32, F32Algebra);
impl_approx_convert! (i16, f32, F32Algebra);
impl_approx_convert! (i32, f32, F32Algebra);
impl_approx_convert! (i64, f32, F32Algebra);
impl_approx_convert! (isize, f32, F32Algebra);
impl_approx_convert! (f32, f32, F32Algebra);
impl_approx_convert! (f64, f32, F32Algebra);

#[derive (Copy, Clone)]
pub struct F64Algebra;

impl_traits_for_ty! (f64, F64Algebra);

impl_convert! (u8, f64, F64Algebra);
impl_convert! (u16, f64, F64Algebra);
impl_convert! (u32, f64, F64Algebra);
impl_convert! (i8, f64, F64Algebra);
impl_convert! (i16, f64, F64Algebra);
impl_convert! (i32, f64, F64Algebra);
impl_convert! (f32, f64, F64Algebra);
impl_convert! (f64, f64, F64Algebra);

impl_approx_convert! (u8, f64, F64Algebra);
impl_approx_convert! (u16, f64, F64Algebra);
impl_approx_convert! (u32, f64, F64Algebra);
impl_approx_convert! (u64, f64, F64Algebra);
impl_approx_convert! (usize, f64, F64Algebra);
impl_approx_convert! (i8, f64, F64Algebra);
impl_approx_convert! (i16, f64, F64Algebra);
impl_approx_convert! (i32, f64, F64Algebra);
impl_approx_convert! (i64, f64, F64Algebra);
impl_approx_convert! (isize, f64, F64Algebra);
impl_approx_convert! (f32, f64, F64Algebra);
impl_approx_convert! (f64, f64, F64Algebra);
