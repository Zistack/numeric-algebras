use syn::{
	Attribute,
	Type,
	Expr,
	DeriveInput,
	Token,
	parse_quote
};
use syn::punctuated::Punctuated;
use syn::parse::{Result, Error};
use syn::fold::Fold;
use syn_derive::{Parse, ToTokens};

use macrospace::substitute::Substitutions;

#[derive (Clone, Debug, Parse, ToTokens)]
pub struct AlgebraMapping
{
	#[parse (Punctuated::parse_terminated)]
	algebra_entries: Punctuated <AlgebraEntry, Token! [,]>
}

impl AlgebraMapping
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			algebra_entries: self
				. algebra_entries
				. into_iter ()
				. map (|entry| entry . substitute (substitutions))
				. collect ()
		}
	}

	pub fn get_from_attributes
	(
		algebra_item: &DeriveInput,
		attrs: &Vec <Attribute>,
		substitutions: &mut Substitutions
	)
	-> Result <Self>
	{
		for attr in attrs
		{
			if attr . path () == &parse_quote! (algebra_mapping)
				|| attr . path () == &parse_quote! (numeric_algebras::algebra_mapping)
			{
				return Ok (attr . parse_args::<Self> ()? . substitute (substitutions));
			}
		}

		Err
		(
			Error::new_spanned
			(
				algebra_item,
				"Missing algebra_mapping attribute"
			)
		)
	}

	pub fn into_parts (self) -> (Vec <Expr>, Vec <Type>)
	{
		let mut algebra_conversion_expressions = Vec::new ();
		let mut algebra_types = Vec::new ();

		for entry in self . algebra_entries
		{
			algebra_conversion_expressions . push (entry . conversion_expr);
			algebra_types . push (entry . ty);
		}

		(algebra_conversion_expressions, algebra_types)
	}
}

#[derive (Clone, Debug, Parse, ToTokens)]
pub struct AlgebraEntry
{
	conversion_expr: Expr,
	colon_token: Token! [:],
	ty: Type
}

impl AlgebraEntry
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			conversion_expr: substitutions . fold_expr (self . conversion_expr),
			colon_token: self . colon_token,
			ty: substitutions . fold_type (self . ty)
		}
	}
}
