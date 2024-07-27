mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_sqrt_traits;
use numeric_algebras::traits::Sqrt;

def_sqrt_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

fn main ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 4.0, y: 9.0} . sqrt ()),
		AggregateType {x: 2.0, y: 3.0}
	);
}
