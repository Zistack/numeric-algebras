mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_sin_cos_traits;
use numeric_algebras::traits::SinCos;

def_sin_cos_traits! (for AggregateType -> AggregateType in AggregateTypeAlgebra);

#[test]
fn sin_coss ()
{
	let (x_sin, x_cos) = (1.0f32) . sin_cos ();
	let (y_sin, y_cos) = (-1.0f32) . sin_cos ();

	assert_eq!
	(
		a! (AggregateTypeAlgebra, AggregateType {x: 1.0, y: -1.0} . sin_cos ()),
		(
			AggregateType {x: x_sin, y: y_sin},
			AggregateType {x: x_cos, y: y_cos}
		)
	);
}
