macro_rules! impl_unary_ops
{
	($T: ty, $Op: ident, $op: ident, $opsym: tt, $Algebra: ident) =>
	{
		impl $Op <$T> for $Algebra
		{
			type Output = $T;

			fn $op (self, x: $T) -> Self::Output
			{
				$opsym x
			}
		}

		impl <'a> $Op <&'a $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, x: &'a $T) -> Self::Output
			{
				$opsym x
			}
		}
	}
}
pub (crate) use impl_unary_ops;

macro_rules! impl_unary_methods
{
	($T: ty, $Op: ident, $op: ident, $Algebra: ident) =>
	{
		impl $Op <$T> for $Algebra
		{
			type Output = $T;

			fn $op (self, x: $T) -> Self::Output
			{
				x . $op ()
			}
		}

		impl <'a> $Op <&'a $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, x: &'a $T) -> Self::Output
			{
				x . $op ()
			}
		}
	}
}
pub (crate) use impl_unary_methods;

macro_rules! impl_sin_cos
{
	($T: ty, $Algebra: ident) =>
	{
		impl SinCos <$T> for $Algebra
		{
			type Output = $T;

			fn sin_cos (self, x: $T) -> (Self::Output, Self::Output)
			{
				x . sin_cos ()
			}
		}

		impl <'a> SinCos <&'a $T> for $Algebra
		{
			type Output = $T;

			fn sin_cos (self, x: &'a $T) -> (Self::Output, Self::Output)
			{
				x . sin_cos ()
			}
		}
	}
}
pub (crate) use impl_sin_cos;

macro_rules! impl_bin_ops
{
	($T: ty, $Op: ident, $op: ident, $opsym: tt, $Algebra: ident) =>
	{
		impl $Op <$T, $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: $T, rhs: $T) -> Self::Output
			{
				lhs $opsym rhs
			}
		}

		impl <'a> $Op <$T, &'a $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: $T, rhs: &'a $T) -> Self::Output
			{
				lhs $opsym rhs
			}
		}

		impl <'a> $Op <&'a $T, $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: &'a $T, rhs: $T) -> Self::Output
			{
				lhs $opsym rhs
			}
		}

		impl <'a, 'b> $Op <&'a $T, &'b $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: &'a $T, rhs: &'b $T) -> Self::Output
			{
				lhs $opsym rhs
			}
		}
	}
}
pub (crate) use impl_bin_ops;

macro_rules! impl_op_assigns
{
	($T: ty, $OpAssign: ident, $op_assign: ident, $opassignsym: tt, $Algebra: ident) =>
	{
		impl $OpAssign <$T, $T> for $Algebra
		{
			fn $op_assign (self, lhs: &mut $T, rhs: $T)
			{
				*lhs $opassignsym rhs;
			}
		}

		impl <'a> $OpAssign <$T, &'a $T> for $Algebra
		{
			fn $op_assign (self, lhs: &mut $T, rhs: &'a $T)
			{
				*lhs $opassignsym rhs;
			}
		}
	}
}
pub (crate) use impl_op_assigns;

macro_rules! impl_hetero_bin_methods
{
	($X: ty, $Y: ty, $Op: ident, $op: ident, $inner_op: ident, $Algebra: ident) =>
	{
		impl $Op <$X, $Y> for $Algebra
		{
			type Output = $X;

			fn $op (self, lhs: $X, rhs: $Y) -> Self::Output
			{
				lhs . $inner_op (rhs . into ())
			}
		}

		impl <'a> $Op <$X, &'a $Y> for $Algebra
		{
			type Output = $X;

			fn $op (self, lhs: $X, rhs: &'a $Y) -> Self::Output
			{
				lhs . $inner_op ((*rhs) . into ())
			}
		}

		impl <'a> $Op <&'a $X, $Y> for $Algebra
		{
			type Output = $X;

			fn $op (self, lhs: &'a $X, rhs: $Y) -> Self::Output
			{
				lhs . $inner_op (rhs . into ())
			}
		}

		impl <'a, 'b> $Op <&'a $X, &'b $Y> for $Algebra
		{
			type Output = $X;

			fn $op (self, lhs: &'a $X, rhs: &'b $Y) -> Self::Output
			{
				lhs . $inner_op ((*rhs) . into ())
			}
		}
	}
}
pub (crate) use impl_hetero_bin_methods;

macro_rules! impl_value_partialeq
{
	($T: ty, $Value: ident, $value: ident, $is_value: ident, $v: expr, $Algebra: ident) =>
	{
		impl $Value <$T> for $Algebra
		{
			fn $value (self) -> $T
			{
				$v
			}

			fn $is_value (self, x: &$T) -> bool
			{
				*x == $v
			}
		}
	}
}
pub (crate) use impl_value_partialeq;

macro_rules! impl_value_predicate
{
	($T: ty, $Value: ident, $value: ident, $is_value: ident, $v: expr, $Algebra: ident) =>
	{
		impl $Value <$T> for $Algebra
		{
			fn $value (self) -> $T
			{
				$v
			}

			fn $is_value (self, x: &$T) -> bool
			{
				x . $is_value ()
			}
		}
	}
}
pub (crate) use impl_value_predicate;
