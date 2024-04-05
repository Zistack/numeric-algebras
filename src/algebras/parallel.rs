#[cfg (any (test, feature = "proptest"))]
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::marker::PhantomData;

use forward_traits::{forward_receiver, forward_traits};
#[cfg (any (test, feature = "proptest"))]
use proptest::strategy::Strategy;

use crate::traits::*;
use crate::a;

#[forward_receiver]
pub struct ParallelAlgebra <A, T>
{
	a: A,
	_t: PhantomData <T>
}

impl <A, T> ParallelAlgebra <A, T>
{
	pub fn new (scalar_algebra: A) -> Self
	{
		Self {a: scalar_algebra, _t: PhantomData::<T>::default ()}
	}
}

impl <A, T> Copy for ParallelAlgebra <A, T>
where A: Copy
{
}

impl <A, T> Clone for ParallelAlgebra <A, T>
where A: Clone
{
	fn clone (&self) -> Self
	{
		Self::new (self . a . clone ())
	}
}

forward_traits!
(
	for ParallelAlgebra . a
	impl for <X> Convert <X, T> + for <X> ApproxConvert <X, T>
);

impl <A, T, const N: usize> Neg <[T; N]> for ParallelAlgebra <A, T>
where A: Clone + Neg <T>
{
	type Output = [A::Output; N];

	fn neg (self, x: [T; N]) -> Self::Output
	{
		x . map (|x_i| a! (self . a, - x_i))
	}
}

impl <'a, A, T, const N: usize> Neg <&'a [T; N]> for ParallelAlgebra <A, T>
where A: Clone + Neg <&'a T>
{
	type Output = [A::Output; N];

	fn neg (self, x: &'a [T; N]) -> Self::Output
	{
		x . each_ref () . map (|x_i| a! (self . a, - x_i))
	}
}

forward_traits! (for ParallelAlgebra . a impl Neg <T> + for <'a> Neg <&'a T>);

macro_rules! impl_unary_method_trait
{
	($Op: ident, $op: ident) =>
	{
		impl <A, T, const N: usize> $Op <[T; N]> for ParallelAlgebra <A, T>
		where A: Clone + $Op <T>
		{
			type Output = [A::Output; N];

			fn $op (self, x: [T; N]) -> Self::Output
			{
				x . map (|x_i| a! (self . a, x_i . $op ()))
			}
		}

		impl <'a, A, T, const N: usize> $Op <&'a [T; N]>
		for ParallelAlgebra <A, T>
		where A: Clone + $Op <&'a T>
		{
			type Output = [A::Output; N];

			fn $op (self, x: &'a [T; N]) -> Self::Output
			{
				x . each_ref () . map (|x_i| a! (self . a, x_i . $op ()))
			}
		}

		forward_traits!
		(
			for ParallelAlgebra . a
			impl $Op <T> + for <'a> $Op <&'a T>
		);
	}
}

impl_unary_method_trait! (Abs, abs);
impl_unary_method_trait! (Recip, recip);
impl_unary_method_trait! (Sqrt, sqrt);
impl_unary_method_trait! (Ln, ln);
impl_unary_method_trait! (Exp, exp);
impl_unary_method_trait! (Sin, sin);
impl_unary_method_trait! (Cos, cos);
impl_unary_method_trait! (Tan, tan);

impl <A, T, const N: usize> SinCos <[T; N]> for ParallelAlgebra <A, T>
where A: Clone + SinCos <T>
{
	type Output = [A::Output; N];

	fn sin_cos (self, x: [T; N]) -> (Self::Output, Self::Output)
	{
		let mut sins: [MaybeUninit <A::Output>; N] = MaybeUninit::uninit_array ();
		let mut coss: [MaybeUninit <A::Output>; N] = MaybeUninit::uninit_array ();

		for (x_i, (sin_i, cos_i))
		in x . into_iter () . zip (sins . iter_mut () . zip (coss . iter_mut ()))
		{
			let (sin, cos) = a! (self . a, x_i . sin_cos ());

			sin_i . write (sin);
			cos_i . write (cos);
		}

		unsafe
		{(
			MaybeUninit::array_assume_init (sins),
			MaybeUninit::array_assume_init (coss)
		)}
	}
}

impl <'a, A, T, const N: usize> SinCos <&'a [T; N]> for ParallelAlgebra <A, T>
where A: Clone + SinCos <&'a T>
{
	type Output = [A::Output; N];

	fn sin_cos (self, x: &'a [T; N]) -> (Self::Output, Self::Output)
	{
		let mut sins = MaybeUninit::uninit_array ();
		let mut coss = MaybeUninit::uninit_array ();

		for (x_i, (sin_i, cos_i))
		in x . into_iter () . zip (sins . iter_mut () . zip (coss . iter_mut ()))
		{
			let (sin, cos) = a! (self . a, x_i . sin_cos ());

			sin_i . write (sin);
			cos_i . write (cos);
		}

		unsafe
		{(
			MaybeUninit::array_assume_init (sins),
			MaybeUninit::array_assume_init (coss)
		)}
	}
}

forward_traits!
(
	for ParallelAlgebra . a
	impl SinCos <T> + for <'a> SinCos <&'a T>
);

macro_rules! impl_bin_op_trait
{
	($Op: ident, $op: ident, $opsym: tt) =>
	{
		impl <A, T, const N: usize> $Op <[T; N], [T; N]>
		for ParallelAlgebra <A, T>
		where A: Clone + $Op <T, T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: [T; N], rhs: [T; N]) -> Self::Output
			{
				let mut output = MaybeUninit::uninit_array ();

				for ((lhs_i, rhs_i), output_i)
				in lhs . into_iter ()
					. zip (rhs . into_iter ())
					. zip (output . iter_mut ())
				{
					output_i . write (a! (self . a, lhs_i $opsym rhs_i));
				}

				unsafe { MaybeUninit::array_assume_init (output) }
			}
		}

		impl <'a, A, T, const N: usize> $Op <[T; N], &'a [T; N]>
		for ParallelAlgebra <A, T>
		where A: Clone + $Op <T, &'a T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: [T; N], rhs: &'a [T; N]) -> Self::Output
			{
				let mut output = MaybeUninit::uninit_array ();

				for ((lhs_i, rhs_i), output_i)
				in lhs . into_iter ()
					. zip (rhs . iter ())
					. zip (output . iter_mut ())
				{
					output_i . write (a! (self . a, lhs_i $opsym rhs_i));
				}

				unsafe { MaybeUninit::array_assume_init (output) }
			}
		}

		impl <'a, A, T, const N: usize> $Op <&'a [T; N], [T; N]>
		for ParallelAlgebra <A, T>
		where A: Clone + $Op <&'a T, T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: &'a [T; N], rhs: [T; N]) -> Self::Output
			{
				let mut output = MaybeUninit::uninit_array ();

				for ((lhs_i, rhs_i), output_i)
				in lhs . iter ()
					. zip (rhs . into_iter ())
					. zip (output . iter_mut ())
				{
					output_i . write (a! (self . a, lhs_i $opsym rhs_i));
				}

				unsafe { MaybeUninit::array_assume_init (output) }
			}
		}

		impl <'a, 'b, A, T, const N: usize> $Op <&'a [T; N], &'b [T; N]>
		for ParallelAlgebra <A, T>
		where A: Clone + $Op <&'a T, &'b T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: &'a [T; N], rhs: &'b [T; N]) -> Self::Output
			{
				let mut output = MaybeUninit::uninit_array ();

				for ((lhs_i, rhs_i), output_i)
				in lhs . iter ()
					. zip (rhs . iter ())
					. zip (output . iter_mut ())
				{
					output_i . write (a! (self . a, lhs_i $opsym rhs_i));
				}

				unsafe { MaybeUninit::array_assume_init (output) }
			}
		}

		// Scalar on RHS

		impl <A, T, const N: usize, O> $Op <[T; N], T>
		for ParallelAlgebra <A, T>
		where A: Clone + for <'a> $Op <T, &'a T, Output = O>
		{
			type Output = [O; N];

			fn $op (self, lhs: [T; N], rhs: T) -> Self::Output
			{
				lhs . map (|lhs_i| a! (self . a, lhs_i $opsym &rhs))
			}
		}

		impl <'a, A, T, const N: usize> $Op <[T; N], &'a T>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + $Op <T, &'a T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: [T; N], rhs: &'a T) -> Self::Output
			{
				lhs . map (|lhs_i| a! (self . a, lhs_i $opsym rhs))
			}
		}

		impl <'a, A, T, const N: usize, O> $Op <&'a [T; N], T>
		for ParallelAlgebra <A, T>
		where A: Clone + for <'b> $Op <&'a T, &'b T, Output = O>
		{
			type Output = [O; N];

			fn $op (self, lhs: &'a [T; N], rhs: T) -> Self::Output
			{
				lhs
					. each_ref ()
					. map (|lhs_i| a! (self . a, lhs_i $opsym &rhs))
			}
		}

		impl <'a, 'b, A, T, const N: usize> $Op <&'a [T; N], &'b T>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + $Op <&'a T, &'b T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: &'a [T; N], rhs: &'b T) -> Self::Output
			{
				lhs . each_ref () . map (|lhs_i| a! (self . a, lhs_i $opsym rhs))
			}
		}

		// Scalar on LHS

		impl <A, T, const N: usize, O> $Op <T, [T; N]>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + for <'a> $Op <&'a T, T, Output = O>
		{
			type Output = [O; N];

			fn $op (self, lhs: T, rhs: [T; N]) -> Self::Output
			{
				rhs . map (|rhs_i| a! (self . a, &lhs $opsym rhs_i))
			}
		}

		impl <'a, A, T, const N: usize, O> $Op <T, &'a [T; N]>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + for <'b> $Op <&'b T, &'a T, Output = O>
		{
			type Output = [O; N];

			fn $op (self, lhs: T, rhs: &'a [T; N]) -> Self::Output
			{
				rhs
					. each_ref ()
					. map (|rhs_i| a! (self . a, &lhs $opsym rhs_i))
			}
		}

		impl <'a, A, T, const N: usize> $Op <&'a T, [T; N]>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + $Op <&'a T, T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: &'a T, rhs: [T; N]) -> Self::Output
			{
				rhs . map (|rhs_i| a! (self . a, lhs $opsym rhs_i))
			}
		}

		impl <'a, 'b, A, T, const N: usize> $Op <&'a T, &'b [T; N]>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + $Op <&'a T, &'b T>
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: &'a T, rhs: &'b [T; N]) -> Self::Output
			{
				rhs . each_ref () . map (|rhs_i| a! (self . a, lhs $opsym rhs_i))
			}
		}

		// Scalar forward

		forward_traits!
		(
			for ParallelAlgebra . a
			impl $Op <T, T>
				+ for <'a> $Op <T, &'a T>
				+ for <'a> $Op <&'a T, T>
				+ for <'a, 'b> $Op <&'a T, &'b T>
		);
	}
}

impl_bin_op_trait! (Add, add, +);
impl_bin_op_trait! (Sub, sub, -);
impl_bin_op_trait! (Mul, mul, *);
impl_bin_op_trait! (Div, div, /);

macro_rules! impl_op_assign_trait
{
	($OpAssign: ident, $op_assign: ident, $opassignsym: tt) =>
	{
		impl <A, T, const N: usize> $OpAssign <[T; N], [T; N]>
		for ParallelAlgebra <A, T>
		where A: Clone + $OpAssign <T, T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: [T; N])
			{
				for (lhs_i, rhs_i) in lhs . iter_mut () . zip (rhs . into_iter ())
				{
					a! (self . a, *lhs_i $opassignsym rhs_i);
				}
			}
		}

		impl <'a, A, T, const N: usize> $OpAssign <[T; N], &'a [T; N]>
		for ParallelAlgebra <A, T>
		where A: Clone + $OpAssign <T, &'a T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: &'a [T; N])
			{
				for (lhs_i, rhs_i) in lhs . iter_mut () . zip (rhs . iter ())
				{
					a! (self . a, *lhs_i $opassignsym rhs_i);
				}
			}
		}

		// Scalar on RHS

		impl <A, T, const N: usize> $OpAssign <[T; N], T>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + for <'a> $OpAssign <T, &'a T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: T)
			{
				for lhs_i in lhs . iter_mut ()
				{
					a! (self . a, *lhs_i $opassignsym &rhs)
				}
			}
		}

		impl <'a, A, T, const N: usize> $OpAssign <[T; N], &'a T>
		for ParallelAlgebra <A, T>
		where T: Clone, A: Clone + $OpAssign <T, &'a T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: &'a T)
			{
				for lhs_i in lhs . iter_mut ()
				{
					a! (self . a, *lhs_i $opassignsym rhs)
				}
			}
		}

		// Scalar forward

		forward_traits!
		(
			for ParallelAlgebra . a
			impl $OpAssign <T, T> + for <'a> $OpAssign <T, &'a T>
		);
	}
}

impl_op_assign_trait! (AddAssign, add_assign, +=);
impl_op_assign_trait! (SubAssign, sub_assign, -=);
impl_op_assign_trait! (MulAssign, mul_assign, *=);
impl_op_assign_trait! (DivAssign, div_assign, /=);

macro_rules! impl_bin_method_trait
{
	($Op: ident, $op: ident) =>
	{
		impl <A, T, const N: usize, X> $Op <[T; N], X>
		for ParallelAlgebra <A, T>
		where
			A: Clone + $Op <T, X>,
			X: Clone
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: [T; N], rhs: X) -> Self::Output
			{
				lhs . map (|lhs_i| a! (self . a, lhs_i . $op (rhs . clone ())))
			}
		}

		impl <'a, A, T, const N: usize, X> $Op <&'a [T; N], X>
		for ParallelAlgebra <A, T>
		where
			A: Clone + $Op <&'a T, X>,
			X: Clone
		{
			type Output = [A::Output; N];

			fn $op (self, lhs: &'a [T; N], rhs: X) -> Self::Output
			{
				lhs
					. each_ref ()
					. map (|lhs_i| a! (self . a, lhs_i . $op (rhs . clone ())))
			}
		}

		// Scalar forward

		forward_traits!
		(
			for ParallelAlgebra . a
			impl for <X> $Op <T, X> + for <'a, X> $Op <&'a T, X>
		);
	}
}

impl_bin_method_trait! (Log, log);
impl_bin_method_trait! (Pow, pow);

macro_rules! impl_value_trait
{
	($Value: ident, $value: ident, $is_value: ident, $init: expr, $reduction: tt) =>
	{
		impl <A, T, const N: usize> $Value <[T; N]> for ParallelAlgebra <A, T>
		where A: Clone + $Value <T>
		{
			fn $value (self) -> [T; N]
			{
				std::array::from_fn (|_| a! (self . a, T::$value ()))
			}

			fn $is_value (self, x: &[T; N]) -> bool
			{
				x
					. iter ()
					. map (|x_i| a! (self . a, x_i . $is_value ()))
					. fold ($init, |v, item| v $reduction item)
			}
		}

		forward_traits! (for ParallelAlgebra . a impl $Value <T>);
	}
}

impl_value_trait! (Zero, zero, is_zero, true, &&);
impl_value_trait! (One, one, is_one, true, &&);
impl_value_trait! (E, e, is_e, true, &&);
impl_value_trait! (Pi, pi, is_pi, true, &&);
impl_value_trait! (Inf, inf, is_inf, false, ||);
impl_value_trait! (NaN, nan, is_nan, false, ||);

pub struct ParallelAccumulatorAlgebra <A, T>
{
	a: A,
	_t: PhantomData <T>
}

impl <A, T> ParallelAccumulatorAlgebra <A, T>
{
	pub fn new (a: A) -> Self
	{
		Self {a, _t: PhantomData::<T>::default ()}
	}

	pub fn t_accumulator (self) -> <A as Accumulatable <T>>::AccumulatorAlgebra
	where A: Accumulatable <T>
	{
		self . a . accumulator ()
	}
}

impl <A, T> Copy for ParallelAccumulatorAlgebra <A, T>
where A: Copy
{
}

impl <A, T> Clone for ParallelAccumulatorAlgebra <A, T>
where A: Clone
{
	fn clone (&self) -> Self
	{
		Self::new (self . a . clone ())
	}
}

impl <A, T, const N: usize> Accumulatable <[T; N]> for ParallelAlgebra <A, T>
where
	A: Clone + Accumulatable <T>,
	A::AccumulatorAlgebra: Clone + Zero <A::Accumulator>
{
	type AccumulatorAlgebra = ParallelAccumulatorAlgebra <A, T>;
	type Accumulator = [A::Accumulator; N];

	fn accumulator (self) -> Self::AccumulatorAlgebra
	{
		ParallelAccumulatorAlgebra::new (self . a)
	}
}

impl <A, T, const N: usize> Convert <[A::Accumulator; N], [T; N]>
for ParallelAlgebra <A, T>
where A: Clone + Accumulatable <T>
{
	fn convert (self, x: [A::Accumulator; N]) -> [T; N]
	{
		x . map (|x_i| a! (self . a, x_i . convert ()))
	}
}

impl <A, T, const N: usize> Zero <[A::Accumulator; N]>
for ParallelAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Clone + Zero <A::Accumulator>
{
	fn zero (self) -> [A::Accumulator; N]
	{
		let t_accumulator = self . t_accumulator ();
		std::array::from_fn (|_| a! (t_accumulator, A::Accumulator::zero ()))
	}

	fn is_zero (self, x: &[A::Accumulator; N]) -> bool
	{
		let t_accumulator = self . t_accumulator ();

		x
			. iter ()
			. map (|x_i| a! (t_accumulator, x_i . is_zero ()))
			. fold (true, |v, item| v && item)
	}
}

impl <A, T, const N: usize> AddAssign <[A::Accumulator; N], [T; N]>
for ParallelAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Clone + AddAssign <A::Accumulator, T>
{
	fn add_assign (self, lhs: &mut [A::Accumulator; N], rhs: [T; N])
	{
		let t_accumulator = self . t_accumulator ();

		for (lhs_i, rhs_i) in lhs . iter_mut () . zip (rhs . into_iter ())
		{
			a! (t_accumulator, *lhs_i += rhs_i);
		}
	}
}

impl <'a, A, T, const N: usize> AddAssign <[A::Accumulator; N], &'a [T; N]>
for ParallelAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Clone + AddAssign <A::Accumulator, &'a T>
{
	fn add_assign (self, lhs: &mut [A::Accumulator; N], rhs: &'a [T; N])
	{
		let t_accumulator = self . t_accumulator ();

		for (lhs_i, rhs_i) in lhs . iter_mut () . zip (rhs . iter ())
		{
			a! (t_accumulator, *lhs_i += rhs_i);
		}
	}
}

#[cfg (any (test, feature = "proptest"))]
impl <A, T, const N: usize> UnitRange <[T; N]> for ParallelAlgebra <A, T>
where
	A: Clone + UnitRange <T>,
	T: Debug,
	Self: Clone + Mul <T, [T; N], Output = [T; N]> + One <[T; N]>
{
	fn unit_range (self) -> impl Strategy <Value = [T; N]>
	{
		self
			. a
			. clone ()
			. unit_range ()
			. prop_map (move |x| a! (self, x * <[T; N]>::one ()))
	}
}
