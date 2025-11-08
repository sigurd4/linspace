use core::{iter::{FusedIterator, TrustedLen, TrustedRandomAccessNoCoerce}, ops::{Add, Range, Try}};

use numscale::NumScale;

use crate::{Linspaced, linspace};

#[derive(Clone)]
pub struct IntoIter<T, const INCLUSIVE: bool>
{
    linspace: Linspaced<T, INCLUSIVE>,
    count: Range<usize>
}

impl<T, const INCLUSIVE: bool> IntoIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    pub(crate) const fn new(linspace: Linspaced<T, INCLUSIVE>) -> Self
    {
        Self {
            count: 0..linspace.len(),
            linspace
        }
    }
}

impl<T, const INCLUSIVE: bool> IntoIterator for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    type Item = T;
    type IntoIter = crate::iter::IntoIter<T, INCLUSIVE>;

    fn into_iter(self) -> Self::IntoIter
    {
        IntoIter::new(self)
    }
}

impl<T, const INCLUSIVE: bool> IntoIter<T, INCLUSIVE>
{
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
        let x = self.linspace.scale(self.count.start);
        self.count.start += 1;
        x
    }

    pub(crate) const fn forward(&mut self) -> Option<T>
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        if !(&*self).is_empty()
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
        let x = self.linspace.scale(self.count.end);
        x
    }

    pub(crate) const fn backward(&mut self) -> Option<T>
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        if !(&*self).is_empty()
        {
            return Some(self.forward_unchecked())
        }
        None
    }
}

impl<T, const INCLUSIVE: bool> Iterator for IntoIter<T, INCLUSIVE>
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
        unsafe { self.linspace.f()(self.count.__iterator_get_unchecked(idx)) }
    }
}

impl<T, const INCLUSIVE: bool> DoubleEndedIterator for IntoIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.backward()
    }
}

impl<T, const INCLUSIVE: bool> ExactSizeIterator for IntoIter<T, INCLUSIVE>
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

impl<T, const INCLUSIVE: bool> FusedIterator for IntoIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    
}
unsafe impl<T, const INCLUSIVE: bool> TrustedLen for IntoIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    
}

#[cfg(test)]
mod test
{
    use bulks::Bulk;

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