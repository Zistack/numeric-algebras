use macrospace_autotransform::AutotransformPath;
use syn::{Generics, Path, Type, Token, parenthesized, parse};
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, quote};

mod kw
{
	syn::custom_keyword! (with);
}

fn def_scalar_arithmetic_inner
(
	generics: Generics,
	aggregate_type: Path,
	scalar_type: Type,
	algebra_type: Path,
	algebra_type_autotransform: AutotransformPath
)
-> proc_macro2::TokenStream
{
	let where_clause = &generics . where_clause;

	quote!
	{
		numeric_algebras::derive::def_scalar_mul_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_scalar_div_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_scalar_pow_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_scalar_log_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_scalar_mul_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_scalar_div_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_scalar_pow_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
			#where_clause
		);
		numeric_algebras::derive::def_scalar_log_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
			#where_clause
		);

		numeric_algebras::derive::def_scalar_multiplication_is_commutative_trait!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
			#where_clause
		);

		macrospace_autotransform::delegate!
		{
			impl #generics #algebra_type
			with [#algebra_type_autotransform] -> []
			#where_clause
			{
				trait numeric_algebras::traits::Neg <#scalar_type>;
				trait <'a> numeric_algebras::traits::Neg <&'a #scalar_type>;

				trait numeric_algebras::traits::Abs <#scalar_type>;
				trait <'a> numeric_algebras::traits::Abs <&'a #scalar_type>;

				trait numeric_algebras::traits::Recip <#scalar_type>;
				trait <'a> numeric_algebras::traits::Recip <&'a #scalar_type>;

				trait numeric_algebras::traits::Sqrt <#scalar_type>;
				trait <'a> numeric_algebras::traits::Sqrt <&'a #scalar_type>;

				trait numeric_algebras::traits::Exp <#scalar_type>;
				trait <'a> numeric_algebras::traits::Exp <&'a #scalar_type>;

				trait numeric_algebras::traits::Ln <#scalar_type>;
				trait <'a> numeric_algebras::traits::Ln <&'a #scalar_type>;

				trait numeric_algebras::traits::Sin <#scalar_type>;
				trait <'a> numeric_algebras::traits::Sin <&'a #scalar_type>;

				trait numeric_algebras::traits::Cos <#scalar_type>;
				trait <'a> numeric_algebras::traits::Cos <&'a #scalar_type>;

				trait numeric_algebras::traits::Tan <#scalar_type>;
				trait <'a> numeric_algebras::traits::Tan <&'a #scalar_type>;

				trait numeric_algebras::traits::SinCos <#scalar_type>;
				trait <'a> numeric_algebras::traits::SinCos <&'a #scalar_type>;

				trait numeric_algebras::traits::Add <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::Add <#scalar_type, &'a #scalar_type>;
				trait <'a> numeric_algebras::traits::Add <&'a #scalar_type, #scalar_type>;
				trait <'a, 'b> numeric_algebras::traits::Add <&'a #scalar_type, &'b #scalar_type>;

				trait numeric_algebras::traits::AddAssign <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::AddAssign <#scalar_type, &'a #scalar_type>;

				trait numeric_algebras::traits::Sub <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::Sub <#scalar_type, &'a #scalar_type>;
				trait <'a> numeric_algebras::traits::Sub <&'a #scalar_type, #scalar_type>;
				trait <'a, 'b> numeric_algebras::traits::Sub <&'a #scalar_type, &'b #scalar_type>;

				trait numeric_algebras::traits::SubAssign <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::SubAssign <#scalar_type, &'a #scalar_type>;

				trait numeric_algebras::traits::Mul <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::Mul <#scalar_type, &'a #scalar_type>;
				trait <'a> numeric_algebras::traits::Mul <&'a #scalar_type, #scalar_type>;
				trait <'a, 'b> numeric_algebras::traits::Mul <&'a #scalar_type, &'b #scalar_type>;

				trait numeric_algebras::traits::MulAssign <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::MulAssign <#scalar_type, &'a #scalar_type>;

				trait numeric_algebras::traits::Div <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::Div <#scalar_type, &'a #scalar_type>;
				trait <'a> numeric_algebras::traits::Div <&'a #scalar_type, #scalar_type>;
				trait <'a, 'b> numeric_algebras::traits::Div <&'a #scalar_type, &'b #scalar_type>;

				trait numeric_algebras::traits::DivAssign <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::DivAssign <#scalar_type, &'a #scalar_type>;

				trait numeric_algebras::traits::Pow <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::Pow <#scalar_type, &'a #scalar_type>;
				trait <'a> numeric_algebras::traits::Pow <&'a #scalar_type, #scalar_type>;
				trait <'a, 'b> numeric_algebras::traits::Pow <&'a #scalar_type, &'b #scalar_type>;

				trait numeric_algebras::traits::PowAssign <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::PowAssign <#scalar_type, &'a #scalar_type>;

				trait numeric_algebras::traits::Log <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::Log <#scalar_type, &'a #scalar_type>;
				trait <'a> numeric_algebras::traits::Log <&'a #scalar_type, #scalar_type>;
				trait <'a, 'b> numeric_algebras::traits::Log <&'a #scalar_type, &'b #scalar_type>;

				trait numeric_algebras::traits::LogAssign <#scalar_type, #scalar_type>;
				trait <'a> numeric_algebras::traits::LogAssign <#scalar_type, &'a #scalar_type>;

				trait numeric_algebras::traits::AdditionIsCommutative <#scalar_type, #scalar_type>;
				trait numeric_algebras::traits::MultiplicationIsCommutative <#scalar_type, #scalar_type>;

				trait numeric_algebras::traits::Zero <#scalar_type>;
				trait numeric_algebras::traits::One <#scalar_type>;
				trait numeric_algebras::traits::E <#scalar_type>;
				trait numeric_algebras::traits::Pi <#scalar_type>;
				trait numeric_algebras::traits::Inf <#scalar_type>;
				trait numeric_algebras::traits::NaN <#scalar_type>;

				trait numeric_algebras::traits::Accumulatable <#scalar_type>;
				trait numeric_algebras::traits::Convert
				<
					<#algebra_type as numeric_algebras::traits::Accumulatable <#scalar_type>>::Accumulator,
					#scalar_type
				>
				where #algebra_type: numeric_algebras::traits::Accumulatable <#scalar_type>;
			}
		}
	}
}

#[derive (Clone, Debug)]
struct DefScalarArithmeticInput
{
	for_token: Token! [for],
	generics: Generics,

	paren_token: syn::token::Paren,
	aggregate_type: Path,
	comma_token: Token! [,],
	scalar_type: Type,

	in_token: Token! [in],
	algebra_type: Path,

	with_token: kw::with,
	algebra_type_autotransform: AutotransformPath
}

impl Parse for DefScalarArithmeticInput
{
	fn parse (input: ParseStream <'_>) -> Result <Self>
	{
		let for_token = input . parse ()?;
		let mut generics: Generics = input . parse ()?;

		let content;
		let paren_token = parenthesized! (content in input);

		let aggregate_type = content . parse ()?;
		let comma_token = content . parse ()?;
		let scalar_type = content . parse ()?;

		let in_token = input . parse ()?;
		let algebra_type = input . parse ()?;

		let with_token = input . parse ()?;
		let algebra_type_autotransform = input . parse ()?;

		generics . where_clause = input . parse ()?;

		let output = Self
		{
			for_token,
			generics,
			paren_token,
			aggregate_type,
			comma_token,
			scalar_type,
			in_token,
			algebra_type,
			with_token,
			algebra_type_autotransform
		};

		Ok (output)
	}
}

impl ToTokens for DefScalarArithmeticInput
{
	fn to_tokens (&self, tokens: &mut proc_macro2::TokenStream)
	{
		self . for_token . to_tokens (tokens);
		self . generics . to_tokens (tokens);

		self . paren_token . surround
		(
			tokens,
			|inner_tokens|
			{
				self . aggregate_type . to_tokens (inner_tokens);
				self . comma_token . to_tokens (inner_tokens);
				self . scalar_type . to_tokens (inner_tokens);
			}
		);

		self . in_token . to_tokens (tokens);
		self . algebra_type . to_tokens (tokens);

		self . with_token . to_tokens (tokens);
		self . algebra_type_autotransform . to_tokens (tokens);

		self . generics . where_clause . to_tokens (tokens);
	}
}

fn try_def_scalar_arithmetic_impl (input: proc_macro::TokenStream)
-> Result <proc_macro2::TokenStream>
{
	let DefScalarArithmeticInput
	{
		generics,
		aggregate_type,
		scalar_type,
		algebra_type,
		algebra_type_autotransform,
		..
	}
		= parse (input)?;

	Ok
	(
		def_scalar_arithmetic_inner
		(
			generics,
			aggregate_type,
			scalar_type,
			algebra_type,
			algebra_type_autotransform
		)
	)
}

pub fn def_scalar_arithmetic_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	try_def_scalar_arithmetic_impl (input)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}
