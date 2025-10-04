mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_mul_assign_traits;
use numeric_algebras::traits::MulAssign;

def_mul_assign_traits!
(
	for (AggregateType, AggregateType) in AggregateTypeAlgebra
);

def_mul_assign_traits!
(
	for (AggregateType, ChoiceType) in AggregateTypeAlgebra
);

#[test]
fn mul_assigns ()
{
	let mut x = AggregateType {x: 1.0, y: -1.0};
	a! (AggregateTypeAlgebra, x *= AggregateType {x: 2.0, y: 2.0});

	assert_eq! (x, AggregateType {x: 2.0, y: -2.0});

	a! (AggregateTypeAlgebra, x *= ChoiceType::Y (2.0));

	assert_eq! (x, AggregateType {x: 2.0, y: -4.0});
}
