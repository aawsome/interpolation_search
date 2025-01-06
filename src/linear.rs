use std::time::{Duration, SystemTime};

use crate::Scalable;

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
