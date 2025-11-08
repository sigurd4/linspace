use core::{iter::{FusedIterator, TrustedLen, TrustedRandomAccessNoCoerce}, marker::Destruct, ops::{Add, Range, Try}};

use bulks::{Bulk, DoubleEndedBulk, IntoBulk};
use numscale::NumScale;

use crate::IntoIter;

#[derive(Clone, Copy)]
pub struct Linspaced<T, const INCLUSIVE: bool>
{
    start: T,
    end: T,
    len: usize,
}

impl<T, const INCLUSIVE: bool> Linspaced<T, INCLUSIVE>
{
    pub(crate) const fn new(start: T, end: T, len: usize) -> Self
    where
        T: Copy
    {
        Self {
            start,
            end,
            len
        }
    }

    pub(crate) const fn scale(self, i: usize) -> T
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        let m = self.len.saturating_sub(INCLUSIVE as usize + 1) + 1;
        let div = m as f64;
        self.start.scale(m.saturating_sub(i) as f64/div)
        + self.end.scale(i as f64/div)
    }

    pub(crate) const fn f<'a>(self) -> impl Fn(usize) -> T + 'a
    where
        T: Copy + NumScale<f64> + Add<Output = T> + 'a
    {
        move |i| self.scale(i)
    }

    pub const fn len(&self) -> usize
    {
        self.len
    }
}

impl<T, const INCLUSIVE: bool> const Bulk for Linspaced<T, INCLUSIVE>
where
    T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
{
    fn len(&self) -> usize
    {
        self.len()
    }
    fn for_each<F>(self, mut f: F)
    where
        Self: Sized,
        F: ~const FnMut(Self::Item) + ~const Destruct
    {
        let mut iter = IntoIter::new(self);
        while let Some(next) = iter.forward()
        {
            f(next)
        }
    }
    fn try_for_each<F, R>(self, mut f: F) -> R
    where
        Self: Sized,
        Self::Item: ~const Destruct,
        F: ~const FnMut(Self::Item) -> R + ~const Destruct,
        R: ~const Try<Output = (), Residual: ~const Destruct>
    {
        let mut iter = IntoIter::new(self);
        while let Some(next) = iter.forward()
        {
            f(next)?
        }
        R::from_output(())
    }
}

impl<T, const INCLUSIVE: bool> const DoubleEndedBulk for Linspaced<T, INCLUSIVE>
where
    T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
{
    fn rev_for_each<F>(self, mut f: F)
    where
        Self: Sized,
        F: ~const FnMut(Self::Item) + ~const Destruct
    {
        let mut iter = IntoIter::new(self);
        while let Some(next) = iter.backward()
        {
            f(next)
        }
    }
    fn try_rev_for_each<F, R>(self, mut f: F) -> R
    where
        Self: Sized,
        Self::Item: ~const Destruct,
        F: ~const FnMut(Self::Item) -> R + ~const Destruct,
        R: ~const Try<Output = (), Residual: ~const Destruct>
    {
        let mut iter = IntoIter::new(self);
        while let Some(next) = iter.backward()
        {
            f(next)?
        }
        R::from_output(())
    }
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