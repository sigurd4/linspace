#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
#![feature(trusted_random_access)]
#![feature(exact_size_is_empty)]
#![feature(trusted_len)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_cmp)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_slice)]

//! Turns a range into a linearly spaced sequence of values.
//! 
//! - [`Linspace::linspace`] returns an iterator.
//! 
//! - [`Linspace::linspace_array`] returns an array.
//! 
//! Only works on bounded ranges like [`Range`](core::ops::Range) and [`RangeInclusive`](core::ops::RangeInclusive).
//! 
//! ## Examples
//! 
//! Both of these will print `[0, 25, 50, 75]`.
//! 
//! ```rust
//! use linspace::*;
//! 
//! let vec: Vec<u32> = (0..100).linspace(4).collect();
//! println!("{:?}", vec);
//! 
//! let array: [u32; 4] = (0..100).linspace_array();
//! println!("{:?}", array);
//! 
//! assert_eq!(vec, array);
//! assert_eq!(vec, [0, 25, 50, 75]);
//! assert_eq!(array, [0, 25, 50, 75]);
//! ```
//! 
//! Both inclusive and exclusive ranges can be used.
//! And these will print `[0, 25, 50, 75, 100]`.
//! 
//! ```rust
//! use linspace::*;
//! 
//! let vec_inclusive: Vec<u32> = (0..=100).linspace(5).collect();
//! println!("{:?}", vec_inclusive);
//! 
//! let array_inclusive: [u32; 5] = (0..=100).linspace_array();
//! println!("{:?}", array_inclusive);
//! 
//! assert_eq!(vec_inclusive, array_inclusive);
//! assert_eq!(vec_inclusive, [0, 25, 50, 75, 100]);
//! assert_eq!(array_inclusive, [0, 25, 50, 75, 100]);
//! ```
//! 
//! Want a non-linear range? That's also possible. After all, [`Linspace::linspace`] just returns an [`Iterator`].
//! 
//! ```rust
//! use linspace::*;
//! 
//! let vec: Vec<f32> = (0.0..1.0)
//!     .linspace(10)
//!     .map(|x| 10.0f32.powf(x))
//!     .collect();
//! println!("{:?}", vec);
//! ```
//! 
//! Very convenient!

moddef::moddef!(
    flat(pub) mod {
        linspace,
        linspaced
    }
);