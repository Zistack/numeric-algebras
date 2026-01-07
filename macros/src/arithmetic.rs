use syn::{Generics, Path, Token, parenthesized, parse};
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, quote};

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
	let where_clause = &generics . where_clause;

	quote!
	{
		numeric_algebras::derive::def_neg_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_abs_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_recip_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_sqrt_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_exp_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_ln_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_sin_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_cos_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_tan_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_sin_cos_traits!
		(
			for #generics #aggregate_type -> #aggregate_type in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_add_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_sub_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_mul_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_div_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_add_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_sub_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_mul_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_div_assign_traits!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_addition_is_commutative_trait!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_multiplication_is_commutative_trait!
		(
			for #generics (#aggregate_type, #aggregate_type) in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_zero_trait!
		(
			for #generics #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_one_trait!
		(
			for #generics #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_e_trait!
		(
			for #generics #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_pi_trait!
		(
			for #generics #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_inf_trait!
		(
			for #generics #aggregate_type in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_nan_trait!
		(
			for #generics #aggregate_type in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_add_assign_traits!
		(
			for #generics (#accumulator_type, #aggregate_type)
			in #accumulator_algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_zero_trait!
		(
			for #generics #accumulator_type in #accumulator_algebra_type
			#where_clause
		);
	}
}

#[derive (Clone, Debug)]
struct DefArithmeticInput
{
	for_token: Token! [for],
	generics: Generics,

	type_paren_token: syn::token::Paren,
	aggregate_type: Path,
	type_comma_token: Token! [,],
	accumulator_type: Path,

	in_token: Token! [in],

	algebra_paren_token: syn::token::Paren,
	algebra_type: Path,
	algebra_comma_token: Token! [,],
	accumulator_algebra_type: Path

	// where clause
}

impl Parse for DefArithmeticInput
{
	fn parse (input: ParseStream <'_>) -> Result <Self>
	{
		let for_token = input . parse ()?;
		let mut generics: Generics = input . parse ()?;

		let content;
		let type_paren_token = parenthesized! (content in input);

		let aggregate_type = content . parse ()?;
		let type_comma_token = content . parse ()?;
		let accumulator_type = content . parse ()?;

		let in_token = input . parse ()?;

		let content;
		let algebra_paren_token = parenthesized! (content in input);

		let algebra_type = content . parse ()?;
		let algebra_comma_token = content . parse ()?;
		let accumulator_algebra_type = content . parse ()?;

		generics . where_clause = input . parse ()?;

		let output = Self
		{
			for_token,
			generics,
			type_paren_token,
			aggregate_type,
			type_comma_token,
			accumulator_type,
			in_token,
			algebra_paren_token,
			algebra_type,
			algebra_comma_token,
			accumulator_algebra_type
		};

		Ok (output)
	}
}

impl ToTokens for DefArithmeticInput
{
	fn to_tokens (&self, tokens: &mut proc_macro2::TokenStream)
	{
		self . for_token . to_tokens (tokens);
		self . generics . to_tokens (tokens);

		self . type_paren_token . surround
		(
			tokens,
			|inner_tokens|
			{
				self . aggregate_type . to_tokens (inner_tokens);
				self . type_comma_token . to_tokens (inner_tokens);
				self . accumulator_type . to_tokens (inner_tokens);
			}
		);

		self . in_token . to_tokens (tokens);

		self . algebra_paren_token . surround
		(
			tokens,
			|inner_tokens|
			{
				self . algebra_type . to_tokens (inner_tokens);
				self . algebra_comma_token . to_tokens (inner_tokens);
				self . accumulator_algebra_type . to_tokens (inner_tokens);
			}
		);

		self . generics . where_clause . to_tokens (tokens);
	}
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
