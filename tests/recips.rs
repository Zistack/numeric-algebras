mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_recip_traits;
use numeric_algebras::traits::Recip;

def_recip_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

#[test]
fn recips ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 2.0, y: -2.0} . recip ()),
		AggregateType {x: 0.5, y: -0.5}
	);
}
