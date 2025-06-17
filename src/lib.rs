pub use sysinfo;

use sysinfo::System;

#[derive(Default, serde::Serialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub available: u64,
}

impl MemoryInfo {
    pub fn from_sys(sys: &mut System) -> Self {
        sys.refresh_memory();
        Self {
            total: sys.total_memory(),
            used: sys.used_memory(),
            free: sys.free_memory(),
            available: sys.available_memory(),
        }
    }

	pub fn new() -> Self {
		let mut sys: System = System::new();
		Self::from_sys(&mut sys)
	}
}

impl MemoryInfo {
    pub fn total_mib(&self) -> u64 {
        self.total >> 20
    }

    pub fn used_mib(&self) -> u64 {
        self.used >> 20
    }

    pub fn free_mib(&self) -> u64 {
        self.free >> 20
    }

    pub fn available_mib(&self) -> u64 {
        self.available >> 20
    }
}
