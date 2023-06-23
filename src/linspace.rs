use std::ops::{Sub, Div, Add, Range, RangeBounds, RangeInclusive};

use numscale::NumScale;

#[const_trait]
pub trait Linspace<T>: RangeBounds<T>
{
    fn linspace(self, len: usize) -> Vec<T>;
}

impl<T> Linspace<T> for Range<T>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T> + NumScale<f64> + Copy
{
    fn linspace(self, len: usize) -> Vec<T>
    {
        let (start, end): (T, T) = unsafe {std::mem::transmute_copy::<Self, (T, T)>(&self)};
        let range = end - start;
        (0..len).map(|i| start + range.scale(i as f64/len as f64)).collect()
    }
}
impl<T> Linspace<T> for RangeInclusive<T>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T> + NumScale<f64> + Copy
{
    fn linspace(self, len: usize) -> Vec<T>
    {
        let (start, end, _): (T, T, bool) = unsafe {std::mem::transmute_copy::<Self, (T, T, bool)>(&self)};
        let range = end - start;
        if len == 1
        {
            return vec![start]
        }
        (0..len).map(|i| start + range.scale(i as f64/(len - 1) as f64)).collect()
    }
}