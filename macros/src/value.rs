use syn::{
	Ident,
	Path,
	Generics,
	Type,
	Member,
	Expr,
	DeriveInput,
	ItemStruct,
	Token,
	parse,
	parse_quote
};
use syn::punctuated::Punctuated;
use syn::parse::{Result, Error};
use syn_derive::{Parse, ToTokens};
use quote::{ToTokens, quote, format_ident};

use macrospace::generics::combine_generics;
use macrospace::path_utils::without_arguments;
use macrospace::struct_utils::constructor;
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::algebra_mapping::AlgebraMapping;

fn def_value_trait_inner <C>
(
	pascal_value: Ident,
	snake_value: Ident,
	algebra_type: Path,
	struct_type: Path,
	mut generics: Generics,
	member_algebras: Vec <(Type, Type)>,
	members: Vec <Member>,
	algebra_conversion_expressions: Vec <Expr>
)
-> proc_macro2::TokenStream
where C: Default + ToTokens
{
	let is_snake_value = format_ident! ("is_{}", snake_value);

	let pascal_value_trait: Path =
		parse_quote! (numeric_algebras::traits::#pascal_value);

	{
		let where_clause = generics . make_where_clause ();

		for (member_algebra_type, member_type) in &member_algebras
		{
			where_clause . predicates . push
			(
				parse_quote!
				(
					#member_algebra_type: #pascal_value_trait <#member_type>
				)
			);
		}
	}

	let (impl_generics, _, where_clause) = generics . split_for_impl ();

	let constructor = constructor
	(
		&parse_quote! (#struct_type),
		&members,
		&algebra_conversion_expressions
			. iter ()
			. map
			(
				|algebra_conversion_expression|
				parse_quote!
				(
					#algebra_conversion_expression (self . clone ())
						. #snake_value ()
				)
			)
			. collect::<Vec <Expr>> ()
	);

	let is_value_expr: Punctuated <Expr, C> = members
		. iter ()
		. zip (&algebra_conversion_expressions)
		. map::<Expr, _>
		(
			|(member, algebra_conversion_expression)| parse_quote!
			(
				#algebra_conversion_expression (self . clone ())
					. #is_snake_value (&x . #member)
			)
		)
		. collect ();

	quote!
	{
		#[automatically_derived]
		impl #impl_generics #pascal_value_trait <#struct_type> for #algebra_type
		#where_clause
		{
			fn #snake_value (self) -> #struct_type
			{
				#constructor
			}

			fn #is_snake_value (self, x: &#struct_type) -> bool
			{
				#is_value_expr
			}
		}
	}
}

#[derive (Parse, ToTokens)]
struct DefValueTrait
{
	for_token: Token! [for],
	generics: Generics,
	struct_type: Path,
	in_token: Token! [in],
	algebra_type: Path
}

fn try_def_value_trait_inner_impl <C>
(
	input: proc_macro::TokenStream,
	pascal_value: Ident,
	snake_value: Ident
)
-> Result <proc_macro2::TokenStream>
where C: Default + ToTokens
{
	let
	(
		(algebra_item, struct_item),
		DefValueTrait {generics, struct_type, algebra_type, ..}
	):
		((DeriveInput, ItemStruct), DefValueTrait)
		= macrospace::parse_args! (2, input)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item, &algebra_type)?;

	let (_, substituted_struct_item) = substitute_arguments_for_struct
	(
		struct_item,
		&struct_type
	)?;

	let algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions,
		&struct_type
	)?;

	let (member_algebras, members, algebra_conversion_expressions) =
		algebra_mapping . into_struct_parts (substituted_struct_item . fields)?;

	let generics = combine_generics
	([
		generics,
		substituted_algebra_item . generics,
		substituted_struct_item . generics
	]);

	let tokens = def_value_trait_inner::<C>
	(
		pascal_value,
		snake_value,
		algebra_type,
		struct_type,
		generics,
		member_algebras,
		members,
		algebra_conversion_expressions
	);

	Ok (tokens)
}

pub fn def_value_trait_inner_impl <C>
(
	input: proc_macro::TokenStream,
	pascal_value: Ident,
	snake_value: Ident
)
-> proc_macro::TokenStream
where C: Default + ToTokens
{
	try_def_value_trait_inner_impl::<C> (input, pascal_value, snake_value)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_def_value_trait_impl (input: proc_macro::TokenStream, snake_value: Ident)
-> Result <proc_macro2::TokenStream>
{
	let def_value_trait = parse (input)?;

	let DefValueTrait {struct_type, algebra_type, ..} = &def_value_trait;

	let struct_type = without_arguments (struct_type . clone ());
	let algebra_type = without_arguments (algebra_type . clone ());

	let inner_macro_ident = format_ident! ("__def_{}_trait_inner", snake_value);

	let inner_macro_path =
		parse_quote! (numeric_algebras::derive::#inner_macro_ident);

	let tokens = macrospace::generate_macrospace_invokation
	(
		inner_macro_path,
		[
			parse_quote! (#algebra_type: struct | enum),
			parse_quote! (#struct_type: struct)
		],
		def_value_trait
	);

	Ok (tokens)
}

pub fn def_value_trait_impl (input: proc_macro::TokenStream, snake_value: Ident)
-> proc_macro::TokenStream
{
	try_def_value_trait_impl (input, snake_value)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_value_trait_macro
{
	($pascal_value: ident, $snake_value: ident, $combinator: tt) =>
	{
		paste::paste!
		{
			#[doc (hidden)]
			#[proc_macro]
			pub fn [<__def_ $snake_value _trait_inner>]
			(
				input: proc_macro::TokenStream
			)
			-> proc_macro::TokenStream
			{
				crate::value
					::def_value_trait_inner_impl::<syn::Token! [$combinator]>
				(
					input,
					syn::parse_quote! ($pascal_value),
					syn::parse_quote! ($snake_value)
				)
			}

			#[proc_macro]
			pub fn [<def_ $snake_value _trait>] (input: proc_macro::TokenStream)
			-> proc_macro::TokenStream
			{
				crate::value::def_value_trait_impl
				(
					input,
					syn::parse_quote! ($snake_value)
				)
			}
		}
	}
}

pub (crate) use __def_value_trait_macro as def_value_trait_macro;
