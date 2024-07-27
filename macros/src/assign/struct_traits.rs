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

pub fn def_struct_assign_traits_inner
(
	pascal_op: Ident,
	snake_op: Ident,
	algebra_type: Path,
	lhs_struct_type: Path,
	rhs_struct_type: Path,
	mut generics: Generics,
	member_algebras: Vec <(Type, Type, Type)>,
	lhs_members: Vec <Member>,
	rhs_members: Vec <Member>,
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

	let mut tokens = proc_macro2::TokenStream::new ();

	// Assign Ops

	{
		let mut generics = generics . clone ();

		{
			let where_clause = generics . make_where_clause ();

			for
			(
				member_algebra_type,
				lhs_member_type,
				rhs_member_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_assign_trait
						<
							#lhs_member_type,
							#rhs_member_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		let op_assign_exprs = &lhs_members
			. iter ()
			. zip (&rhs_members)
			. zip (algebra_conversion_expressions . iter ())
			. map
			(
				|((lhs_member, rhs_member), algebra_conversion_expression)|
				parse_quote!
				(
					#algebra_conversion_expression (self . clone ())
						. #snake_op_assign
						(
							&mut lhs . #lhs_member,
							rhs . #rhs_member
						)
				)
			)
			. collect::<Vec <Expr>> ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_assign_trait <#lhs_struct_type, #rhs_struct_type>
			for #algebra_type
			#where_clause
			{
				fn #snake_op_assign
				(
					self,
					lhs: &mut #lhs_struct_type,
					rhs: #rhs_struct_type
				)
				{
					#(#op_assign_exprs;)*
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
				lhs_member_type,
				rhs_member_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_assign_trait
						<
							#lhs_member_type,
							&#lifetime_a #rhs_member_type
						>
					)
				);
			}
		}

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		let op_assign_exprs = &lhs_members
			. iter ()
			. zip (&rhs_members)
			. zip (algebra_conversion_expressions . iter ())
			. map
			(
				|((lhs_member, rhs_member), algebra_conversion_expression)|
				parse_quote!
				(
					#algebra_conversion_expression (self . clone ())
						. #snake_op_assign
						(
							&mut lhs . #lhs_member,
							&rhs . #rhs_member
						)
				)
			)
			. collect::<Vec <Expr>> ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_assign_trait
			<
				#lhs_struct_type,
				&#lifetime_a #rhs_struct_type
			>
			for #algebra_type
			#where_clause
			{
				fn #snake_op_assign
				(
					self,
					lhs: &mut #lhs_struct_type,
					rhs: &#lifetime_a #rhs_struct_type
				)
				{
					#(#op_assign_exprs;)*
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	tokens
}
