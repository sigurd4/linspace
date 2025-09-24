use core::{mem::MaybeUninit, ops::{Add, Range, RangeBounds, RangeInclusive, Sub}};

use numscale::NumScale;

use crate::Linspaced;

#[const_trait]
pub trait Linspace<T>: RangeBounds<T> + Sized
{
    type Output: ExactSizeIterator<Item = T>;

    fn linspace_array<const N: usize>(&self) -> [T; N]
    {
        let mut array = [const {MaybeUninit::uninit()}; N];
        unsafe {
            self.linspace_slice_uninit(&mut array);
            MaybeUninit::array_assume_init(array)
        }
    }
    fn linspace_slice(&self, slice: &mut [T]);
    unsafe fn linspace_slice_uninit<'a>(&self, slice: &'a mut [MaybeUninit<T>]) -> &'a mut [T];
    fn linspace(&self, count: usize) -> Self::Output;
}

macro_rules! impl_linspace {
    ($($r:ident => $incl:expr => |$this:ident| $bounds:expr);*$(;)?) => {
        $(
            impl<T> const Linspace<T> for $r<T>
            where
                T: Copy + ~const Add<Output = T> + ~const Sub<Output = T> + ~const NumScale<f64>,
                Linspaced<T, $incl>: ExactSizeIterator<Item = T>
            {
                type Output = Linspaced<T, $incl>;

                unsafe fn linspace_slice_uninit<'a>(&self, slice: &'a mut [MaybeUninit<T>]) -> &'a mut [T]
                {
                    let mut iter = self.linspace(slice.len());
                    loop
                    {
                        let i = iter.pos();
                        if i >= iter.total_len()
                        {
                            break
                        }
                        slice[i].write(iter.next_unchecked());
                    }
                    slice.assume_init_mut()
                }
                fn linspace_slice(&self, slice: &mut [T])
                {
                    let mut iter = self.linspace(slice.len());
                    loop
                    {
                        let i = iter.pos();
                        if i >= iter.total_len()
                        {
                            break
                        }
                        slice[i] = iter.next_unchecked()
                    }
                }
                fn linspace(&self, count: usize) -> Self::Output
                {
                    let $this = self;
                    let (start, end): (T, T) = $bounds;
                    Linspaced::new(start, end, count)
                }
            }
        )*
    };
}

impl_linspace!(
    Range => false => |range| (range.start, range.end);
    RangeInclusive => true => |range| (*range.start(), *range.end());
);