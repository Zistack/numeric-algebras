use std::iter::zip;

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
use quote::{ToTokens, quote};

use macrospace::struct_utils::constructor;

pub fn def_struct_binary_traits_inner
(
	pascal_op: Ident,
	snake_op: Ident,
	algebra_type: Path,
	lhs_struct_type: Path,
	rhs_struct_type: Path,
	output_struct_type: Path,
	mut generics: Generics,
	member_algebras: Vec <(Type, Type, Type, Type)>,
	lhs_members: Vec <Member>,
	rhs_members: Vec <Member>,
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

	// Ops

	{
		let mut generics = generics . clone ();

		{
			let where_clause = generics . make_where_clause ();

			for
			(
				member_algebra_type,
				lhs_member_type,
				rhs_member_type,
				output_member_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_trait
						<
							#lhs_member_type,
							#rhs_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let mut output_values = Vec::new ();

		for ((lhs_member, rhs_member), algebra_conversion_expression)
		in zip (zip (&lhs_members, &rhs_members), &algebra_conversion_expressions)
		{
			output_values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. #snake_op (lhs . #lhs_member, rhs . #rhs_member)
				)
			);
		}
		for _
		in algebra_conversion_expressions . len () .. output_members . len ()
		{
			output_values . push (quote! (std::default::Default::default ()));
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&output_values
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <#lhs_struct_type, #rhs_struct_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_struct_type;

				fn #snake_op
				(
					self,
					lhs: #lhs_struct_type,
					rhs: #rhs_struct_type
				)
				-> Self::Output
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

			for
			(
				member_algebra_type,
				lhs_member_type,
				rhs_member_type,
				output_member_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_trait
						<
							&#lifetime_a #lhs_member_type,
							#rhs_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let mut output_values = Vec::new ();

		for ((lhs_member, rhs_member), algebra_conversion_expression)
		in zip (zip (&lhs_members, &rhs_members), &algebra_conversion_expressions)
		{
			output_values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. #snake_op (&lhs . #lhs_member, rhs . #rhs_member)
				)
			);
		}
		for _
		in algebra_conversion_expressions . len () .. output_members . len ()
		{
			output_values . push (quote! (std::default::Default::default ()));
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&output_values
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <&#lifetime_a #lhs_struct_type, #rhs_struct_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_struct_type;

				fn #snake_op
				(
					self,
					lhs: &#lifetime_a #lhs_struct_type,
					rhs: #rhs_struct_type
				)
				-> Self::Output
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

			for
			(
				member_algebra_type,
				lhs_member_type,
				rhs_member_type,
				output_member_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_trait
						<
							#lhs_member_type,
							&#lifetime_a #rhs_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let mut output_values = Vec::new ();

		for ((lhs_member, rhs_member), algebra_conversion_expression)
		in zip (zip (&lhs_members, &rhs_members), &algebra_conversion_expressions)
		{
			output_values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. #snake_op (lhs . #lhs_member, &rhs . #rhs_member)
				)
			);
		}
		for _
		in algebra_conversion_expressions . len () .. output_members . len ()
		{
			output_values . push (quote! (std::default::Default::default ()));
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&output_values
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <#lhs_struct_type, &#lifetime_a #rhs_struct_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_struct_type;

				fn #snake_op
				(
					self,
					lhs: #lhs_struct_type,
					rhs: &#lifetime_a #rhs_struct_type
				)
				-> Self::Output
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
		let lifetime_b = Lifetime::new ("'b", proc_macro2::Span::mixed_site ());

		generics . params . push (parse_quote! (#lifetime_a));
		generics . params . push (parse_quote! (#lifetime_b));

		{
			let where_clause = generics . make_where_clause ();

			for
			(
				member_algebra_type,
				lhs_member_type,
				rhs_member_type,
				output_member_type
			)
			in &member_algebras
			{
				where_clause . predicates . push
				(
					parse_quote!
					(
						#member_algebra_type: #pascal_op_trait
						<
							&#lifetime_a #lhs_member_type,
							&#lifetime_b #rhs_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let mut output_values = Vec::new ();

		for ((lhs_member, rhs_member), algebra_conversion_expression)
		in zip (zip (&lhs_members, &rhs_members), &algebra_conversion_expressions)
		{
			output_values . push
			(
				quote!
				(
					#algebra_conversion_expression (self . clone ())
						. #snake_op (&lhs . #lhs_member, &rhs . #rhs_member)
				)
			);
		}
		for _
		in algebra_conversion_expressions . len () .. output_members . len ()
		{
			output_values . push (quote! (std::default::Default::default ()));
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&output_values
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait
			<
				&#lifetime_a #lhs_struct_type,
				&#lifetime_b #rhs_struct_type
			>
			for #algebra_type
			#where_clause
			{
				type Output = #output_struct_type;

				fn #snake_op
				(
					self,
					lhs: &#lifetime_a #lhs_struct_type,
					rhs: &#lifetime_b #rhs_struct_type
				)
				-> Self::Output
				{
					#constructor
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	// Symmetric Ops

	if lhs_struct_type != rhs_struct_type
	{
		{
			let mut generics = generics . clone ();

			{
				let where_clause = generics . make_where_clause ();

				for
				(
					member_algebra_type,
					lhs_member_type,
					rhs_member_type,
					output_member_type
				)
				in &member_algebras
				{
					where_clause . predicates . push
					(
						parse_quote!
						(
							#member_algebra_type: #pascal_op_trait
							<
								#rhs_member_type,
								#lhs_member_type,
								Output = #output_member_type
							>
						)
					);
				}
			}

			let mut output_values = Vec::new ();

			for ((rhs_member, lhs_member), algebra_conversion_expression)
			in zip (zip (&rhs_members, &lhs_members), &algebra_conversion_expressions)
			{
				output_values . push
				(
					quote!
					(
						#algebra_conversion_expression (self . clone ())
							. #snake_op (lhs . #rhs_member, rhs . #lhs_member)
					)
				);
			}
			for _
			in algebra_conversion_expressions . len () .. output_members . len ()
			{
				output_values . push (quote! (std::default::Default::default ()));
			}

			let constructor = constructor
			(
				&parse_quote! (Self::Output),
				&output_members,
				&output_values
			);

			let (impl_generics, _, where_clause) = generics . split_for_impl ();

			quote!
			{
				#[automatically_derived]
				impl #impl_generics
				#pascal_op_trait <#rhs_struct_type, #lhs_struct_type>
				for #algebra_type
				#where_clause
				{
					type Output = #output_struct_type;

					fn #snake_op
					(
						self,
						lhs: #rhs_struct_type,
						rhs: #lhs_struct_type
					)
					-> Self::Output
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

				for
				(
					member_algebra_type,
					lhs_member_type,
					rhs_member_type,
					output_member_type
				)
				in &member_algebras
				{
					where_clause . predicates . push
					(
						parse_quote!
						(
							#member_algebra_type: #pascal_op_trait
							<
								&#lifetime_a #rhs_member_type,
								#lhs_member_type,
								Output = #output_member_type
							>
						)
					);
				}
			}

			let mut output_values = Vec::new ();

			for ((rhs_member, lhs_member), algebra_conversion_expression)
			in zip (zip (&rhs_members, &lhs_members), &algebra_conversion_expressions)
			{
				output_values . push
				(
					quote!
					(
						#algebra_conversion_expression (self . clone ())
							. #snake_op (&lhs . #rhs_member, rhs . #lhs_member)
					)
				);
			}
			for _
			in algebra_conversion_expressions . len () .. output_members . len ()
			{
				output_values . push (quote! (std::default::Default::default ()));
			}

			let constructor = constructor
			(
				&parse_quote! (Self::Output),
				&output_members,
				&output_values
			);

			let (impl_generics, _, where_clause) = generics . split_for_impl ();

			quote!
			{
				#[automatically_derived]
				impl #impl_generics
				#pascal_op_trait <&#lifetime_a #rhs_struct_type, #lhs_struct_type>
				for #algebra_type
				#where_clause
				{
					type Output = #output_struct_type;

					fn #snake_op
					(
						self,
						lhs: &#lifetime_a #rhs_struct_type,
						rhs: #lhs_struct_type
					)
					-> Self::Output
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

				for
				(
					member_algebra_type,
					lhs_member_type,
					rhs_member_type,
					output_member_type
				)
				in &member_algebras
				{
					where_clause . predicates . push
					(
						parse_quote!
						(
							#member_algebra_type: #pascal_op_trait
							<
								#rhs_member_type,
								&#lifetime_a #lhs_member_type,
								Output = #output_member_type
							>
						)
					);
				}
			}

			let mut output_values = Vec::new ();

			for ((rhs_member, lhs_member), algebra_conversion_expression)
			in zip (zip (&rhs_members, &lhs_members), &algebra_conversion_expressions)
			{
				output_values . push
				(
					quote!
					(
						#algebra_conversion_expression (self . clone ())
							. #snake_op (lhs . #rhs_member, &rhs . #lhs_member)
					)
				);
			}
			for _
			in algebra_conversion_expressions . len () .. output_members . len ()
			{
				output_values . push (quote! (std::default::Default::default ()));
			}

			let constructor = constructor
			(
				&parse_quote! (Self::Output),
				&output_members,
				&output_values
			);

			let (impl_generics, _, where_clause) = generics . split_for_impl ();

			quote!
			{
				#[automatically_derived]
				impl #impl_generics
				#pascal_op_trait <#rhs_struct_type, &#lifetime_a #lhs_struct_type>
				for #algebra_type
				#where_clause
				{
					type Output = #output_struct_type;

					fn #snake_op
					(
						self,
						lhs: #rhs_struct_type,
						rhs: &#lifetime_a #lhs_struct_type
					)
					-> Self::Output
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
			let lifetime_b = Lifetime::new ("'b", proc_macro2::Span::mixed_site ());

			generics . params . push (parse_quote! (#lifetime_a));
			generics . params . push (parse_quote! (#lifetime_b));

			{
				let where_clause = generics . make_where_clause ();

				for
				(
					member_algebra_type,
					lhs_member_type,
					rhs_member_type,
					output_member_type
				)
				in &member_algebras
				{
					where_clause . predicates . push
					(
						parse_quote!
						(
							#member_algebra_type: #pascal_op_trait
							<
								&#lifetime_a #rhs_member_type,
								&#lifetime_b #lhs_member_type,
								Output = #output_member_type
							>
						)
					);
				}
			}

			let mut output_values = Vec::new ();

			for ((rhs_member, lhs_member), algebra_conversion_expression)
			in zip (zip (&rhs_members, &lhs_members), &algebra_conversion_expressions)
			{
				output_values . push
				(
					quote!
					(
						#algebra_conversion_expression (self . clone ())
							. #snake_op (&lhs . #rhs_member, &rhs . #lhs_member)
					)
				);
			}
			for _
			in algebra_conversion_expressions . len () .. output_members . len ()
			{
				output_values . push (quote! (std::default::Default::default ()));
			}

			let constructor = constructor
			(
				&parse_quote! (Self::Output),
				&output_members,
				&output_values
			);

			let (impl_generics, _, where_clause) = generics . split_for_impl ();

			quote!
			{
				#[automatically_derived]
				impl #impl_generics
				#pascal_op_trait
				<
					&#lifetime_a #rhs_struct_type,
					&#lifetime_b #lhs_struct_type
				>
				for #algebra_type
				#where_clause
				{
					type Output = #output_struct_type;

					fn #snake_op
					(
						self,
						lhs: &#lifetime_a #rhs_struct_type,
						rhs: &#lifetime_b #lhs_struct_type
					)
					-> Self::Output
					{
						#constructor
					}
				}
			}
				. to_tokens (&mut tokens);
		}
	}

	tokens
}
