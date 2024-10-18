// SPDX-License-Identifier: MPL-2.0

use ostd::timer;
use spin::Once;

use super::loadavg;

/// The global scheduler statistic singleton
static SCHEDULER_STATS: Once<&'static dyn SchedulerStats> = Once::new();

/// Inject the scheduler statistics into the system.
///
/// This function should be called once to inject the scheduler statistics.
/// It is used to get running stats from the scheduler and to periodically
/// calculate the system load average.
pub fn inject_scheduler_stats(scheduler: &'static dyn SchedulerStats) {
    SCHEDULER_STATS.call_once(|| scheduler);

    timer::register_callback(|| {
        loadavg::update_loadavg(nr_queued());
    });
}

/// The trait for the scheduler statistics.
pub trait SchedulerStats: Sync + Send {
    /// This number represents the total number of tasks in the runqueues
    ///
    /// It is used to calculate the system load average.
    fn nr_queued(&self) -> u64;

    /// Gets the amount of actually running tasks in the system
    fn nr_running(&self) -> u64;
}

/// Get the amount of tasks in the runqueues.
pub fn nr_queued() -> u64 {
    SCHEDULER_STATS.get().unwrap().nr_queued()
}

/// Get the amount of actually running tasks.
pub fn nr_running() -> u64 {
    SCHEDULER_STATS.get().unwrap().nr_running()
}
