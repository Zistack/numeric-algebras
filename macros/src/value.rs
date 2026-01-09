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
use syn::parse::{Parse, ParseStream, Result, Error};
use quote::{ToTokens, quote, format_ident};

use macrospace::generics::combine_generics;
use macrospace::path_utils::without_arguments;
use macrospace::struct_utils::{constructor, get_members_and_types_split};
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

	generics
		. make_where_clause ()
		. predicates
		. push (parse_quote! (Self: std::clone::Clone));

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

	let mut values = Vec::new ();

	for algebra_conversion_expression in &algebra_conversion_expressions
	{
		values . push
		(
			quote!
			(
				#algebra_conversion_expression (self . clone ())
					. #snake_value ()
			)
		);
	}
	for _ in algebra_conversion_expressions . len () .. members . len ()
	{
		values . push (quote! (std::default::Default::default ()));
	}

	let constructor = constructor
	(
		&struct_type,
		&members,
		&values
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

	let (impl_generics, _, where_clause) = generics . split_for_impl ();

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

struct DefValueTrait
{
	for_token: Token! [for],
	generics: Generics,
	struct_type: Path,
	in_token: Token! [in],
	algebra_type: Path

	// where clause
}

impl Parse for DefValueTrait
{
	fn parse (input: ParseStream <'_>) -> Result <Self>
	{
		let for_token = input . parse ()?;
		let mut generics: Generics = input . parse ()?;
		let struct_type = input . parse ()?;

		let in_token = input . parse ()?;
		let algebra_type = input . parse ()?;

		generics . where_clause = input . parse ()?;

		let output = Self
		{
			for_token,
			generics,
			struct_type,
			in_token,
			algebra_type
		};

		Ok (output)
	}
}

impl ToTokens for DefValueTrait
{
	fn to_tokens (&self, tokens: &mut proc_macro2::TokenStream)
	{
		self . for_token . to_tokens (tokens);
		self . generics . to_tokens (tokens);
		self . struct_type . to_tokens (tokens);

		self . in_token . to_tokens (tokens);
		self . algebra_type . to_tokens (tokens);

		self . generics . where_clause . to_tokens (tokens);
	}
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
		substitute_arguments_for_derive_input (algebra_item . clone (), &algebra_type)?;

	let (_, substituted_struct_item) = substitute_arguments_for_struct
	(
		struct_item,
		&struct_type
	)?;

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
