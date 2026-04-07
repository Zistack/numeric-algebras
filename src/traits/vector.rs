// Elements Sum

#[macrospace::item]
pub trait ElementsSum <X>
{
	type Output;

	fn elements_sum (self, x: X) -> Self::Output;
}

pub trait ElementsSums <X>:
	ElementsSum <X, Output = <Self as ElementsSums <X>>::Output>
	+ for <'a> ElementsSum <&'a X, Output = <Self as ElementsSums <X>>::Output>
{
	type Output;
}

impl <X, O, T> ElementsSums <X> for T
where T: ElementsSum <X, Output = O> + for <'a> ElementsSum <&'a X, Output = O>
{
	type Output = O;
}

// Elements Product

#[macrospace::item]
pub trait ElementsProduct <X>
{
	type Output;

	fn elements_product (self, x: X) -> Self::Output;
}

pub trait ElementsProducts <X>:
	ElementsProduct <X, Output = <Self as ElementsProducts <X>>::Output>
	+ for <'a> ElementsProduct <&'a X, Output = <Self as ElementsProducts <X>>::Output>
{
	type Output;
}

impl <X, O, T> ElementsProducts <X> for T
where T: ElementsProduct <X, Output = O> + for <'a> ElementsProduct <&'a X, Output = O>
{
	type Output = O;
}

// P-Norm

#[macrospace::item]
pub trait PNorm <X, P>
{
	type Output;

	fn p_norm (self, x: X, p: P) -> Self::Output;

	fn p_sum (self, x: X, p: P) -> Self::Output;
}

pub trait PNorms <X, P>:
	PNorm <X, P, Output = <Self as PNorms <X, P>>::Output>
	+ for <'a> PNorm <X, &'a P, Output = <Self as PNorms <X, P>>::Output>
	+ for <'a> PNorm <&'a X, P, Output = <Self as PNorms <X, P>>::Output>
	+ for <'a, 'b> PNorm <&'a X, &'b P, Output = <Self as PNorms <X, P>>::Output>
{
	type Output;
}

impl <X, P, O, T> PNorms <X, P> for T
where T: PNorm <X, P, Output = O>
	+ for <'a> PNorm <X, &'a P, Output = O>
	+ for <'a> PNorm <&'a X, P, Output = O>
	+ for <'a, 'b> PNorm <&'a X, &'b P, Output = O>
{
	type Output = O;
}

// Euclidean Norm

#[macrospace::item]
pub trait EuclideanNorm <X>
{
	type Output;

	fn euclidean_norm (self, x: X) -> Self::Output;

	fn sum_of_squares (self, x: X) -> Self::Output;
}

pub trait EuclideanNorms <X>:
	EuclideanNorm <X, Output = <Self as EuclideanNorms <X>>::Output>
	+ for <'a> EuclideanNorm <&'a X, Output = <Self as EuclideanNorms <X>>::Output>
{
	type Output;
}

impl <X, O, T> EuclideanNorms <X> for T
where T: EuclideanNorm <X, Output = O> + for <'a> EuclideanNorm <&'a X, Output = O>
{
	type Output = O;
}
