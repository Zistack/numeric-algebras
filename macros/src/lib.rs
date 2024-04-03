use proc_macro::TokenStream;

mod def_unary_op_traits;

#[proc_macro]
pub fn def_unary_op (input: TokenStream) -> TokenStream
{
	def_unary_op_traits::def_unary_op_impl (input)
}

mod def_binary_op_traits;

#[proc_macro]
pub fn def_binary_op (input: TokenStream) -> TokenStream
{
	def_binary_op_traits::def_binary_op_impl (input)
}

#[proc_macro]
pub fn def_symmetric_binary_op (input: TokenStream) -> TokenStream
{
	def_binary_op_traits::def_symmetric_binary_op_impl (input)
}

#[proc_macro]
pub fn def_try_binary_op (input: TokenStream) -> TokenStream
{
	def_binary_op_traits::def_try_binary_op_impl (input)
}
