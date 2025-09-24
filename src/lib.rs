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
//! let x: Vec<u32> = (0..100).linspace(4).collect();
//! assert_eq!(x, [0, 25, 50, 75]);
//! println!("{:?}", vec);
//! 
//! let y: [u32; 4] = (0..100).linspace_array();
//! assert_eq!(y, [0, 25, 50, 75]);
//! println!("{:?}", array);
//! 
//! assert_eq!(x, y);
//! ```
//! 
//! Both inclusive and exclusive ranges can be used.
//! And these will print `[0, 25, 50, 75, 100]`.
//! 
//! ```rust
//! use linspace::*;
//! 
//! let x: Vec<u32> = (0..=100).linspace(5).collect();
//! assert_eq!(x, [0, 25, 50, 75, 100]);
//! println!("{:?}", x);
//! 
//! let y: [u32; 5] = (0..=100).linspace_array();
//! assert_eq!(y, [0, 25, 50, 75, 100]);
//! println!("{:?}", y);
//! 
//! assert_eq!(x, y);
//! ```
//! 
//! Want a non-linear range? That's also possible. After all, [`Linspace::linspace`] just returns an [`Iterator`].
//! 
//! ```rust
//! use linspace::*;
//! 
//! let x: Vec<f32> = (0.0..1.0)
//!     .linspace(10)
//!     .map(|z| 10.0f32.powf(z))
//!     .collect();
//! println!("{:?}", x);
//! ```
//! 
//! Very convenient!

moddef::moddef!(
    flat(pub) mod {
        linspace,
        linspaced
    }
);