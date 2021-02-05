pub const PI: f32 = std::f32::consts::PI;

pub const DEG_2_RAD: f32 = PI / 180.0;
pub const RAD_2_DEG: f32 = 180.0 / PI;

pub const SAFE_F: f32 = 0.0001;

pub const SAFE_HALF_PI_MAX: f32 = (PI / 2.0) - SAFE_F;
pub const SAFE_HALF_PI_MIN: f32 = (-PI / 2.0) + SAFE_F;
