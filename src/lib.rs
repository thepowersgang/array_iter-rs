#![no_std]
//! Provides a by-value iterator for fixed-size arrays (currently for arrays sized 0 to 32 items)
//!
//! ```
//! use array_iter::ArrayExt;
//!
//! let my_arr = [vec![1,2,3], vec![9; 3]];
//! for mut vec in my_arr.into_iter() {
//!     vec.push(4);
//!     println!("{:?}", vec);
//! }
//! ```

/// Extension trait for arrays providing 'into_iter'
pub trait ArrayExt: Sized + Array
{
	fn into_iter(self) -> Iter<Self>;
}

/// Trait representing the required features of an array (internal detail mainly)
pub unsafe trait Array
{
	type Item;
	fn len(&self) -> usize;
	fn get_ptr(&self, ofs: usize) -> *const Self::Item;
}

/// By-value array iterator
//pub struct Iter<T, const usize N> {
//    data: ::core::mem::ManuallyDrop<[T; N]>,
//    ofs: usize,
//}
pub struct Iter<T: Array>
{
	data: ::core::mem::ManuallyDrop<T>,
	ofs: usize,
}
impl<T: Array> Iterator for Iter<T>
{
	type Item = T::Item;
	fn next(&mut self) -> Option<T::Item>
	{
		if self.ofs == self.data.len() {
			None
		}
		else {
			// SAFE: Only ever read in sequence, and never dropped
			let rv = Some(unsafe {
				::core::ptr::read(self.data.get_ptr(self.ofs))
				});
			self.ofs += 1;
			rv
		}
	}
}
impl<T: Array> Drop for Iter<T>
{
	fn drop(&mut self) {
		for _ in self {
		}
	}
}

macro_rules! def {
	($($s:expr)+) => {
		$(
		unsafe impl<T> Array for [T; $s]
		{
			type Item = T;
			fn len(&self) -> usize { $s }
			fn get_ptr(&self, i: usize) -> *const Self::Item { &self[i] }
		}
		)+
		};
}
impl<T: Array> ArrayExt for T {
	fn into_iter(self) -> Iter<T> {
		Iter {
			data: ::core::mem::ManuallyDrop::new(self),
			ofs: 0,
			}
	}
}

// Create implementations for arrays sized from 0 to 32
def! {  0  1  2  3  4  5  6  7  8  9 }
def! { 10 11 12 13 14 15 16 17 18 19 }
def! { 20 21 22 23 24 25 26 27 28 29 }
def! { 30 31 32 }

#[cfg(test)]
mod tests
{
	use ::ArrayExt;

	struct DropTrace<'a>(&'a ::core::cell::Cell<isize>);
	impl<'a> Drop for DropTrace<'a>
	{
		fn drop(&mut self) { self.0.set(self.0.get() + 1); }
	}

    #[test]
    fn empty()
	{
		assert_eq!( [].into_iter().next(), None::<i8> );
	}

	#[test]
	fn one_item_used()
	{
		let v = Default::default();
		let mut it = [ DropTrace(&v) ].into_iter();
		assert_eq!( v.get(), 0 );
		assert!( it.next().is_some() );
		assert_eq!( v.get(), 1 );
		drop(it);
		assert_eq!( v.get(), 1 );
	}
		
	#[test]
	fn one_used_one_unused()
	{
		let v = Default::default();
		let mut it = [ DropTrace(&v), DropTrace(&v) ].into_iter();
		assert_eq!( v.get(), 0 );
		assert!( it.next().is_some() );
		assert_eq!( v.get(), 1 );
		drop(it);
		assert_eq!( v.get(), 2 );
	}
}
