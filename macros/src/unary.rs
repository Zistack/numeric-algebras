use syn::{
	Ident,
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
	parse_quote
};
use syn::parse::{Result, Error};
use syn_derive::{Parse, ToTokens};
use quote::{ToTokens, quote, format_ident};

use macrospace::generics::combine_generics;
use macrospace::path_utils::without_arguments;
use macrospace::struct_utils::{constructor, get_members_and_types_split};
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::check_num_parts;
use numeric_algebras_core::algebra_mapping::AlgebraMapping;

fn def_unary_traits_inner
(
	pascal_op: Ident,
	snake_op: Ident,
	algebra_type: Path,
	input_type: Path,
	output_type: Path,
	mut generics: Generics,
	member_algebras: Vec <(Type, Type, Type)>,
	input_members: Vec <Member>,
	output_members: Vec <Member>,
	algebra_conversion_expressions: Vec <Expr>
)
-> proc_macro2::TokenStream
{
	let pascal_op_trait: Path =
		parse_quote! (numeric_algebras::traits::#pascal_op);

	generics
		. make_where_clause ()
		. predicates
		. push (parse_quote! (Self: Clone));

	let mut tokens = proc_macro2::TokenStream::new ();

	{
		let mut generics = generics . clone ();

		{
			let where_clause = generics . make_where_clause ();

			for (member_algebra_type, input_member_type, output_member_type)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_trait
						<
							#input_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_members
				. iter ()
				. zip (algebra_conversion_expressions . iter ())
				. map
				(
					|(member, algebra_conversion_expression)|
					parse_quote!
					(
						#algebra_conversion_expression (self . clone ())
							. #snake_op (x . #member)
					)
				)
				. collect::<Vec <Expr>> ()
		);

		quote!
		{
			#[automatically_derived]
			impl #impl_generics #pascal_op_trait <#input_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op (self, x: #input_type) -> Self::Output
				{
					#constructor
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	{
		let mut generics = generics . clone ();

		let lifetime_a = Lifetime::new ("'a", proc_macro2::Span::mixed_site ());

		generics . params . push (parse_quote! (#lifetime_a));

		{
			let where_clause = generics . make_where_clause ();

			for (member_algebra_type, input_member_type, output_member_type)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_trait
						<
							&#lifetime_a #input_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_members
				. iter ()
				. zip (algebra_conversion_expressions . iter ())
				. map
				(
					|(member, algebra_conversion_expression)|
					parse_quote!
					(
						#algebra_conversion_expression (self . clone ())
							. #snake_op (&x . #member)
					)
				)
				. collect::<Vec <Expr>> ()
		);

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <&#lifetime_a #input_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op (self, x: &#lifetime_a #input_type)
				-> Self::Output
				{
					#constructor
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	tokens
}

#[derive (Parse, ToTokens)]
struct DefUnaryTraits
{
	for_token: Token! [for],
	generics: Generics,
	input_type: Path,
	arrow_token: Token! [->],
	output_type: Path,
	in_token: Token! [in],
	algebra_type: Path
}

fn try_def_unary_traits_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_op: Ident,
	snake_op: Ident
)
-> Result <proc_macro2::TokenStream>
{
	let
	(
		(algebra_item, input_item, output_item),
		DefUnaryTraits
		{
			generics,
			input_type,
			output_type,
			algebra_type,
			..
		}
	):
		((DeriveInput, ItemStruct, ItemStruct), DefUnaryTraits)
		= macrospace::parse_args! (3, input)?;

	check_num_parts
	(
		input_item . fields . len (),
		output_item . fields . len (),
		&input_type,
		&output_type,
		"Input",
		"output"
	)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item . clone (), &algebra_type)?;

	let (_, substituted_input_item) = substitute_arguments_for_struct
	(
		input_item,
		&input_type
	)?;

	let (input_members, input_member_types) =
		get_members_and_types_split (&substituted_input_item . fields);

	let (_, substituted_output_item) = substitute_arguments_for_struct
	(
		output_item,
		&output_type
	)?;

	let (output_members, output_member_types) =
		get_members_and_types_split (&substituted_output_item . fields);

	let algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&algebra_item,
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions
	)?;

	let (algebra_conversion_expressions, member_algebra_types) =
		algebra_mapping . into_parts ();

	let mut member_algebras = Vec::new ();

	for ((member_algebra_type, input_member_type), output_member_type)
	in member_algebra_types
		. into_iter ()
		. zip (input_member_types)
		. zip (output_member_types)
	{
		member_algebras . push
		((
			member_algebra_type,
			input_member_type,
			output_member_type
		));
	}

	let generics = combine_generics
	([
		generics,
		substituted_algebra_item . generics,
		substituted_input_item . generics,
		substituted_output_item . generics
	]);

	let tokens = def_unary_traits_inner
	(
		pascal_op,
		snake_op,
		algebra_type,
		input_type,
		output_type,
		generics,
		member_algebras,
		input_members,
		output_members,
		algebra_conversion_expressions
	);

	Ok (tokens)
}

pub fn def_unary_traits_inner_impl
(
	input: proc_macro::TokenStream,
	pascal_op: Ident,
	snake_op: Ident
)
-> proc_macro::TokenStream
{
	try_def_unary_traits_inner_impl (input, pascal_op, snake_op)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_def_unary_traits_impl (input: proc_macro::TokenStream, snake_op: Ident)
-> Result <proc_macro2::TokenStream>
{
	let def_unary_traits = parse (input)?;

	let DefUnaryTraits
	{
		input_type,
		output_type,
		algebra_type,
		..
	}
		= &def_unary_traits;

	let input_type = without_arguments (input_type . clone ());
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
			parse_quote! (#input_type: struct),
			parse_quote! (#output_type: struct)
		],
		def_unary_traits
	);

	Ok (tokens)
}

pub fn def_unary_traits_impl (input: proc_macro::TokenStream, snake_op: Ident)
-> proc_macro::TokenStream
{
	try_def_unary_traits_impl (input, snake_op)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_unary_traits_macro
{
	($pascal_op: ident, $snake_op: ident) =>
	{
		paste::paste!
		{
			#[doc (hidden)]
			#[proc_macro]
			pub fn [<__def_ $snake_op _traits_inner>] (input: proc_macro::TokenStream)
			-> proc_macro::TokenStream
			{
				crate::unary::def_unary_traits_inner_impl
				(
					input,
					syn::parse_quote! ($pascal_op),
					syn::parse_quote! ($snake_op)
				)
			}

			#[proc_macro]
			pub fn [<def_ $snake_op _traits>] (input: proc_macro::TokenStream)
			-> proc_macro::TokenStream
			{
				crate::unary::def_unary_traits_impl
				(
					input,
					syn::parse_quote! ($snake_op)
				)
			}
		}
	}
}

pub (crate) use __def_unary_traits_macro as def_unary_traits_macro;
