mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::{def_scalar_arithmetic, def_p_norm_traits};
use numeric_algebras::traits::PNorm;

def_scalar_arithmetic!
(
	for (AggregateType, f32)
	in AggregateTypeAlgebra
	with AggregateTypeAlgebraToF32Algebra
);

def_p_norm_traits!
(
	for (AggregateType, f32) -> f32 in AggregateTypeAlgebra
);

#[test]
fn elements_sums ()
{
	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 1.0, y: 2.0} . p_norm (2.0)
		),
		5.0_f32 . powf (0.5)
	);
}
