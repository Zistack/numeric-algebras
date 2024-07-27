mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_nan_trait;
use numeric_algebras::traits::NaN;

def_nan_trait!
(
	for AggregateType in AggregateTypeAlgebra
);

fn main ()
{
	let ag_nan = a! (AggregateTypeAlgebra, AggregateType::nan ());

	assert_ne!
	(
		ag_nan,
		AggregateType {x: f32::NAN, y: f32::NAN}
	);

	assert_ne! (ag_nan, ag_nan);

	assert! (a! (AggregateTypeAlgebra, (&ag_nan) . is_nan ()));
}
