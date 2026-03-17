use sysinfo::Disks;

pub struct DiskInfo {
    disks: Disks,
}

impl DiskInfo {
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        Self { disks }
    }

    pub fn partitions(&mut self) -> Vec<DiskPartition> {
        self.disks.refresh(true);
        self.disks
            .iter()
            .map(|d| DiskPartition {
                name: d.mount_point().to_string_lossy().to_string(),
                total: d.total_space(),
                available: d.available_space(),
                used: d.total_space().saturating_sub(d.available_space()),
            })
            .collect()
    }
}

pub struct DiskPartition {
    pub name: String,
    pub total: u64,
    pub available: u64,
    pub used: u64,
}
