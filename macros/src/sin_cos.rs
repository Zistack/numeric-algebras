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
use macrospace::struct_utils::{constructor, get_member_types};
use macrospace::substitute::{
	substitute_arguments_for_struct,
	substitute_arguments_for_derive_input
};

use numeric_algebras_core::algebra_mapping::AlgebraMapping;
use numeric_algebras_core::check_parts::check_num_parts;

fn def_sin_cos_traits_inner
(
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
	let sin_cos_trait: Path = parse_quote! (numeric_algebras::traits::SinCos);

	generics
		. make_where_clause ()
		. predicates
		. push (parse_quote! (Self: Clone));

	let sin_vars: Vec <Ident> = (0..(input_members . len ()))
		. map (|i| format_ident! ("sin{}", i))
		. collect ();

	let cos_vars: Vec <Ident> = (0..(input_members . len ()))
		. map (|i| format_ident! ("cos{}", i))
		. collect ();

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
						#member_algebra_type: #sin_cos_trait
						<
							#input_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		let sin_constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&sin_vars
		);

		let cos_constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&cos_vars
		);

		quote!
		{
			#[automatically_derived]
			impl #impl_generics #sin_cos_trait <#input_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn sin_cos (self, x: #input_type)
				-> (Self::Output, Self::Output)
				{
					#(let (#sin_vars, #cos_vars) =
						#algebra_conversion_expressions (self . clone ())
							. sin_cos (x . #input_members);)*


					(#sin_constructor, #cos_constructor)
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	{
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
						#member_algebra_type: #sin_cos_trait
						<
							&#lifetime_a #input_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		let sin_constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&sin_vars
		);

		let cos_constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&cos_vars
		);

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#sin_cos_trait <&#lifetime_a #input_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn sin_cos (self, x: &#lifetime_a #input_type)
				-> (Self::Output, Self::Output)
				{
					#(let (#sin_vars, #cos_vars) =
						#algebra_conversion_expressions (self . clone ())
							. sin_cos (&x . #input_members);)*


					(#sin_constructor, #cos_constructor)
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	tokens
}

#[derive (Parse, ToTokens)]
struct DefSinCosTraits
{
	for_token: Token! [for],
	generics: Generics,
	input_type: Path,
	arrow_token: Token! [->],
	output_type: Path,
	in_token: Token! [in],
	algebra_type: Path,
}

fn try_def_sin_cos_traits_inner_impl (input: proc_macro::TokenStream)
-> Result <proc_macro2::TokenStream>
{
	let
	(
		(algebra_item, input_item, output_item),
		DefSinCosTraits
		{
			generics,
			input_type,
			output_type,
			algebra_type,
			..
		}
	):
		((DeriveInput, ItemStruct, ItemStruct), DefSinCosTraits)
		= macrospace::parse_args! (3, input)?;

	let (mut algebra_substitutions, substituted_algebra_item) =
		substitute_arguments_for_derive_input (algebra_item, &algebra_type)?;

	let (_, substituted_input_item) = substitute_arguments_for_struct
	(
		input_item . clone (),
		&input_type
	)?;

	let (_, substituted_output_item) = substitute_arguments_for_struct
	(
		output_item . clone (),
		&output_type
	)?;

	let input_algebra_mapping = AlgebraMapping::get_from_attributes
	(
		&substituted_algebra_item . attrs,
		&mut algebra_substitutions,
		&input_type
	)?;

	let (input_member_algebras, input_members, algebra_conversion_expressions) =
		input_algebra_mapping
		. into_struct_parts (substituted_input_item . fields)?;

	let output_member_types = get_member_types
	(
		&substituted_output_item . fields
	);

	check_num_parts
	(
		&input_members,
		&output_member_types,
		&input_item . fields,
		&output_item . fields,
		"Input",
		"output"
	)?;

	let mut output_members = Vec::new ();
	let mut member_algebras = Vec::new ();

	for
	(
		(input_member_algebra_type, input_member_type),
		(output_member, output_member_type)
	)
	in input_member_algebras . into_iter () . zip (output_member_types)
	{
		output_members . push (output_member);
		member_algebras . push
		((
			input_member_algebra_type,
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

	let tokens = def_sin_cos_traits_inner
	(
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

pub fn def_sin_cos_traits_inner_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	try_def_sin_cos_traits_inner_impl (input)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

fn try_def_sin_cos_traits_impl (input: proc_macro::TokenStream)
-> Result <proc_macro2::TokenStream>
{
	let def_sin_cos_traits = parse (input)?;

	let DefSinCosTraits
	{
		input_type,
		output_type,
		algebra_type,
		..
	}
		= &def_sin_cos_traits;

	let input_type = without_arguments (input_type . clone ());
	let output_type = without_arguments (output_type . clone ());
	let algebra_type = without_arguments (algebra_type . clone ());

	let inner_macro_path =
		parse_quote! (numeric_algebras::derive::__def_sin_cos_traits_inner);

	let tokens = macrospace::generate_macrospace_invokation
	(
		inner_macro_path,
		[
			parse_quote! (#algebra_type: struct | enum),
			parse_quote! (#input_type: struct),
			parse_quote! (#output_type: struct)
		],
		def_sin_cos_traits
	);

	Ok (tokens)
}

pub fn def_sin_cos_traits_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	try_def_sin_cos_traits_impl (input)
		. unwrap_or_else (Error::into_compile_error)
		. into ()
}

macro_rules! __def_sin_cos_traits_macro
{
	() =>
	{
		#[doc (hidden)]
		#[proc_macro]
		pub fn __def_sin_cos_traits_inner (input: proc_macro::TokenStream)
		-> proc_macro::TokenStream
		{
			crate::sin_cos::def_sin_cos_traits_inner_impl (input)
		}

		#[proc_macro]
		pub fn def_sin_cos_traits (input: proc_macro::TokenStream)
		-> proc_macro::TokenStream
		{
			crate::sin_cos::def_sin_cos_traits_impl (input)
		}
	}
}

pub (crate) use __def_sin_cos_traits_macro as def_sin_cos_traits_macro;
