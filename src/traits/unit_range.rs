use std::fmt::Debug;

use forward_traits::forwardable;
use proptest::strategy::Strategy;

#[forwardable]
pub trait UnitRange <X>
where X: Debug
{
	fn unit_range (self) -> impl Strategy <Value = X> + Debug;
}
