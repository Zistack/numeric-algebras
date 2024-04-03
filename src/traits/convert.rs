use forward_traits::forwardable;

#[forwardable]
pub trait Convert <X, Y>
{
	fn convert (self, x: X) -> Y;
}

#[forwardable]
pub trait TryConvert <X, Y>
{
	type Error;

	fn try_convert (self, x: X) -> Result <Y, Self::Error>;
}

#[forwardable]
pub trait ApproxConvert <X, Y>
{
	fn approx_convert (self, x: X) -> Y;
}
