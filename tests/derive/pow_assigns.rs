mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_pow_assign_traits;
use numeric_algebras::traits::PowAssign;

def_pow_assign_traits!
(
	for (AggregateType, AggregateType) in AggregateTypeAlgebra
);

def_pow_assign_traits!
(
	for (AggregateType, ChoiceType) in AggregateTypeAlgebra
);

fn main ()
{
	let mut x = AggregateType {x: 2.0, y: -2.0};
	a! (AggregateTypeAlgebra, (&mut x) . pow_assign (AggregateType {x: 2.0, y: 2.0}));

	assert_eq! (x, AggregateType {x: 4.0, y: 4.0});

	a! (AggregateTypeAlgebra, (&mut x) . pow_assign (ChoiceType::Y (0.5)));

	assert_eq! (x, AggregateType {x: 4.0, y: 2.0});
}
