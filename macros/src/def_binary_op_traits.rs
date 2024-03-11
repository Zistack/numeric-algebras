use syn::{Ident, Token, parse_macro_input};
use quote::{quote, format_ident};
use syn_derive::Parse;

#[allow (dead_code)]
#[derive (Parse)]
struct DefBinaryOp
{
	pascal_op: Ident,
	comma_token: Token! [,],
	snake_op: Ident
}

pub fn def_binary_op_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	let DefBinaryOp {pascal_op, snake_op, ..} = parse_macro_input! (input);

	let pascal_ops = format_ident! ("{}s", pascal_op);

	let pascal_op_assign = format_ident! ("{}Assign", pascal_op);
	let snake_op_assign = format_ident! ("{}_assign", snake_op);

	let pascal_op_assigns = format_ident! ("{}s", pascal_op_assign);

	let pascal_ops_to_lhs = format_ident! ("{}ToLhs", pascal_ops);

	let pascal_ops_monoid = format_ident! ("{}Monoid", pascal_ops);

	quote!
	{
		#[forward_traits::forwardable]
		pub trait #pascal_op <Lhs, Rhs>
		{
			type Output;

			fn #snake_op (self, lhs: Lhs, rhs: Rhs) -> Self::Output;
		}

		pub trait #pascal_ops <Lhs, Rhs>:
			#pascal_op <Lhs, Rhs, Output = <Self as #pascal_ops <Lhs, Rhs>>::Output>
			+ for <'a> #pascal_op <Lhs, &'a Rhs, Output = <Self as #pascal_ops <Lhs, Rhs>>::Output>
			+ for <'a> #pascal_op <&'a Lhs, Rhs, Output = <Self as #pascal_ops <Lhs, Rhs>>::Output>
			+ for <'a, 'b> #pascal_op <&'a Lhs, &'b Rhs, Output = <Self as #pascal_ops <Lhs, Rhs>>::Output>
		{
			type Output;
		}

		impl <Lhs, Rhs, O, T> #pascal_ops <Lhs, Rhs> for T
		where T:
			#pascal_op <Lhs, Rhs, Output = O>
			+ for <'a> #pascal_op <Lhs, &'a Rhs, Output = O>
			+ for <'a> #pascal_op <&'a Lhs, Rhs, Output = O>
			+ for <'a, 'b> #pascal_op <&'a Lhs, &'b Rhs, Output = O>
		{
			type Output = O;
		}

		#[forward_traits::forwardable]
		pub trait #pascal_op_assign <Lhs, Rhs>
		{
			fn #snake_op_assign (self, lhs: &mut Lhs, rhs: Rhs);
		}

		pub trait #pascal_op_assigns <Lhs, Rhs>:
			#pascal_op_assign <Lhs, Rhs>
			+ for <'a> #pascal_op_assign <Lhs, &'a Rhs>
		{
		}

		impl <Lhs, Rhs, T> #pascal_op_assigns <Lhs, Rhs> for T
		where T:
			#pascal_op_assign <Lhs, Rhs>
			+ for <'a> #pascal_op_assign <Lhs, &'a Rhs>
		{
		}

		pub trait #pascal_ops_to_lhs <Lhs, Rhs>:
			#pascal_ops <Lhs, Rhs, Output = Lhs>
			+ #pascal_op_assigns <Lhs, Rhs>
		{
		}

		impl <Lhs, Rhs, T> #pascal_ops_to_lhs <Lhs, Rhs> for T
		where T:
			#pascal_ops <Lhs, Rhs, Output = Lhs>
			+ #pascal_op_assigns <Lhs, Rhs>
		{
		}

		pub trait #pascal_ops_monoid <X>:
			#pascal_ops <X, X, Output = X>
			+ #pascal_op_assigns <X, X>
		{
		}

		impl <X, T> #pascal_ops_monoid <X> for T
		where T:
			#pascal_ops <X, X, Output = X>
			+ #pascal_op_assigns <X, X>
		{
		}
	}
		. into ()
}

#[allow (dead_code)]
#[derive (Parse)]
struct DefSymmetricBinaryOp
{
	pascal_long_op: Ident,
	comma_token1: Token! [,],
	pascal_op: Ident,
	comma_token2: Token! [,],
	snake_op: Ident
}

pub fn def_symmetric_binary_op_impl (input: proc_macro::TokenStream)
-> proc_macro::TokenStream
{
	let DefSymmetricBinaryOp {pascal_long_op, pascal_op, snake_op, ..} =
		parse_macro_input! (input);

	let pascal_ops = format_ident! ("{}s", pascal_op);
	let pascal_op_assigns = format_ident! ("{}Assigns", pascal_op);

	let pascal_ops_to_rhs = format_ident! ("{}ToRhs", pascal_ops);

	let pascal_long_op_is_commutative =
		format_ident! ("{}IsCommutative", pascal_long_op);

	let pascal_comm_op = format_ident! ("Comm{}", pascal_op);
	let pascal_comm_ops = format_ident! ("{}s", pascal_comm_op);
	let pascal_comm_ops_to_lhs = format_ident! ("{}ToLhs", pascal_comm_ops);
	let pascal_comm_ops_to_rhs = format_ident! ("{}ToRhs", pascal_comm_ops);
	let pascal_comm_ops_monoid = format_ident! ("{}Monoid", pascal_comm_ops);

	let pascal_long_op_is_associative =
		format_ident! ("{}IsAssociative", pascal_long_op);

	quote!
	{
		numeric_algebras_macros::def_binary_op! (#pascal_op, #snake_op);

		pub trait #pascal_ops_to_rhs <Lhs, Rhs>:
			#pascal_ops <Lhs, Rhs, Output = Rhs>
		{
		}

		impl <Lhs, Rhs, T> #pascal_ops_to_rhs <Lhs, Rhs> for T
		where T: #pascal_ops <Lhs, Rhs, Output = Rhs>
		{
		}

		pub trait #pascal_long_op_is_commutative
		{
		}

		pub trait #pascal_comm_op <Lhs, Rhs>:
			#pascal_op <Lhs, Rhs, Output = <Self as #pascal_comm_op <Lhs, Rhs>>::Output>
			+ #pascal_op <Rhs, Lhs, Output = <Self as #pascal_comm_op <Lhs, Rhs>>::Output>
			+ #pascal_long_op_is_commutative
		{
			type Output;
		}

		impl <Lhs, Rhs, O, T> #pascal_comm_op <Lhs, Rhs> for T
		where T:
			#pascal_op <Lhs, Rhs, Output = O>
			+ #pascal_op <Rhs, Lhs, Output = O>
			+ #pascal_long_op_is_commutative
		{
			type Output = O;
		}

		pub trait #pascal_comm_ops <Lhs, Rhs>:
			#pascal_comm_op <Lhs, Rhs, Output = <Self as #pascal_comm_ops <Lhs, Rhs>>::Output>
			+ for <'a> #pascal_comm_op <Lhs, &'a Rhs, Output = <Self as #pascal_comm_ops <Lhs, Rhs>>::Output>
			+ for <'a> #pascal_comm_op <&'a Lhs, Rhs, Output = <Self as #pascal_comm_ops <Lhs, Rhs>>::Output>
			+ for <'a, 'b> #pascal_comm_op <&'a Lhs, &'b Rhs, Output = <Self as #pascal_comm_ops <Lhs, Rhs>>::Output>
		{
			type Output;
		}

		impl <Lhs, Rhs, O, T> #pascal_comm_ops <Lhs, Rhs> for T
		where T:
			#pascal_comm_op <Lhs, Rhs, Output = O>
			+ for <'a> #pascal_comm_op <Lhs, &'a Rhs, Output = O>
			+ for <'a> #pascal_comm_op <&'a Lhs, Rhs, Output = O>
			+ for <'a, 'b> #pascal_comm_op <&'a Lhs, &'b Rhs, Output = O>
		{
			type Output = O;
		}

		pub trait #pascal_comm_ops_to_lhs <Lhs, Rhs>:
			#pascal_comm_ops <Lhs, Rhs, Output = Lhs>
			+ #pascal_op_assigns <Lhs, Rhs>
		{
		}

		impl <Lhs, Rhs, T> #pascal_comm_ops_to_lhs <Lhs, Rhs> for T
		where T:
			#pascal_comm_ops <Lhs, Rhs, Output = Lhs>
			+ #pascal_op_assigns <Lhs, Rhs>
		{
		}

		pub trait #pascal_comm_ops_to_rhs <Lhs, Rhs>:
			#pascal_comm_ops <Lhs, Rhs, Output = Rhs>
			+ #pascal_op_assigns <Rhs, Lhs>
		{
		}

		impl <Lhs, Rhs, T> #pascal_comm_ops_to_rhs <Lhs, Rhs> for T
		where T:
			#pascal_comm_ops <Lhs, Rhs, Output = Rhs>
			+ #pascal_op_assigns <Rhs, Lhs>
		{
		}

		pub trait #pascal_comm_ops_monoid <X>:
			#pascal_comm_ops <X, X, Output = X>
			+ #pascal_op_assigns <X, X>
		{
		}

		// For some reason, asking for #pascal_ops directly here doesn't work.
		// I think it's a bug in handling quantified lifetimes when trait
		// signatures collapse.  Also, I can trigger a recusion overflow if I
		// try to use #pascal_ops_monoid in place of the first two bounds here.
		impl <X, T> #pascal_comm_ops_monoid <X> for T
		where T:
			#pascal_ops <X, X, Output = X>
			+ #pascal_op_assigns <X, X>
			+ #pascal_long_op_is_commutative
		{
		}

		pub trait #pascal_long_op_is_associative
		{
		}
	}
		. into ()
}
