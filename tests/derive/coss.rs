mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_cos_traits;
use numeric_algebras::traits::Cos;

def_cos_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

fn main ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 1.0, y: -1.0} . cos ()),
		AggregateType {x: (1.0f32) . cos (), y: (-1.0f32) . cos ()}
	);
}
