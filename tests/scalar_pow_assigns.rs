mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_scalar_pow_assign_traits;
use numeric_algebras::traits::PowAssign;

def_scalar_pow_assign_traits!
(
	for (AggregateType, f32) in AggregateTypeAlgebra
);

#[test]
fn scalar_pow_assigns ()
{
	let mut x = AggregateType {x: 2.0, y: -1.0};

	a! (AggregateTypeAlgebra, (&mut x) . pow_assign (2.0));

	assert_eq! (x, AggregateType {x: 4.0, y: 1.0});
}
