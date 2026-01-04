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

use macrospace::enum_utils::{get_variants, get_variant_types};
use macrospace::generics::combine_generics;
use macrospace::path_utils::without_arguments;
use macrospace::struct_utils::{
	get_members,
	get_member_types,
	get_members_and_types_split
};
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::check_num_parts;
use numeric_algebras_core::algebra_mapping::AlgebraMapping;

#[derive (Parse, ToTokens)]
struct DefAssignTraits
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

fn try_def_assign_traits_inner_impl
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
		),
		DefAssignTraits
		{
			generics,
			lhs_type,
			rhs_type,
			algebra_type,
			..
		}
	):
		(
			(DeriveInput, ItemStruct, DeriveInput),
			DefAssignTraits
		)
		= macrospace::parse_args! (3, input)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item . clone (), &algebra_type)?;

	let (_, substituted_lhs_item) = substitute_arguments_for_struct
	(
		lhs_item,
		&lhs_type
	)?;

	let (lhs_members, lhs_member_types) =
		get_members_and_types_split (&substituted_lhs_item . fields);

	let (_, substituted_rhs_item) = substitute_arguments_for_derive_input
	(
		rhs_item,
		&rhs_type
	)?;

	let rhs_part_types = match &substituted_rhs_item . data
	{
		Data::Struct (struct_data) =>
		{
			check_num_parts
			(
				substituted_lhs_item . fields . len (),
				struct_data . fields . len (),
				&lhs_type,
				&rhs_type,
				"LHS",
				"RHS"
			)?;

			get_member_types (&struct_data . fields)
		},
		Data::Enum (enum_data) =>
		{
			check_num_parts
			(
				substituted_lhs_item . fields . len (),
				enum_data . variants . len (),
				&lhs_type,
				&rhs_type,
				"LHS",
				"RHS"
			)?;

			get_variant_types (&enum_data . variants)?
		},
		_ => unreachable! ()
	};

	let algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&algebra_item,
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions
	)?;

	let (algebra_conversion_expressions, member_algebra_types) =
		algebra_mapping . into_parts ();

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
			rhs_part_type,
		));
	}

	let generics = combine_generics
	([
		generics,
		substituted_algebra_item . generics,
		substituted_lhs_item . generics,
		substituted_rhs_item . generics
	]);

	let tokens = match &substituted_rhs_item . data
	{
		Data::Struct (struct_data) =>
		struct_traits::def_struct_assign_traits_inner
		(
			pascal_op,
			snake_op,
			algebra_type,
			lhs_type,
			rhs_type,
			generics,
			member_algebras,
			lhs_members,
			get_members (&struct_data . fields),
			algebra_conversion_expressions
		),
		Data::Enum (enum_data) =>
		enum_traits::def_enum_assign_traits_inner
		(
			pascal_op,
			snake_op,
			algebra_type,
			lhs_type,
			rhs_type,
			generics,
			member_algebras,
			lhs_members,
			get_variants (&enum_data . variants),
			algebra_conversion_expressions
		),
		_ => unreachable! ()
	};

	Ok (tokens)
}

pub fn def_assign_traits_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_op: Ident,
	snake_op: Ident
)
-> proc_macro::TokenStream
{
	try_def_assign_traits_inner_impl (input, pascal_op, snake_op)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_def_assign_traits_impl
(
	input: proc_macro::TokenStream,
	snake_op: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let def_assign_traits = parse (input)?;

	let DefAssignTraits
	{
		lhs_type,
		rhs_type,
		algebra_type,
		..
	}
		= &def_assign_traits;

	let lhs_type = without_arguments (lhs_type . clone ());
	let rhs_type = without_arguments (rhs_type . clone ());
	let algebra_type = without_arguments (algebra_type . clone ());

	let inner_macro_ident =
		format_ident! ("__def_{}_assign_traits_inner", snake_op);

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
		def_assign_traits
	);

	Ok (tokens)
}

pub fn def_assign_traits_impl
(
	input: proc_macro::TokenStream,
	snake_op: Ident
)
-> proc_macro::TokenStream
{
	try_def_assign_traits_impl (input, snake_op)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_assign_traits_macro
{
	($pascal_op: ident, $snake_op: ident) =>
	{
		paste::paste!
		{
			#[doc (hidden)]
			#[proc_macro]
			pub fn [<__def_ $snake_op _assign_traits_inner>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::assign::def_assign_traits_inner_impl
				(
					input,
					syn::parse_quote! ($pascal_op),
					syn::parse_quote! ($snake_op)
				)
			}

			#[proc_macro]
			pub fn [<def_ $snake_op _assign_traits>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::assign::def_assign_traits_impl
				(
					input,
					syn::parse_quote! ($snake_op)
				)
			}
		}
	}
}

pub (crate) use __def_assign_traits_macro as def_assign_traits_macro;
