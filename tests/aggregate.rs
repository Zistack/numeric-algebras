#![allow (unused_imports)]

use macrospace_autotransform::define_autotransform;
use numeric_algebras::derive::algebra_mapping;

#[allow (dead_code)]
#[derive (Copy, Clone, PartialEq, Debug)]
#[macrospace::item]
pub struct AggregateType
{
	pub x: f32,
	pub y: f32
}

#[allow (dead_code)]
#[derive (Copy, Clone, PartialEq, Debug)]
#[macrospace::item]
pub enum ChoiceType
{
	X (f32),
	Y (f32)
}

#[allow (dead_code)]
#[derive (Copy, Clone)]
#[macrospace::item]
#[algebra_mapping (
	(|_a| F32Algebra): F32Algebra,
	(|_a| F32Algebra): F32Algebra
)]
pub struct AggregateTypeAlgebra;

define_autotransform!
{
	pub autotransform AggregateTypeAlgebraToF32Algebra
	[AggregateTypeAlgebra] -> [F32Algebra]
	{
		F32Algebra
	}
}

#[allow (dead_code)]
#[derive (Copy, Clone, PartialEq, Debug)]
#[macrospace::item]
pub struct AggregateAccumulatorType
{
	pub x: f32,
	pub y: f32
}

#[allow (dead_code)]
#[derive (Copy, Clone)]
#[macrospace::item]
#[algebra_mapping (
	(|_a| F32Algebra): F32Algebra,
	(|_a| F32Algebra): F32Algebra
)]
pub struct AggregateAccumulatorTypeAlgebra;
