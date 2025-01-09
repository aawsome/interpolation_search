use std::time::Duration;

/// Extends the duration type with a `fraction_of` method.
///
/// Interpolation search uses linear interpolation to estimate the index of the target. The
/// items in the array must be [`Linear`][Linear], and the distance between the items must be
/// `Scalable`. Given an array `[first, .. , last]` and a `target`, where `first < target < last`,
/// the algorithm needs to know the fraction of `first.distance_to(target)` in
/// `first.distance_to(last)`.
///
/// # Examples
///
/// ```
/// use interpolation_search::scalable::Scalable;
/// use std::time::Duration;
///
/// assert_eq!(10_usize.fraction_of(&20), 0.5);
/// assert_eq!(Duration::from_secs(1).fraction_of(&Duration::from_secs(2)), 0.5);
/// ```
pub trait Scalable {
    fn fraction_of(&self, other: &Self) -> f32;
}

macro_rules! impl_scalable_for_integers {
    ($t: ty) => {
        impl Scalable for $t {
            fn fraction_of(&self, other: &Self) -> f32 {
                if *other == 0 {
                    return 0.5;
                }
                (*self as f32 / *other as f32)
            }
        }
    };
}

impl_scalable_for_integers!(u8);
impl_scalable_for_integers!(u16);
impl_scalable_for_integers!(u32);
impl_scalable_for_integers!(u64);
impl_scalable_for_integers!(u128);
impl_scalable_for_integers!(usize);

impl Scalable for Duration {
    fn fraction_of(&self, other: &Self) -> f32 {
        if other.is_zero() {
            return 0.5;
        }
        self.div_duration_f32(*other)
    }
}

impl Scalable for Vec<u8> {
    fn fraction_of(&self, other: &Self) -> f32 {
        self.iter()
            .zip(other.iter())
            .map(|(a, b)| a.fraction_of(&b))
            .enumerate()
            .map(|(idx, f)| f * (u16::MAX as f32).powi(-(idx as i32)))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_fractions() {
        assert_eq!(5_usize.fraction_of(&10), 0.5);
        assert_eq!(0_usize.fraction_of(&5), 0.0);
        assert_eq!(5_usize.fraction_of(&5), 1.0);
        assert_eq!(5_usize.fraction_of(&0), 0.5);
        assert_eq!(0_usize.fraction_of(&0), 0.5);
    }

    #[test]
    fn test_duration_fractions() {
        assert_eq!(
            Duration::from_millis(500).fraction_of(&Duration::from_secs(1)),
            0.5
        );
        assert_eq!(Duration::ZERO.fraction_of(&Duration::from_secs(1)), 0.0);
        assert_eq!(Duration::from_secs(1).fraction_of(&Duration::ZERO), 0.5);
        assert_eq!(Duration::ZERO.fraction_of(&Duration::ZERO), 0.5);
    }
}
