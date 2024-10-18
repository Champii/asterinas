// SPDX-License-Identifier: MPL-2.0

//! This module implements the CPU load average calculation.
//!
//! Reference: <https://github.com/torvalds/linux/blob/master/kernel/sched/loadavg.c>

use lazy_static::lazy_static;
use ostd::{arch::timer::TIMER_FREQ, sync::RwLock, timer};

/// nr of bits of precision
pub const FSHIFT: u64 = 11;
/// 1.0 as fixed-point
pub const FIXED_1: u64 = 1 << FSHIFT;
/// 5 sec intervals
pub const LOAD_FREQ: u64 = 5 * TIMER_FREQ + 1;
/// 1/exp(5sec/1min) as fixed-point
pub const EXP_1: u64 = 1884;
/// 1/exp(5sec/5min)
pub const EXP_5: u64 = 2014;
/// 1/exp(5sec/15min)
pub const EXP_15: u64 = 2037;

/// Load average of all CPU cores.
///
/// The load average is calculated as an exponential moving average of the load
/// over the last 1, 5, and 15 minutes.
#[derive(Default, Clone)]
pub struct LoadAverage {
    load: [u64; 3],
    last_update: u64, // jiffies
}

lazy_static! {
    /// The global load average.
    pub static ref LOAD_AVG: RwLock<LoadAverage> = RwLock::new(LoadAverage::default());
}

/// Returns the calculated load average of the system.
pub fn get_loadavg() -> [u64; 3] {
    let mut load = LOAD_AVG.read().clone().load;

    load[0] += FIXED_1 / 200;
    load[1] += FIXED_1 / 200;
    load[2] += FIXED_1 / 200;

    load
}

/// Returns the calculated load average of the system as floating point numbers.
pub fn get_loadavg_float() -> [f64; 3] {
    let load = get_loadavg();

    [
        load_int(load[0]) as f64 + load_frac(load[0]) as f64 / 100.0,
        load_int(load[1]) as f64 + load_frac(load[1]) as f64 / 100.0,
        load_int(load[2]) as f64 + load_frac(load[2]) as f64 / 100.0,
    ]
}

/// Updates the load average of the system.
pub fn update_loadavg(newload: u64) {
    let jiffies = timer::Jiffies::elapsed().as_u64();

    // Return if the load average was updated less than 5 seconds ago.
    if jiffies < LOAD_AVG.read().last_update + LOAD_FREQ {
        return;
    }

    let mut loadavg = LOAD_AVG.write();

    loadavg.last_update = jiffies;

    let load = &mut loadavg.load;

    let newload = newload * FIXED_1;

    load[0] = calc_load_avg(load[0], EXP_1, newload);
    load[1] = calc_load_avg(load[1], EXP_5, newload);
    load[2] = calc_load_avg(load[2], EXP_15, newload);
}

fn calc_load_avg(load: u64, exp: u64, active: u64) -> u64 {
    let mut newload = load * exp + active * (FIXED_1 - exp);

    if active >= load {
        newload += FIXED_1 - 1;
    }

    newload / FIXED_1
}

/// Returns the integer part of the fixed-point number.
pub fn load_int(x: u64) -> u64 {
    x >> FSHIFT
}

/// Returns the fractional part of the fixed-point number.
pub fn load_frac(x: u64) -> u64 {
    load_int((x & (FIXED_1 - 1)) * 100)
}
