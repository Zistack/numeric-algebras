use numeric_algebras_macros::*;

use forward_traits::forwardable;

// Unary

def_unary_op! (Neg, neg);
def_unary_op! (Abs, abs);
def_unary_op! (Recip, recip);
def_unary_op! (Sqrt, sqrt);

def_unary_op! (Exp, exp);
def_unary_op! (Ln, ln);

def_unary_op! (Sin, sin);
def_unary_op! (Cos, cos);
def_unary_op! (Tan, tan);

#[forwardable]
pub trait SinCos <X>
{
	type Output;

	fn sin_cos (self, x: X) -> (Self::Output, Self::Output);
}

pub trait SinCoss <X>:
	SinCos <X, Output = <Self as SinCoss <X>>::Output>
	+ for <'a> SinCos <&'a X, Output = <Self as SinCoss <X>>::Output>
{
	type Output;
}

impl <X, O, T> SinCoss <X> for T
where T: SinCos <X, Output = O> + for <'a> SinCos <&'a X, Output = O>
{
	type Output = O;
}

pub trait SinCossMonoid <X>: SinCoss <X, Output = X>
{
}

impl <X, T> SinCossMonoid <X> for T
where T: SinCoss <X, Output = X>
{
}

// Binary

def_symmetric_binary_op! (Addition, Add, add);
def_binary_op! (Sub, sub);
def_symmetric_binary_op! (Multiplication, Mul, mul);
def_binary_op! (Div, div);

def_try_binary_op! (Pow, pow);
def_try_binary_op! (Log, log);
