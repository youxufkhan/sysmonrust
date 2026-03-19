use serde::Serialize;
use std::sync::Mutex;
pub mod metrics;
pub mod tray;
use metrics::{CpuInfo, DiskInfo, GpuInfo, GpuType, MemoryInfo, NetworkInfo, TemperatureInfo};

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

/// Application state holding persistent metric instances
pub struct AppState {
    pub cpu: Mutex<CpuInfo>,
    pub memory: Mutex<MemoryInfo>,
    pub network: Mutex<NetworkInfo>,
    pub disk: Mutex<DiskInfo>,
    pub temp: Mutex<TemperatureInfo>,
    pub gpu: Mutex<GpuInfo>,
}

impl AppState {
    pub fn new() -> Self {
        let mut cpu = CpuInfo::new();
        std::thread::sleep(std::time::Duration::from_millis(200));
        cpu.refresh_for_delta();

        Self {
            cpu: Mutex::new(cpu),
            memory: Mutex::new(MemoryInfo::new()),
            network: Mutex::new(NetworkInfo::new()),
            disk: Mutex::new(DiskInfo::new()),
            temp: Mutex::new(TemperatureInfo::new()),
            gpu: Mutex::new(GpuInfo::new()),
        }
    }
}

#[tauri::command]
fn get_system_metrics(state: tauri::State<'_, Mutex<AppState>>) -> SystemMetrics {
    let state = state.lock().unwrap();

    // CPU metrics - collect all before dropping lock
    let (global, per_core, frequency) = {
        let mut cpu = state.cpu.lock().unwrap();
        (cpu.global_usage(), cpu.per_core_usage(), cpu.frequency())
    };

    // Memory metrics
    let (total, used, available, swap_total, swap_used) = {
        let mut mem = state.memory.lock().unwrap();
        (
            mem.total_ram(),
            mem.used_ram(),
            mem.available_ram(),
            mem.total_swap(),
            mem.used_swap(),
        )
    };

    // Network metrics
    let (rx_speed, tx_speed, total_rx, total_tx) = {
        let mut net = state.network.lock().unwrap();
        let (rx, tx) = net.speeds();
        let (trx, ttx) = net.total();
        (rx, tx, trx, ttx)
    };

    // Disk metrics
    let disk_list = {
        let mut disk = state.disk.lock().unwrap();
        disk.partitions()
    };

    // Temperature
    let temp = {
        let mut t = state.temp.lock().unwrap();
        t.get_cpu_temp()
    };

    // GPU metrics
    let gpu = {
        let mut g = state.gpu.lock().unwrap();
        match g.gpu_type {
            GpuType::Nvidia => g.nvidia_metrics().map(|m| GpuMetrics {
                name: g.name(),
                gpu_type: format!("{:?}", g.gpu_type),
                utilization: m.utilization,
                memory_used: m.memory_used,
                memory_total: m.memory_total,
                temperature: m.temperature,
            }),
            GpuType::Intel => g.intel_metrics().map(|m| GpuMetrics {
                name: "Intel Graphics".to_string(),
                gpu_type: "Intel".to_string(),
                utilization: m.utilization,
                memory_used: m.memory_used,
                memory_total: m.memory_total,
                temperature: m.temperature,
            }),
            _ => None,
        }
    };

    // Build response
    SystemMetrics {
        cpu: CpuMetrics {
            global,
            per_core,
            frequency,
        },
        memory: MemoryMetrics {
            total,
            used,
            available,
            swap_total,
            swap_used,
        },
        network: NetworkMetrics {
            rx_speed,
            tx_speed,
            total_rx,
            total_tx,
        },
        disk: disk_list
            .into_iter()
            .map(|p| DiskMetrics {
                name: p.name,
                total: p.total,
                used: p.used,
                available: p.available,
            })
            .collect(),
        gpu,
        temperature: temp.map(|t| TemperatureMetrics { cpu: t }),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _ = tray::setup_tray(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_system_metrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
