use syn::{
	Ident,
	Path,
	Generics,
	Type,
	DeriveInput,
	ItemStruct,
	Data,
	Token,
	parenthesized,
	parse,
	parse_quote
};
use syn::token::Paren;
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, format_ident, quote};

use macrospace::enum_utils::get_variant_types;
use macrospace::generics::combine_generics;
use macrospace::path_utils::without_arguments;
use macrospace::struct_utils::get_member_types;
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::algebra_mapping::AlgebraMapping;

fn def_marker_trait_inner
(
	pascal_marker: Ident,
	algebra_type: Path,
	lhs_type: Path,
	rhs_type: Path,
	generics: Generics,
	member_algebras: Vec <(Type, Type, Type)>
)
-> proc_macro2::TokenStream
{
	let marker_trait: Path =
		parse_quote! (numeric_algebras::traits::#pascal_marker);

	let mut tokens = proc_macro2::TokenStream::new ();

	{
		let mut generics = generics . clone ();

		let where_clause = generics . make_where_clause ();

		for (member_type_algebra, lhs_member_type, rhs_part_type)
		in &member_algebras
		{
			where_clause . predicates . push
			(
				parse_quote!
				(
					#member_type_algebra:
						#marker_trait <#lhs_member_type, #rhs_part_type>
				)
			);
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics #marker_trait <#lhs_type, #rhs_type>
			for #algebra_type
			#where_clause
			{
			}
		}
			. to_tokens (&mut tokens);
	}

	if lhs_type != rhs_type
	{
		let mut generics = generics . clone ();

		let where_clause = generics . make_where_clause ();

		for (member_type_algebra, lhs_member_type, rhs_part_type)
		in &member_algebras
		{
			where_clause . predicates . push
			(
				parse_quote!
				(
					#member_type_algebra:
						#marker_trait <#rhs_part_type, #lhs_member_type>
				)
			);
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics #marker_trait <#rhs_type, #lhs_type>
			for #algebra_type
			#where_clause
			{
			}
		}
			. to_tokens (&mut tokens)
	}

	tokens
}

struct DefMarkerTrait
{
	for_token: Token! [for],
	generics: Generics,

	paren_token: Paren,
	lhs_type: Path,
	comma_token: Token! [,],
	rhs_type: Path,

	in_token: Token! [in],
	algebra_type: Path

	// where clause
}

impl Parse for DefMarkerTrait
{
	fn parse (input: ParseStream <'_>) -> Result <Self>
	{
		let for_token = input . parse ()?;
		let mut generics: Generics = input . parse ()?;

		let content;
		let paren_token = parenthesized! (content in input);

		let lhs_type = content . parse ()?;
		let comma_token = content . parse ()?;
		let rhs_type = content . parse ()?;

		let in_token = input . parse ()?;
		let algebra_type = input . parse ()?;

		generics . where_clause = input . parse ()?;

		let output = Self
		{
			for_token,
			generics,
			paren_token,
			lhs_type,
			comma_token,
			rhs_type,
			in_token,
			algebra_type
		};

		Ok (output)
	}
}

impl ToTokens for DefMarkerTrait
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
				self . lhs_type . to_tokens (inner_tokens);
				self . comma_token . to_tokens (inner_tokens);
				self . rhs_type . to_tokens (inner_tokens);
			}
		);

		self . in_token . to_tokens (tokens);
		self . algebra_type . to_tokens (tokens);

		self . generics . where_clause . to_tokens (tokens);
	}
}

fn try_def_marker_trait_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_marker: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let
	(
		(algebra_item, lhs_item, rhs_item),
		DefMarkerTrait {generics, lhs_type, rhs_type, algebra_type, ..}
	):
		((DeriveInput, ItemStruct, DeriveInput), DefMarkerTrait)
		= macrospace::parse_args! (3, input)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item . clone (), &algebra_type)?;

	let (_, substituted_lhs_item) =
		substitute_arguments_for_struct (lhs_item, &lhs_type)?;

	let lhs_member_types = get_member_types (&substituted_lhs_item . fields);

	let (_, substituted_rhs_item) =
		substitute_arguments_for_derive_input (rhs_item, &rhs_type)?;

	let rhs_part_types = match &substituted_rhs_item . data
	{
		Data::Struct (struct_data) => get_member_types (&struct_data . fields),
		Data::Enum (enum_data) => get_variant_types (&enum_data . variants)?,
		_ => unreachable! ()
	};

	let algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&algebra_item,
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions
	)?;

	let (_, member_algebra_types) = algebra_mapping . into_parts ();

	let mut member_algebras = Vec::new ();

	for ((member_algebra_type, lhs_member_type), rhs_part_type)
	in member_algebra_types
		. into_iter ()
		. zip (lhs_member_types)
		. zip (rhs_part_types)
	{
		member_algebras . push
		((
			member_algebra_type,
			lhs_member_type,
			rhs_part_type
		));
	}

	let generics = combine_generics
	([
		generics,
		substituted_algebra_item . generics,
		substituted_lhs_item . generics,
		substituted_rhs_item . generics
	]);

	let tokens = def_marker_trait_inner
	(
		pascal_marker,
		algebra_type,
		lhs_type,
		rhs_type,
		generics,
		member_algebras
	);

	Ok (tokens)
}

pub fn def_marker_trait_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_marker: Ident
)
-> proc_macro::TokenStream
{
	try_def_marker_trait_inner_impl (input, pascal_marker)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_def_marker_trait_impl
(
	input: proc_macro::TokenStream,
	snake_marker: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let def_marker_trait = parse (input)?;

	let DefMarkerTrait
	{
		lhs_type,
		rhs_type,
		algebra_type,
		..
	}
		= &def_marker_trait;

	let lhs_type = without_arguments (lhs_type . clone ());
	let rhs_type = without_arguments (rhs_type . clone ());
	let algebra_type = without_arguments (algebra_type . clone ());

	let inner_macro_ident = format_ident! ("__def_{}_trait_inner", snake_marker);

	let inner_macro_path =
		parse_quote! (numeric_algebras::derive::#inner_macro_ident);

	let tokens = macrospace::generate_macrospace_invokation
	(
		inner_macro_path,
		[
			parse_quote! (#algebra_type: struct | enum),
			parse_quote! (#lhs_type: struct),
			parse_quote! (#rhs_type: struct | enum)
		],
		def_marker_trait
	);

	Ok (tokens)
}

pub fn def_marker_trait_impl
(
	input: proc_macro::TokenStream,
	snake_marker: Ident
)
-> proc_macro::TokenStream
{
	try_def_marker_trait_impl (input, snake_marker)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_marker_trait_macro
{
	($pascal_marker: ident, $snake_marker: ident) =>
	{
		paste::paste!
		{
			#[doc (hidden)]
			#[proc_macro]
			pub fn [<__def_ $snake_marker _trait_inner>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::marker::def_marker_trait_inner_impl
				(
					input,
					syn::parse_quote! ($pascal_marker)
				)
			}

			#[proc_macro]
			pub fn [<def_ $snake_marker _trait>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::marker::def_marker_trait_impl
				(
					input,
					syn::parse_quote! ($snake_marker)
				)
			}
		}
	}
}

pub (crate) use __def_marker_trait_macro as def_marker_trait_macro;
