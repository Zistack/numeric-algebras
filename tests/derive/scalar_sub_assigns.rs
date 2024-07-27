mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_scalar_sub_assign_traits;
use numeric_algebras::traits::SubAssign;

def_scalar_sub_assign_traits!
(
	for (AggregateType, f32) in AggregateTypeAlgebra
);

fn main ()
{
	let mut x = AggregateType {x: 1.0, y: -1.0};

	a! (AggregateTypeAlgebra, x -= 2.0);

	assert_eq! (x, AggregateType {x: -1.0, y: -3.0});
}
