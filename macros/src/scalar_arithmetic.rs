use macrospace_autotransform::AutotransformPath;
use syn::{Generics, Path, Type, Token, parse};
use syn::parse::{Result, Error};
use syn_derive::{Parse, ToTokens};
use quote::quote;

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
	quote!
	{
		numeric_algebras::derive::def_scalar_mul_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
		);
		numeric_algebras::derive::def_scalar_div_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
		);
		numeric_algebras::derive::def_scalar_pow_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
		);
		numeric_algebras::derive::def_scalar_log_traits!
		(
			for #generics (#aggregate_type, #scalar_type) -> #aggregate_type
			in #algebra_type
		);

		numeric_algebras::derive::def_scalar_mul_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
		);
		numeric_algebras::derive::def_scalar_div_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
		);
		numeric_algebras::derive::def_scalar_pow_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
		);
		numeric_algebras::derive::def_scalar_log_assign_traits!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
		);

		numeric_algebras::derive::def_scalar_multiplication_is_commutative_trait!
		(
			for #generics (#aggregate_type, #scalar_type) in #algebra_type
		);

		macrospace_autotransform::delegate!
		{
			impl #generics #algebra_type
			with [#algebra_type_autotransform] -> []
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

#[derive (Clone, Debug, Parse, ToTokens)]
struct DefScalarArithmeticInput
{
	for_token: Token! [for],
	generics: Generics,

	#[syn (parenthesized)]
	paren_token: syn::token::Paren,
	#[syn (in = paren_token)]
	aggregate_type: Path,
	#[syn (in = paren_token)]
	comma_token: Token! [,],
	#[syn (in = paren_token)]
	scalar_type: Type,

	in_token: Token! [in],

	algebra_type: Path,

	with_token: kw::with,

	algebra_type_autotransform: AutotransformPath
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
