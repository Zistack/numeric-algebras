mod aggregate;
use aggregate::*;

use numeric_algebras::a;
use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::{def_scalar_arithmetic, def_elements_product_traits};
use numeric_algebras::traits::ElementsProduct;

def_scalar_arithmetic!
(
	for (AggregateType, f32)
	in AggregateTypeAlgebra
	with AggregateTypeAlgebraToF32Algebra
);

def_elements_product_traits!
(
	for AggregateType -> f32 in AggregateTypeAlgebra
);

#[test]
fn elements_products ()
{
	assert_eq!
	(
		a!
		(
			AggregateTypeAlgebra,
			AggregateType {x: 1.0, y: 2.0} . elements_product ()
		),
		2.0
	);
}
