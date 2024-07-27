mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_sin_traits;
use numeric_algebras::traits::Sin;

def_sin_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

fn main ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 1.0, y: -1.0} . sin ()),
		AggregateType {x: (1.0f32) . sin (), y: (-1.0f32) . sin ()}
	);
}
