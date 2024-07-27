mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_exp_traits;
use numeric_algebras::traits::Exp;

def_exp_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

fn main ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 2.0, y: 2.0} . exp ()),
		AggregateType {x: (2.0f32) . exp (), y: (2.0f32) . exp ()}
	);
}
