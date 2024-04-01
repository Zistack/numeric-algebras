use super::{AddAssign, Acc, Zero};

use crate::a;

pub trait Sum <X, Y>
{
	fn sum <I> (self, iter: I) -> X
	where I: Iterator <Item = Y>;
}

pub trait Sums <X, Y>: Sum <X, Y> + for <'a> Sum <X, &'a Y>
{
}

impl <X, Y, T> Sums <X, Y> for T
where T: Sum <X, Y> + for <'a> Sum <X, &'a Y>
{
}

pub trait SumsMonoid <X>: Sums <X, X>
{
}

impl <X, T> SumsMonoid <X> for T
where T: Sums <X, X>
{
}

// So, this technically works, but is it actually a good idea?
impl <X, Y, T> Sum <X, Y> for T
where
	T: Clone + Acc <X, Y>,
	T::AccumulatorAlgebra: Clone
		+ AddAssign <T::Accumulator, Y>
		+ Zero <T::Accumulator>,
{
	fn sum <I> (self, iter: I) -> X
	where I: Iterator <Item = Y>
	{
		let accumulator = self . clone () . accumulator ();

		let mut acc = a! (accumulator, T::Accumulator::zero ());

		for y in iter { a! (accumulator, acc += y); }

		self . convert (acc)
	}
}

pub trait IteratorExt: Iterator
{
	fn sum <A, T> (self, algebra: A) -> T
	where A: Sum <T, Self::Item>;
}

impl <T> IteratorExt for T
where T: Iterator
{
	fn sum <A, X> (self, algebra: A) -> X
	where A: Sum <X, Self::Item>
	{
		algebra . sum (self)
	}
}
