use super::Acc;

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
where T: Clone + Acc <X, Y>
{
	fn sum <I> (self, iter: I) -> X
	where I: Iterator <Item = Y>
	{
		let mut accumulator = a! (self, T::Accumulator::zero_accumulator ());

		for y in iter { a! (self, (&mut accumulator) . accumulate (y)); }

		self . convert (accumulator)
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
