use forward_traits::forwardable;

use super::Convert;

#[forwardable]
pub trait Accumulatable <T>: Convert <Self::Accumulator, T>
{
	type Accumulator;

	fn zero_accumulator (self) -> Self::Accumulator;
}

#[forwardable]
pub trait Acc <T, X>: Accumulatable <T>
{
	fn accumulate (self, acc: &mut Self::Accumulator, x: X);
}
