#![feature (impl_trait_in_assoc_type)]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn algebra_mapping (_attr: TokenStream, item: TokenStream)
-> TokenStream
{
	item
}

mod unary;
use unary::def_unary_traits_macro;

def_unary_traits_macro! (Neg, neg);
def_unary_traits_macro! (Abs, abs);
def_unary_traits_macro! (Recip, recip);
def_unary_traits_macro! (Sqrt, sqrt);

def_unary_traits_macro! (Exp, exp);
def_unary_traits_macro! (Ln, ln);

def_unary_traits_macro! (Sin, sin);
def_unary_traits_macro! (Cos, cos);
def_unary_traits_macro! (Tan, tan);

mod sin_cos;
use sin_cos::def_sin_cos_traits_macro;

def_sin_cos_traits_macro! ();

mod binary;
use binary::def_binary_traits_macro;

def_binary_traits_macro! (Add, add);
def_binary_traits_macro! (Sub, sub);
def_binary_traits_macro! (Mul, mul);
def_binary_traits_macro! (Div, div);

def_binary_traits_macro! (Pow, pow);
def_binary_traits_macro! (Log, log);

mod assign;
use assign::def_assign_traits_macro;

def_assign_traits_macro! (Add, add);
def_assign_traits_macro! (Sub, sub);
def_assign_traits_macro! (Mul, mul);
def_assign_traits_macro! (Div, div);

def_assign_traits_macro! (Pow, pow);
def_assign_traits_macro! (Log, log);

mod scalar;
use scalar::def_scalar_traits_macro;

def_scalar_traits_macro! (Add, add);
def_scalar_traits_macro! (Sub, sub);
def_scalar_traits_macro! (Mul, mul);
def_scalar_traits_macro! (Div, div);

def_scalar_traits_macro! (Pow, pow);
def_scalar_traits_macro! (Log, log);

mod scalar_assign;
use scalar_assign::def_scalar_assign_traits_macro;

def_scalar_assign_traits_macro! (Add, add);
def_scalar_assign_traits_macro! (Sub, sub);
def_scalar_assign_traits_macro! (Mul, mul);
def_scalar_assign_traits_macro! (Div, div);

def_scalar_assign_traits_macro! (Pow, pow);
def_scalar_assign_traits_macro! (Log, log);

mod value;
use value::def_value_trait_macro;

def_value_trait_macro! (Zero, zero, &&);
def_value_trait_macro! (One, one, &&);
def_value_trait_macro! (E, e, &&);
def_value_trait_macro! (Pi, pi, &&);
def_value_trait_macro! (Inf, inf, ||);
def_value_trait_macro! (NaN, nan, ||);

mod marker;
use marker::def_marker_trait_macro;

def_marker_trait_macro! (AdditionIsCommutative, addition_is_commutative);
def_marker_trait_macro! (MultiplicationIsCommutative, multiplication_is_commutative);

mod scalar_marker;
use scalar_marker::def_scalar_marker_trait_macro;

def_scalar_marker_trait_macro! (AdditionIsCommutative, addition_is_commutative);
def_scalar_marker_trait_macro! (MultiplicationIsCommutative, multiplication_is_commutative);

mod arithmetic;

#[proc_macro]
pub fn def_arithmetic (input: TokenStream) -> TokenStream
{
	arithmetic::def_arithmetic_impl (input)
}

mod scalar_arithmetic;

#[proc_macro]
pub fn def_scalar_arithmetic (input: TokenStream) -> TokenStream
{
	scalar_arithmetic::def_scalar_arithmetic_impl (input)
}
