use syn::{Path, Ident, Expr, Type};

pub trait MappingTransform
{
	fn transform_multipart_type_path (&mut self, multipart_type_path: Path)
	-> Path;

	fn transform_member_ident (&mut self, member_ident: Ident) -> Ident;

	fn transform_variant_ident (&mut self, variant_ident: Ident) -> Ident;

	fn transform_algebra_conversion_expression
	(
		&mut self,
		algebra_conversion_expression: Expr
	)
	-> Expr;

	fn transform_member_algebra_type (&mut self, member_algebra_type: Type)
	-> Type;

	fn transform_variant_algebra_type (&mut self, variant_algebra_type: Type)
	-> Type;
}
