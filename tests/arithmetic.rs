mod aggregate;
use aggregate::*;

use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_arithmetic;

def_arithmetic!
(
	for (AggregateType, AggregateAccumulatorType)
	in (AggregateTypeAlgebra, AggregateAccumulatorTypeAlgebra)
);
