use core::{iter::{FusedIterator, TrustedLen, TrustedRandomAccessNoCoerce}, ops::Add};

use numscale::NumScale;

use crate::Linspaced;

pub struct AsIter<T, const INCLUSIVE: bool = false>
{
    linspace: Linspaced<T, INCLUSIVE>
}

pub type IterInclusive<T> = AsIter<T, true>;

impl<T, const INCLUSIVE: bool> AsIter<T, INCLUSIVE>
{
    pub(crate) const fn new(linspace: Linspaced<T, INCLUSIVE>) -> Self
    {
        Self {
            linspace
        }
    }
}

impl<T, const INCLUSIVE: bool> From<AsIter<T, INCLUSIVE>> for Linspaced<T, INCLUSIVE>
{
    fn from(value: AsIter<T, INCLUSIVE>) -> Self
    {
        let AsIter { linspace } = value;
        linspace
    }
}

impl<T, const INCLUSIVE: bool> Iterator for AsIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        self.linspace.forward()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        self.linspace.size_hint()
    }

    #[inline]
    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item
    where
        Self: TrustedRandomAccessNoCoerce
    {
        // SAFETY: the caller must uphold the contract for
        // `Iterator::__iterator_get_unchecked`.
        unsafe { self.linspace.__iterator_get_unchecked(idx) }
    }
}

impl<T, const INCLUSIVE: bool> DoubleEndedIterator for AsIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.linspace.backward()
    }
}

impl<T, const INCLUSIVE: bool> ExactSizeIterator for AsIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    fn len(&self) -> usize
    {
        self.linspace.len()
    }

    fn is_empty(&self) -> bool
    {
        self.linspace.is_empty()
    }
}

impl<T, const INCLUSIVE: bool> FusedIterator for AsIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>,
    Linspaced<T, INCLUSIVE>: FusedIterator
{
    
}
unsafe impl<T, const INCLUSIVE: bool> TrustedLen for AsIter<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>,
    Linspaced<T, INCLUSIVE>: TrustedLen
{
    
}

#[cfg(test)]
mod test
{
    #[allow(unused)]
    use bulks::*;

    use crate::Linspace;

    #[test]
    fn it_works()
    {
        let r = (0.0..=1.0).linspace(5)
            .as_iter()
            .map(|x| 1.0 - x)
            .collect::<Vec<_>>();

        assert_eq!(r, [1.0, 0.75, 0.5, 0.25, 0.0])
    }
}