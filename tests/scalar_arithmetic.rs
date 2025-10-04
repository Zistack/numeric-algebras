mod aggregate;
use aggregate::*;

use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_scalar_arithmetic;

def_scalar_arithmetic!
(
	for (AggregateType, f32)
	in AggregateTypeAlgebra
	with AggregateTypeAlgebraToF32Algebra
);
