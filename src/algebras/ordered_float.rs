use ::ordered_float::OrderedFloat;

use macrospace_autotransform::{
	define_autotransform,
	group_autotransforms,
	delegate
};
use macrospace_autotransform::common_transforms::*;

use crate::traits::*;

use super::float::*;

macro_rules! def_ordered_float_algebra
{
	($f: tt, $F: ident) => {def_ordered_float_algebra! (@inner $f, $F, $);};
	(@inner $f: tt, $F: ident, $D: tt) =>
	{
		paste::paste!
		{
			define_autotransform!
			{
				autotransform [<OwnedOrdered $F ToOwned $F>]
				[OrderedFloat <$f>] -> [$f]
				{
					arg . 0
				}
			}

			define_autotransform!
			{
				autotransform [<RefOrdered $F ToRef $F>]
				[& $D($D a: lifetime)? OrderedFloat <$f>] -> [& $D($D a: lifetime)? $f]
				{
					&arg . 0
				}
			}

			define_autotransform!
			{
				autotransform [<RefMutOrdered $F ToRefMut $F>]
				[& $D($D a: lifetime)? mut OrderedFloat <$f>] -> [& $D($D a: lifetime)? mut $f]
				{
					&mut arg . 0
				}
			}

			group_autotransforms!
			{
				autotransform [<Ordered $F To $F>]
				[
					[<OwnedOrdered $F ToOwned $F>],
					[<RefOrdered $F ToRef $F>],
					[<RefMutOrdered $F ToRefMut $F>]
				]
			}

			define_autotransform!
			{
				autotransform [<$F ToOrdered $F>]
				[$f] -> [OrderedFloat <$f>]
				{
					OrderedFloat (arg)
				}
			}

			#[derive (Copy, Clone, Debug)]
			pub struct [<Ordered $F Algebra>];

			define_autotransform!
			{
				autotransform [<Ordered $F AlgebraTo $F Algebra>]
				[[<Ordered $F Algebra>]] -> [[<$F Algebra>]]
				{
					[<$F Algebra>]
				}
			}

			delegate!
			{
				impl [<Ordered $F Algebra>]
				with [[<Ordered $F AlgebraTo $F Algebra>], [<Ordered $F To $F>]]
					-> [[<$F ToOrdered $F>], Tuple]
				{
					trait Neg <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Neg <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait Abs <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Abs <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait Recip <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Recip <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait Sqrt <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Sqrt <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait Ln <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Ln <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait Exp <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Exp <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait Sin <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Sin <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait Cos <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Cos <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait Tan <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Tan <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait SinCos <OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> SinCos <&'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait Add <OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Add <OrderedFloat <$f>, &'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Add <&'a OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a, 'b> Add <&'a OrderedFloat <$f>, &'b OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait AddAssign <OrderedFloat <$f>, OrderedFloat <$f>>;
					trait <'a> AddAssign <OrderedFloat <$f>, &'a OrderedFloat <$f>>;

					trait Sub <OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Sub <OrderedFloat <$f>, &'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Sub <&'a OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a, 'b> Sub <&'a OrderedFloat <$f>, &'b OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait SubAssign <OrderedFloat <$f>, OrderedFloat <$f>>;
					trait <'a> SubAssign <OrderedFloat <$f>, &'a OrderedFloat <$f>>;

					trait Mul <OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Mul <OrderedFloat <$f>, &'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Mul <&'a OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a, 'b> Mul <&'a OrderedFloat <$f>, &'b OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait MulAssign <OrderedFloat <$f>, OrderedFloat <$f>>;
					trait <'a> MulAssign <OrderedFloat <$f>, &'a OrderedFloat <$f>>;

					trait Div <OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Div <OrderedFloat <$f>, &'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Div <&'a OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a, 'b> Div <&'a OrderedFloat <$f>, &'b OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait DivAssign <OrderedFloat <$f>, OrderedFloat <$f>>;
					trait <'a> DivAssign <OrderedFloat <$f>, &'a OrderedFloat <$f>>;

					trait Log <OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Log <OrderedFloat <$f>, &'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Log <&'a OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a, 'b> Log <&'a OrderedFloat <$f>, &'b OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait LogAssign <OrderedFloat <$f>, OrderedFloat <$f>>;
					trait <'a> LogAssign <OrderedFloat <$f>, &'a OrderedFloat <$f>>;

					trait Pow <OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Pow <OrderedFloat <$f>, &'a OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a> Pow <&'a OrderedFloat <$f>, OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}
					trait <'a, 'b> Pow <&'a OrderedFloat <$f>, &'b OrderedFloat <$f>>
					{
						type Output = OrderedFloat <$f>;
					}

					trait PowAssign <OrderedFloat <$f>, OrderedFloat <$f>>;
					trait <'a> PowAssign <OrderedFloat <$f>, &'a OrderedFloat <$f>>;

					trait Zero <OrderedFloat <$f>>;
					trait One <OrderedFloat <$f>>;
					trait E <OrderedFloat <$f>>;
					trait Pi <OrderedFloat <$f>>;
					trait Inf <OrderedFloat <$f>>;

					trait NaN <OrderedFloat <$f>>;
				}
			}

			impl AdditionIsCommutative <OrderedFloat <$f>, OrderedFloat <$f>>
			for [<Ordered $F Algebra>]
			{
			}

			impl MultiplicationIsCommutative <OrderedFloat <$f>, OrderedFloat <$f>>
			for [<Ordered $F Algebra>]
			{
			}

			impl Accumulatable <OrderedFloat <$f>> for [<Ordered $F Algebra>]
			{
				type AccumulatorAlgebra = Self;
				type Accumulator = OrderedFloat <$f>;

				fn accumulator (self) -> Self::AccumulatorAlgebra
				{
					self
				}
			}
		}
	}
}

def_ordered_float_algebra! (f32, F32);
def_ordered_float_algebra! (f64, F64);

delegate!
{
	impl OrderedF32Algebra
	with [OrderedF32AlgebraToF32Algebra, OrderedF32ToF32, OrderedF64ToF64]
		-> [F32ToOrderedF32]
	{
		trait Convert <OrderedFloat <f32>, OrderedFloat <f32>>;

		trait Convert <u8, OrderedFloat <f32>>;
		trait Convert <u16, OrderedFloat <f32>>;
		trait Convert <i8, OrderedFloat <f32>>;
		trait Convert <i16, OrderedFloat <f32>>;
		trait Convert <f32, OrderedFloat <f32>>;

		trait ApproxConvert <OrderedFloat <f32>, OrderedFloat <f32>>;
		trait ApproxConvert <OrderedFloat <f64>, OrderedFloat <f32>>;

		trait ApproxConvert <u8, OrderedFloat <f32>>;
		trait ApproxConvert <u16, OrderedFloat <f32>>;
		trait ApproxConvert <u32, OrderedFloat <f32>>;
		trait ApproxConvert <u64, OrderedFloat <f32>>;
		trait ApproxConvert <usize, OrderedFloat <f32>>;
		trait ApproxConvert <i8, OrderedFloat <f32>>;
		trait ApproxConvert <i16, OrderedFloat <f32>>;
		trait ApproxConvert <i32, OrderedFloat <f32>>;
		trait ApproxConvert <i64, OrderedFloat <f32>>;
		trait ApproxConvert <isize, OrderedFloat <f32>>;
		trait ApproxConvert <f32, OrderedFloat <f32>>;
		trait ApproxConvert <f64, OrderedFloat <f32>>;
	}
}

delegate!
{
	impl OrderedF64Algebra
	with [OrderedF64AlgebraToF64Algebra, OrderedF64ToF64, OrderedF32ToF32]
		-> [F64ToOrderedF64]
	{
		trait Convert <OrderedFloat <f32>, OrderedFloat <f64>>;
		trait Convert <OrderedFloat <f64>, OrderedFloat <f64>>;

		trait Convert <u8, OrderedFloat <f64>>;
		trait Convert <u16, OrderedFloat <f64>>;
		trait Convert <u32, OrderedFloat <f64>>;
		trait Convert <i8, OrderedFloat <f64>>;
		trait Convert <i16, OrderedFloat <f64>>;
		trait Convert <i32, OrderedFloat <f64>>;
		trait Convert <f32, OrderedFloat <f64>>;
		trait Convert <f64, OrderedFloat <f64>>;

		trait ApproxConvert <OrderedFloat <f32>, OrderedFloat <f64>>;
		trait ApproxConvert <OrderedFloat <f64>, OrderedFloat <f64>>;

		trait ApproxConvert <u8, OrderedFloat <f64>>;
		trait ApproxConvert <u16, OrderedFloat <f64>>;
		trait ApproxConvert <u32, OrderedFloat <f64>>;
		trait ApproxConvert <u64, OrderedFloat <f64>>;
		trait ApproxConvert <usize, OrderedFloat <f64>>;
		trait ApproxConvert <i8, OrderedFloat <f64>>;
		trait ApproxConvert <i16, OrderedFloat <f64>>;
		trait ApproxConvert <i32, OrderedFloat <f64>>;
		trait ApproxConvert <i64, OrderedFloat <f64>>;
		trait ApproxConvert <isize, OrderedFloat <f64>>;
		trait ApproxConvert <f32, OrderedFloat <f64>>;
		trait ApproxConvert <f64, OrderedFloat <f64>>;
	}
}
