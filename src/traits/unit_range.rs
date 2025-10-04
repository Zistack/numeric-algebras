use std::fmt::Debug;

use proptest::strategy::Strategy;

#[macrospace::item]
pub trait UnitRange <X>
where X: Debug
{
	fn unit_range (self) -> impl Strategy <Value = X> + Debug;
}
