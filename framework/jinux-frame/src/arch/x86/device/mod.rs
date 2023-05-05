//! Device-related APIs.
//! This module mainly contains the APIs that should exposed to the device driver like PCI, RTC

pub mod cmos;
pub mod io_port;
pub mod pci;
pub mod serial;