mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_zero_trait;
use numeric_algebras::traits::Zero;

def_zero_trait!
(
	for AggregateType in AggregateTypeAlgebra
);

fn main ()
{
	let ag_zero = a! (AggregateTypeAlgebra, AggregateType::zero ());

	assert_eq! (ag_zero, AggregateType {x: 0f32, y: 0f32});

	assert! (a! (AggregateTypeAlgebra, (&ag_zero) . is_zero ()));
}
