use std::marker::PhantomData;

use crate::partial_init_array::*;
use crate::traits::*;

pub struct ElementwiseAlgebra <A, T>
{
	a: A,
	_t: PhantomData <T>
}

impl <A, T> ElementwiseAlgebra <A, T>
{
	pub fn new (a: A) -> Self
	{
		Self {a,  _t: PhantomData::default ()}
	}
}

macro_rules! impl_unary
{
	($Op: ident, $op: ident) =>
	{
		impl <A, T, const N: usize> $Op <[T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, x: [T; N]) -> Self::Output
			{
				x . map (|e| self . a . clone () . $op (e))
			}
		}

		impl <'a, A, T, const N: usize> $Op <&'a [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <&'a T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, x: &'a [T; N]) -> Self::Output
			{
				x . each_ref () . map (|e| self . a . clone () . $op (e))
			}
		}
	}
}

impl_unary! (Neg, neg);

impl_unary! (Abs, abs);
impl_unary! (Recip, recip);
impl_unary! (Sqrt, sqrt);

impl_unary! (Ln, ln);
impl_unary! (Exp, exp);

impl_unary! (Sin, sin);
impl_unary! (Cos, cos);
impl_unary! (Tan, tan);

impl <A, T, const N: usize> SinCos <[T; N]> for ElementwiseAlgebra <A, T>
where A: Clone + SinCos <T, Output = T>
{
	type Output = [T; N];

	fn sin_cos (self, x: [T; N]) -> (Self::Output, Self::Output)
	{
		let mut sins = PartialInitArray::new ();
		let mut coss = PartialInitArray::new ();

		for x_i in x
		{
			let (sin, cos) = self . a . clone () . sin_cos (x_i);

			unsafe
			{
				sins . push_unchecked (sin);
				coss . push_unchecked (cos);
			}
		}

		let sins = unsafe { sins . into_init_array () };
		let coss = unsafe { coss . into_init_array () };

		(sins, coss)
	}
}

impl <'a, A, T, const N: usize> SinCos <&'a [T; N]>
for ElementwiseAlgebra <A, T>
where A: Clone + SinCos <&'a T, Output = T>
{
	type Output = [T; N];

	fn sin_cos (self, x: &'a [T; N]) -> (Self::Output, Self::Output)
	{
		let mut sins = PartialInitArray::new ();
		let mut coss = PartialInitArray::new ();

		for x_i in x
		{
			let (sin, cos) = self . a . clone () . sin_cos (x_i);

			unsafe
			{
				sins . push_unchecked (sin);
				coss . push_unchecked (cos);
			}
		}

		let sins = unsafe { sins . into_init_array () };
		let coss = unsafe { coss . into_init_array () };

		(sins, coss)
	}
}

macro_rules! impl_bin
{
	($Op: ident, $op: ident) =>
	{
		impl <A, T, const N: usize> $Op <[T; N], [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <T, T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: [T; N], rhs: [T; N]) -> Self::Output
			{
				let mut output = PartialInitArray::new ();

				for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
				{
					let output_i = self . a . clone () . $op (lhs_i, rhs_i);

					unsafe { output . push_unchecked (output_i); }
				}

				unsafe { output . into_init_array () }
			}
		}

		impl <'a, A, T, const N: usize> $Op <[T; N], &'a [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <T, &'a T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: [T; N], rhs: &'a [T; N]) -> Self::Output
			{
				let mut output = PartialInitArray::new ();

				for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
				{
					let output_i = self . a . clone () . $op (lhs_i, rhs_i);

					unsafe { output . push_unchecked (output_i); }
				}

				unsafe { output . into_init_array () }
			}
		}

		impl <'a, A, T, const N: usize> $Op <&'a [T; N], [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <&'a T, T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: &'a [T; N], rhs: [T; N]) -> Self::Output
			{
				let mut output = PartialInitArray::new ();

				for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
				{
					let output_i = self . a . clone () . $op (lhs_i, rhs_i);

					unsafe { output . push_unchecked (output_i); }
				}

				unsafe { output . into_init_array () }
			}
		}

		impl <'a, 'b, A, T, const N: usize> $Op <&'a [T; N], &'b [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <&'a T, &'b T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: &'a [T; N], rhs: &'b [T; N]) -> Self::Output
			{
				let mut output = PartialInitArray::new ();

				for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
				{
					let output_i = self . a . clone () . $op (lhs_i, rhs_i);

					unsafe { output . push_unchecked (output_i); }
				}

				unsafe { output . into_init_array () }
			}
		}
	}
}

impl_bin! (Add, add);
impl_bin! (Sub, sub);
impl_bin! (Mul, mul);
impl_bin! (Div, div);

impl_bin! (Log, log);
impl_bin! (Pow, pow);

macro_rules! impl_bin_assign
{
	($OpAssign: ident, $op_assign: ident) =>
	{
		impl <A, T, const N: usize> $OpAssign <[T; N], [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $OpAssign <T, T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: [T; N])
			{
				for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
				{
					self . a . clone () . $op_assign (lhs_i, rhs_i);
				}
			}
		}

		impl <'a, A, T, const N: usize> $OpAssign <[T; N], &'a [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $OpAssign <T, &'a T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: &'a [T; N])
			{
				for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
				{
					self . a . clone () . $op_assign (lhs_i, rhs_i);
				}
			}
		}
	}
}

impl_bin_assign! (AddAssign, add_assign);
impl_bin_assign! (SubAssign, sub_assign);
impl_bin_assign! (MulAssign, mul_assign);
impl_bin_assign! (DivAssign, div_assign);

impl_bin_assign! (PowAssign, pow_assign);
impl_bin_assign! (LogAssign, log_assign);

impl <A, T, const N: usize> AdditionIsCommutative <[T; N], [T; N]>
for ElementwiseAlgebra <A, T>
where A: AdditionIsCommutative <T, T>
{
}

impl <A, T, const N: usize> MultiplicationIsCommutative <[T; N], [T; N]>
for ElementwiseAlgebra <A, T>
where A: MultiplicationIsCommutative <T, T>
{
}

macro_rules! impl_value
{
	($Value: ident, $value: ident, $is_value: ident) =>
	{
		impl <A, T, const N: usize> $Value <[T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Value <T>
		{
			fn $value (self) -> [T; N]
			{
				std::array::from_fn (|_| self . a . clone () . $value ())
			}

			fn $is_value (self, x: &[T; N]) -> bool
			{
				for x_i in x
				{
					if ! self . a . clone () . $is_value (x_i) { return false; }
				}

				true
			}
		}
	}
}

impl_value! (Zero, zero, is_zero);
impl_value! (One, one, is_one);
impl_value! (E, e, is_e);
impl_value! (Pi, pi, is_pi);
impl_value! (Inf, inf, is_inf);
impl_value! (NaN, nan, is_nan);

pub type ElementwiseAccumulator <A, T, const N: usize> =
	[<A as Accumulatable <T>>::Accumulator; N];

pub struct ElementwiseAccumulatorAlgebra <A, T>
where A: Accumulatable <T>
{
	a: <A as Accumulatable <T>>::AccumulatorAlgebra,
	_t: PhantomData <T>
}

impl <A, T> Copy for ElementwiseAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Copy
{
}

impl <A, T> Clone for ElementwiseAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Clone
{
	fn clone (&self) -> Self
	{
		Self::new (self . a . clone ())
	}
}

impl <A, T> ElementwiseAccumulatorAlgebra <A, T>
where A: Accumulatable <T>
{
	fn new (a: A::AccumulatorAlgebra) -> Self
	{
		Self {a, _t: PhantomData::default ()}
	}
}

impl <A, T, const N: usize> Accumulatable <[T; N]>
for ElementwiseAlgebra <A, T>
where
	A: Clone + Accumulatable <T>,
	A::AccumulatorAlgebra: Clone
{
	type AccumulatorAlgebra = ElementwiseAccumulatorAlgebra <A, T>;
	type Accumulator = ElementwiseAccumulator <A, T, N>;

	fn accumulator (self) -> Self::AccumulatorAlgebra
	{
		Self::AccumulatorAlgebra::new (self . a . accumulator ())
	}
}

impl <A, T, const N: usize> Zero <ElementwiseAccumulator <A, T, N>>
for ElementwiseAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Clone + Zero <A::Accumulator>
{
	fn zero (self) -> ElementwiseAccumulator <A, T, N>
	{
		std::array::from_fn (|_| self . a . clone () . zero ())
	}

	fn is_zero (self, x: &ElementwiseAccumulator <A, T, N>) -> bool
	{
		for x_i in x
		{
			if ! self . a . clone () . is_zero (x_i) { return false; }
		}

		true
	}
}

impl <A, T, const N: usize>
AddAssign <ElementwiseAccumulator <A, T, N>, [T; N]>
for ElementwiseAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Clone + AddAssign <A::Accumulator, T>
{
	fn add_assign
	(
		self,
		lhs: &mut ElementwiseAccumulator <A, T, N>,
		rhs: [T; N]
	)
	{
		for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
		{
			self . a . clone () . add_assign (lhs_i, rhs_i);
		}
	}
}

impl <'a, A, T, const N: usize>
AddAssign <ElementwiseAccumulator <A, T, N>, &'a [T; N]>
for ElementwiseAccumulatorAlgebra <A, T>
where
	A: Accumulatable <T>,
	A::AccumulatorAlgebra: Clone + AddAssign <A::Accumulator, &'a T>
{
	fn add_assign
	(
		self,
		lhs: &mut ElementwiseAccumulator <A, T, N>,
		rhs: &'a [T; N]
	)
	{
		for (lhs_i, rhs_i) in std::iter::zip (lhs, rhs)
		{
			self . a . clone () . add_assign (lhs_i, rhs_i);
		}
	}
}

impl <A, X, T, const N: usize> Convert <[X; N], [T; N]>
for ElementwiseAlgebra <A, T>
where A: Clone + Convert <X, T>
{
	fn convert (self, x: [X; N]) -> [T; N]
	{
		x . map (|x_i| self . a . clone () . convert (x_i))
	}
}

impl <A, X, T, const N: usize> ApproxConvert <[X; N], [T; N]>
for ElementwiseAlgebra <A, T>
where A: Clone + ApproxConvert <X, T>
{
	fn approx_convert (self, x: [X; N]) -> [T; N]
	{
		x . map (|x_i| self . a . clone () . approx_convert (x_i))
	}
}

macro_rules! impl_scalar_bin
{
	($Op: ident, $op: ident) =>
	{
		impl <A, T, const N: usize> $Op <[T; N], T>
		for ElementwiseAlgebra <A, T>
		where A: Clone + for <'a> $Op <T, &'a T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: [T; N], rhs: T) -> Self::Output
			{
				lhs . map (|lhs_i| self . a . clone () . $op (lhs_i, &rhs))
			}
		}

		impl <'a, A, T, const N: usize> $Op <[T; N], &'a T>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <T, &'a T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: [T; N], rhs: &'a T) -> Self::Output
			{
				lhs . map (|lhs_i| self . a . clone () . $op (lhs_i, rhs))
			}
		}

		impl <'a, A, T, const N: usize> $Op <&'a [T; N], T>
		for ElementwiseAlgebra <A, T>
		where A: Clone + for <'b> $Op <&'a T, &'b T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: &'a [T; N], rhs: T) -> Self::Output
			{
				lhs
					. each_ref ()
					. map (|lhs_i| self . a . clone () . $op (lhs_i, &rhs))
			}
		}

		impl <'a, 'b, A, T, const N: usize> $Op <&'a [T; N], &'b T>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <&'a T, &'b T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: &'a [T; N], rhs: &'b T) -> Self::Output
			{
				lhs
					. each_ref ()
					. map (|lhs_i| self . a . clone () . $op (lhs_i, rhs))
			}
		}

		impl <A, T, const N: usize> $Op <T, [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + for <'a> $Op <&'a T, T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: T, rhs: [T; N]) -> Self::Output
			{
				rhs . map (|rhs_i| self . a . clone () . $op (&lhs, rhs_i))
			}
		}

		impl <'a, A, T, const N: usize> $Op <T, &'a [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + for <'b> $Op <&'b T, &'a T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: T, rhs: &'a [T; N]) -> Self::Output
			{
				rhs
					. each_ref ()
					. map (|rhs_i| self . a . clone () . $op (&lhs, rhs_i))
			}
		}

		impl <'a, A, T, const N: usize> $Op <&'a T, [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <&'a T, T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: &'a T, rhs: [T; N]) -> Self::Output
			{
				rhs . map (|rhs_i| self . a . clone () . $op (lhs, rhs_i))
			}
		}

		impl <'a, 'b, A, T, const N: usize> $Op <&'a T, &'b [T; N]>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $Op <&'a T, &'b T, Output = T>
		{
			type Output = [T; N];

			fn $op (self, lhs: &'a T, rhs: &'b [T; N]) -> Self::Output
			{
				rhs
					. each_ref ()
					. map (|rhs_i| self . a . clone () . $op (lhs, rhs_i))
			}
		}
	}
}

impl_scalar_bin! (Mul, mul);
impl_scalar_bin! (Div, div);

impl_scalar_bin! (Log, log);
impl_scalar_bin! (Pow, pow);

macro_rules! impl_scalar_assign
{
	($OpAssign: ident, $op_assign: ident) =>
	{
		impl <A, T, const N: usize> $OpAssign <[T; N], T>
		for ElementwiseAlgebra <A, T>
		where A: Clone + for <'a> $OpAssign <T, &'a T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: T)
			{
				for lhs_i in lhs
				{
					self . a . clone () . $op_assign (lhs_i, &rhs);
				}
			}
		}

		impl <'a, A, T, const N: usize> $OpAssign <[T; N], &'a T>
		for ElementwiseAlgebra <A, T>
		where A: Clone + $OpAssign <T, &'a T>
		{
			fn $op_assign (self, lhs: &mut [T; N], rhs: &'a T)
			{
				for lhs_i in lhs
				{
					self . a . clone () . $op_assign (lhs_i, rhs);
				}
			}
		}
	}
}

impl_scalar_assign! (MulAssign, mul_assign);
impl_scalar_assign! (DivAssign, div_assign);

impl_scalar_assign! (LogAssign, log_assign);
impl_scalar_assign! (PowAssign, pow_assign);
