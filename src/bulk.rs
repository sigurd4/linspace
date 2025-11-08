use core::{marker::Destruct, ops::{Add, Try}};

use bulks::Bulk;
use numscale::NumScale;

use crate::{AsIter, Linspaced};

pub struct AsBulk<T, const INCLUSIVE: bool = false>
{
    linspace: Linspaced<T, INCLUSIVE>
}

pub type BulkInclusive<T> = AsBulk<T, true>;

impl<T, const INCLUSIVE: bool> AsBulk<T, INCLUSIVE>
{
    pub(crate) const fn new(linspace: Linspaced<T, INCLUSIVE>) -> Self
    {
        Self {
            linspace
        }
    }
}

impl<T, const INCLUSIVE: bool> From<AsBulk<T, INCLUSIVE>> for Linspaced<T, INCLUSIVE>
{
    fn from(value: AsBulk<T, INCLUSIVE>) -> Self
    {
        let AsBulk { linspace } = value;
        linspace
    }
}

impl<T, const INCLUSIVE: bool> IntoIterator for AsBulk<T, INCLUSIVE>
where
    T: Copy + NumScale<f64> + Add<Output = T>
{
    type Item = T;
    type IntoIter = AsIter<T, INCLUSIVE>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.linspace.as_iter()
    }
}
impl<T, const INCLUSIVE: bool> const Bulk for AsBulk<T, INCLUSIVE>
where
    T: Copy + ~const NumScale<f64> + ~const Add<Output = T>
{
    fn len(&self) -> usize
    {
        Bulk::len(&self.linspace)
    }
    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: ~const FnMut(Self::Item) + ~const Destruct
    {
        Bulk::for_each(self.linspace, f)
    }
    fn try_for_each<F, R>(self, f: F) -> R
    where
        Self: Sized,
        Self::Item: ~const Destruct,
        F: ~const FnMut(Self::Item) -> R + ~const Destruct,
        R: ~const Try<Output = (), Residual: ~const Destruct>
    {
        Bulk::try_for_each(self.linspace, f)
    }
}

#[cfg(test)]
mod test
{
    use bulks::Bulk;

    use crate::Linspace;

    #[test]
    fn it_works()
    {
        let r = (0.0..=1.0).linspace(5)
            .as_bulk()
            .map(|x| 1.0 - x)
            .collect::<Vec<_>>();

        assert_eq!(r, [1.0, 0.75, 0.5, 0.25, 0.0])
    }
}