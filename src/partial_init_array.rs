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

	// Safety: 0 <= self . num_init < N
	pub unsafe fn push_unchecked (&mut self, e: T)
	{
		self . array . get_unchecked_mut (self . num_init) . write (e);
		self . num_init = self . num_init . unchecked_add (1);
	}

	// Safety: self . num_init == N.
	pub unsafe fn into_init_array (mut self) -> [T; N]
	{
		let array = MaybeUninit::array_assume_init (ManuallyDrop::take (&mut self . array));

		// Because rust will call 'drop' on self at the end of this method, I
		// have to make sure that we don't try to drop the members of the array
		// that we just moved out of the array.  To that end, I am setting the
		// number of initialized entries to 0, which should prevent that
		// particular issue.
		self . num_init = 0;

		array
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
