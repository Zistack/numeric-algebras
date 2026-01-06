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
	parenthesized,
	parse,
	parse_quote
};
use syn::token::Paren;
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, format_ident};

use macrospace::enum_utils::{get_variant_idents, get_variant_types};
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

struct DefBinaryTraits
{
	for_token: Token! [for],
	generics: Generics,

	paren_token: Paren,
	lhs_type: Path,
	comma_token: Token! [,],
	rhs_type: Path,

	arrow_token: Token! [->],
	output_type: Path,

	in_token: Token! [in],
	algebra_type: Path
}

impl Parse for DefBinaryTraits
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

		let arrow_token = input . parse ()?;
		let output_type = input . parse ()?;

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
			arrow_token,
			output_type,
			in_token,
			algebra_type
		};

		Ok (output)
	}
}

impl ToTokens for DefBinaryTraits
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

		self . arrow_token . to_tokens (tokens);
		self . output_type . to_tokens (tokens);

		self . in_token . to_tokens (tokens);
		self . algebra_type . to_tokens (tokens);

		self . generics . where_clause . to_tokens (tokens);
	}
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
		}
		_ => unreachable! ()
	};

	let (_, substituted_output_item) = substitute_arguments_for_struct
	(
		output_item,
		&output_type
	)?;

	let (output_members, output_member_types) =
		get_members_and_types_split (&substituted_output_item . fields);

	check_num_parts
	(
		substituted_lhs_item . fields . len (),
		substituted_output_item . fields . len (),
		&lhs_type,
		&output_type,
		"Input",
		"output"
	)?;

	let algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&algebra_item,
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions
	)?;

	let (algebra_conversion_expressions, member_algebra_types) =
		algebra_mapping . into_parts ();

	let mut member_algebras = Vec::new ();

	for (((member_algebra_type, lhs_member_type), rhs_part_type), output_member_type)
	in member_algebra_types
		. into_iter ()
		. zip (lhs_member_types)
		. zip (rhs_part_types)
		. zip (output_member_types)
	{
		member_algebras . push
		((
			member_algebra_type,
			lhs_member_type,
			rhs_part_type,
			output_member_type
		));
	}

	let generics = combine_generics
	([
		generics,
		substituted_algebra_item . generics,
		substituted_lhs_item . generics,
		substituted_rhs_item . generics,
		substituted_output_item . generics
	]);

	let tokens = match &substituted_rhs_item . data
	{
		Data::Struct (struct_data) =>
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
			get_members (&struct_data . fields),
			output_members,
			algebra_conversion_expressions
		),
		Data::Enum (enum_data) =>
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
			get_variant_idents (&enum_data . variants),
			output_members,
			algebra_conversion_expressions
		),
		_ => unreachable! ()
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

	let lhs_type = without_arguments (lhs_type . clone ());
	let rhs_type = without_arguments (rhs_type . clone ());
	let output_type = without_arguments (output_type . clone ());
	let algebra_type = without_arguments (algebra_type . clone ());

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
