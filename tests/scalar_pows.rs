mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_scalar_pow_traits;
use numeric_algebras::traits::Pow;

def_scalar_pow_traits!
(
	for (AggregateType, f32) -> AggregateType in AggregateTypeAlgebra
);

#[test]
fn scalar_pows ()
{
	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 2.0, y: -1.0} . pow (2.0)
		),
		AggregateType {x: 4.0, y: 1.0}
	);
}
