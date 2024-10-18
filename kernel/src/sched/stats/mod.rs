// SPDX-License-Identifier: MPL-2.0

pub mod loadavg;
mod scheduler_stats;

pub use scheduler_stats::{inject_scheduler_stats, nr_queued, nr_running, SchedulerStats};
