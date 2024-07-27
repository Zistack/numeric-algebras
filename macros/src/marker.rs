use syn::{
	Ident,
	Path,
	Generics,
	Type,
	DeriveInput,
	ItemStruct,
	Data,
	Token,
	parse,
	parse_quote
};
use syn::token::Paren;
use syn::parse::{Result, Error};
use syn_derive::{Parse, ToTokens};
use quote::{ToTokens, format_ident, quote};

use macrospace::generics::combine_generics;
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::algebra_mapping::{AlgebraMapping, TypeParts};
use numeric_algebras_core::check_parts::{
	check_num_parts,
	check_algebra_type_pair,
	check_algebra_conversion_expression_pairs
};

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

#[derive (Parse, ToTokens)]
struct DefMarkerTrait
{
	for_token: Token! [for],
	generics: Generics,

	#[syn (parenthesized)]
	paren_token: Paren,
	#[syn (in = paren_token)]
	lhs_type: Path,
	#[syn (in = paren_token)]
	comma_token: Token! [,],
	#[syn (in = paren_token)]
	rhs_type: Path,

	in_token: Token! [in],
	algebra_type: Path
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
		substitute_arguments_for_derive_input (algebra_item, &algebra_type)?;

	let (_, substituted_lhs_item) =
		substitute_arguments_for_struct (lhs_item . clone (), &lhs_type)?;

	let (_, substituted_rhs_item) =
		substitute_arguments_for_derive_input (rhs_item . clone (), &rhs_type)?;

	let lhs_algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions,
		&lhs_type
	)?;

	let rhs_algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions,
		&rhs_type
	)?;

	let (lhs_member_algebras, lhs_members, lhs_algebra_conversion_expressions) =
		lhs_algebra_mapping . into_struct_parts (substituted_lhs_item . fields)?;

	let (rhs_part_algebras, rhs_parts, rhs_algebra_conversion_expressions) =
		rhs_algebra_mapping . into_parts (substituted_rhs_item . clone ())?;

	match (&rhs_parts, &rhs_item . data)
	{
		(TypeParts::Struct (rhs_members), Data::Struct (struct_data)) => check_num_parts
		(
			&lhs_members,
			&rhs_members,
			&lhs_item . fields,
			&struct_data . fields,
			"LHS",
			"RHS"
		)?,
		(TypeParts::Enum (rhs_variants), Data::Enum (enum_data)) => check_num_parts
		(
			&lhs_members,
			&rhs_variants,
			&lhs_item . fields,
			&enum_data . variants,
			"LHS",
			"RHS"
		)?,
		_ => unreachable! ()
	}

	let mut member_algebras = Vec::new ();

	for
	(
		(lhs_member_algebra_type, lhs_member_type),
		(rhs_part_algebra_type, rhs_part_type)
	)
	in lhs_member_algebras . into_iter () . zip (rhs_part_algebras)
	{
		check_algebra_type_pair
		(
			&lhs_member_algebra_type,
			&rhs_part_algebra_type,
			"LHS",
			"RHS"
		)?;

		member_algebras . push
		((
			lhs_member_algebra_type,
			lhs_member_type,
			rhs_part_type
		));
	}

	check_algebra_conversion_expression_pairs
	(
		&lhs_algebra_conversion_expressions,
		&rhs_algebra_conversion_expressions,
		"LHS",
		"RHS"
	)?;

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
