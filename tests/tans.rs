mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_tan_traits;
use numeric_algebras::traits::Tan;

def_tan_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

#[test]
fn tans ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 1.0, y: -1.0} . tan ()),
		AggregateType {x: (1.0f32) . tan (), y: (-1.0f32) . tan ()}
	);
}
