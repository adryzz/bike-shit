use core::f32::consts::PI;
pub const WATCHDOG_TIMER_MS: u64 = 1000;
pub const OSD_REFRESH_MS: u64 = 20;
pub const WHEEL_DIAMETER_MM: u64 = 650;

pub const WHEEL_CIRCUMFERENCE_MM: f32 = WHEEL_DIAMETER_MM as f32 * PI;
