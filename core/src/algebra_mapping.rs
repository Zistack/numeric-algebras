use syn::{
	Attribute,
	Path,
	Type,
	Fields,
	Variant,
	Member,
	Ident,
	Expr,
	DeriveInput,
	Data,
	Token,
	parse_quote
};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream, Result, Error};
use syn::fold::Fold;
use syn_derive::ToTokens;
use quote::{ToTokens, TokenStreamExt};

use macrospace::substitute::Substitutions;

use crate::field_algebras::*;
use crate::variant_algebras::*;
use crate::mapping_transform::MappingTransform;

#[derive (ToTokens)]
pub struct StructAlgebraMapping
{
	struct_token: Token! [struct],
	field_algebras: FieldAlgebras
}

impl StructAlgebraMapping
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			struct_token: self . struct_token,
			field_algebras: self . field_algebras . substitute (substitutions)
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		Self
		{
			struct_token: self . struct_token,
			field_algebras: self . field_algebras . transform (transformations)
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
			struct_token: <Token! [struct]>::default (),
			field_algebras: FieldAlgebras::from_parts
			(
				member_algebra_types,
				members,
				algebra_conversion_expressions
			)
		}
	}
}

#[derive (ToTokens)]
pub struct EnumAlgebraMapping
{
	enum_token: Token! [enum],
	variant_algebras: VariantAlgebras
}

impl EnumAlgebraMapping
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			enum_token: self . enum_token,
			variant_algebras: self . variant_algebras . substitute (substitutions)
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		Self
		{
			enum_token: self . enum_token,
			variant_algebras: self . variant_algebras . transform (transformations)
		}
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
			enum_token: <Token! [enum]>::default (),
			variant_algebras: VariantAlgebras::from_parts
			(
				variant_algebra_types,
				variants,
				algebra_conversion_expressions
			)
		}
	}
}

#[derive (ToTokens)]
pub enum TypeAlgebraMapping
{
	Struct (StructAlgebraMapping),
	Enum (EnumAlgebraMapping)
}

impl TypeAlgebraMapping
{
	pub fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		match self
		{
			Self::Struct (struct_algebra_mapping) =>
				Self::Struct (struct_algebra_mapping . substitute (substitutions)),
			Self::Enum (enum_algebra_mapping) =>
				Self::Enum (enum_algebra_mapping . substitute (substitutions))
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		match self
		{
			Self::Struct (struct_algebra_mapping) =>
				Self::Struct (struct_algebra_mapping . transform (transformations)),
			Self::Enum (enum_algebra_mapping) =>
				Self::Enum (enum_algebra_mapping . transform (transformations))
		}
	}
}

pub struct AlgebraMapping
{
	pub multipart_type: Path,
	pub type_algebra_mapping: TypeAlgebraMapping
}

pub enum TypeParts
{
	Struct (Vec <Member>),
	Enum (Vec <Ident>)
}

impl TypeParts
{
	fn from_struct_parts (members: Vec <Member>) -> Self
	{
		Self::Struct (members)
	}

	fn from_enum_parts (variants: Vec <Ident>) -> Self
	{
		Self::Enum (variants)
	}

	pub fn len (&self) -> usize
	{
		match self
		{
			Self::Struct (members) => members . len (),
			Self::Enum (variants) => variants . len ()
		}
	}
}

impl AlgebraMapping
{
	fn substitute (self, substitutions: &mut Substitutions) -> Self
	{
		Self
		{
			multipart_type: substitutions . fold_path (self . multipart_type),
			type_algebra_mapping: self
				. type_algebra_mapping
				. substitute (substitutions)
		}
	}

	pub fn transform <T> (self, transformations: &mut T) -> Self
	where T: MappingTransform
	{
		Self
		{
			multipart_type: transformations
				. transform_multipart_type_path (self . multipart_type),
			type_algebra_mapping: self
				. type_algebra_mapping
				. transform (transformations)
		}
	}

	fn attr_path_matches (attr_path: &Path) -> bool
	{
		attr_path == &parse_quote! (algebra_mapping)
			|| attr_path == &parse_quote! (numeric_algebras::algebra_mapping)
	}

	fn check_attr
	(
		attribute: &Attribute,
		substitutions: &mut Substitutions,
		multipart_type: &Path
	)
	-> Option <Result <Self>>
	{
		if Self::attr_path_matches (&attribute . path ())
		{
			match attribute . parse_args::<Self> ()
			{
				Ok (mapping) =>
				{
					let mapping = mapping . substitute (substitutions);

					if &mapping . multipart_type == multipart_type
					{
						Some (Ok (mapping))
					}
					else
					{
						None
					}
				},
				Err (error) => Some (Err (error))
			}
		}
		else { None }
	}

	pub fn get_from_attributes
	(
		attributes: &Vec <Attribute>,
		substitutions: &mut Substitutions,
		multipart_type: &Path
	)
	-> Result <Self>
	{
		let mut attr_error = None;

		for attribute in attributes
		{
			match Self::check_attr (attribute, substitutions, multipart_type)
			{
				Some (Ok (mapping)) => return Ok (mapping),
				Some (Err (error)) => attr_error = Some (error),
				None => {}
			}
		}

		let error = if let Some (error) = attr_error { error }
		else
		{
			let mut attribute_tokens = proc_macro2::TokenStream::new ();

			attribute_tokens . append_all (attributes);

			Error::new_spanned
			(
				attribute_tokens,
				format_args!
				(
					"Missing algebra_mapping attribute for `{}`",
					multipart_type . to_token_stream ()
				)
			)
		};

		Err (error)
	}

	pub fn into_struct_parts (self, struct_fields: Fields)
	-> Result <(Vec <(Type, Type)>, Vec <Member>, Vec <Expr>)>
	{
		match self . type_algebra_mapping
		{
			TypeAlgebraMapping::Struct (struct_mapping) =>
				struct_mapping . field_algebras . into_parts (struct_fields),
			TypeAlgebraMapping::Enum (enum_mapping) => Err
			(
				Error::new_spanned
				(
					enum_mapping,
					"Type is a struct, not an enum"
				)
			)
		}
	}

	pub fn into_enum_parts (self, enum_variants: Punctuated <Variant, Token! [,]>)
	-> Result <(Vec <(Type, Type)>, Vec <Ident>, Vec <Expr>)>
	{
		match self . type_algebra_mapping
		{
			TypeAlgebraMapping::Struct (struct_mapping) => Err
			(
				Error::new_spanned
				(
					struct_mapping,
					"Type is an enum, not a struct"
				)
			),
			TypeAlgebraMapping::Enum (enum_mapping) =>
				enum_mapping . variant_algebras . into_parts (enum_variants)
		}
	}

	pub fn into_parts (self, item: DeriveInput)
	-> Result <(Vec <(Type, Type)>, TypeParts, Vec <Expr>)>
	{
		match (self . type_algebra_mapping, item . data)
		{
			(
				TypeAlgebraMapping::Struct (struct_mapping),
				Data::Struct (struct_data)
			) =>
			{
				let (member_algebras, members, algebra_conversion_expressions) =
					struct_mapping
					. field_algebras
					. into_parts (struct_data . fields)?;

				Ok ((
					member_algebras,
					TypeParts::from_struct_parts (members),
					algebra_conversion_expressions
				))
			},
			(
				TypeAlgebraMapping::Enum (enum_mapping),
				Data::Enum (enum_data)
			) =>
			{
				let (variant_algebras, variants, algebra_conversion_expressions) =
					enum_mapping
					. variant_algebras
					. into_parts (enum_data . variants)?;

				Ok ((
					variant_algebras,
					TypeParts::from_enum_parts (variants),
					algebra_conversion_expressions
				))
			},
			(ref mapping @ _, Data::Struct (_)) => Err
			(
				Error::new_spanned (mapping, "Type is a struct")
			),
			(ref mapping @ _, Data::Enum (_)) => Err
			(
				Error::new_spanned (mapping, "Type is an enum")
			),
			(_, Data::Union (union_data)) => Err
			(
				Error::new_spanned
				(
					union_data . union_token,
					"Unions are not supported"
				)
			)
		}
	}

	pub fn from_struct_parts
	(
		multipart_type: Path,
		member_algebra_types: Vec <Type>,
		members: Vec <Member>,
		algebra_conversion_expressions: Vec <Expr>
	)
	-> Self
	{
		Self
		{
			multipart_type,
			type_algebra_mapping: TypeAlgebraMapping::Struct
			(
				StructAlgebraMapping::from_parts
				(
					member_algebra_types,
					members,
					algebra_conversion_expressions
				)
			)
		}
	}

	pub fn from_enum_parts
	(
		multipart_type: Path,
		variant_algebra_types: Vec <Type>,
		variants: Vec <Ident>,
		algebra_conversion_expressions: Vec <Expr>
	)
	-> Self
	{
		Self
		{
			multipart_type,
			type_algebra_mapping: TypeAlgebraMapping::Enum
			(
				EnumAlgebraMapping::from_parts
				(
					variant_algebra_types,
					variants,
					algebra_conversion_expressions
				)
			)
		}
	}

	pub fn from_parts
	(
		multipart_type: Path,
		part_algebra_types: Vec <Type>,
		parts: TypeParts,
		algebra_conversion_expressions: Vec <Expr>
	)
	-> Self
	{
		match parts
		{
			TypeParts::Struct (members) => Self::from_struct_parts
			(
				multipart_type,
				part_algebra_types,
				members,
				algebra_conversion_expressions
			),
			TypeParts::Enum (variants) => Self::from_enum_parts
			(
				multipart_type,
				part_algebra_types,
				variants,
				algebra_conversion_expressions
			)
		}
	}
}

impl Parse for AlgebraMapping
{
	fn parse (input: ParseStream <'_>) -> Result <Self>
	{
		let lookahead = input . lookahead1 ();

		if lookahead . peek (Token! [struct])
		{
			let struct_token = input . parse ()?;
			let multipart_type = input . parse ()?;
			let field_algebras = input . parse ()?;

			Ok
			(
				Self
				{
					multipart_type,
					type_algebra_mapping: TypeAlgebraMapping::Struct
					(
						StructAlgebraMapping {struct_token, field_algebras}
					)
				}
			)
		}
		else if lookahead . peek (Token! [enum])
		{
			let enum_token = input . parse ()?;
			let multipart_type = input . parse ()?;
			let variant_algebras = input . parse ()?;

			Ok
			(
				Self
				{
					multipart_type,
					type_algebra_mapping: TypeAlgebraMapping::Enum
					(
						EnumAlgebraMapping {enum_token, variant_algebras}
					)
				}
			)
		}
		else
		{
			Err (lookahead . error ())
		}
	}
}


impl ToTokens for AlgebraMapping
{
	fn to_tokens (&self, tokens: &mut proc_macro2::TokenStream)
	{
		match &self . type_algebra_mapping
		{
			TypeAlgebraMapping::Struct (struct_algebra_mapping) =>
			{
				struct_algebra_mapping . struct_token . to_tokens (tokens);
				self . multipart_type . to_tokens (tokens);
				struct_algebra_mapping . field_algebras . to_tokens (tokens);
			}
			TypeAlgebraMapping::Enum (enum_algebra_mapping) =>
			{
				enum_algebra_mapping . enum_token . to_tokens (tokens);
				self . multipart_type . to_tokens (tokens);
				enum_algebra_mapping . variant_algebras . to_tokens (tokens);
			}
		}
	}
}
