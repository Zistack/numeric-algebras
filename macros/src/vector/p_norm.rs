use std::iter::zip;

use syn::{
	Path,
	Generics,
	Type,
	Member,
	Expr,
	Lifetime,
	DeriveInput,
	ItemStruct,
	Token,
	parse,
	parenthesized,
	parse_quote
};
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, quote};

use macrospace::generics::combine_generics;
use macrospace::path_utils::without_arguments;
use macrospace::struct_utils::get_members_and_types_split;
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::algebra_mapping::AlgebraMapping;

fn def_p_norm_traits_inner
(
	algebra_type: Path,
	struct_type: Path,
	power_type: Path,
	scalar_type: Path,
	generics: Generics,
	member_algebras: Vec <(Type, Type)>,
	members: Vec <Member>,
	algebra_conversion_expressions: Vec <Expr>
)
-> proc_macro2::TokenStream
{
	let p_norm_trait: Path =
		parse_quote! (numeric_algebras::traits::PNorm);

	let mut tokens = proc_macro2::TokenStream::new ();

	{
		let mut generics = generics . clone ();

		let where_clause = generics . make_where_clause ();

		for (member_algebra_type, member_type) in &member_algebras
		{
			where_clause . predicates . push
			(
				parse_quote!
				(
					#member_algebra_type:
						#p_norm_trait <#member_type, #power_type, Output = #scalar_type>
				)
			);
		}

		where_clause . predicates . push
		(
			parse_quote!
			(
				Self:
					std::clone::Clone
					+ numeric_algebras::traits::Recip <#power_type, Output = #power_type>
					+ numeric_algebras::traits::Pow <#scalar_type, #power_type, Output = #scalar_type>
					+ numeric_algebras::traits::Sum <#scalar_type, #scalar_type>
			)
		);
		where_clause . predicates . push (parse_quote! (#power_type: Clone));

		let mut values = Vec::new ();

		for (member, algebra_conversion_expression)
		in zip (&members, &algebra_conversion_expressions)
		{
			values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. p_sum (x . #member, p . clone ())
				)
			);
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#p_norm_trait <#struct_type, #power_type>
			for #algebra_type
			#where_clause
			{
				type Output = #scalar_type;

				fn p_norm (self, x: #struct_type, p: #power_type)
				-> Self::Output
				{
					use numeric_algebras::traits::{Pow, Recip};

					numeric_algebras::a! (self, x . p_sum (p . clone ()) . pow (p . recip ()))
				}

				fn p_sum (self, x: #struct_type, p: #power_type) -> Self::Output
				{
					use numeric_algebras::traits::Sum;

					let values = [#(#values),*];
					self . sum (values . into_iter ())
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	{
		let mut generics = generics . clone ();

		let lifetime_a = Lifetime::new ("'a", proc_macro2::Span::mixed_site ());

		generics . params . push (parse_quote! (#lifetime_a));

		let where_clause = generics . make_where_clause ();

		for (member_algebra_type, member_type) in &member_algebras
		{
			where_clause . predicates . push
			(
				parse_quote!
				(
					#member_algebra_type:
						#p_norm_trait <#member_type, &#lifetime_a #power_type, Output = #scalar_type>
				)
			);
		}

		where_clause . predicates . push
		(
			parse_quote!
			(
				Self:
					std::clone::Clone
					+ numeric_algebras::traits::Recip <&#lifetime_a #power_type, Output = #power_type>
					+ numeric_algebras::traits::Pow <#scalar_type, #power_type, Output = #scalar_type>
					+ numeric_algebras::traits::Sum <#scalar_type, #scalar_type>
			)
		);

		let mut values = Vec::new ();

		for (member, algebra_conversion_expression)
		in zip (&members, &algebra_conversion_expressions)
		{
			values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. p_sum (x . #member, p)
				)
			);
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#p_norm_trait <#struct_type, &#lifetime_a #power_type>
			for #algebra_type
			#where_clause
			{
				type Output = #scalar_type;

				fn p_norm (self, x: #struct_type, p: &#lifetime_a #power_type)
				-> Self::Output
				{
					use numeric_algebras::traits::{Pow, Recip};

					numeric_algebras::a! (self, x . p_sum (p) . pow (p . recip ()))
				}

				fn p_sum (self, x: #struct_type, p: &#lifetime_a #power_type)
				-> Self::Output
				{
					use numeric_algebras::traits::Sum;

					let values = [#(#values),*];
					self . sum (values . into_iter ())
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	{
		let mut generics = generics . clone ();

		let lifetime_a = Lifetime::new ("'a", proc_macro2::Span::mixed_site ());

		generics . params . push (parse_quote! (#lifetime_a));

		let where_clause = generics . make_where_clause ();

		for (member_algebra_type, member_type) in &member_algebras
		{
			where_clause . predicates . push
			(
				parse_quote!
				(
					#member_algebra_type:
						#p_norm_trait <&#lifetime_a #member_type, #power_type, Output = #scalar_type>
				)
			);
		}

		where_clause . predicates . push
		(
			parse_quote!
			(
				Self:
					std::clone::Clone
					+ numeric_algebras::traits::Recip <#power_type, Output = #power_type>
					+ numeric_algebras::traits::Pow <#scalar_type, #power_type, Output = #scalar_type>
					+ numeric_algebras::traits::Sum <#scalar_type, #scalar_type>
			)
		);
		where_clause . predicates . push (parse_quote! (#power_type: Clone));

		let mut values = Vec::new ();

		for (member, algebra_conversion_expression)
		in zip (&members, &algebra_conversion_expressions)
		{
			values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. p_sum (&x . #member, p . clone ())
				)
			);
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#p_norm_trait <&#lifetime_a #struct_type, #power_type>
			for #algebra_type
			#where_clause
			{
				type Output = #scalar_type;

				fn p_norm (self, x: &#lifetime_a #struct_type, p: #power_type)
				-> Self::Output
				{
					use numeric_algebras::traits::{Pow, Recip};

					numeric_algebras::a! (self, x . p_sum (p . clone ()) . pow (p . recip ()))
				}

				fn p_sum (self, x: &#lifetime_a #struct_type, p: #power_type)
				-> Self::Output
				{
					use numeric_algebras::traits::Sum;

					let values = [#(#values),*];
					self . sum (values . into_iter ())
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	{
		let mut generics = generics . clone ();

		let lifetime_a = Lifetime::new ("'a", proc_macro2::Span::mixed_site ());
		let lifetime_b = Lifetime::new ("'b", proc_macro2::Span::mixed_site ());

		generics . params . push (parse_quote! (#lifetime_a));
		generics . params . push (parse_quote! (#lifetime_b));

		let where_clause = generics . make_where_clause ();

		for (member_algebra_type, member_type) in &member_algebras
		{
			where_clause . predicates . push
			(
				parse_quote!
				(
					#member_algebra_type:
						#p_norm_trait <&#lifetime_a #member_type, &#lifetime_b #power_type, Output = #scalar_type>
				)
			);
		}

		where_clause . predicates . push
		(
			parse_quote!
			(
				Self:
					std::clone::Clone
					+ numeric_algebras::traits::Recip <&#lifetime_b #power_type, Output = #power_type>
					+ numeric_algebras::traits::Pow <#scalar_type, #power_type, Output = #scalar_type>
					+ numeric_algebras::traits::Sum <#scalar_type, #scalar_type>
			)
		);

		let mut values = Vec::new ();

		for (member, algebra_conversion_expression)
		in zip (&members, &algebra_conversion_expressions)
		{
			values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. p_sum (&x . #member, p)
				)
			);
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#p_norm_trait <&#lifetime_a #struct_type, &#lifetime_b #power_type>
			for #algebra_type
			#where_clause
			{
				type Output = #scalar_type;

				fn p_norm
				(
					self,
					x: &#lifetime_a #struct_type,
					p: &#lifetime_b #power_type
				)
				-> Self::Output
				{
					use numeric_algebras::traits::{Pow, Recip};

					numeric_algebras::a! (self, x . p_sum (p) . pow (p . recip ()))
				}

				fn p_sum
				(
					self,
					x: &#lifetime_a #struct_type,
					p: &#lifetime_b #power_type
				)
				-> Self::Output
				{
					use numeric_algebras::traits::Sum;

					let values = [#(#values),*];
					self . sum (values . into_iter ())
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	tokens
}

struct DefPNormTraits
{
	for_token: Token! [for],
	generics: Generics,

	paren_token: syn::token::Paren,
	struct_type: Path,
	comma_token: Token! [,],
	power_type: Path,

	arrow_token: Token! [->],
	scalar_type: Path,

	in_token: Token! [in],
	algebra_type: Path

	// where clause
}

impl Parse for DefPNormTraits
{
	fn parse (input: ParseStream <'_>) -> Result <Self>
	{
		let for_token = input . parse ()?;
		let mut generics: Generics = input . parse ()?;

		let content;
		let paren_token = parenthesized! (content in input);

		let struct_type = content . parse ()?;
		let comma_token = content . parse ()?;
		let power_type = content . parse ()?;

		let arrow_token = input . parse ()?;
		let scalar_type = input . parse ()?;

		let in_token = input . parse ()?;
		let algebra_type = input . parse ()?;

		generics . where_clause = input . parse ()?;

		let output = Self
		{
			for_token,
			generics,
			paren_token,
			struct_type,
			comma_token,
			power_type,
			arrow_token,
			scalar_type,
			in_token,
			algebra_type
		};

		Ok (output)
	}
}

impl ToTokens for DefPNormTraits
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
				self . struct_type . to_tokens (inner_tokens);
				self . comma_token . to_tokens (inner_tokens);
				self . power_type . to_tokens (inner_tokens);
			}
		);

		self . arrow_token . to_tokens (tokens);
		self . scalar_type . to_tokens (tokens);

		self . in_token . to_tokens (tokens);
		self . algebra_type . to_tokens (tokens);

		self . generics . where_clause . to_tokens (tokens);
	}
}

fn try_def_p_norm_traits_inner_impl (input: proc_macro::TokenStream)
-> Result <proc_macro2::TokenStream>
{
	let
	(
		(algebra_item, struct_item),
		DefPNormTraits
		{
			generics,
			struct_type,
			power_type,
			scalar_type,
			algebra_type,
			..
		}
	):
		((DeriveInput, ItemStruct), DefPNormTraits)
		= macrospace::parse_args! (2, input)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item . clone (), &algebra_type)?;

	let (_, substituted_struct_item) =
		substitute_arguments_for_struct (struct_item, &struct_type)?;

	let (members, member_types) =
		get_members_and_types_split (&substituted_struct_item . fields);

	let algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&algebra_item,
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions
	)?;

	let (algebra_conversion_expressions, member_algebra_types) =
		algebra_mapping . into_parts ();

	let mut member_algebras = Vec::new ();

	for (member_algebra_type, member_type)
	in member_algebra_types . into_iter () . zip (member_types)
	{
		member_algebras . push ((member_algebra_type, member_type));
	}

	let generics = combine_generics
	([
		generics,
		substituted_algebra_item . generics,
		substituted_struct_item . generics
	]);

	let tokens = def_p_norm_traits_inner
	(
		algebra_type,
		struct_type,
		power_type,
		scalar_type,
		generics,
		member_algebras,
		members,
		algebra_conversion_expressions
	);

	Ok (tokens)
}

pub fn __def_p_norm_traits_inner_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	try_def_p_norm_traits_inner_impl (input)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_def_p_norm_traits_impl (input: proc_macro::TokenStream)
-> Result <proc_macro2::TokenStream>
{
	let def_p_norm_traits = parse (input)?;

	let DefPNormTraits
	{
		struct_type,
		algebra_type,
		..
	}
		= &def_p_norm_traits;

	let struct_type = without_arguments (struct_type . clone ());
	let algebra_type = without_arguments (algebra_type . clone ());

	let inner_macro_path =
		parse_quote! (numeric_algebras::derive::__def_p_norm_traits_inner);

	let  tokens = macrospace::generate_macrospace_invokation
	(
		inner_macro_path,
		[
			parse_quote! (#algebra_type: struct | enum),
			parse_quote! (#struct_type: struct)
		],
		def_p_norm_traits
	);

	Ok (tokens)
}

pub fn def_p_norm_traits_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	try_def_p_norm_traits_impl (input)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_p_norm_traits_macro
{
	() =>
	{
		#[doc (hidden)]
		#[proc_macro]
		pub fn __def_p_norm_traits_inner (input: proc_macro::TokenStream)
		-> proc_macro::TokenStream
		{
			crate::vector::__def_p_norm_traits_inner_impl (input)
		}

		#[proc_macro]
		pub fn def_p_norm_traits (input: proc_macro::TokenStream)
		-> proc_macro::TokenStream
		{
			crate::vector::def_p_norm_traits_impl (input)
		}
	}
}

pub (crate) use __def_p_norm_traits_macro as def_p_norm_traits_macro;
