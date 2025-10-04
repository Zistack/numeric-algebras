use syn::{Generics, Path, Token, parse};
use syn::parse::{Result, Error};
use syn_derive::{Parse, ToTokens};
use quote::quote;

fn def_arithmetic_inner
(
	generics: Generics,
	aggregate_type: Path,
	accumulator_type: Path,
	algebra_type: Path,
	accumulator_algebra_type: Path
)
-> proc_macro2::TokenStream
{
	quote!
	{
		numeric_algebras::derive::def_neg_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_abs_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_recip_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_sqrt_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_exp_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_ln_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_sin_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_cos_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_tan_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);

		numeric_algebras::derive::def_sin_cos_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
		);

		numeric_algebras::derive::def_add_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
		);
		numeric_algebras::derive::def_sub_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
		);
		numeric_algebras::derive::def_mul_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
		);
		numeric_algebras::derive::def_div_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
		);

		numeric_algebras::derive::def_add_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
		);
		numeric_algebras::derive::def_sub_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
		);
		numeric_algebras::derive::def_mul_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
		);
		numeric_algebras::derive::def_div_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
		);

		numeric_algebras::derive::def_addition_is_commutative_trait!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
		);
		numeric_algebras::derive::def_multiplication_is_commutative_trait!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
		);

		numeric_algebras::derive::def_zero_trait!
		(
			for #generics #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_one_trait!
		(
			for #generics #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_e_trait!
		(
			for #generics #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_pi_trait!
		(
			for #generics #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_inf_trait!
		(
			for #generics #aggregate_type in #algebra_type
		);
		numeric_algebras::derive::def_nan_trait!
		(
			for #generics #aggregate_type in #algebra_type
		);

		numeric_algebras::derive::def_add_assign_traits!
		(
			for #generics (#accumulator_type, #aggregate_type)
			in #accumulator_algebra_type
		);
		numeric_algebras::derive::def_zero_trait!
		(
			for #generics #accumulator_type in #accumulator_algebra_type
		);
	}
}

#[derive (Clone, Debug, Parse, ToTokens)]
struct DefArithmeticInput
{
	for_token: Token! [for],
	generics: Generics,

	#[syn (parenthesized)]
	type_paren_token: syn::token::Paren,
	#[syn (in = type_paren_token)]
	aggregate_type: Path,
	#[syn (in = type_paren_token)]
	type_comma_token: Token! [,],
	#[syn (in = type_paren_token)]
	accumulator_type: Path,

	in_token: Token! [in],

	#[syn (parenthesized)]
	algebra_paren_token: syn::token::Paren,
	#[syn (in = algebra_paren_token)]
	algebra_type: Path,
	#[syn (in = algebra_paren_token)]
	algebra_comma_token: Token! [,],
	#[syn (in = algebra_paren_token)]
	accumulator_algebra_type: Path
}

fn try_def_arithmetic_impl (input: proc_macro::TokenStream)
-> Result <proc_macro2::TokenStream>
{
	let DefArithmeticInput
	{
		generics,
		aggregate_type,
		accumulator_type,
		algebra_type,
		accumulator_algebra_type,
		..
	}
		= parse (input)?;

	Ok
	(
		def_arithmetic_inner
		(
			generics,
			aggregate_type,
			accumulator_type,
			algebra_type,
			accumulator_algebra_type
		)
	)
}

pub fn def_arithmetic_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	try_def_arithmetic_impl (input)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}
