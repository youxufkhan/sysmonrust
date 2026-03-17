use sysinfo::System;

pub struct CpuInfo {
    sys: System,
}

impl CpuInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        Self { sys }
    }

    pub fn global_usage(&mut self) -> f32 {
        self.sys.refresh_cpu_all();
        self.sys.global_cpu_usage()
    }

    pub fn per_core_usage(&mut self) -> Vec<f32> {
        self.sys.refresh_cpu_all();
        self.sys.cpus().iter().map(|c| c.cpu_usage()).collect()
    }

    pub fn frequency(&self) -> u64 {
        self.sys.cpus().first().map(|c| c.frequency()).unwrap_or(0)
    }
}
