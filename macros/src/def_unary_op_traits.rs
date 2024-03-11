use syn::{Ident, Token, parse_macro_input};
use quote::{quote, format_ident};
use syn_derive::Parse;

#[allow (dead_code)]
#[derive (Parse)]
struct DefUnaryOp
{
	pascal_op_ident: Ident,
	comma_token: Token! [,],
	snake_op_ident: Ident
}

pub fn def_unary_op_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	let DefUnaryOp {pascal_op_ident, snake_op_ident, ..} =
		parse_macro_input! (input);

	let pascal_ops_ident = format_ident! ("{}s", pascal_op_ident);
	let pascal_ops_monoid_ident = format_ident! ("{}Monoid", pascal_ops_ident);

	quote!
	{
		#[forward_traits::forwardable]
		pub trait #pascal_op_ident <X>
		{
			type Output;

			fn #snake_op_ident (self, x: X) -> Self::Output;
		}

		pub trait #pascal_ops_ident <X>:
			#pascal_op_ident <X, Output = <Self as #pascal_ops_ident <X>>::Output>
			+ for <'a> #pascal_op_ident <&'a X, Output = <Self as #pascal_ops_ident <X>>::Output>
		{
			type Output;
		}

		impl <X, O, T> #pascal_ops_ident <X> for T
		where T:
			#pascal_op_ident <X, Output = O>
			+ for <'a> #pascal_op_ident <&'a X, Output = O>
		{
			type Output = O;
		}

		pub trait #pascal_ops_monoid_ident <X>: #pascal_ops_ident <X, Output = X>
		{
		}

		impl <X, T> #pascal_ops_monoid_ident <X> for T
		where T: #pascal_ops_ident <X, Output = X>
		{
		}
	}
		. into ()
}
