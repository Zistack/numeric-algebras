#[macrospace::item]
pub trait Convert <X, Y>
{
	fn convert (self, x: X) -> Y;
}

#[macrospace::item]
pub trait TryConvert <X, Y>
{
	type Error;

	fn try_convert (self, x: X) -> Result <Y, Self::Error>;
}

#[macrospace::item]
pub trait ApproxConvert <X, Y>
{
	fn approx_convert (self, x: X) -> Y;
}
