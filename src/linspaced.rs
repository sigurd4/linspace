use core::{iter::{FusedIterator, TrustedLen, TrustedRandomAccessNoCoerce}, ops::{Add, Range}};

#[cfg(feature = "bulks")]
use core::{marker::Destruct, ops::Try};

#[cfg(feature = "bulks")]
use bulks::{Bulk, DoubleEndedBulk};
use numscale::NumScale;

#[cfg(feature = "bulks")]
use crate::{AsBulk, AsIter};

#[derive(Clone)]
pub struct Linspaced<T, const INCLUSIVE: bool = false>
{
    start: T,
    end: T,
    len: usize,
    count: Range<usize>
}

pub type LinspacedInclusive<T> = Linspaced<T, true>;

impl<T, const INCLUSIVE: bool> Linspaced<T, INCLUSIVE>
{
    pub(crate) const fn new(start: T, end: T, len: usize) -> Self
    where
        T: Copy
    {
        Self {
            start,
            end,
            len,
            count: 0..len
        }
    }

    #[cfg(feature = "bulks")]
    pub const fn as_iter(self) -> AsIter<T, INCLUSIVE>
    {
        AsIter::new(self)
    }

    #[cfg(feature = "bulks")]
    pub const fn as_bulk(self) -> AsBulk<T, INCLUSIVE>
    {
        AsBulk::new(self)
    }

    pub const fn is_empty(&self) -> bool
    {
        self.count.start >= self.count.end
    }
    pub const fn len(&self) -> usize
    {
        self.count.end.saturating_sub(self.count.start)
    }
    pub const fn size_hint(&self) -> (usize, Option<usize>)
    {
        (self.len(), self.count.end.checked_sub(self.count.start))
    }

    pub(crate) const fn forward_unchecked(&mut self) -> T
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        let x = self.scale(self.count.start);
        self.count.start += 1;
        x
    }

    pub(crate) const fn forward(&mut self) -> Option<T>
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        if !(*self).is_empty()
        {
            return Some(self.forward_unchecked())
        }
        None
    }

    pub(crate) const fn backward_unchecked(&mut self) -> T
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        self.count.end -= 1;
        self.scale(self.count.end)
    }

    pub(crate) const fn backward(&mut self) -> Option<T>
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        if !(*self).is_empty()
        {
            return Some(self.backward_unchecked())
        }
        None
    }

    #[inline]
    #[doc(hidden)]
    pub(crate) unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> T
    where
        T: Copy + NumScale<f64> + Add<Output = T>
    {
        unsafe { self.f()(self.count.__iterator_get_unchecked(idx)) }
    }

    const fn scale_vals(i: usize, len: usize, start: T, end: T) -> T
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        let m = len.saturating_sub(INCLUSIVE as usize + 1) + 1;
        let div = m as f64;
        start.scale(m.saturating_sub(i) as f64/div)
        + end.scale(i as f64/div)
    }

    pub(crate) const fn scale(&self, i: usize) -> T
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        let Self { start, end, len, count: _ } = *self;
        Self::scale_vals(i, len, start, end)
    }

    pub(crate) const fn f<'a>(&self) -> impl Fn(usize) -> T + 'a
    where
        T: Copy + NumScale<f64> + Add<Output = T> + 'a
    {
        let Self { start, end, len, count: _ } = *self;
        move |i| Self::scale_vals(i, len, start, end)
    }
}

impl<T, const INCLUSIVE: bool> Iterator for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        self.forward()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        self.size_hint()
    }

    #[inline]
    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item
    where
        Self: TrustedRandomAccessNoCoerce
    {
        // SAFETY: the caller must uphold the contract for
        // `Iterator::__iterator_get_unchecked`.
        unsafe {
            self.__iterator_get_unchecked(idx)
        }
    }
}

impl<T, const INCLUSIVE: bool> DoubleEndedIterator for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.backward()
    }
}

impl<T, const INCLUSIVE: bool> ExactSizeIterator for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    fn len(&self) -> usize
    {
        self.len()
    }

    fn is_empty(&self) -> bool
    {
        self.is_empty()
    }
}

impl<T, const INCLUSIVE: bool> FusedIterator for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    
}
unsafe impl<T, const INCLUSIVE: bool> TrustedLen for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    
}

#[cfg(feature = "bulks")]
impl<T, const INCLUSIVE: bool> const Bulk for Linspaced<T, INCLUSIVE>
where
    T: Copy + ~const NumScale<f64> + ~const Add<Output = T>,
    Self: IntoIterator<Item = T, IntoIter: ExactSizeIterator>
{
    fn len(&self) -> usize
    {
        self.len()
    }
    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: ~const FnMut(Self::Item) + ~const Destruct
    {
        while let Some(next) = self.forward()
        {
            f(next)
        }
    }
    fn try_for_each<F, R>(mut self, mut f: F) -> R
    where
        Self: Sized,
        Self::Item: ~const Destruct,
        F: ~const FnMut(Self::Item) -> R + ~const Destruct,
        R: ~const Try<Output = (), Residual: ~const Destruct>
    {
        while let Some(next) = self.forward()
        {
            f(next)?
        }
        R::from_output(())
    }
}

#[cfg(feature = "bulks")]
impl<T, const INCLUSIVE: bool> const DoubleEndedBulk for Linspaced<T, INCLUSIVE>
where
    T: Copy + ~const NumScale<f64> + ~const Add<Output = T>,
    Self: IntoIterator<Item = T, IntoIter: ExactSizeIterator + DoubleEndedIterator>
{
    fn rev_for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: ~const FnMut(Self::Item) + ~const Destruct
    {
        while let Some(next) = self.backward()
        {
            f(next)
        }
    }
    fn try_rev_for_each<F, R>(mut self, mut f: F) -> R
    where
        Self: Sized,
        Self::Item: ~const Destruct,
        F: ~const FnMut(Self::Item) -> R + ~const Destruct,
        R: ~const Try<Output = (), Residual: ~const Destruct>
    {
        while let Some(next) = self.backward()
        {
            f(next)?
        }
        R::from_output(())
    }
}

#[cfg(test)]
mod test
{
    use crate::Linspace;

    #[test]
    fn inclusive_bounds()
    {
        const DEGC_TO_K: f64 = 273.15;
        let theta_min = DEGC_TO_K;
        let theta_max = 50.0 + DEGC_TO_K;

        let r = (theta_min..=theta_max).linspace(128).last().unwrap();

        assert_eq!(r, theta_max);
    }
}