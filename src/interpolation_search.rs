use crate::InterpolationFactor;
use std::cmp::{
    Ord,
    Ordering::{Equal, Greater, Less},
};

pub trait InterpolationSearch<T> {
    /// Interpolation searches this slice for a given element. If the slice is not sorted, the returned result is unspecified and meaningless.
    ///
    /// The interface of this funciton is similar to its `binary_search` counterpart. If the value is found then `Result::Ok` is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. The index is chosen deterministically, but is subject to change in future versions of the crate. If the value is not found then `Result::Err` is returned, containing the index where a matching element could be inserted while maintaining sorted order.
    ///
    /// **Examples**
    ///
    /// ```
    /// use interpolation_search::InterpolationSearch;
    ///
    /// let arr = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    ///
    /// assert_eq!(arr.interpolation_search(&13), Ok(9));
    /// assert_eq!(arr.interpolation_search(&14), Err(10));
    /// ```
    fn interpolation_search(&self, target: &T) -> Result<usize, usize>;
}

impl<T> InterpolationSearch<T> for [T]
where
    T: Ord + InterpolationFactor,
{
    fn interpolation_search(&self, target: &T) -> Result<usize, usize> {
        let mut first_idx = 0;
        let mut last_idx = self.len();
        loop {
            match &self[first_idx..last_idx] {
                [] => return Err(first_idx),
                [first, ..] if target < first => return Err(first_idx),
                [single] if single == target => return Ok(first_idx),
                [.., last] if last < target => return Err(self.len()),
                [first, .., last] => {
                    let f = target.interpolation_factor(&first, &last);
                    let mid_idx = lerp_idx(first_idx, last_idx, f);
                    let mid = &self[mid_idx];
                    match mid.cmp(target) {
                        Equal => return Ok(mid_idx),
                        Greater => last_idx = mid_idx,
                        Less => first_idx = mid_idx + 1,
                    }
                }
                [_] => return Err(0), // Should not happen if the array is sorted
            }
        }
    }
}

// Returns an index in a given inclusive-exclusive index range (`[first, last)`).
fn lerp_idx(first: usize, last: usize, f: f32) -> usize {
    if first >= last {
        return first;
    }
    (first + ((last - first) as f32 * normalize(f)) as usize).min(last - 1)
}

fn normalize(f: f32) -> f32 {
    if !f.is_normal() && f != 0.0 {
        0.5
    } else {
        f.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_against_binary_search() {
        let arr = [1, 2, 3, 3, 4, 5, 6, 6, 6, 7, 8, 8, 8, 8, 9, 10];
        for n in 0..=11 {
            match arr.interpolation_search(&n) {
                Ok(idx) => {
                    assert!(arr.binary_search(&n).is_ok());
                    assert_eq!(arr[idx], n);
                }
                Err(idx) => assert_eq!(Err(idx), arr.binary_search(&n)),
            }
        }
    }

    #[test]
    fn test_empty_array() {
        let arr = [];
        assert_eq!(arr.interpolation_search(&0), Err(0));
        assert_eq!(arr.interpolation_search(&1), Err(0));
        assert_eq!(arr.interpolation_search(&-1), Err(0));
    }

    #[test]
    fn test_single_element() {
        let arr = [0];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        assert_eq!(arr.interpolation_search(&1), Err(1));
        assert_eq!(arr.interpolation_search(&-1), Err(0));
    }

    #[test]
    fn test_repeating_element() {
        let arr = [0, 0, 0, 0, 0];
        assert!(arr.interpolation_search(&0).is_ok_and(|n| n < 5));
        assert_eq!(arr.interpolation_search(&1), Err(5));
        assert_eq!(arr.interpolation_search(&-1), Err(0));
    }

    #[test]
    fn test_integer_types() {
        let arr = [0_i8];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_i16];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_i32];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_i64];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_i128];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_isize];
        assert_eq!(arr.interpolation_search(&0), Ok(0));

        let arr = [0_u8];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_u16];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_u32];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_u64];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_u128];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
        let arr = [0_usize];
        assert_eq!(arr.interpolation_search(&0), Ok(0));
    }

    #[test]
    fn test_time_points() {
        let t0 = SystemTime::now();
        let arr = (0..10)
            .map(|n| Duration::from_secs(n))
            .map(|delay| t0 + delay)
            .collect::<Vec<_>>();
        assert_eq!(arr.interpolation_search(&t0), Ok(0));
        assert_eq!(
            arr.interpolation_search(&(t0 + Duration::from_secs(5))),
            Ok(5)
        );
        assert_eq!(
            arr.interpolation_search(&(t0 + Duration::from_secs(15))),
            Err(10)
        );
    }

    #[test]
    fn test_chars() {
        let arr = ('a'..='z').collect::<Vec<_>>();
        assert_eq!(arr.interpolation_search(&'a'), Ok(0));
        assert_eq!(arr.interpolation_search(&'z'), Ok(25));
        assert_eq!(arr.interpolation_search(&'A'), Err(0));
        assert_eq!(arr.interpolation_search(&'{'), Err(26));
        assert_eq!(arr.interpolation_search(&'ա'), Err(26));

        let arr = ('ա'..='և').collect::<Vec<_>>();
        assert_eq!(arr.interpolation_search(&'ա'), Ok(0));
        assert_eq!(arr.interpolation_search(&'և'), Ok(38));
        assert_eq!(arr.interpolation_search(&'Ա'), Err(0));
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(0.0), 0.0);
        assert_eq!(normalize(1.0), 1.0);
        assert_eq!(normalize(0.5), 0.5);
        assert_eq!(normalize(0.25), 0.25);
        assert_eq!(normalize(0.75), 0.75);

        assert_eq!(normalize(-1.0), 0.0);
        assert_eq!(normalize(2.0), 1.0);

        assert_eq!(normalize(f32::NAN), 0.5);
        assert_eq!(normalize(f32::INFINITY), 0.5);
        assert_eq!(normalize(f32::NEG_INFINITY), 0.5);
        assert_eq!(normalize(f32::MIN_POSITIVE), f32::MIN_POSITIVE);

        assert_eq!(normalize(f32::MIN_POSITIVE / 2.0), 0.5);
        assert_eq!(normalize(f32::MIN_POSITIVE * -1.0 / 2.0), 0.5);
    }

    #[test]
    fn test_lerp_idx() {
        assert_eq!(lerp_idx(0, 10, 0.0), 0);
        assert_eq!(lerp_idx(0, 10, 1.0), 9);
        assert_eq!(lerp_idx(0, 10, 0.5), 5);
        assert_eq!(lerp_idx(0, 10, 0.25), 2);
        assert_eq!(lerp_idx(0, 10, 0.75), 7);

        assert_eq!(lerp_idx(0, 1, 0.0), 0);
        assert_eq!(lerp_idx(0, 1, 1.0), 0);

        assert_eq!(lerp_idx(0, 0, 0.0), 0);
        assert_eq!(lerp_idx(0, 0, 1.0), 0);

        // Testing out-of-bounds factors.
        assert_eq!(lerp_idx(0, 10, -1.0), 0);
        assert_eq!(lerp_idx(0, 10, 2.0), 9);
        assert_eq!(lerp_idx(0, 10, f32::NAN), 5);
        assert_eq!(lerp_idx(0, 10, f32::INFINITY), 5);
        assert_eq!(lerp_idx(0, 10, f32::NEG_INFINITY), 5);
        assert_eq!(lerp_idx(0, 10, f32::MIN_POSITIVE / 2.0), 5);
        assert_eq!(lerp_idx(0, 10, f32::MIN_POSITIVE * -1.0 / 2.0), 5);

        assert_eq!(lerp_idx(5, 15, 0.0), 5);
        assert_eq!(lerp_idx(5, 15, 1.0), 14);
        assert_eq!(lerp_idx(5, 15, 0.5), 10);

        assert_eq!(lerp_idx(10, 5, 0.0), 10);
    }

    #[test]
    fn test_str_interpolation_search() {
        let strings = vec!["apple", "banana", "cherry", "date", "elderberry"];

        assert_eq!(strings.interpolation_search(&"apple"), Ok(0));
        assert_eq!(strings.interpolation_search(&"date"), Ok(3));
        assert_eq!(strings.interpolation_search(&"grape"), Err(5));
        assert_eq!(strings.interpolation_search(&"apricot"), Err(1));
        assert_eq!(strings.interpolation_search(&"bat"), Err(2));

        let empty_strings: Vec<&str> = Vec::new();
        assert_eq!(empty_strings.interpolation_search(&"anything"), Err(0));

        let single_string = vec!["only"];
        assert_eq!(single_string.interpolation_search(&"only"), Ok(0));
        assert_eq!(single_string.interpolation_search(&"aaa"), Err(0));
        assert_eq!(single_string.interpolation_search(&"zzz"), Err(1));

        let repeated_strings = vec!["same", "same", "same"];
        assert!(repeated_strings
            .interpolation_search(&"same")
            .is_ok_and(|n| n < 3));
    }
}
