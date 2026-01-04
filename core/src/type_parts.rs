fn check_phantom (attrs: &Vec <Attribute>) -> bool
{
	for attr in attrs
	{
		let attr_path = attr . path ();

		if attr_path == &parse_quote! (phantom)
			|| attr_path == &parse_quote! (numeric_algebras::phantom)
		{
			return true;
		}
	}

	false
}

pub enum StructMemberType
{
	Arithmetic (Type),
	Phantom
}

impl StructMemberType
{
	pub fn is_phantom (&self) -> bool
	{
		match self
		{
			Self::Phantom => true,
			_ => false
		}
	}
}

pub struct StructMemberInfo
{
	member: Member,
	member_type: StructMemberType
}

impl StructMemberInfo
{
	pub fn is_phantom (&self) -> bool
	{
		self . member_type . is_phantom ()
	}
}

pub fn into_struct_parts (fields: &Fields) -> Vec <StructMemberInfo>
{
	match fields
	{
		Fields::Named (named_fields) =>
		{
			let mut parts = Vec::new ();

			for field in named_fields . named
			{
				let member = field . ident;

				let member_type = match check_phantom (field . attrs)
				{
					false => StructMemberType::Arithmetic (field . ty),
					true => StructMemberType::Phantom
				};

				parts . push (StructMemberInfo {member, member_type});
			}

			parts
		},
		Fields::Unnamed (unnamed_fields) =>
		{
			for (index, field)
			in unnamed_fields . unnamed . iter () . enumerate ()
			{
				let member = index;

				let member_type = match check_phantom (field . attrs)
				{
					false => StructMemberType::Arithmetic (field . ty),
					true => StructMemberType::Phantom
				};

				parts . push (StructMemberInfo {member, member_type});
			}

			parts
		},
		Fields::Unit => Vec::new ()
	}
}

pub struct EnumVariantInfo
{
	member: Ident,
	ty: Type
}

impl EnumVariantInfo
{
	pub fn from_variant (variant: &Variant) -> Result <Self>
	{
		let member = variant . ident . clone ();

		let ty = match variant . fields
		{
			Fields::Unnamed (unnamed_fields) =>
			{
				if unnamed_fields . unnamed . len () != 1
				{
					return Err
					(
						Error::new_spanned
						(
							unnamed_fields . unnamed,
							"Cannot derive arithmetic for enum with multi-field variants"
						)
					);
				}

				unnamed_fields . unnamed . first () . unwrap () . ty
			}
			_ =>
			{
				return Err
				(
					Error::new_spanned
					(
						variant . fields,
						"Cannot derive arithmetic for enum with non-tuple variants"
					)
				);
			}
		};

		Ok (Self {member, ty})
	}
}

pub fn into_enum_parts (variants: &Punctuated <Variant, Token! [,]>)
-> Result <Vec <EnumVariantInfo>>
{
	let mut parts = Vec::new ();

	for variant in variants
	{
		parts . push (EnumVariantInfo::from_variant (variant)?);
	}

	Ok (parts)
}
