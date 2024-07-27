use syn::{
	Ident,
	Path,
	Generics,
	Type,
	DeriveInput,
	ItemStruct,
	Token,
	parse,
	parse_quote
};
use syn::token::Paren;
use syn::parse::{Result, Error};
use syn_derive::{Parse, ToTokens};
use quote::{format_ident, quote};

use macrospace::generics::combine_generics;
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

#[derive (Parse, ToTokens)]
struct DefScalarMarkerTrait
{
	for_token: Token! [for],
	generics: Generics,

	#[syn (parenthesized)]
	paren_token: Paren,
	#[syn (in = paren_token)]
	struct_type: Path,
	#[syn (in = paren_token)]
	comma_token: Token! [,],
	#[syn (in = paren_token)]
	scalar_type: Path,

	in_token: Token! [in],
	algebra_type: Path
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
		substitute_arguments_for_derive_input (algebra_item, &algebra_type)?;

	let (_, substituted_struct_item) =
		substitute_arguments_for_struct (struct_item . clone (), &struct_type)?;

	let struct_algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions,
		&struct_type
	)?;

	let (struct_member_algebras, _, _) = struct_algebra_mapping
		. into_struct_parts (substituted_struct_item . fields)?;

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
		struct_member_algebras
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
