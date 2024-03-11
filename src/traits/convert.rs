#[forward_traits::forwardable]
pub trait Convert <X, Y>
{
	fn convert (self, x: X) -> Y;
}
