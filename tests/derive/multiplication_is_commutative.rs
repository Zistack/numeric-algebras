mod aggregate;
use aggregate::*;

use numeric_algebras::algebras::float::F32Algebra;
use numeric_algebras::derive::def_multiplication_is_commutative_trait;

def_multiplication_is_commutative_trait! (for (AggregateType, AggregateType) in AggregateTypeAlgebra);

fn main ()
{
}
