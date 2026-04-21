use std::borrow::Borrow;

use ::ordered_float::NotNan;

use macrospace_autotransform::{
	define_autotransform,
	group_autotransforms,
	delegate
};
use macrospace_autotransform::common_transforms::*;

use crate::traits::*;

use super::float::*;

macro_rules! def_not_nan_op_assign
{
	($f: tt, $F: ident, $OpAssign: ident, $op_assign: ident, $opassignsym: tt) =>
	{
		paste::paste!
		{
			impl $OpAssign <NotNan <$f>, NotNan <$f>> for [<NotNan $F Algebra>]
			{
				fn $op_assign (self, lhs: &mut NotNan <$f>, rhs: NotNan <$f>)
				{
					*lhs $opassignsym rhs;
				}
			}

			impl <'a> $OpAssign <NotNan <$f>, &'a NotNan <$f>>
			for [<NotNan $F Algebra>]
			{
				fn $op_assign (self, lhs: &mut NotNan <$f>, rhs: &'a NotNan <$f>)
				{
					*lhs $opassignsym rhs;
				}
			}
		}
	}
}

macro_rules! def_not_nan_algebra
{
	($f: tt, $F: ident) => {def_not_nan_algebra! (@inner $f, $F, $);};
	(@inner $f: tt, $F: ident, $D: tt) =>
	{
		paste::paste!
		{
			define_autotransform!
			{
				autotransform [<OwnedNotNan $F ToOwned $F>]
				[NotNan <$f>] -> [$f]
				{
					arg . into_inner ()
				}
			}

			define_autotransform!
			{
				autotransform [<RefNotNan $F ToRef $F>]
				[& $D($D a: lifetime)? NotNan <$f>] -> [& $D($D a: lifetime)? $f]
				{
					arg . borrow ()
				}
			}

			group_autotransforms!
			{
				autotransform [<NotNan $F To $F>]
				[
					[<OwnedNotNan $F ToOwned $F>],
					[<RefNotNan $F ToRef $F>]
				]
			}

			define_autotransform!
			{
				autotransform [<$F ToNotNan $F>]
				[$f] -> [NotNan <$f>]
				{
					NotNan::new (arg) . unwrap ()
				}
			}

			#[derive (Copy, Clone, Debug)]
			pub struct [<NotNan $F Algebra>];

			define_autotransform!
			{
				autotransform [<NotNan $F AlgebraTo $F Algebra>]
				[[<NotNan $F Algebra>]] -> [[<$F Algebra>]]
				{
					[<$F Algebra>]
				}
			}

			delegate!
			{
				impl [<NotNan $F Algebra>]
				with [[<NotNan $F AlgebraTo $F Algebra>], [<NotNan $F To $F>]]
					-> [[<$F ToNotNan $F>], Tuple]
				{
					trait Neg <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Neg <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait Abs <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Abs <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait Recip <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Recip <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait Sqrt <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Sqrt <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Ln <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Ln <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait Exp <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Exp <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Sin <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Sin <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait Cos <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Cos <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait Tan <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Tan <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait SinCos <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> SinCos <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Add <NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Add <NotNan <$f>, &'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Add <&'a NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a, 'b> Add <&'a NotNan <$f>, &'b NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Sub <NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Sub <NotNan <$f>, &'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Sub <&'a NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a, 'b> Sub <&'a NotNan <$f>, &'b NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Mul <NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Mul <NotNan <$f>, &'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Mul <&'a NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a, 'b> Mul <&'a NotNan <$f>, &'b NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Div <NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Div <NotNan <$f>, &'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Div <&'a NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a, 'b> Div <&'a NotNan <$f>, &'b NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Log <NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Log <NotNan <$f>, &'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Log <&'a NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a, 'b> Log <&'a NotNan <$f>, &'b NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Pow <NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Pow <NotNan <$f>, &'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> Pow <&'a NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a, 'b> Pow <&'a NotNan <$f>, &'b NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait Zero <NotNan <$f>>;
					trait One <NotNan <$f>>;
					trait E <NotNan <$f>>;
					trait Pi <NotNan <$f>>;
					trait Inf <NotNan <$f>>;

					trait ElementsSum <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> ElementsSum <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait ElementsProduct <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> ElementsProduct <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait PNorm <NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> PNorm <NotNan <$f>, &'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> PNorm <&'a NotNan <$f>, NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a, 'b> PNorm <&'a NotNan <$f>, &'b NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}

					trait EuclideanNorm <NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
					trait <'a> EuclideanNorm <&'a NotNan <$f>>
					{
						type Output = NotNan <$f>;
					}
				}
			}

			def_not_nan_op_assign! ($f, $F, AddAssign, add_assign, +=);
			def_not_nan_op_assign! ($f, $F, SubAssign, sub_assign, -=);
			def_not_nan_op_assign! ($f, $F, MulAssign, mul_assign, *=);
			def_not_nan_op_assign! ($f, $F, DivAssign, div_assign, /=);

			impl PowAssign <NotNan <$f>, NotNan <$f>> for [<NotNan $F Algebra>]
			{
				fn pow_assign (self, lhs: &mut NotNan <$f>, rhs: NotNan <$f>)
				{
					use ordered_float::Pow;
					*lhs = lhs . pow (rhs);
				}
			}
			impl <'a> PowAssign <NotNan <$f>, &'a NotNan <$f>>
			for [<NotNan $F Algebra>]
			{
				fn pow_assign (self, lhs: &mut NotNan <$f>, rhs: &'a NotNan <$f>)
				{
					use ordered_float::Pow;
					*lhs = lhs . pow (rhs);
				}
			}

			impl LogAssign <NotNan <$f>, NotNan <$f>> for [<NotNan $F Algebra>]
			{
				fn log_assign (self, lhs: &mut NotNan <$f>, rhs: NotNan <$f>)
				{
					*lhs = NotNan::new
					(
						lhs . into_inner () . log (rhs . into_inner ())
					) . unwrap ();
				}
			}
			impl <'a> LogAssign <NotNan <$f>, &'a NotNan <$f>>
			for [<NotNan $F Algebra>]
			{
				fn log_assign (self, lhs: &mut NotNan <$f>, rhs: &'a NotNan <$f>)
				{
					*lhs = NotNan::new
					(
						lhs . into_inner () . log (rhs . into_inner ())
					) . unwrap ();
				}
			}

			impl AdditionIsCommutative <NotNan <$f>, NotNan <$f>>
			for [<NotNan $F Algebra>]
			{
			}

			impl MultiplicationIsCommutative <NotNan <$f>, NotNan <$f>>
			for [<NotNan $F Algebra>]
			{
			}

			impl Accumulatable <NotNan <$f>> for [<NotNan $F Algebra>]
			{
				type AccumulatorAlgebra = Self;
				type Accumulator = NotNan <$f>;

				fn accumulator (self) -> Self::AccumulatorAlgebra
				{
					self
				}
			}
		}
	}
}

def_not_nan_algebra! (f32, F32);
def_not_nan_algebra! (f64, F64);

delegate!
{
	impl NotNanF32Algebra
	with [NotNanF32AlgebraToF32Algebra, NotNanF32ToF32, NotNanF64ToF64]
		-> [F32ToNotNanF32]
	{
		trait Convert <NotNan <f32>, NotNan <f32>>;

		trait Convert <u8, NotNan <f32>>;
		trait Convert <u16, NotNan <f32>>;
		trait Convert <i8, NotNan <f32>>;
		trait Convert <i16, NotNan <f32>>;
		trait Convert <f32, NotNan <f32>>;

		trait ApproxConvert <NotNan <f32>, NotNan <f32>>;
		trait ApproxConvert <NotNan <f64>, NotNan <f32>>;

		trait ApproxConvert <u8, NotNan <f32>>;
		trait ApproxConvert <u16, NotNan <f32>>;
		trait ApproxConvert <u32, NotNan <f32>>;
		trait ApproxConvert <u64, NotNan <f32>>;
		trait ApproxConvert <usize, NotNan <f32>>;
		trait ApproxConvert <i8, NotNan <f32>>;
		trait ApproxConvert <i16, NotNan <f32>>;
		trait ApproxConvert <i32, NotNan <f32>>;
		trait ApproxConvert <i64, NotNan <f32>>;
		trait ApproxConvert <isize, NotNan <f32>>;
		trait ApproxConvert <f32, NotNan <f32>>;
		trait ApproxConvert <f64, NotNan <f32>>;
	}
}

delegate!
{
	impl NotNanF64Algebra
	with [NotNanF64AlgebraToF64Algebra, NotNanF64ToF64, NotNanF32ToF32]
		-> [F64ToNotNanF64]
	{
		trait Convert <NotNan <f32>, NotNan <f64>>;
		trait Convert <NotNan <f64>, NotNan <f64>>;

		trait Convert <u8, NotNan <f64>>;
		trait Convert <u16, NotNan <f64>>;
		trait Convert <u32, NotNan <f64>>;
		trait Convert <i8, NotNan <f64>>;
		trait Convert <i16, NotNan <f64>>;
		trait Convert <i32, NotNan <f64>>;
		trait Convert <f32, NotNan <f64>>;
		trait Convert <f64, NotNan <f64>>;

		trait ApproxConvert <NotNan <f32>, NotNan <f64>>;
		trait ApproxConvert <NotNan <f64>, NotNan <f64>>;

		trait ApproxConvert <u8, NotNan <f64>>;
		trait ApproxConvert <u16, NotNan <f64>>;
		trait ApproxConvert <u32, NotNan <f64>>;
		trait ApproxConvert <u64, NotNan <f64>>;
		trait ApproxConvert <usize, NotNan <f64>>;
		trait ApproxConvert <i8, NotNan <f64>>;
		trait ApproxConvert <i16, NotNan <f64>>;
		trait ApproxConvert <i32, NotNan <f64>>;
		trait ApproxConvert <i64, NotNan <f64>>;
		trait ApproxConvert <isize, NotNan <f64>>;
		trait ApproxConvert <f32, NotNan <f64>>;
		trait ApproxConvert <f64, NotNan <f64>>;
	}
}
