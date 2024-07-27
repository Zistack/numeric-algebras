mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_abs_traits;
use numeric_algebras::traits::Abs;

def_abs_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

fn main ()
{
	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 1.0, y: -1.0} . abs ()),
		AggregateType {x: 1.0, y: 1.0}
	);
}
