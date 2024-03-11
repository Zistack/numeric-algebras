use forward_traits::forwardable;

#[forwardable]
pub trait Zero <X>
{
	fn zero (self) -> X;

	fn is_zero (self, x: &X) -> bool;
}

#[forwardable]
pub trait One <X>
{
	fn one (self) -> X;

	fn is_one (self, x: &X) -> bool;
}

#[forwardable]
pub trait E <X>
{
	fn e (self) -> X;

	fn is_e (self, x: &X) -> bool;
}

#[forwardable]
pub trait Pi <X>
{
	fn pi (self) -> X;

	fn is_pi (self, x: &X) -> bool;
}

#[forwardable]
pub trait Inf <X>
{
	fn inf (self) -> X;

	fn is_inf (self, x: &X) -> bool;
}

#[forwardable]
pub trait NaN <X>
{
	fn nan (self) -> X;

	fn is_nan (self, x: &X) -> bool;
}
