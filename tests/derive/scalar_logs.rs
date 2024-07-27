mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_scalar_log_traits;
use numeric_algebras::traits::Log;

def_scalar_log_traits!
(
	for (AggregateType, f32) -> AggregateType in AggregateTypeAlgebra
);

fn main ()
{
	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 2.0, y: 3.0} . log (2.0)
		),
		AggregateType {x: (2.0f32) . log (2.0), y: (3.0f32) . log (2.0)}
	);
}
