macro_rules! __def_unary_op
{
	($PascalOp: ident, $snake_op: ident) =>
	{
		paste::paste!
		{
			#[forward_traits::forwardable]
			pub trait $PascalOp <X>
			{
				type Output;

				fn $snake_op (self, x: X) -> Self::Output;
			}

			pub trait [<$PascalOp s>] <X>:
				$PascalOp <X, Output = <Self as [<$PascalOp s>] <X>>::Output>
				+ for <'a> $PascalOp <&'a X, Output = <Self as [<$PascalOp s>] <X>>::Output>
			{
				type Output;
			}

			impl <X, T, O> [<$PascalOp s>] <X> for T
			where T: $PascalOp <X, Output = O>
				+ for <'a> $PascalOp <&'a X, Output = O>
			{
				type Output = O;
			}

			pub trait [<$PascalOp sToSelf>] <X>: [<$PascalOp s>] <X, Output = X>
			{
			}

			impl <X, T> [<$PascalOp sToSelf>] <X> for T
			where T: [<$PascalOp s>] <X, Output = X>
			{
			}
		}
	}
}

pub (crate) use __def_unary_op as def_unary_op;

macro_rules! __def_binary_op
{
	($PascalOp: ident, $snake_op: ident) =>
	{
		paste::paste!
		{
			#[forward_traits::forwardable]
			pub trait $PascalOp <X, Y>
			{
				type Output;

				fn $snake_op (self, x: X, y: Y) -> Self::Output;
			}

			pub trait [<$PascalOp s>] <X, Y>:
				$PascalOp <X, Y, Output = <Self as [<$PascalOp s>] <X, Y>>::Output>
				+ for <'a> $PascalOp <&'a X, Y, Output = <Self as [<$PascalOp s>] <X, Y>>::Output>
				+ for <'a> $PascalOp <X, &'a Y, Output = <Self as [<$PascalOp s>] <X, Y>>::Output>
				+ for <'a, 'b> $PascalOp <&'a X, &'b Y, Output = <Self as [<$PascalOp s>] <X, Y>>::Output>
			{
				type Output;
			}

			impl <X, Y, T, O> [<$PascalOp s>] <X, Y> for T
			where T: $PascalOp <X, Y, Output = O>
				+ for <'a> $PascalOp <&'a X, Y, Output = O>
				+ for <'a> $PascalOp <X, &'a Y, Output = O>
				+ for <'a, 'b> $PascalOp <&'a X, &'b Y, Output = O>
			{
				type Output = O;
			}

			#[forward_traits::forwardable]
			pub trait [<$PascalOp Assign>] <X, Y>
			{
				fn [<$snake_op _assign>] (self, x: &mut X, y: Y);
			}

			pub trait [<$PascalOp Assigns>] <X, Y>:
				[<$PascalOp Assign>] <X, Y>
				+ for <'a> [<$PascalOp Assign>] <X, &'a Y>
			{
			}

			impl <X, Y, T> [<$PascalOp Assigns>] <X, Y> for T
			where T: [<$PascalOp Assign>] <X, Y>
				+ for <'a> [<$PascalOp Assign>] <X, &'a Y>
			{
			}

			pub trait [<$PascalOp sToLHS>] <X, Y>:
				[<$PascalOp s>] <X, Y, Output = X>
				+ [<$PascalOp Assigns>] <X, Y>
			{
			}

			impl <X, Y, T> [<$PascalOp sToLHS>] <X, Y> for T
			where T: [<$PascalOp s>] <X, Y, Output = X>
				+ [<$PascalOp Assigns>] <X, Y>
			{
			}

			pub trait [<$PascalOp sToSelf>] <X>:
				[<$PascalOp s>] <X, X, Output = X>
				+ [<$PascalOp Assigns>] <X, X>
			{
			}

			impl <X, T> [<$PascalOp sToSelf>] <X> for T
			where T: [<$PascalOp s>] <X, X, Output = X>
				+ [<$PascalOp Assigns>] <X, X>
			{
			}
		}
	}
}

pub (crate) use __def_binary_op as def_binary_op;

macro_rules! __def_symmetric_binary_op
{
	($PascalOperation: ident, $PascalOp: ident, $snake_op: ident) =>
	{
		def_binary_op! ($PascalOp, $snake_op);

		paste::paste!
		{
			pub trait [<Symmetric $PascalOp s>] <X, Y>:
				[<$PascalOp s>] <X, Y, Output = <Self as [<Symmetric $PascalOp s>] <X, Y>>::Output>
				+ [<$PascalOp s>] <Y, X, Output = <Self as [<Symmetric $PascalOp s>] <X, Y>>::Output>
			{
				type Output;
			}

			impl <X, Y, T, O> [<Symmetric $PascalOp s>] <X, Y> for T
			where T: [<$PascalOp s>] <X, Y, Output = O>
				+ [<$PascalOp s>] <Y, X, Output = O>
			{
				type Output = O;
			}

			#[forward_traits::forwardable]
			pub trait [<$PascalOperation IsCommutative>] <X, Y>
			{
			}

			pub trait [<Comm $PascalOp s>] <X, Y>:
				[<$PascalOperation IsCommutative>] <X, Y>
				+ [<Symmetric $PascalOp s>] <X, Y, Output = <Self as [<Comm $PascalOp s>] <X, Y>>::Output>
			{
				type Output;
			}

			impl <X, Y, T, O> [<Comm $PascalOp s>] <X, Y> for T
			where T: [<$PascalOperation IsCommutative>] <X, Y>
				+ [<Symmetric $PascalOp s>] <X, Y, Output = O>
			{
				type Output = O;
			}

			pub trait [<Comm $PascalOp sToLHS>] <X, Y>:
				[<$PascalOperation IsCommutative>] <X, Y>
				+ [<$PascalOp sToLHS>] <X, Y>
				+ [<$PascalOp s>] <Y, X, Output = X>
			{
			}

			impl <X, T, Y> [<Comm $PascalOp sToLHS>] <X, Y> for T
			where T: [<$PascalOperation IsCommutative>] <X, Y>
				+ [<$PascalOp sToLHS>] <X, Y>
				+ [<$PascalOp s>] <Y, X, Output = X>
			{
			}

			pub trait [<Comm $PascalOp sToSelf>] <X>:
				[<$PascalOperation IsCommutative>] <X, X>
				+ [<$PascalOp sToSelf>] <X>
			{
			}

			impl <X, T> [<Comm $PascalOp sToSelf>] <X> for T
			where T: [<$PascalOperation IsCommutative>] <X, X>
				+ [<$PascalOp sToSelf>] <X>
			{
			}
		}
	}
}

pub (crate) use __def_symmetric_binary_op as def_symmetric_binary_op;
