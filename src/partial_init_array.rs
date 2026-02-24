use std::mem::{ManuallyDrop, MaybeUninit};

pub struct PartialInitArray <T, const N: usize>
{
	num_init: usize,
	array: ManuallyDrop <[MaybeUninit <T>; N]>
}

impl <T, const N: usize> PartialInitArray <T, N>
{
	pub fn new () -> Self
	{
		Self
		{
			num_init: 0,
			array: ManuallyDrop::new ([const { MaybeUninit::uninit () }; N])
		}
	}

	pub unsafe fn push_unchecked (&mut self, e: T)
	{
		self . array . get_unchecked_mut (self . num_init) . write (e);
		self . num_init = self . num_init . unchecked_add (1);
	}

	pub unsafe fn into_init_array (mut self) -> [T; N]
	{
		MaybeUninit::array_assume_init (ManuallyDrop::take (&mut self . array))
	}
}

impl <T, const N: usize> Drop for PartialInitArray <T, N>
{
	fn drop (&mut self)
	{
		unsafe
		{
			self
				. array
				. get_unchecked_mut (.. self . num_init)
				. assume_init_drop ();

			ManuallyDrop::drop (&mut self . array);
		}
	}
}
