use super::{DEG_2_RAD, RAD_2_DEG};

#[inline]
pub fn clamp_mut(v: &mut f32, lo: f32, hi: f32) {
    if *v < lo {
        *v = lo
    } else if hi < *v {
        *v = hi
    }
}

#[inline]
pub fn one_when_zero(value: f32) -> f32 {
    if value == 0.0 {
        1.0
    } else {
        value
    }
}

#[inline]
pub fn to_rad(degrees: f32) -> f32 {
    degrees * DEG_2_RAD
}

#[inline]
pub fn to_deg(radians: f32) -> f32 {
    radians * RAD_2_DEG
}
