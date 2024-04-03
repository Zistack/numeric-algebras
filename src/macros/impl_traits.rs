macro_rules! impl_unary_ops
{
	($T: ty, $Op: ident, $op: ident, $opsym: tt, $Algebra: ty) =>
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
	($T: ty, $Op: ident, $op: ident, $Algebra: ty) =>
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
	($T: ty, $Algebra: ty) =>
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
	($T: ty, $Op: ident, $op: ident, $opsym: tt, $Algebra: ty) =>
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
	($T: ty, $OpAssign: ident, $op_assign: ident, $opassignsym: tt, $Algebra: ty) =>
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

macro_rules! impl_convert
{
	($X: ty, $Y: ty, $Algebra: ty) =>
	{
		impl Convert <$X, $Y> for $Algebra
		{
			fn convert (self, x: $X) -> $Y
			{
				<$Y>::from (x)
			}
		}
	}
}
pub (crate) use impl_convert;

macro_rules! impl_approx_convert
{
	($X: ty, $Y: ty, $Algebra: ty) =>
	{
		impl ApproxConvert <$X, $Y> for $Algebra
		{
			fn approx_convert (self, x: $X) -> $Y
			{
				(x as $Y)
			}
		}
	}
}
pub (crate) use impl_approx_convert;

macro_rules! impl_bin_methods
{
	(
		$T: ty,
		$Op: ident,
		$op: ident,
		$inner_op: ident,
		$Algebra: ty
	) =>
	{
		impl $Op <$T, $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: $T, rhs: $T) -> Self::Output
			{
				lhs . $inner_op (rhs)
			}
		}

		impl <'a> $Op <$T, &'a $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: $T, rhs: &'a $T) -> Self::Output
			{
				lhs . $inner_op (*rhs)
			}
		}

		impl <'a> $Op <&'a $T, $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: &'a $T, rhs: $T) -> Self::Output
			{
				lhs . $inner_op (rhs)
			}
		}

		impl <'a, 'b> $Op <&'a $T, &'b $T> for $Algebra
		{
			type Output = $T;

			fn $op (self, lhs: &'a $T, rhs: &'b $T) -> Self::Output
			{
				lhs . $inner_op (*rhs)
			}
		}
	}
}
pub (crate) use impl_bin_methods;

macro_rules! impl_value_partialeq
{
	($T: ty, $Value: ident, $value: ident, $is_value: ident, $v: expr, $Algebra: ty) =>
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
	($T: ty, $Value: ident, $value: ident, $is_value: ident, $v: expr, $Algebra: ty) =>
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
