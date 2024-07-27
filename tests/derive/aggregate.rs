#![allow (unused_imports)]

use numeric_algebras::derive::algebra_mapping;

#[derive (Copy, Clone, PartialEq, Debug)]
#[macrospace::item]
pub struct AggregateType
{
	pub x: f32,
	pub y: f32
}

#[derive (Copy, Clone, PartialEq, Debug)]
#[macrospace::item]
pub enum ChoiceType
{
	X (f32),
	Y (f32)
}

#[derive (Copy, Clone)]
#[macrospace::item]
#[algebra_mapping (
	struct AggregateType
	{
		x => (|_a| F32Algebra): F32Algebra,
		y => (|_a| F32Algebra): F32Algebra
	}
)]
#[algebra_mapping (
	enum ChoiceType
	{
		X => (|_a| F32Algebra): F32Algebra,
		Y => (|_a| F32Algebra): F32Algebra
	}
)]
pub struct AggregateTypeAlgebra;
