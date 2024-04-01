use forward_traits::forwardable;

use super::{Convert, AddAssign, AddAssigns, Zero};

#[forwardable]
pub trait Accumulatable <X>: Convert <Self::Accumulator, X>
{
	type AccumulatorAlgebra: Zero <Self::Accumulator>;
	type Accumulator;

	fn accumulator (self) -> Self::AccumulatorAlgebra;
}

pub trait Acc <X, Y>: Accumulatable <X>
where Self::AccumulatorAlgebra: AddAssign <Self::Accumulator, Y>
{
}

impl <X, Y, T> Acc <X, Y> for T
where
	T: Accumulatable <X>,
	T::AccumulatorAlgebra: Zero <T::Accumulator> + AddAssign <T::Accumulator, Y>
{
}

pub trait Accs <X, Y>: Acc <X, Y> + for <'a> Acc <X, &'a Y>
where Self::AccumulatorAlgebra: AddAssigns <Self::Accumulator, Y>
{
}

impl <X, Y, T> Accs <X, Y> for T
where
	T: Acc <X, Y> + for <'a> Acc <X, &'a Y>,
	T::AccumulatorAlgebra: Zero <T::Accumulator> + AddAssigns <T::Accumulator, Y>
{
}
