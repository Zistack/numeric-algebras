mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_log_assign_traits;
use numeric_algebras::traits::LogAssign;

def_log_assign_traits!
(
	for (AggregateType, AggregateType) in AggregateTypeAlgebra
);

def_log_assign_traits!
(
	for (AggregateType, ChoiceType) in AggregateTypeAlgebra
);

fn main ()
{
	let mut x = AggregateType {x: 3.0, y: 4.0};
	a!
	(
		AggregateTypeAlgebra,
		(&mut x) . log_assign (AggregateType {x: 2.0, y: 2.0})
	);

	assert_eq!
	(
		x,
		AggregateType {x: (3.0f32) . log (2.0), y: (4.0f32) . log (2.0)}
	);

	a! (AggregateTypeAlgebra, (&mut x) . log_assign (ChoiceType::Y (2.0)));

	assert_eq!
	(
		x,
		AggregateType
		{
			x: (3.0f32) . log (2.0),
			y: (4.0f32) . log (2.0) . log (2.0)
		}
	);
}
