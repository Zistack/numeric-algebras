mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_log_traits;
use numeric_algebras::traits::Log;

def_log_traits!
(
	for (AggregateType, AggregateType) -> AggregateType in AggregateTypeAlgebra
);

def_log_traits!
(
	for (AggregateType, ChoiceType) -> AggregateType in AggregateTypeAlgebra
);

#[test]
fn logs ()
{
	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 3.0, y: 4.0} . log (AggregateType {x: 2.0, y: 2.0})
		),
		AggregateType {x: (3.0f32) . log (2.0), y: (4.0f32) . log (2.0)}
	);

	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 3.0, y: 4.0} . log (ChoiceType::Y (2.0))
		),
		AggregateType {x: 3.0, y: (4.0f32) . log (2.0)}
	);
}
