use syn::{
	Ident,
	Path,
	Generics,
	Type,
	DeriveInput,
	ItemStruct,
	Token,
	parenthesized,
	parse,
	parse_quote
};
use syn::token::Paren;
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, format_ident, quote};

use macrospace::generics::combine_generics;
use macrospace::path_utils::without_arguments;
use macrospace::struct_utils::get_member_types;
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::algebra_mapping::AlgebraMapping;

fn def_scalar_marker_trait_inner
(
	pascal_marker: Ident,
	algebra_type: Path,
	struct_type: Path,
	scalar_type: Path,
	mut generics: Generics,
	member_algebras: Vec <(Type, Type)>
)
-> proc_macro2::TokenStream
{
	let marker_trait: Path =
		parse_quote! (numeric_algebras::traits::#pascal_marker);

	let where_clause = generics . make_where_clause ();

	for (member_type_algebra, struct_member_type) in &member_algebras
	{
		where_clause . predicates . push
		(
			parse_quote!
			(
				#member_type_algebra:
					#marker_trait <#struct_member_type, #scalar_type>
					+ #marker_trait <#scalar_type, #struct_member_type>
			)
		);
	}

	let (impl_generics, _, where_clause) = generics . split_for_impl ();

	quote!
	{
		#[automatically_derived]
		impl #impl_generics #marker_trait <#struct_type, #scalar_type>
		for #algebra_type
		#where_clause
		{
		}

		#[automatically_derived]
		impl #impl_generics #marker_trait <#scalar_type, #struct_type>
		for #algebra_type
		#where_clause
		{
		}
	}
}

struct DefScalarMarkerTrait
{
	for_token: Token! [for],
	generics: Generics,

	paren_token: Paren,
	struct_type: Path,
	comma_token: Token! [,],
	scalar_type: Path,

	in_token: Token! [in],
	algebra_type: Path

	// where clause
}

impl Parse for DefScalarMarkerTrait
{
	fn parse (input: ParseStream <'_>) -> Result <Self>
	{
		let for_token = input . parse ()?;
		let mut generics: Generics = input . parse ()?;

		let content;
		let paren_token = parenthesized! (content in input);

		let struct_type = content . parse ()?;
		let comma_token = content . parse ()?;
		let scalar_type = content . parse ()?;

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
			scalar_type,
			in_token,
			algebra_type
		};

		Ok (output)
	}
}

impl ToTokens for DefScalarMarkerTrait
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
				self . scalar_type . to_tokens (inner_tokens);
			}
		);

		self . in_token . to_tokens (tokens);
		self . algebra_type . to_tokens (tokens);

		self . generics . where_clause . to_tokens (tokens);
	}
}

fn try_def_scalar_marker_trait_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_marker: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let
	(
		(algebra_item, struct_item),
		DefScalarMarkerTrait {generics, struct_type, scalar_type, algebra_type, ..}
	):
		((DeriveInput, ItemStruct), DefScalarMarkerTrait)
		= macrospace::parse_args! (2, input)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item . clone (), &algebra_type)?;

	let (_, substituted_struct_item) =
		substitute_arguments_for_struct (struct_item, &struct_type)?;

	let member_types = get_member_types (&substituted_struct_item . fields);

	let algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&algebra_item,
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions
	)?;

	let (_, member_algebra_types) = algebra_mapping . into_parts ();

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

	let tokens = def_scalar_marker_trait_inner
	(
		pascal_marker,
		algebra_type,
		struct_type,
		scalar_type,
		generics,
		member_algebras
	);

	Ok (tokens)
}

pub fn def_scalar_marker_trait_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_marker: Ident
)
-> proc_macro::TokenStream
{
	try_def_scalar_marker_trait_inner_impl (input, pascal_marker)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_scalar_def_scalar_marker_trait_impl
(
	input: proc_macro::TokenStream,
	snake_marker: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let def_scalar_marker_trait = parse (input)?;

	let DefScalarMarkerTrait
	{
		struct_type,
		algebra_type,
		..
	}
		= &def_scalar_marker_trait;

	let struct_type = without_arguments (struct_type . clone ());
	let algebra_type = without_arguments (algebra_type . clone ());

	let inner_macro_ident = format_ident! ("__def_scalar_{}_trait_inner", snake_marker);

	let inner_macro_path =
		parse_quote! (numeric_algebras::derive::#inner_macro_ident);

	let tokens = macrospace::generate_macrospace_invokation
	(
		inner_macro_path,
		[
			parse_quote! (#algebra_type: struct | enum),
			parse_quote! (#struct_type: struct)
		],
		def_scalar_marker_trait
	);

	Ok (tokens)
}

pub fn def_scalar_marker_trait_impl
(
	input: proc_macro::TokenStream,
	snake_marker: Ident
)
-> proc_macro::TokenStream
{
	try_scalar_def_scalar_marker_trait_impl (input, snake_marker)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_scalar_marker_trait_macro
{
	($pascal_marker: ident, $snake_marker: ident) =>
	{
		paste::paste!
		{
			#[doc (hidden)]
			#[proc_macro]
			pub fn [<__def_scalar_ $snake_marker _trait_inner>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::scalar_marker::def_scalar_marker_trait_inner_impl
				(
					input,
					syn::parse_quote! ($pascal_marker)
				)
			}

			#[proc_macro]
			pub fn [<def_scalar_ $snake_marker _trait>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::scalar_marker::def_scalar_marker_trait_impl
				(
					input,
					syn::parse_quote! ($snake_marker)
				)
			}
		}
	}
}

pub (crate) use __def_scalar_marker_trait_macro as def_scalar_marker_trait_macro;
