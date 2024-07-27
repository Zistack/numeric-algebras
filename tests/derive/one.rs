mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_one_trait;
use numeric_algebras::traits::One;

def_one_trait!
(
	for AggregateType in AggregateTypeAlgebra
);

fn main ()
{
	let ag_one = a! (AggregateTypeAlgebra, AggregateType::one ());

	assert_eq! (ag_one, AggregateType {x: 1f32, y: 1f32});

	assert! (a! (AggregateTypeAlgebra, (&ag_one) . is_one ()));
}
