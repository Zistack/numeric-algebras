mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_sub_assign_traits;
use numeric_algebras::traits::SubAssign;

def_sub_assign_traits!
(
	for (AggregateType, AggregateType) in AggregateTypeAlgebra
);

def_sub_assign_traits!
(
	for (AggregateType, ChoiceType) in AggregateTypeAlgebra
);

#[test]
fn sub_assigns ()
{
	let mut x = AggregateType {x: 1.0, y: -1.0};
	a! (AggregateTypeAlgebra, x -= AggregateType {x: 2.0, y: 2.0});

	assert_eq! (x, AggregateType {x: -1.0, y: -3.0});

	a! (AggregateTypeAlgebra, x -= ChoiceType::X (2.0));

	assert_eq! (x, AggregateType {x: -3.0, y: -3.0});
}
