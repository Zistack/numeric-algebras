#[cfg (any (test, feature = "proptest"))]
use proptest::strategy::Strategy;

use crate::traits::*;
use crate::macros::*;
use crate::a;

#[derive (Copy, Clone)]
pub struct FloatMath ();

macro_rules! impl_traits_for_ty
{
	($T: tt) =>
	{
		impl_unary_ops! ($T, Neg, neg, -, FloatMath);

		impl_unary_methods! ($T, Abs, abs, FloatMath);
		impl_unary_methods! ($T, Recip, recip, FloatMath);
		impl_unary_methods! ($T, Sqrt, sqrt, FloatMath);

		impl_unary_methods! ($T, Ln, ln, FloatMath);
		impl_unary_methods! ($T, Exp, exp, FloatMath);

		impl_unary_methods! ($T, Sin, sin, FloatMath);
		impl_unary_methods! ($T, Cos, cos, FloatMath);
		impl_unary_methods! ($T, Tan, tan, FloatMath);

		impl_sin_cos! ($T, FloatMath);

		impl_bin_ops! ($T, Add, add, +, FloatMath);
		impl_bin_ops! ($T, Sub, sub, -, FloatMath);
		impl_bin_ops! ($T, Mul, mul, *, FloatMath);
		impl_bin_ops! ($T, Div, div, /, FloatMath);

		impl_op_assigns! ($T, AddAssign, add_assign, +=, FloatMath);
		impl_op_assigns! ($T, SubAssign, sub_assign, -=, FloatMath);
		impl_op_assigns! ($T, MulAssign, mul_assign, *=, FloatMath);
		impl_op_assigns! ($T, DivAssign, div_assign, /=, FloatMath);

		impl_value_partialeq! ($T, Zero, zero, is_zero, 0.0, FloatMath);
		impl_value_partialeq! ($T, One, one, is_one, 1.0, FloatMath);
		impl_value_partialeq! ($T, E, e, is_e, std::$T::consts::E, FloatMath);
		impl_value_partialeq! ($T, Pi, pi, is_pi, std::$T::consts::PI, FloatMath);
		impl_value_partialeq! ($T, Inf, inf, is_inf, $T::INFINITY, FloatMath);

		impl_value_predicate! ($T, NaN, nan, is_nan, $T::NAN, FloatMath);

		impl Accumulatable <$T> for FloatMath
		{
			type Accumulator = $T;

			fn zero_accumulator (self) -> Self::Accumulator
			{
				a! (self, Self::Accumulator::zero ())
			}
		}

		impl Acc <$T, $T> for FloatMath
		{
			fn accumulate (self, acc: &mut Self::Accumulator, x: $T)
			{
				a! (self, *acc += x);
			}
		}

		impl Convert <$T, $T> for FloatMath
		{
			fn convert (self, x: $T) -> $T { x }
		}

		#[cfg (any (test, feature = "proptest"))]
		impl UnitRange <$T> for FloatMath
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

impl_traits_for_ty! (f32);

impl_hetero_bin_methods! (f32, u8, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f32, u16, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f32, i8, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f32, i16, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f32, f32, Log, log, log, FloatMath);

impl_hetero_bin_methods! (f32, u8, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f32, u16, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f32, i8, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f32, i16, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f32, i32, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f32, f32, Pow, pow, powf, FloatMath);

impl_traits_for_ty! (f64);

impl_hetero_bin_methods! (f64, u8, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f64, u16, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f64, u32, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f64, i8, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f64, i16, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f64, i32, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f64, f32, Log, log, log, FloatMath);
impl_hetero_bin_methods! (f64, f64, Log, log, log, FloatMath);

impl_hetero_bin_methods! (f64, u8, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f64, u16, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f64, u32, Pow, pow, powf, FloatMath);
impl_hetero_bin_methods! (f64, i8, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f64, i16, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f64, i32, Pow, pow, powi, FloatMath);
impl_hetero_bin_methods! (f64, f32, Pow, pow, powf, FloatMath);
impl_hetero_bin_methods! (f64, f64, Pow, pow, powf, FloatMath);
