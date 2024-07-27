mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_ln_traits;
use numeric_algebras::traits::Ln;

def_ln_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

fn main ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 2.0, y: 3.0} . ln ()),
		AggregateType {x: (2.0f32) . ln (), y: (3.0f32) . ln ()}
	);
}
