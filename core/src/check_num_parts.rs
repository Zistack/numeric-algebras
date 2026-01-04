use std::fmt::Display;

use syn::parse::{Result, Error};
use quote::ToTokens;

pub fn check_num_parts <TX, TY, NX, NY>
(
	x_len: usize,
	y_len: usize,
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
	if x_len != y_len
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
