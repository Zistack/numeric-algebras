#![feature (maybe_uninit_uninit_array)]
#![feature (maybe_uninit_array_assume_init)]

mod macros;

pub mod traits;
pub mod algebras;

pub use abstract_algebras::use_algebra;

#[macro_export]
macro_rules! a
{
	($algebra: expr, $expr: expr) =>
	{
		$crate::use_algebra!
		(
			$algebra, $expr;
			(-)
			(+, -, *, /, +=, -=, *=, /=)
			(abs, recip, sqrt, pow, log, exp, ln, sin, cos, tan, sin_cos,
				is_zero, is_one, is_e, is_pi, is_inf, is_nan, sum, convert,
				try_convert, approx_convert)
			(zero, one, e, pi, inf, nan, zero_accumulator)
		)
	}
}
