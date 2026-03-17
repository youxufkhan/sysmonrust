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
}

pub struct NvidiaGpuMetrics {
    pub utilization: u32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: i32,
}
