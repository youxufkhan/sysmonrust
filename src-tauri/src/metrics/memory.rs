use sysinfo::System;

pub struct MemoryInfo {
    sys: System,
}

impl MemoryInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_memory();
        Self { sys }
    }

    pub fn total_ram(&mut self) -> u64 {
        self.sys.refresh_memory();
        self.sys.total_memory()
    }

    pub fn used_ram(&mut self) -> u64 {
        self.sys.refresh_memory();
        self.sys.used_memory()
    }

    pub fn available_ram(&mut self) -> u64 {
        self.sys.refresh_memory();
        self.sys.available_memory()
    }

    pub fn total_swap(&mut self) -> u64 {
        self.sys.refresh_memory();
        self.sys.total_swap()
    }

    pub fn used_swap(&mut self) -> u64 {
        self.sys.refresh_memory();
        self.sys.used_swap()
    }
}
