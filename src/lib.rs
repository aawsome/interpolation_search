//! **Interpolation search is an algorithm for searching in a sorted array.**
//!
//! It improves upon the famous binary search by using linear interpolation to better estimate the target's position within the array.
//! Interpolation search reduces the asymptotic time complexity of the search to *O(log log N)*.
//! However, in the worst case scenario (array elements grow exponentially) the complexity becomes linear (*O(N)*).
//!
//! To extend `slice` with the `interpolation_search` method this crate provides, import the
//! `InterpolationSearch` trait. Now the `interpolation_search` method is available on arrays, slices, and `Vec`s:
//!
//! ```
//! use interpolation_search::InterpolationSearch;
//!
//! let arr = [1, 2, 3, 4, 5];
//! let target = 3;
//! match arr.interpolation_search(&target) {
//!     Ok(idx) => println!("Target found at index {}", idx),
//!     Err(idx) => println!("Target not found, possible insertion point: {}", idx),
//! }
//! ```

mod interpolation_search;
mod linear;
mod scalable;

pub use interpolation_search::InterpolationSearch;
pub use linear::Linear;
pub use scalable::Scalable;
