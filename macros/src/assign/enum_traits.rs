use syn::{
	Ident,
	Type,
	Path,
	Generics,
	Member,
	Expr,
	Lifetime,
	parse_quote
};
use quote::{ToTokens, quote, format_ident};

use macrospace::path_utils::as_prefix;

pub fn def_enum_assign_traits_inner
(
	pascal_op: Ident,
	snake_op: Ident,
	algebra_type: Path,
	input_struct_type: Path,
	input_enum_type: Path,
	mut generics: Generics,
	member_algebras: Vec <(Type, Type, Type)>,
	input_struct_members: Vec <Member>,
	input_enum_variants: Vec <Ident>,
	algebra_conversion_expressions: Vec <Expr>
)
-> proc_macro2::TokenStream
{
	let pascal_op_assign = format_ident! ("{}Assign", pascal_op);

	let snake_op_assign = format_ident! ("{}_assign", snake_op);

	let pascal_op_assign_trait: Path =
		parse_quote! (numeric_algebras::traits::#pascal_op_assign);

	generics
		. make_where_clause ()
		. predicates
		. push (parse_quote! (Self: Clone));

	let x = Ident::new ("x", proc_macro2::Span::mixed_site ());

	let var_names: Vec <Ident> = (0..(input_struct_members . len ()))
		. map (|i| format_ident! ("{}{}", x, i))
		. collect ();

	let input_enum_type_prefix = as_prefix (input_enum_type . clone ());

	let mut tokens = proc_macro2::TokenStream::new ();

	// Assign Ops

	{
		let mut generics = generics . clone ();

		{
			let where_clause = generics . make_where_clause ();

			for
			(
				member_algebra_type,
				input_struct_type,
				input_enum_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_assign_trait
						<
							#input_struct_type,
							#input_enum_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_assign_trait <#input_struct_type, #input_enum_type>
			for #algebra_type
			#where_clause
			{
				fn #snake_op_assign
				(
					self,
					lhs: &mut #input_struct_type,
					rhs: #input_enum_type
				)
				{
					match rhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variants (#var_names) =>
							#algebra_conversion_expressions (self)
							. #snake_op_assign
							(
								&mut lhs . #input_struct_members,
								#var_names
							)),*
					}
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	{
		let mut generics = generics . clone ();

		let lifetime_a =
			Lifetime::new ("'a", proc_macro2::Span::mixed_site ());

		generics . params . push (parse_quote! (#lifetime_a));

		{
			let where_clause = generics . make_where_clause ();

			for
			(
				member_algebra_type,
				input_struct_type,
				input_enum_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_assign_trait
						<
							#input_struct_type,
							&#lifetime_a #input_enum_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_assign_trait
			<
				#input_struct_type,
				&#lifetime_a #input_enum_type
			>
			for #algebra_type
			#where_clause
			{
				fn #snake_op_assign
				(
					self,
					lhs: &mut #input_struct_type,
					rhs: &#lifetime_a #input_enum_type
				)
				{
					match rhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variants (#var_names) =>
							#algebra_conversion_expressions (self)
							. #snake_op_assign
							(
								&mut lhs . #input_struct_members,
								#var_names
							)),*
					}
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	tokens
}
