mod linear;
mod scalable;

use linear::Linear;
use scalable::Scalable;
use std::cmp::{
    Ord,
    Ordering::{Equal, Greater, Less},
};

pub trait InterpolationSearch<T, V> {
    fn interpolation_search(&self, target: &T) -> Result<usize, usize>;
}

impl<T, V> InterpolationSearch<T, V> for [T]
where
    T: Ord + Linear<V>,
    V: Scalable,
{
    fn interpolation_search(&self, target: &T) -> Result<usize, usize> {
        match self {
            [] => Err(0),
            [first, ..] if target < first => Err(0),
            [single] if single == target => Ok(0),
            [.., last] if last < target => Err(self.len()),
            [first, .., last] => {
                let fraction = first
                    .distance_to(target)
                    .fraction_of(&first.distance_to(last));
                let mid_idx = lerp_len(self.len(), fraction);
                let left = &self[0..mid_idx];
                let mid = &self[mid_idx];
                let right = &self[mid_idx + 1..];
                match (left, mid.cmp(target), right) {
                    (_, Equal, _) => Ok(mid_idx),
                    (left, Greater, _) => left.interpolation_search(target),
                    (_, Less, right) => right
                        .interpolation_search(target)
                        .map(|idx| idx + mid_idx + 1)
                        .map_err(|idx| idx + mid_idx + 1),
                }
            }
            [_] => Err(0), // Should not happen if the array is sorted
        }
    }
}

fn lerp_len(len: usize, f: f32) -> usize {
    match len {
        0 | 1 => 0,
        _ => ((len as f32 * normalize(f)) as usize).min(len - 1),
    }
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

        //checking for subnormal numbers
        assert_eq!(normalize(f32::MIN_POSITIVE / 2.0), 0.5);
        assert_eq!(normalize(f32::MIN_POSITIVE * -1.0 / 2.0), 0.5);
    }

    #[test]
    fn test_lerp_len() {
        assert_eq!(lerp_len(10, 0.0), 0);
        assert_eq!(lerp_len(10, 1.0), 9);
        assert_eq!(lerp_len(10, 0.5), 5);
        assert_eq!(lerp_len(10, 0.25), 2);
        assert_eq!(lerp_len(10, 0.75), 7);

        assert_eq!(lerp_len(1, 0.0), 0);
        assert_eq!(lerp_len(1, 1.0), 0);

        assert_eq!(lerp_len(0, 0.0), 0);
        assert_eq!(lerp_len(0, 1.0), 0);

        assert_eq!(lerp_len(10, -1.0), 0);
        assert_eq!(lerp_len(10, 2.0), 9);

        assert_eq!(lerp_len(10, f32::NAN), 5);
        assert_eq!(lerp_len(10, f32::INFINITY), 5);
        assert_eq!(lerp_len(10, f32::NEG_INFINITY), 5);

        assert_eq!(lerp_len(10, f32::MIN_POSITIVE / 2.0), 5);
        assert_eq!(lerp_len(10, f32::MIN_POSITIVE * -1.0 / 2.0), 5);
    }
}
