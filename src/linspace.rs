use core::{mem::MaybeUninit, ops::{Add, Range, RangeInclusive}};

use bulks::Bulk;
use numscale::NumScale;

use crate::{Linspaced, IntoIter};

#[const_trait]
pub trait Linspace<T>: Sized
{
    type Output: Bulk<Item = T>;

    /// Returns an iterator of evenly spaced values. `count` must be specified.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use linspace::*;
    /// 
    /// let x: Vec<u32> = (0..100).linspace(4).collect();
    /// assert_eq!(x, [0, 25, 50, 75]);
    /// 
    /// let x: Vec<u32> = (0..=100).linspace(5).collect();
    /// assert_eq!(x, [0, 25, 50, 75, 100]);
    /// ```
    fn linspace(&self, count: usize) -> Self::Output;

    /// Returns an array of evenly spaced values.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use linspace::*;
    /// 
    /// let x: [u32; 4] = (0..100).linspace_array();
    /// assert_eq!(x, [0, 25, 50, 75]);
    /// 
    /// let x: [u32; 5] = (0..=100).linspace_array();
    /// assert_eq!(x, [0, 25, 50, 75, 100]);
    /// ```
    fn linspace_array<const N: usize>(&self) -> [T; N]
    {
        let mut array = [const {MaybeUninit::uninit()}; N];
        unsafe {
            self.linspace_uninit_slice(&mut array);
            MaybeUninit::array_assume_init(array)
        }
    }

    /// Fills a slice with evenly spaced values.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use linspace::*;
    /// 
    /// let mut x = [0, 0, 0, 0, 0];
    /// 
    /// (0..=100).linspace_slice(&mut x);
    /// assert_eq!(x, [0, 25, 50, 75, 100]);
    /// ```
    fn linspace_slice(&self, slice: &mut [T]);

    /// Fills a slice with evenly spaced values.
    /// 
    /// # Safety
    /// 
    /// Values in the slice will be overwritten. They will not be dropped.
    /// 
    /// If `T` is not trivially droppable, the slice must be uninitialized.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use core::mem::MaybeUninit;
    /// 
    /// use linspace::*;
    /// 
    /// let mut x = [const {MaybeUninit::uninit()}; 5];
    /// 
    /// let x = unsafe {
    ///     (0..=100).linspace_uninit_slice(&mut x)
    /// };
    /// assert_eq!(x, [0, 25, 50, 75, 100]);
    /// ```
    unsafe fn linspace_uninit_slice<'a>(&self, slice: &'a mut [MaybeUninit<T>]) -> &'a mut [T];
}

macro_rules! impl_linspace {
    ($($r:ident => $incl:expr => |$this:ident| $bounds:expr);*$(;)?) => {
        $(
            impl<T> const Linspace<T> for $r<T>
            where
                T: Copy + ~const Add<Output = T> + ~const NumScale<f64>
            {
                type Output = Linspaced<T, $incl>;

                fn linspace(&self, count: usize) -> Self::Output
                {
                    let $this = self;
                    let (start, end): (T, T) = $bounds;
                    Linspaced::new(start, end, count)
                }
                unsafe fn linspace_uninit_slice<'a>(&self, slice: &'a mut [MaybeUninit<T>]) -> &'a mut [T]
                {
                    let mut iter = IntoIter::new(self.linspace(slice.len()));
                    let mut i = 0;
                    while let Some(next) = iter.forward()
                    {
                        slice[i].write(next);
                        i += 1;
                    }
                    unsafe {
                        slice.assume_init_mut()
                    }
                }
                fn linspace_slice(&self, slice: &mut [T])
                {
                    let mut iter = IntoIter::new(self.linspace(slice.len()));
                    let mut i = 0;
                    while let Some(next) = iter.forward()
                    {
                        slice[i] = next;
                        i += 1;
                    }
                }
            }
        )*
    };
}

impl_linspace!(
    Range => false => |range| (range.start, range.end);
    RangeInclusive => true => |range| (*range.start(), *range.end());
);

#[cfg(test)]
mod test
{
    use bulks::Bulk;

    use crate::{Linspace};

    #[test]
    fn linspace()
    {
        let vec: Vec<u32> = (0..100).linspace(4).collect();
        println!("{:?}", vec);

        let array: [u32; 4] = (0..100).linspace_array();
        println!("{:?}", array);

        assert_eq!(vec, array);
        assert_eq!(vec, [0, 25, 50, 75]);
        assert_eq!(array, [0, 25, 50, 75]);
    }

    #[test]
    fn linspace_inclusive()
    {
        let vec_inclusive: Vec<u32> = (0..=100).linspace(5).collect();
        println!("{:?}", vec_inclusive);

        let array_inclusive: [u32; 5] = (0..=100).linspace_array();
        println!("{:?}", array_inclusive);

        assert_eq!(vec_inclusive, array_inclusive);
        assert_eq!(vec_inclusive, [0, 25, 50, 75, 100]);
        assert_eq!(array_inclusive, [0, 25, 50, 75, 100]);
    }

    #[test]
    fn logspace()
    {
        let vec: Vec<f32> = (0.0..1.0)
            .linspace(10)
            .map(|x| 10.0f32.powf(x))
            .collect();
        println!("{:?}", vec);
    }
}