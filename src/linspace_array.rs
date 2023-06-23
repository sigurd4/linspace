use std::ops::{Add, Sub, Div, Range, RangeInclusive};

use numscale::NumScale;

use super::*;

#[const_trait]
pub trait LinspaceArray<T, const LENGTH: usize>: Linspace<T>
{
    fn linspace_array(self) -> [T; LENGTH];
}

impl<T, const LENGTH: usize> const LinspaceArray<T, LENGTH> for Range<T>
where
    T: ~const Add<T, Output = T> + ~const Sub<T, Output = T> + ~const Div<T, Output = T> + ~const NumScale<f64> + Copy
{
    fn linspace_array(self) -> [T; LENGTH]
    {
        let [start, end]: [T; 2] = unsafe {std::mem::transmute_copy::<Self, [T; 2]>(&self)};
        if LENGTH == 0
        {
            return [start; LENGTH]
        }

        let range = end - start;
        let mut a = [start; LENGTH];
        let mut i = 0;
        while i < LENGTH
        {
            a[i] = start + range.scale((i as f64)/(LENGTH as f64));
            i += 1;
        }
        a
    }
}
impl<T, const LENGTH: usize> const LinspaceArray<T, LENGTH> for RangeInclusive<T>
where
    T: ~const Add<T, Output = T> + ~const Sub<T, Output = T> + ~const Div<T, Output = T> + ~const NumScale<f64> + Copy
{
    fn linspace_array(self) -> [T; LENGTH]
    {
        let (start, end, _): (T, T, bool) = unsafe {std::mem::transmute_copy::<Self, (T, T, bool)>(&self)};
        if LENGTH == 0
        {
            return [start; LENGTH]
        }

        let range = end - start;
        let mut a = [start; LENGTH];
        if LENGTH == 1
        {
            a[0] = start;
            return a;
        }
        let mut i = 0;
        while i < LENGTH
        {
            a[i] = start + range.scale((i as f64)/((LENGTH - 1) as f64));
            i += 1;
        }
        a
    }
}