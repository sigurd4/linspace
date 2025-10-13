use core::{iter::{FusedIterator, TrustedLen, TrustedRandomAccessNoCoerce}, ops::{Add, Range, Try}};

use numscale::NumScale;

#[derive(Clone)]
pub struct Linspaced<T, const INCLUSIVE: bool>
{
    start: T,
    end: T,
    count: Range<usize>
}

impl<T, const INCLUSIVE: bool> Linspaced<T, INCLUSIVE>
{
    pub(crate) const fn new(start: T, end: T, count: usize) -> Self
    where
        T: Copy
    {
        Self {
            start,
            end,
            count: 0..count
        }
    }

    pub(crate) const fn scale(start: T, end: T, i: usize, n: usize) -> T
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        let div = (n.saturating_sub(INCLUSIVE as usize)).max(1) as f64;
        start.scale(n.saturating_sub(i) as f64/div)
        + end.scale(i as f64/div)
    }

    const fn f<'a>(&self) -> impl Fn(usize) -> T + 'a
    where
        T: Copy + NumScale<f64> + Add<Output = T> + 'a
    {
        let start = self.start;
        let end = self.end;
        let n = self.count.end;
        move |i| Self::scale(start, end, i, n)
    }

    pub(crate) const fn next_unchecked(&mut self) -> T
    where
        T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
    {
        let x = Self::scale(self.start, self.end, self.count.start, self.count.end);
        self.count.start += 1;
        x
    }

    pub(crate) const fn pos(&self) -> usize
    {
        self.count.start
    }
    pub(crate) const fn total_len(&self) -> usize
    {
        self.count.end
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
        self.count.next().map(self.f())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        self.count.size_hint()
    }

    fn try_fold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R
    where
        Self: Sized,
        G: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        let f = self.f();
        self.count.try_fold(init, move |acc, elt| g(acc, f(elt)))
    }

    fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        let f = self.f();
        self.count.fold(init, move |acc, elt| g(acc, f(elt)))
    }

    #[inline]
    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> Self::Item
    where
        Self: TrustedRandomAccessNoCoerce,
    {
        // SAFETY: the caller must uphold the contract for
        // `Iterator::__iterator_get_unchecked`.
        unsafe { self.f()(self.count.__iterator_get_unchecked(idx)) }
    }
}

impl<T, const INCLUSIVE: bool> DoubleEndedIterator for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.count.next_back().map(self.f())
    }

    fn try_rfold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R
    where
        Self: Sized,
        G: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        let f = self.f();
        self.count.try_rfold(init, move |acc, elt| g(acc, f(elt)))
    }

    fn rfold<Acc, G>(self, init: Acc, mut g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        let f = self.f();
        self.count.rfold(init, move |acc, elt| g(acc, f(elt)))
    }
}

impl<T, const INCLUSIVE: bool> ExactSizeIterator for Linspaced<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    fn len(&self) -> usize
    {
        self.count.len()
    }

    fn is_empty(&self) -> bool
    {
        self.count.is_empty()
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

#[cfg(test)]
mod test
{
    use crate::Linspace;

    #[test]
    fn inclusive_bounds()
    {
        let r = (0.0..=50.0).linspace(128).last().unwrap();

        assert_eq!(r, 50.0);
    }
}