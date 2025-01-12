# Interpolation Search

Interpolation search is an algorithm for searching in a sorted array. It improves upon the famous binary search by using linear interpolation to better estimate the target's position within the array. Interpolation search reduces the asymptotic time complexity of the search to _O(log log N)_. However, in the worst case scenario (array elements grow exponentially) the complexity becomes linear (_O(N)_).

This crate provides and implements the `InterpolationSearch` trait for slices (and consequently `Vec`s) to provide an `interpolation_search()` alternative to the existing `binary_search()`.

## Usage

1.  Add the crate to your `Cargo.toml` dependencies:

    ```
    cargo add interpolation_search
    ```

2.  Import the trait and the implementation:

    ```rust
    use interpolation_search::InterpolationSearch;
    ```

3.  Use the `interpolation_search` method on sorted arrays:

    ```rust
    let arr = [1, 2, 3, 4, 5];
    let target = 3;

    match arr.interpolation_search(&target) {
        Ok(idx) => println!("Target found at index {}", idx),
        Err(idx) => println!("Target not found, possible insertion point: {}", idx),
    }
    ```
## Enabling Interpolation Search for user-defined types

[`InterpolationSearch`] requires the items in the array to be [`Ord`] and [`Linear`] over some `Distance` type, such that `Distance` is [`Scalable`].

Consider a simple struct that represents points on a cartesian grid, an `(x, y)` pair. To have them sorted in an array they must be [`Ord`], of course, which can be simply derived. Then we implement [`Linear`] for this type.

```
use interpolation_search::{InterpolationSearch, Linear};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Linear<u64> for Point2D {
    fn distance_to(&self, other: &Self) -> u64 {
        let dx = self.x.distance_to(&other.x) as u64;
        let dy = self.y.distance_to(&other.y) as u64;
        dx << 32 + dy
    }
}
```

This crate already implements [`Scalable`] for `u64`.

Here's another example where both [`Linear`] and [`Scalable`] must be implemented for the type. Consider a tuple of 3 bytes that represent an RGB color.

```
use interpolation_search::{InterpolationSearch, Linear, Scalable};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Rgb(u8, u8, u8);

impl Linear for Rgb {
    fn distance_to(&self, other: &Self) -> Self {
        Self(
            self.0.distance_to(&other.0),
            self.1.distance_to(&other.1),
            self.2.distance_to(&other.2),
        )
    }
}

impl Scalable for Rgb {
    fn fraction_of(&self, other: &Self) -> f32 {
        self.0.fraction_of(&other.0)
            + self.1.fraction_of(&other.1) * 256.0_f32.powi(-1)
            + self.2.fraction_of(&other.2) * 256.0_f32.powi(-2)
    }
}
```

>Note: we couldn't just implement [`Linear`] and [`Scalable`] for the tuple `(u8, u8, u8)` as it's a foreign type. We're using the well-known [newtype idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).

## Consistency

The [`Linear`] property of a type must be consistent with its [`Ord`]. That is, for `a <= b <= c` must follow `a.distance_to(b) <= a.distance_to(c)`. Consequently, `a.distance_to(b).fraction_of(a.distance_to(c))` must be in the `[0.0, 1.0]` range.

