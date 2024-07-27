use std::fmt::Display;

use syn::{Type, Expr};
use syn::parse::{Result, Error};
use quote::ToTokens;

pub fn check_num_parts <X, Y, TX, TY, NX, NY>
(
	x: &Vec <X>,
	y: &Vec <Y>,
	x_tokens: &TX,
	y_tokens: &TY,
	x_name: NX,
	y_name: NY
)
-> Result <()>
where
	TX: ToTokens,
	TY: ToTokens,
	NX: Display,
	NY: Display
{
	if x . len () != y . len ()
	{
		let mut tokens = proc_macro2::TokenStream::new ();
		x_tokens . to_tokens (&mut tokens);
		y_tokens . to_tokens (&mut tokens);

		Err
		(
			Error::new_spanned
			(
				tokens,
				format!
				(
					"{} and {} types must have the same number of parts",
					x_name,
					y_name
				)
			)
		)
	}
	else
	{
		Ok (())
	}
}

pub fn check_algebra_type_pair <NX, NY>
(
	x: &Type,
	y: &Type,
	x_name: NX,
	y_name: NY
)
-> Result <()>
where
	NX: Display,
	NY: Display
{
	if x != y
	{
		let mut tokens = proc_macro2::TokenStream::new ();
		x . to_tokens (&mut tokens);
		y . to_tokens (&mut tokens);

		Err
		(
			Error::new_spanned
			(
				tokens,
				format!
				(
					"{} and {} parts must use the same algebra type",
					x_name,
					y_name
				)
			)
		)
	}
	else
	{
		Ok (())
	}
}

pub fn check_algebra_type_pairs <'a, 'b, IX, IY, NX, NY>
(
	x_iter: IX,
	y_iter: IY,
	x_name: NX,
	y_name: NY
)
-> Result <()>
where
	IX: IntoIterator <Item = &'a Type>,
	IY: IntoIterator <Item = &'b Type>,
	NX: Clone + Display,
	NY: Clone + Display
{
	for (x, y) in x_iter . into_iter () . zip (y_iter)
	{
		check_algebra_type_pair (x, y, x_name . clone (), y_name . clone ())?;
	}

	Ok (())
}

pub fn check_algebra_conversion_expression_pair <NX, NY>
(
	x: &Expr,
	y: &Expr,
	x_name: NX,
	y_name: NY
)
-> Result <()>
where
	NX: Display,
	NY: Display
{
	if x != y
	{
		let mut tokens = proc_macro2::TokenStream::new ();
		x . to_tokens (&mut tokens);
		y . to_tokens (&mut tokens);

		Err
		(
			Error::new_spanned
			(
				tokens,
				format!
				(
					"{} and {} parts must use the same algebra",
					x_name,
					y_name
				)
			)
		)
	}
	else
	{
		Ok (())
	}
}

pub fn check_algebra_conversion_expression_pairs <'a, 'b, IX, IY, NX, NY>
(
	x_iter: IX,
	y_iter: IY,
	x_name: NX,
	y_name: NY
)
-> Result <()>
where
	IX: IntoIterator <Item = &'a Expr>,
	IY: IntoIterator <Item = &'b Expr>,
	NX: Clone + Display,
	NY: Clone + Display
{
	for (x, y) in x_iter . into_iter () . zip (y_iter)
	{
		check_algebra_conversion_expression_pair
		(
			x,
			y,
			x_name . clone (),
			y_name . clone ()
		)?;
	}

	Ok (())
}
