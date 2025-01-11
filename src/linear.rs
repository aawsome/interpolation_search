use std::{
    str::Chars,
    time::{Duration, SystemTime},
};

use crate::Scalable;

/// Extends types with a `distance_to` method to calculate the distance between
/// two instances of the same type.
///
/// Interpolation search uses linear interpolation to estimate the index of the target. The items
/// in the array must be `Linear`, and the distance between the items must be
/// [`Scalable`][Scalable]. Given an array `[first, .. , last]` and a `target`, where `first <
/// target < last`, the algorithm calculates the distances from `first` to `target` and `last` to
/// better reduce the search space. This trait provides default implementations for basic numeric
/// types and signed integers with their corresponding unsigned types.  Custom implementations are
/// provided for `char` and `SystemTime`.
///
/// # Examples
///
/// ```
/// use interpolation_search::Linear;
/// use std::time::{Duration, SystemTime};
///
/// assert_eq!(5_u8.distance_to(&10_u8), 5_u8);
/// let five_sec = Duration::from_secs(5);
/// let t0 = SystemTime::now();
/// let t1 = t0 + five_sec;
/// assert_eq!(t0.distance_to(&t1), five_sec);
/// ```
pub trait Linear<Distance: Scalable = Self> {
    fn distance_to(&self, other: &Self) -> Distance;
}

macro_rules! trivially_linear {
    ($t:ty) => {
        impl Linear for $t {
            fn distance_to(&self, other: &Self) -> Self {
                other - self
            }
        }
    };
}

macro_rules! signed_linear {
    ($signed:ty, $unsigned:ty) => {
        impl Linear<$unsigned> for $signed {
            fn distance_to(&self, other: &Self) -> $unsigned {
                self.abs_diff(*other)
            }
        }
    };
}

trivially_linear!(u8);
trivially_linear!(u16);
trivially_linear!(u32);
trivially_linear!(u64);
trivially_linear!(u128);
trivially_linear!(usize);
signed_linear!(i8, u8);
signed_linear!(i16, u16);
signed_linear!(i32, u32);
signed_linear!(i64, u64);
signed_linear!(i128, u128);
signed_linear!(isize, usize);

impl Linear<u16> for char {
    fn distance_to(&self, other: &Self) -> u16 {
        if other > self {
            *other as u16 - *self as u16
        } else {
            0
        }
    }
}

impl Linear<Duration> for SystemTime {
    fn distance_to(&self, other: &Self) -> Duration {
        other.duration_since(*self).unwrap_or_default()
    }
}

impl Linear<Vec<u8>> for Chars<'_> {
    fn distance_to(&self, other: &Self) -> Vec<u8> {
        self.clone()
            .zip(other.clone())
            .map(|(a, b)| a.distance_to(&b))
            .map(|d| d.min(u8::MAX.into()) as u8)
            .collect()
    }
}

impl Linear<Vec<u8>> for &str {
    fn distance_to(&self, other: &Self) -> Vec<u8> {
        self.chars().distance_to(&other.chars())
    }
}

impl Linear<Vec<u8>> for String {
    fn distance_to(&self, other: &Self) -> Vec<u8> {
        self.chars().distance_to(&other.chars())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn u8_distance() {
        assert_eq!(5u8.distance_to(&10u8), 5u8);
        assert_eq!(5u8.distance_to(&5u8), 0u8);
        assert_eq!(0u8.distance_to(&255u8), 255u8);
    }

    #[test]
    fn i8_distance() {
        assert_eq!((-5 as i8).distance_to(&10i8), 15u8);
        assert_eq!((-128 as i8).distance_to(&127i8), 255u8);
    }

    #[test]
    fn char_distance_positive() {
        assert_eq!('a'.distance_to(&'a'), 0u16);
        assert_eq!('a'.distance_to(&'d'), 3u16);
        assert_eq!('ա'.distance_to(&'դ'), 3u16);
    }

    #[test]
    fn system_time_distance_future() {
        let now = SystemTime::now();
        let five_sec = std::time::Duration::from_secs(5);
        let future = now + five_sec;
        assert_eq!(now.distance_to(&future), five_sec);
        assert_eq!(now.distance_to(&now), Duration::ZERO);
    }
    #[test]
    fn test_str_distance() {
        assert_eq!("".distance_to(&""), Vec::new());
        assert_eq!("a".distance_to(&"a"), vec![0]);
        assert_eq!("a".distance_to(&"b"), vec![1]);
        assert_eq!("ab".distance_to(&"ac"), vec![0, 1]);
        assert_eq!("abc".distance_to(&"abd"), vec![0, 0, 1]);
        assert_eq!("abc".distance_to(&"dbd"), vec![3, 0, 1]);
        assert_eq!("աբգ".distance_to(&"աբդ"), vec![0, 0, 1]);
        assert_eq!("ab".distance_to(&"a"), vec![0]);
        assert_eq!("a".distance_to(&"abc"), vec![0]);
    }
}
