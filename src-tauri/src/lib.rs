use serde::Serialize;
pub mod metrics;
pub mod tray;
use metrics::{CpuInfo, DiskInfo, GpuInfo, MemoryInfo, NetworkInfo, TemperatureInfo};

#[derive(Serialize)]
pub struct SystemMetrics {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub network: NetworkMetrics,
    pub disk: Vec<DiskMetrics>,
    pub gpu: Option<GpuMetrics>,
    pub temperature: Option<TemperatureMetrics>,
}

#[derive(Serialize)]
pub struct CpuMetrics {
    pub global: f32,
    pub per_core: Vec<f32>,
    pub frequency: u64,
}

#[derive(Serialize)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Serialize)]
pub struct NetworkMetrics {
    pub rx_speed: u64,
    pub tx_speed: u64,
    pub total_rx: u64,
    pub total_tx: u64,
}

#[derive(Serialize)]
pub struct DiskMetrics {
    pub name: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

#[derive(Serialize)]
pub struct GpuMetrics {
    pub name: String,
    pub gpu_type: String,
    pub utilization: u32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: i32,
}

#[derive(Serialize)]
pub struct TemperatureMetrics {
    pub cpu: f32,
}

#[tauri::command]
fn get_system_metrics() -> SystemMetrics {
    let mut cpu = CpuInfo::new();
    let mut memory = MemoryInfo::new();
    let mut network = NetworkInfo::new();
    let mut disk = DiskInfo::new();
    let gpu = GpuInfo::new();
    let mut temp = TemperatureInfo::new();

    SystemMetrics {
        cpu: CpuMetrics {
            global: cpu.global_usage(),
            per_core: cpu.per_core_usage(),
            frequency: cpu.frequency(),
        },
        memory: MemoryMetrics {
            total: memory.total_ram(),
            used: memory.used_ram(),
            available: memory.available_ram(),
            swap_total: memory.total_swap(),
            swap_used: memory.used_swap(),
        },
        network: {
            let (rx_speed, tx_speed) = network.speeds();
            let (total_rx, total_tx) = network.total();
            NetworkMetrics {
                rx_speed,
                tx_speed,
                total_rx,
                total_tx,
            }
        },
        disk: disk
            .partitions()
            .into_iter()
            .map(|p| DiskMetrics {
                name: p.name,
                total: p.total,
                used: p.used,
                available: p.available,
            })
            .collect(),
        gpu: None,
        temperature: temp.get_cpu_temp().map(|t| TemperatureMetrics { cpu: t }),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _ = tray::setup_tray(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_system_metrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
