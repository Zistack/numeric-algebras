mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_inf_trait;
use numeric_algebras::traits::Inf;

def_inf_trait!
(
	for AggregateType in AggregateTypeAlgebra
);

#[test]
fn infs ()
{
	let ag_inf = a! (AggregateTypeAlgebra, AggregateType::inf ());

	assert_eq!
	(
		ag_inf,
		AggregateType {x: f32::INFINITY, y: f32::INFINITY}
	);

	assert! (a! (AggregateTypeAlgebra, (&ag_inf) . is_inf ()));
}
