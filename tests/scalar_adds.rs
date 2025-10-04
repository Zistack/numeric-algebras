mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_scalar_add_traits;
use numeric_algebras::traits::Add;

def_scalar_add_traits!
(
	for (AggregateType, f32) -> AggregateType in AggregateTypeAlgebra
);

#[test]
fn scalar_adds ()
{
	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 1.0, y: -1.0} + 2.0
		),
		AggregateType {x: 3.0, y: 1.0}
	);
}
