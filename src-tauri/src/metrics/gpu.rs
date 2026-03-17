use nvml_wrapper::Nvml;

pub enum GpuType {
    Nvidia,
    Amd,
    Intel,
    None,
}

pub struct GpuInfo {
    gpu_type: GpuType,
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
        std::path::Path::new("/sys/class/drm/card0/device").exists()
    }

    fn detect_intel() -> bool {
        std::path::Path::new("/sys/class/drm/card1/device").exists()
    }

    pub fn nvidia_metrics(&self) -> Option<NvidiaGpuMetrics> {
        let nvml = self.nvml.as_ref()?;
        let device = nvml.device_by_index(0).ok()?;

        Some(NvidiaGpuMetrics {
            utilization: device.utilization_rates().ok()?.gpu,
            memory_used: device.memory_info()?.used,
            memory_total: device.memory_info()?.total,
            temperature: device.temperature().ok()?,
        })
    }
}

pub struct NvidiaGpuMetrics {
    pub utilization: u32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: i32,
}
