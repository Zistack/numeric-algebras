pub use numeric_algebras_macros::{
	algebra_mapping,

	def_neg_traits,
	__def_neg_traits_inner,
	def_abs_traits,
	__def_abs_traits_inner,
	def_recip_traits,
	__def_recip_traits_inner,
	def_sqrt_traits,
	__def_sqrt_traits_inner,
	def_exp_traits,
	__def_exp_traits_inner,
	def_ln_traits,
	__def_ln_traits_inner,
	def_sin_traits,
	__def_sin_traits_inner,
	def_cos_traits,
	__def_cos_traits_inner,
	def_tan_traits,
	__def_tan_traits_inner,
	def_sin_cos_traits,
	__def_sin_cos_traits_inner,

	def_add_traits,
	__def_add_traits_inner,
	def_sub_traits,
	__def_sub_traits_inner,
	def_mul_traits,
	__def_mul_traits_inner,
	def_div_traits,
	__def_div_traits_inner,
	def_pow_traits,
	__def_pow_traits_inner,
	def_log_traits,
	__def_log_traits_inner,

	def_add_assign_traits,
	__def_add_assign_traits_inner,
	def_sub_assign_traits,
	__def_sub_assign_traits_inner,
	def_mul_assign_traits,
	__def_mul_assign_traits_inner,
	def_div_assign_traits,
	__def_div_assign_traits_inner,
	def_pow_assign_traits,
	__def_pow_assign_traits_inner,
	def_log_assign_traits,
	__def_log_assign_traits_inner,

	def_scalar_add_traits,
	__def_scalar_add_traits_inner,
	def_scalar_sub_traits,
	__def_scalar_sub_traits_inner,
	def_scalar_mul_traits,
	__def_scalar_mul_traits_inner,
	def_scalar_div_traits,
	__def_scalar_div_traits_inner,
	def_scalar_pow_traits,
	__def_scalar_pow_traits_inner,
	def_scalar_log_traits,
	__def_scalar_log_traits_inner,

	def_scalar_add_assign_traits,
	__def_scalar_add_assign_traits_inner,
	def_scalar_sub_assign_traits,
	__def_scalar_sub_assign_traits_inner,
	def_scalar_mul_assign_traits,
	__def_scalar_mul_assign_traits_inner,
	def_scalar_div_assign_traits,
	__def_scalar_div_assign_traits_inner,
	def_scalar_pow_assign_traits,
	__def_scalar_pow_assign_traits_inner,
	def_scalar_log_assign_traits,
	__def_scalar_log_assign_traits_inner,

	def_zero_trait,
	__def_zero_trait_inner,
	def_one_trait,
	__def_one_trait_inner,
	def_e_trait,
	__def_e_trait_inner,
	def_pi_trait,
	__def_pi_trait_inner,
	def_inf_trait,
	__def_inf_trait_inner,
	def_nan_trait,
	__def_nan_trait_inner,

	def_addition_is_commutative_trait,
	__def_addition_is_commutative_trait_inner,
	def_multiplication_is_commutative_trait,
	__def_multiplication_is_commutative_trait_inner,
	def_scalar_addition_is_commutative_trait,
	__def_scalar_addition_is_commutative_trait_inner,
	def_scalar_multiplication_is_commutative_trait,
	__def_scalar_multiplication_is_commutative_trait_inner,
};

#[macro_export]
macro_rules! __def_arithmetic__
{
	(
		for $(<$($generics: tt)*>)?
		(
			$AggregateType: path,
			$AccumulatorType: path
		)
		in
		(
			$AlgebraType: path,
			$AccumulatorAlgebraType: path
		)
	) =>
	{
		$crate::derive::def_neg_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_abs_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_recip_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_sqrt_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_exp_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_ln_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_sin_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_cos_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_tan_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);

		$crate::derive::def_sin_cos_traits!
		(
			for $(<$($generics)*>)? $AggregateType -> $AggregateType
			in $AlgebraType
		);

		$crate::derive::def_add_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $AggregateType) -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_sub_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $AggregateType) -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_mul_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $AggregateType) -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_div_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $AggregateType) -> $AggregateType
			in $AlgebraType
		);

		$crate::derive::def_add_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $AggregateType)
			in $AlgebraType
		);
		$crate::derive::def_sub_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $AggregateType)
			in $AlgebraType
		);
		$crate::derive::def_mul_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $AggregateType)
			in $AlgebraType
		);
		$crate::derive::def_div_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $AggregateType)
			in $AlgebraType
		);

		$crate::derive::def_addition_is_commutative_trait!
		(
			for $(<$($generics)*>)? ($AggregateType, $AggregateType)
			in $AlgebraType
		);
		$crate::derive::def_multiplication_is_commutative_trait!
		(
			for $(<$($generics)*>)? ($AggregateType, $AggregateType)
			in $AlgebraType
		);

		$crate::derive::def_zero_trait!
		(
			for $(<$($generics)*>)? $AggregateType in $AlgebraType
		);
		$crate::derive::def_one_trait!
		(
			for $(<$($generics)*>)? $AggregateType in $AlgebraType
		);
		$crate::derive::def_e_trait!
		(
			for $(<$($generics)*>)? $AggregateType in $AlgebraType
		);
		$crate::derive::def_pi_trait!
		(
			for $(<$($generics)*>)? $AggregateType in $AlgebraType
		);
		$crate::derive::def_inf_trait!
		(
			for $(<$($generics)*>)? $AggregateType in $AlgebraType
		);
		$crate::derive::def_nan_trait!
		(
			for $(<$($generics)*>)? $AggregateType in $AlgebraType
		);

		$crate::derive::def_add_assign_traits!
		(
			for $(<$($generics)*>)? ($AccumulatorType, $AggregateType)
			in $AccumulatorAlgebraType
		);
		$crate::derive::def_zero_trait!
		(
			for $(<$($generics)*>)? $AccumulatorType in $AccumulatorAlgebraType
		);
	}
}
pub use __def_arithmetic__ as def_arithmetic;

#[macro_export]
macro_rules! __def_scalar_arithmetic__
{
	(
		for $(<$($generics: tt)*>)?
		(
			$AggregateType: path,
			$ScalarType: ty
		)
		in
		(
			$AlgebraType: path,
			$ScalarAlgebraType: path
		)
	) =>
	{
		$crate::derive::def_scalar_mul_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $ScalarType) -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_scalar_div_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $ScalarType) -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_scalar_pow_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $ScalarType) -> $AggregateType
			in $AlgebraType
		);
		$crate::derive::def_scalar_log_traits!
		(
			for $(<$($generics)*>)?
			($AggregateType, $ScalarType) -> $AggregateType
			in $AlgebraType
		);

		$crate::derive::def_scalar_mul_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $ScalarType)
			in $AlgebraType
		);
		$crate::derive::def_scalar_div_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $ScalarType)
			in $AlgebraType
		);
		$crate::derive::def_scalar_pow_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $ScalarType)
			in $AlgebraType
		);
		$crate::derive::def_scalar_log_assign_traits!
		(
			for $(<$($generics)*>)? ($AggregateType, $ScalarType)
			in $AlgebraType
		);

		$crate::derive::def_scalar_multiplication_is_commutative_trait!
		(
			for $(<$($generics)*>)? ($AggregateType, $ScalarType)
			in $AlgebraType
		);

		forward_traits::forward_traits!
		(
			for $AlgebraType -> $ScalarAlgebraType
			impl $crate::traits::Neg <$ScalarType>
				+ for <'a> $crate::traits::Neg <&'a $ScalarType>
				+ $crate::traits::Abs <$ScalarType>
				+ for <'a> $crate::traits::Abs <&'a $ScalarType>
				+ $crate::traits::Recip <$ScalarType>
				+ for <'a> $crate::traits::Recip <&'a $ScalarType>
				+ $crate::traits::Sqrt <$ScalarType>
				+ for <'a> $crate::traits::Sqrt <&'a $ScalarType>
				+ $crate::traits::Exp <$ScalarType>
				+ for <'a> $crate::traits::Exp <&'a $ScalarType>
				+ $crate::traits::Ln <$ScalarType>
				+ for <'a> $crate::traits::Ln <&'a $ScalarType>
				+ $crate::traits::Sin <$ScalarType>
				+ for <'a> $crate::traits::Sin <&'a $ScalarType>
				+ $crate::traits::Cos <$ScalarType>
				+ for <'a> $crate::traits::Cos <&'a $ScalarType>
				+ $crate::traits::Tan <$ScalarType>
				+ for <'a> $crate::traits::Tan <&'a $ScalarType>
				+ $crate::traits::SinCos <$ScalarType>
				+ for <'a> $crate::traits::SinCos <&'a $ScalarType>

				+ $crate::traits::Add <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::Add <$ScalarType, &'a $ScalarType>
				+ for <'a> $crate::traits::Add <&'a $ScalarType, $ScalarType>
				+ for <'a, 'b> $crate::traits::Add <&'a $ScalarType, &'b $ScalarType>
				+ $crate::traits::AddAssign <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::AddAssign <$ScalarType, &'a $ScalarType>
				+ $crate::traits::Sub <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::Sub <$ScalarType, &'a $ScalarType>
				+ for <'a> $crate::traits::Sub <&'a $ScalarType, $ScalarType>
				+ for <'a, 'b> $crate::traits::Sub <&'a $ScalarType, &'b $ScalarType>
				+ $crate::traits::SubAssign <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::SubAssign <$ScalarType, &'a $ScalarType>
				+ $crate::traits::Mul <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::Mul <$ScalarType, &'a $ScalarType>
				+ for <'a> $crate::traits::Mul <&'a $ScalarType, $ScalarType>
				+ for <'a, 'b> $crate::traits::Mul <&'a $ScalarType, &'b $ScalarType>
				+ $crate::traits::MulAssign <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::MulAssign <$ScalarType, &'a $ScalarType>
				+ $crate::traits::Div <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::Div <$ScalarType, &'a $ScalarType>
				+ for <'a> $crate::traits::Div <&'a $ScalarType, $ScalarType>
				+ for <'a, 'b> $crate::traits::Div <&'a $ScalarType, &'b $ScalarType>
				+ $crate::traits::DivAssign <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::DivAssign <$ScalarType, &'a $ScalarType>
				+ $crate::traits::Pow <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::Pow <$ScalarType, &'a $ScalarType>
				+ for <'a> $crate::traits::Pow <&'a $ScalarType, $ScalarType>
				+ for <'a, 'b> $crate::traits::Pow <&'a $ScalarType, &'b $ScalarType>
				+ $crate::traits::PowAssign <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::PowAssign <$ScalarType, &'a $ScalarType>
				+ $crate::traits::Log <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::Log <$ScalarType, &'a $ScalarType>
				+ for <'a> $crate::traits::Log <&'a $ScalarType, $ScalarType>
				+ for <'a, 'b> $crate::traits::Log <&'a $ScalarType, &'b $ScalarType>
				+ $crate::traits::LogAssign <$ScalarType, $ScalarType>
				+ for <'a> $crate::traits::LogAssign <$ScalarType, &'a $ScalarType>

				+ $crate::traits::AdditionIsCommutative <$ScalarType, $ScalarType>
				+ $crate::traits::MultiplicationIsCommutative <$ScalarType, $ScalarType>

				+ $crate::traits::Zero <$ScalarType>
				+ $crate::traits::One <$ScalarType>
				+ $crate::traits::E <$ScalarType>
				+ $crate::traits::Pi <$ScalarType>
				+ $crate::traits::Inf <$ScalarType>
				+ $crate::traits::NaN <$ScalarType>

				+ $crate::traits::Accumulatable <$ScalarType>
				+ $crate::traits::Convert
				<
					<$AlgebraType as $crate::traits::Accumulatable <$ScalarType>>::Accumulator,
					$ScalarType
				>
				where $AlgebraType: $crate::traits::Accumulatable <$ScalarType>;
		);
	}
}
pub use __def_scalar_arithmetic__ as def_scalar_arithmetic;
