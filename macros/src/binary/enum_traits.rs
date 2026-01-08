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

use macrospace::struct_utils::constructor;
use macrospace::path_utils::as_prefix;

pub fn def_enum_binary_traits_inner
(
	pascal_op: Ident,
	snake_op: Ident,
	algebra_type: Path,
	input_struct_type: Path,
	input_enum_type: Path,
	output_type: Path,
	mut generics: Generics,
	member_algebras: Vec <(Type, Type, Type, Type)>,
	input_struct_members: Vec <Member>,
	input_enum_variant_idents: Vec <Ident>,
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

	let x = Ident::new ("x", proc_macro2::Span::mixed_site ());

	let var_names: Vec <Ident> = (0..(input_struct_members . len ()))
		. map (|i| format_ident! ("{}{}", x, i))
		. collect ();

	let input_enum_type_prefix = as_prefix (input_enum_type . clone ());

	let mut tokens = proc_macro2::TokenStream::new ();

	// Ops

	{
		let mut generics = generics . clone ();

		{
			let where_clause = generics . make_where_clause ();

			for
			(
				member_algebra_type,
				input_struct_member_type,
				input_enum_variant_type,
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
							#input_struct_member_type,
							#input_enum_variant_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <#input_struct_type, #input_enum_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: #input_struct_type,
					rhs: #input_enum_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members = lhs . #input_struct_members;)*

					match rhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#input_struct_members, #var_names);
						})*
					}

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
				input_struct_member_type,
				input_enum_variant_type,
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
							#input_struct_member_type,
							#input_enum_variant_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <&#lifetime_a #input_struct_type, #input_enum_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: &#lifetime_a #input_struct_type,
					rhs: #input_enum_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members =
						lhs . #input_struct_members . clone ();)*

					match rhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#input_struct_members, #var_names);
						})*
					}

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
				input_struct_member_type,
				input_enum_variant_type,
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
							#input_struct_member_type,
							&#lifetime_a #input_enum_variant_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <#input_struct_type, &#lifetime_a #input_enum_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: #input_struct_type,
					rhs: &#lifetime_a #input_enum_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members = lhs . #input_struct_members;)*

					match rhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#input_struct_members, #var_names);
						})*
					}

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
				input_struct_member_type,
				input_enum_variant_type,
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
							#input_struct_member_type,
							&#lifetime_b #input_enum_variant_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait
			<
				&#lifetime_a #input_struct_type,
				&#lifetime_b #input_enum_type
			>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: &#lifetime_a #input_struct_type,
					rhs: &#lifetime_b #input_enum_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members =
						lhs . #input_struct_members . clone ();)*

					match rhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#input_struct_members, #var_names);
						})*
					}

					#constructor
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	// Symmetric Ops

	{
		let mut generics = generics . clone ();

		{
			let where_clause = generics . make_where_clause ();

			for
			(
				member_algebra_type,
				input_struct_member_type,
				input_enum_variant_type,
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
							#input_enum_variant_type,
							#input_struct_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <#input_enum_type, #input_struct_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: #input_enum_type,
					rhs: #input_struct_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members = rhs . #input_struct_members;)*

					match lhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#var_names, #input_struct_members);
						})*
					}

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
				input_struct_member_type,
				input_enum_variant_type,
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
							&#lifetime_a #input_enum_variant_type,
							#input_struct_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <&#lifetime_a #input_enum_type, #input_struct_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: &#lifetime_a #input_enum_type,
					rhs: #input_struct_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members = rhs . #input_struct_members;)*

					match lhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#var_names, #input_struct_members);
						})*
					}

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
				input_struct_member_type,
				input_enum_variant_type,
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
							#input_enum_variant_type,
							#input_struct_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait <#input_enum_type, &#lifetime_a #input_struct_type>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: #input_enum_type,
					rhs: &#lifetime_a #input_struct_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members =
						rhs . #input_struct_members . clone ();)*

					match lhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#var_names, #input_struct_members);
						})*
					}

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
				input_struct_member_type,
				input_enum_variant_type,
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
							&#lifetime_a #input_enum_variant_type,
							#input_struct_member_type,
							Output = #output_member_type
						>
					)
				);
			}
		}

		let constructor = constructor
		(
			&parse_quote! (Self::Output),
			&output_members,
			&input_struct_members
		);

		let (impl_generics, _, where_clause) = generics . split_for_impl ();

		quote!
		{
			#[automatically_derived]
			impl #impl_generics
			#pascal_op_trait
			<
				&#lifetime_a #input_enum_type,
				&#lifetime_b #input_struct_type
			>
			for #algebra_type
			#where_clause
			{
				type Output = #output_type;

				fn #snake_op
				(
					self,
					lhs: &#lifetime_a #input_enum_type,
					rhs: &#lifetime_b #input_struct_type
				)
				-> Self::Output
				{
					#(let mut #input_struct_members =
						rhs . #input_struct_members . clone ();)*

					match lhs
					{
						#(#input_enum_type_prefix
							::#input_enum_variant_idents (#var_names) =>
						{
							#input_struct_members =
								#algebra_conversion_expressions (self)
								. #snake_op (#var_names, #input_struct_members);
						})*
					}

					#constructor
				}
			}
		}
			. to_tokens (&mut tokens);
	}

	tokens
}
