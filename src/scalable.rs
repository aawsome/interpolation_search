use std::time::Duration;

pub trait Scalable {
    fn fraction_of(&self, other: &Self) -> f32;
}

macro_rules! impl_scalable_for_integers {
    ($t: ty) => {
        impl Scalable for $t {
            fn fraction_of(&self, other: &Self) -> f32 {
                (*self as f32 / *other as f32)
            }
        }
    };
}

impl_scalable_for_integers!(i8);
impl_scalable_for_integers!(i16);
impl_scalable_for_integers!(i32);
impl_scalable_for_integers!(i64);
impl_scalable_for_integers!(i128);
impl_scalable_for_integers!(isize);
impl_scalable_for_integers!(u8);
impl_scalable_for_integers!(u16);
impl_scalable_for_integers!(u32);
impl_scalable_for_integers!(u64);
impl_scalable_for_integers!(u128);
impl_scalable_for_integers!(usize);

impl Scalable for Duration {
    fn fraction_of(&self, other: &Self) -> f32 {
        other.div_duration_f32(*self)
    }
}
