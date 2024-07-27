mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_pow_traits;
use numeric_algebras::traits::Pow;

def_pow_traits!
(
	for (AggregateType, AggregateType) -> AggregateType in AggregateTypeAlgebra
);

def_pow_traits!
(
	for (AggregateType, ChoiceType) -> AggregateType in AggregateTypeAlgebra
);

fn main ()
{
	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 1.0, y: -1.0} . pow (AggregateType {x: 2.0, y: 2.0})
		),
		AggregateType {x: 1.0, y: 1.0}
	);

	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 1.0, y: -1.0} . pow (ChoiceType::Y (2.0))
		),
		AggregateType {x: 1.0, y: 1.0}
	);
}
