[![Build Status (nightly)](https://github.com/sigurd4/linspace/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/linspace/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/linspace/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/linspace/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/linspace/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/linspace/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/linspace/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/linspace/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/linspace/workflows/Test/badge.svg)](https://github.com/sigurd4/linspace/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/linspace/workflows/Lint/badge.svg)](https://github.com/sigurd4/linspace/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/linspace.svg)](https://crates.io/crates/linspace)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/linspace)](https://docs.rs/linspace)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/linspace)](https://app.codecov.io/github/sigurd4/linspace)

# linspace

Turns a range into a linearly spaced sequence of values.

- `Linspace::linspace` returns an iterator.

- `Linspace::linspace_array` returns an array.

Only works on bounded ranges like [`Range`](core::ops::Range) and [`RangeInclusive`](core::ops::RangeInclusive).

## Examples

Both of these will print `[0, 25, 50, 75]`.

```rust
use linspace::*;

let x: Vec<u32> = (0..100).linspace(4).collect();
assert_eq!(x, [0, 25, 50, 75]);
println!("{:?}", x);

let y: [u32; 4] = (0..100).linspace_array();
assert_eq!(y, [0, 25, 50, 75]);
println!("{:?}", y);

assert_eq!(x, y);
```

Both inclusive and exclusive ranges can be used.
And these will print `[0, 25, 50, 75, 100]`.

```rust
use linspace::*;

let x: Vec<u32> = (0..=100).linspace(5).collect();
assert_eq!(x, [0, 25, 50, 75, 100]);
println!("{:?}", x);

let y: [u32; 5] = (0..=100).linspace_array();
assert_eq!(y, [0, 25, 50, 75, 100]);
println!("{:?}", y);

assert_eq!(x, y);
```

Want a non-linear range? That's also possible. After all, `Linspace::linspace` just returns an `Iterator`.

```rust
use linspace::*;

let x: Vec<f32> = (0.0..1.0)
    .linspace(10)
    .map(|z| 10.0f32.powf(z))
    .collect();
println!("{:?}", x);
```

Very convenient!