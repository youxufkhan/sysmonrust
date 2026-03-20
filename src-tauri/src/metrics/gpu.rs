use nvml_wrapper::Nvml;

#[derive(Debug, PartialEq)]
pub enum GpuType {
    Nvidia,
    Amd,
    Intel,
    None,
}

pub struct GpuInfo {
    pub gpu_type: GpuType,
    nvml: Option<Nvml>,
}

impl GpuInfo {
    pub fn new() -> Self {
        let nvml = Nvml::init().ok();
        let gpu_type = if nvml.is_some() {
            GpuType::Nvidia
        } else if Self::detect_amd() {
            GpuType::Amd
        } else if Self::detect_intel() {
            GpuType::Intel
        } else {
            GpuType::None
        };

        Self { gpu_type, nvml }
    }

    fn detect_amd() -> bool {
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("card") {
                        let vendor_path = path.join("device/vendor");
                        if let Ok(content) = std::fs::read_to_string(&vendor_path) {
                            if content.trim() == "0x1002" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn detect_intel() -> bool {
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("card") {
                        let vendor_path = path.join("device/vendor");
                        if let Ok(content) = std::fs::read_to_string(&vendor_path) {
                            if content.trim() == "0x8086" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn nvidia_metrics(&self) -> Option<NvidiaGpuMetrics> {
        let nvml = self.nvml.as_ref()?;
        let device = nvml.device_by_index(0).ok()?;

        let util = device.utilization_rates().ok()?;
        let mem = device.memory_info().ok()?;

        // Temperature requires specifying the sensor
        let temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .ok();

        Some(NvidiaGpuMetrics {
            utilization: util.gpu,
            memory_used: mem.used,
            memory_total: mem.total,
            temperature: temp.unwrap_or(0) as i32,
        })
    }

    /// Returns Intel GPU metrics by reading from sysfs.
    /// - Utilization: Derived from GPU frequency (gt_act_freq_mhz as proxy)
    /// - Temperature: /sys/class/drm/card*/device/hwmon/*/temp1_input (millidegrees / 1000)
    pub fn intel_metrics(&mut self) -> Option<IntelGpuMetrics> {
        if self.gpu_type != GpuType::Intel {
            return None;
        }

        let mut utilization = 0u32;
        let mut temperature = 0i32;
        let mut memory_used = 0u64;
        let memory_total = 0u64;

        // Find the Intel DRM device path
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("card") {
                        let vendor_path = path.join("device/vendor");
                        if let Ok(content) = std::fs::read_to_string(&vendor_path) {
                            if content.trim() == "0x8086" {
                                // Found Intel GPU

                                // Read frequency values for activity proxy
                                let min_freq: u32 =
                                    std::fs::read_to_string(&path.join("gt_min_freq_mhz"))
                                        .ok()
                                        .and_then(|s| s.trim().parse().ok())
                                        .unwrap_or(200);
                                let max_freq: u32 =
                                    std::fs::read_to_string(&path.join("gt_max_freq_mhz"))
                                        .ok()
                                        .and_then(|s| s.trim().parse().ok())
                                        .unwrap_or(1150);
                                let act_freq: u32 =
                                    std::fs::read_to_string(&path.join("gt_act_freq_mhz"))
                                        .ok()
                                        .and_then(|s| s.trim().parse().ok())
                                        .unwrap_or(min_freq);

                                // Calculate utilization from frequency ratio
                                if act_freq > min_freq && max_freq > min_freq {
                                    utilization = ((act_freq - min_freq) as f64
                                        / (max_freq - min_freq) as f64
                                        * 100.0)
                                        as u32;
                                }

                                // Temperature via hwmon
                                let hwmon_path = path.join("device/hwmon");
                                if let Ok(hwmon_entries) = std::fs::read_dir(&hwmon_path) {
                                    for hwmon_entry in hwmon_entries.flatten() {
                                        let temp_path = hwmon_entry.path().join("temp1_input");
                                        if let Ok(content) = std::fs::read_to_string(&temp_path) {
                                            if let Ok(millideg) = content.trim().parse::<i32>() {
                                                temperature = millideg / 1000;
                                            }
                                            break;
                                        }
                                    }
                                }

                                // Memory info from gem_pages
                                let gem_path = path.join("device/gem_pages");
                                if let Ok(content) = std::fs::read_to_string(&gem_path) {
                                    if let Ok(pages) = content.trim().parse::<u64>() {
                                        memory_used = pages * 4096;
                                    }
                                }

                                break;
                            }
                        }
                    }
                }
            }
        }

        Some(IntelGpuMetrics {
            utilization,
            temperature,
            memory_used,
            memory_total,
        })
    }

    /// Returns the GPU device name, or a placeholder string if unavailable.
    pub fn name(&self) -> String {
        let nvml = match &self.nvml {
            Some(n) => n,
            None => return String::new(),
        };
        let device = match nvml.device_by_index(0) {
            Ok(d) => d,
            Err(_) => return String::new(),
        };
        device.name().unwrap_or_default()
    }
}

pub struct NvidiaGpuMetrics {
    pub utilization: u32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: i32,
}

pub struct IntelGpuMetrics {
    pub utilization: u32,
    pub temperature: i32,
    pub memory_used: u64,
    pub memory_total: u64,
}
