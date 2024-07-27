mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_pi_trait;
use numeric_algebras::traits::Pi;

def_pi_trait!
(
	for AggregateType in AggregateTypeAlgebra
);

fn main ()
{
	let ag_pi = a! (AggregateTypeAlgebra, AggregateType::pi ());

	assert_eq!
	(
		ag_pi,
		AggregateType {x: std::f32::consts::PI, y: std::f32::consts::PI}
	);

	assert! (a! (AggregateTypeAlgebra, (&ag_pi) . is_pi ()));
}
