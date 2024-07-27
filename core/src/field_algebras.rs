use std::collections::{HashMap, HashSet};

use syn::{Ident, Expr, Type, Fields, Member, Token};
use syn::token::{Brace, Paren};
use syn::punctuated::Punctuated;
use syn::parse::{Result, Error};
use syn::fold::Fold;
use syn_derive::{Parse, ToTokens};

use macrospace::substitute::Substitutions;

use crate::mapping_transform::MappingTransform;

#[derive (Parse, ToTokens)]
pub struct NamedFieldAlgebra
{
	member_ident: Ident,
	arrow_token: Token! [=>],
	algebra_conversion_expression: Expr,
	colon_token: Token! [:],
	member_algebra_type: Type
}

impl NamedFieldAlgebra
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			member_ident: self . member_ident,
			arrow_token: self . arrow_token,
			algebra_conversion_expression:
				substitutions . fold_expr (self . algebra_conversion_expression),
			colon_token: self . colon_token,
			member_algebra_type:
				substitutions . fold_type (self . member_algebra_type)
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		Self
		{
			member_ident:
				transformations . transform_member_ident (self . member_ident),
			arrow_token: self . arrow_token,
			algebra_conversion_expression:
				transformations
				. transform_algebra_conversion_expression
				(
					self . algebra_conversion_expression
				),
			colon_token: self . colon_token,
			member_algebra_type:
				transformations
				. transform_member_algebra_type (self . member_algebra_type)
		}
	}

	pub fn from_parts
	(
		member_ident: Ident,
		algebra_conversion_expression: Expr,
		member_algebra_type: Type
	)
	-> Self
	{
		Self
		{
			member_ident,
			arrow_token: <Token! [=>]>::default (),
			algebra_conversion_expression,
			colon_token: <Token! [:]>::default (),
			member_algebra_type
		}
	}
}

#[derive (Parse, ToTokens)]
pub struct NamedFieldAlgebras
{
	#[syn (braced)]
	brace: Brace,
	#[syn (in = brace)]
	#[parse (Punctuated::parse_terminated)]
	map_items: Punctuated <NamedFieldAlgebra, Token! [,]>
}

impl NamedFieldAlgebras
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

	pub fn from_parts
	(
		member_algebra_types: Vec <Type>,
		members: Vec <Member>,
		algebra_conversion_expressions: Vec <Expr>
	)
	-> Self
	{
		Self
		{
			brace: Brace::default (),
			map_items: itertools::izip!
			(
				member_algebra_types,
				members,
				algebra_conversion_expressions
			)
				. map
				(
					|(
						member_algebra_type,
						member,
						algebra_conversion_expression
					)|
					if let Member::Named (member_ident) = member
					{
						NamedFieldAlgebra::from_parts
						(
							member_ident,
							algebra_conversion_expression,
							member_algebra_type
						)
					}
					else
					{
						unreachable! ()
					}
				)
				. collect ()
		}
	}
}

#[derive (Parse, ToTokens)]
pub struct UnnamedFieldAlgebra
{
	algebra_conversion_expression: Expr,
	colon_token: Token! [:],
	member_algebra_type: Type
}

impl UnnamedFieldAlgebra
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			algebra_conversion_expression:
				substitutions . fold_expr (self . algebra_conversion_expression),
			colon_token: self . colon_token,
			member_algebra_type:
				substitutions . fold_type (self . member_algebra_type)
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		Self
		{
			algebra_conversion_expression:
				transformations
				. transform_algebra_conversion_expression
				(
					self . algebra_conversion_expression
				),
			colon_token: self . colon_token,
			member_algebra_type:
				transformations
				. transform_member_algebra_type (self . member_algebra_type)
		}
	}

	pub fn from_parts
	(
		algebra_conversion_expression: Expr,
		member_algebra_type: Type
	)
	-> Self
	{
		Self
		{
			algebra_conversion_expression,
			colon_token: <Token! [:]>::default (),
			member_algebra_type
		}
	}
}

#[derive (Parse, ToTokens)]
pub struct UnnamedFieldAlgebras
{
	#[syn (parenthesized)]
	paren: Paren,
	#[syn (in = paren)]
	#[parse (Punctuated::parse_terminated)]
	map_items: Punctuated <UnnamedFieldAlgebra, Token! [,]>
}

impl UnnamedFieldAlgebras
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			paren: self . paren,
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
			paren: self . paren,
			map_items: self
				. map_items
				. into_iter ()
				. map (|item| item . transform (transformations))
				. collect ()
		}
	}

	pub fn from_parts
	(
		member_algebra_types: Vec <Type>,
		algebra_conversion_expressions: Vec <Expr>
	)
	-> Self
	{
		Self
		{
			paren: Paren::default (),
			map_items: itertools::izip!
			(
				member_algebra_types,
				algebra_conversion_expressions
			)
				. map
				(
					|(member_algebra_type, algebra_conversion_expression)|
					UnnamedFieldAlgebra::from_parts
					(
						algebra_conversion_expression,
						member_algebra_type
					)
				)
				. collect ()
		}
	}
}

#[derive (Parse, ToTokens)]
pub enum FieldAlgebras
{
	#[parse (peek = Brace)]
	Named (NamedFieldAlgebras),
	#[parse (peek = Paren)]
	Unnamed (UnnamedFieldAlgebras)
}

impl FieldAlgebras
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		match self
		{
			Self::Named (named) =>
				Self::Named (named . substitute (substitutions)),
			Self::Unnamed (unnamed) =>
				Self::Unnamed (unnamed . substitute (substitutions))
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		match self
		{
			Self::Named (named) =>
				Self::Named (named . transform (transformations)),
			Self::Unnamed (unnamed) =>
				Self::Unnamed (unnamed . transform (transformations))
		}
	}

	pub fn into_parts (self, struct_fields: Fields)
	-> Result <(Vec <(Type, Type)>, Vec <Member>, Vec <Expr>)>
	{
		let mut member_algebras = Vec::new ();
		let mut members = Vec::new ();
		let mut algebra_conversion_expressions = Vec::new ();

		match (self, struct_fields . clone ())
		{
			(FieldAlgebras::Named (named_items), Fields::Named (named_fields)) =>
			{
				let member_type_map: HashMap <Ident, Type> = named_fields
					. named
					. into_iter ()
					. map (|field| (field . ident . unwrap (), field . ty))
					. collect ();

				let attribute_members: HashSet <Ident> = named_items
					. map_items
					. iter ()
					. map (|item| item . member_ident . clone ())
					. collect ();

				for member_ident in member_type_map . keys ()
				{
					if let None = attribute_members . get (member_ident)
					{
						return Err
						(
							Error::new_spanned
							(
								&named_items,
								"Attribute doesn't name all fields in struct type"
							)
						);
					}
				}

				for named_item in named_items . map_items
				{
					let NamedFieldAlgebra
					{
						member_ident,
						algebra_conversion_expression,
						member_algebra_type,
						..
					}
						= named_item;

					let member_type = member_type_map
						. get (&member_ident)
						. ok_or_else
						(
							||
							Error::new_spanned
							(
								&member_ident,
								"Member is not present in struct type"
							)
						)?;

					member_algebras . push
					((
						member_algebra_type,
						member_type . clone ()
					));

					members . push (Member::from (member_ident));
					algebra_conversion_expressions
						. push (algebra_conversion_expression);
				}
			}
			(FieldAlgebras::Unnamed (unnamed_items), Fields::Unnamed (unnamed_fields)) =>
			{
				if unnamed_items . map_items . len () !=
					unnamed_fields . unnamed . len ()
				{
					return Err
					(
						Error::new_spanned
						(
							&unnamed_items,
							"Mapping has different number of fields than struct type"
						)
					);
				}

				for (i, (unnamed_item, unnamed_field))
				in unnamed_items
					. map_items
					. into_iter ()
					. zip (unnamed_fields . unnamed)
					. enumerate ()
				{
					let UnnamedFieldAlgebra
					{
						algebra_conversion_expression,
						member_algebra_type,
						..
					}
						= unnamed_item;

					let member_type = unnamed_field . ty;

					member_algebras . push ((member_algebra_type, member_type));

					members . push (Member::from (i));
					algebra_conversion_expressions
						. push (algebra_conversion_expression);
				}
			}
			(FieldAlgebras::Named (named_items), Fields::Unnamed (_unnamed_fields)) =>
			{
				return Err
				(
					Error::new_spanned
					(
						named_items,
						"Struct type is a tuple struct"
					)
				);
			}
			(FieldAlgebras::Unnamed (unnamed_items), Fields::Named (_named_fields)) =>
			{
				return Err
				(
					Error::new_spanned
					(
						unnamed_items,
						"Struct type is not a tuple struct"
					)
				);
			}
			(_, Fields::Unit) =>
			{
				return Err
				(
					Error::new_spanned
					(
						struct_fields,
						"Struct type has no fields"
					)
				);
			}
		}

		Ok ((member_algebras, members, algebra_conversion_expressions))
	}

	pub fn from_parts
	(
		member_algebra_types: Vec <Type>,
		members: Vec <Member>,
		algebra_conversion_expressions: Vec <Expr>
	)
	-> Self
	{
		match members . first ()
		{
			Some (Member::Named (_)) => Self::Named
			(
				NamedFieldAlgebras::from_parts
				(
					member_algebra_types,
					members,
					algebra_conversion_expressions
				)
			),
			_ => Self::Unnamed
			(
				UnnamedFieldAlgebras::from_parts
				(
					member_algebra_types,
					algebra_conversion_expressions
				)
			)
		}
	}
}
