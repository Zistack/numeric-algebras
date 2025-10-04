mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_scalar_log_assign_traits;
use numeric_algebras::traits::LogAssign;

def_scalar_log_assign_traits!
(
	for (AggregateType, f32) in AggregateTypeAlgebra
);

#[test]
fn scalar_log_assigns ()
{
	let mut x = AggregateType {x: 2.0, y: 3.0};

	a! (AggregateTypeAlgebra, (&mut x) . log_assign (2.0));

	assert_eq!
	(
		x,
		AggregateType {x: (2.0f32) . log (2.0), y: (3.0f32) . log (2.0)}
	);
}
