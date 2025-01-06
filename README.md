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
