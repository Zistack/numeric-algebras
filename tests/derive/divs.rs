mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_div_traits;
use numeric_algebras::traits::Div;

def_div_traits!
(
	for (AggregateType, AggregateType) -> AggregateType in AggregateTypeAlgebra
);

def_div_traits!
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
			AggregateType {x: 1.0, y: -1.0} / AggregateType {x: 2.0, y: 2.0}
		),
		AggregateType {x: 0.5, y: -0.5}
	);

	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 1.0, y: -1.0} / ChoiceType::Y (2.0)
		),
		AggregateType {x: 1.0, y: -0.5}
	);
}
