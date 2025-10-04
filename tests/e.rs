mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_e_trait;
use numeric_algebras::traits::E;

def_e_trait!
(
	for AggregateType in AggregateTypeAlgebra
);

#[test]
fn e ()
{
	let ag_e = a! (AggregateTypeAlgebra, AggregateType::e ());

	assert_eq!
	(
		ag_e,
		AggregateType {x: std::f32::consts::E, y: std::f32::consts::E}
	);

	assert! (a! (AggregateTypeAlgebra, (&ag_e) . is_e ()));
}
