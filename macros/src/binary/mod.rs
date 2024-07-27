mod struct_traits;
mod enum_traits;

use syn::{
	Ident,
	Path,
	Generics,
	DeriveInput,
	Data,
	ItemStruct,
	Token,
	parse,
	parse_quote
};
use syn::token::Paren;
use syn::parse::{Result, Error};
use syn_derive::{Parse, ToTokens};
use quote::format_ident;

use macrospace::generics::combine_generics;
use macrospace::struct_utils::get_member_types;
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

#[derive (Parse, ToTokens)]
struct DefBinaryTraits
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

	arrow_token: Token! [->],
	output_type: Path,

	in_token: Token! [in],
	algebra_type: Path
}

fn try_def_binary_traits_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_op: Ident,
	snake_op: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let
	(
		(
			algebra_item,
			lhs_item,
			rhs_item,
			output_item
		),
		DefBinaryTraits
		{
			generics,
			lhs_type,
			rhs_type,
			output_type,
			algebra_type,
			..
		}
	):
		(
			(DeriveInput, ItemStruct, DeriveInput, ItemStruct),
			DefBinaryTraits
		)
		= macrospace::parse_args! (4, input)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item, &algebra_type)?;

	let (_, substituted_lhs_item) = substitute_arguments_for_struct
	(
		lhs_item . clone (),
		&lhs_type
	)?;

	let (_, substituted_rhs_item) = substitute_arguments_for_derive_input
	(
		rhs_item . clone (),
		&rhs_type
	)?;

	let (_, substituted_output_item) = substitute_arguments_for_struct
	(
		output_item . clone (),
		&output_type
	)?;

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

	let output_member_types =
		get_member_types (&substituted_output_item . fields);

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

	check_num_parts
	(
		&lhs_members,
		&output_member_types,
		&lhs_item . fields,
		&output_item . fields,
		"Input",
		"output"
	)?;

	let mut output_members = Vec::new ();
	let mut member_algebras = Vec::new ();

	for
	(
		(
			(lhs_member_algebra_type, lhs_member_type),
			(rhs_part_algebra_type, rhs_part_type)
		),
		(output_member, output_member_type)
	)
	in lhs_member_algebras
		. into_iter ()
		. zip (rhs_part_algebras)
		. zip (output_member_types)
	{
		check_algebra_type_pair
		(
			&lhs_member_algebra_type,
			&rhs_part_algebra_type,
			"LHS",
			"RHS"
		)?;

		output_members . push (output_member);
		member_algebras . push
		((
			lhs_member_algebra_type,
			lhs_member_type,
			rhs_part_type,
			output_member_type
		));
	}

	// I want to check that the types match before checking that the expressions
	// match too.
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
		substituted_rhs_item . generics,
		substituted_output_item . generics
	]);

	let tokens = match rhs_parts
	{
		TypeParts::Struct (rhs_members) =>
		struct_traits::def_struct_binary_traits_inner
		(
			pascal_op,
			snake_op,
			algebra_type,
			lhs_type,
			rhs_type,
			output_type,
			generics,
			member_algebras,
			lhs_members,
			rhs_members,
			output_members,
			lhs_algebra_conversion_expressions
		),
		TypeParts::Enum (rhs_variants) =>
		enum_traits::def_enum_binary_traits_inner
		(
			pascal_op,
			snake_op,
			algebra_type,
			lhs_type,
			rhs_type,
			output_type,
			generics,
			member_algebras,
			lhs_members,
			rhs_variants,
			output_members,
			lhs_algebra_conversion_expressions
		)
	};

	Ok (tokens)
}

pub fn def_binary_traits_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_op: Ident,
	snake_op: Ident
)
-> proc_macro::TokenStream
{
	try_def_binary_traits_inner_impl (input, pascal_op, snake_op)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_def_binary_traits_impl
(
	input: proc_macro::TokenStream,
	snake_op: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let def_binary_traits = parse (input)?;

	let DefBinaryTraits
	{
		lhs_type,
		rhs_type,
		output_type,
		algebra_type,
		..
	}
		= &def_binary_traits;

	let inner_macro_ident = format_ident! ("__def_{}_traits_inner", snake_op);

	let inner_macro_path =
		parse_quote! (numeric_algebras::derive::#inner_macro_ident);

	let tokens = macrospace::generate_macrospace_invokation
	(
		inner_macro_path,
		[
			parse_quote! (#algebra_type: struct | enum),
			parse_quote! (#lhs_type: struct),
			parse_quote! (#rhs_type: struct | enum),
			parse_quote! (#output_type: struct)
		],
		def_binary_traits
	);

	Ok (tokens)
}

pub fn def_binary_traits_impl
(
	input: proc_macro::TokenStream,
	snake_op: Ident
)
-> proc_macro::TokenStream
{
	try_def_binary_traits_impl (input, snake_op)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_binary_traits_macro
{
	($pascal_op: ident, $snake_op: ident) =>
	{
		paste::paste!
		{
			#[doc (hidden)]
			#[proc_macro]
			pub fn [<__def_ $snake_op _traits_inner>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::binary::def_binary_traits_inner_impl
				(
					input,
					syn::parse_quote! ($pascal_op),
					syn::parse_quote! ($snake_op)
				)
			}

			#[proc_macro]
			pub fn [<def_ $snake_op _traits>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::binary::def_binary_traits_impl
				(
					input,
					syn::parse_quote! ($snake_op)
				)
			}
		}
	}
}

pub (crate) use __def_binary_traits_macro as def_binary_traits_macro;
