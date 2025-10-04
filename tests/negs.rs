mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_neg_traits;
use numeric_algebras::traits::Neg;

def_neg_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

#[test]
fn negs ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, - AggregateType {x: 1.0, y: -1.0}),
		AggregateType {x: -1.0, y: 1.0}
	);
}
