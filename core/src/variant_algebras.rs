use std::collections::{HashMap, HashSet};

use syn::{Attribute, Ident, Expr, Type, Variant, Token, parse2};
use syn::token::{Paren, Brace};
use syn::punctuated::Punctuated;
use syn::parse::{Result, Error};
use syn::fold::Fold;
use syn_derive::{Parse, ToTokens};
use quote::{ToTokens, TokenStreamExt};

use macrospace::substitute::Substitutions;

use crate::mapping_transform::MappingTransform;

#[derive (Parse, ToTokens)]
pub struct VariantAlgebra
{
	variant_ident: Ident,
	arrow_token: Token! [=>],
	algebra_conversion_expression: Expr,
	colon_token: Token! [:],
	variant_algebra_type: Type
}

impl VariantAlgebra
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			variant_ident: self . variant_ident,
			arrow_token: self . arrow_token,
			algebra_conversion_expression:
				substitutions . fold_expr (self . algebra_conversion_expression),
			colon_token: self . colon_token,
			variant_algebra_type:
				substitutions . fold_type (self . variant_algebra_type)
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		Self
		{
			variant_ident:
				transformations
				. transform_variant_ident (self . variant_ident),
			arrow_token: self . arrow_token,
			algebra_conversion_expression:
				transformations
				. transform_algebra_conversion_expression
				(
					self . algebra_conversion_expression
				),
			colon_token: self . colon_token,
			variant_algebra_type:
				transformations
				. transform_variant_algebra_type (self . variant_algebra_type)
		}
	}

	pub fn from_parts
	(
		variant_ident: Ident,
		algebra_conversion_expression: Expr,
		variant_algebra_type: Type
	)
	-> Self
	{
		Self
		{
			variant_ident,
			arrow_token: <Token! [=>]>::default (),
			algebra_conversion_expression,
			colon_token: <Token! [:]>::default (),
			variant_algebra_type
		}
	}
}

#[derive (Parse, ToTokens)]
struct MonoTypeVariant
{
	#[parse (Attribute::parse_outer)]
	#[to_tokens  (|tokens, attrs| tokens . append_all (attrs))]
	pub attrs: Vec <Attribute>,
	pub ident: Ident,
	#[syn (parenthesized)]
	pub paren: Paren,
	#[syn (in = paren)]
	pub ty: Type,
}

#[derive (Parse, ToTokens)]
pub struct VariantAlgebras
{
	#[syn (braced)]
	brace: Brace,
	#[syn (in = brace)]
	#[parse (Punctuated::parse_terminated)]
	map_items: Punctuated <VariantAlgebra, Token! [,]>
}

impl VariantAlgebras
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			brace: self . brace,
			map_items: self
				. map_items
				. into_iter ()
				. map (|item| item . substitute (substitutions))
				. collect ()
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		Self
		{
			brace: self . brace,
			map_items: self
				. map_items
				. into_iter ()
				. map (|item| item . transform (transformations))
				. collect ()
		}
	}

	pub fn into_parts (self, enum_variants: Punctuated <Variant, Token! [,]>)
	-> Result <(Vec <(Type, Type)>, Vec <Ident>, Vec <Expr>)>
	{
		let mut variant_algebras = Vec::new ();
		let mut variants = Vec::new ();
		let mut algebra_conversion_expressions = Vec::new ();

		let mut variant_type_map = HashMap::new ();

		for variant in enum_variants
		{
			let MonoTypeVariant {ident, ty, ..} =
				match parse2 (variant . to_token_stream ())
			{
				Ok (mono_type_variant) => mono_type_variant,
				Err (_) =>
				{
					return Err
					(
						Error::new_spanned
						(
							variant,
							"Enum variants must all have exactly one unnamed field"
						)
					);
				}
			};

			variant_type_map . insert (ident, ty);
		}

		let attribute_variants: HashSet <Ident> = self
			. map_items
			. iter ()
			. map (|item| item . variant_ident . clone ())
			. collect ();

		for variant_ident in variant_type_map . keys ()
		{
			if let None = attribute_variants . get (variant_ident)
			{
				return Err
				(
					Error::new_spanned
					(
						&self,
						"Attribute doesn't name all variants in enum type"
					)
				);
			}
		}

		for item in self . map_items
		{
			let VariantAlgebra
			{
				variant_ident,
				algebra_conversion_expression,
				variant_algebra_type,
				..
			}
				= item;

			let variant_type = variant_type_map
				. get (&variant_ident)
				. ok_or_else
				(
					||
					Error::new_spanned
					(
						&variant_ident,
						"Variant is not present in enum type"
					)
				)?;

			variant_algebras . push
			((
				variant_algebra_type,
				variant_type . clone ()
			));

			variants . push (variant_ident);
			algebra_conversion_expressions
				. push (algebra_conversion_expression);
		}

		Ok((variant_algebras, variants, algebra_conversion_expressions))
	}

	pub fn from_parts
	(
		variant_algebra_types: Vec <Type>,
		variants: Vec <Ident>,
		algebra_conversion_expressions: Vec <Expr>
	)
	-> Self
	{
		Self
		{
			brace: Brace::default (),
			map_items: itertools::izip!
			(
				variant_algebra_types,
				variants,
				algebra_conversion_expressions
			)
				. map
				(
					|(
						variant_algebra_type,
						variant_ident,
						algebra_conversion_expression
					)|
					VariantAlgebra::from_parts
					(
						variant_ident,
						algebra_conversion_expression,
						variant_algebra_type
					)
				)
				. collect ()
		}
	}
}
