use sysinfo::Components;

pub struct TemperatureInfo {
    components: Components,
}

impl TemperatureInfo {
    pub fn new() -> Self {
        let components = Components::new_with_refreshed_list();
        Self { components }
    }

    pub fn get_cpu_temp(&mut self) -> Option<f32> {
        self.components.refresh(true);
        for component in self.components.iter() {
            if component.label().to_lowercase().contains("cpu")
                || component.label().to_lowercase().contains("core")
            {
                return Some(component.temperature());
            }
        }

        Self::read_sys_thermal()
    }

    fn read_sys_thermal() -> Option<f32> {
        let thermal_dir = std::path::Path::new("/sys/class/thermal");
        if !thermal_dir.exists() {
            return None;
        }

        let entries = std::fs::read_dir(thermal_dir).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            let temp_file = path.join("temp");
            if temp_file.exists() {
                if let Ok(content) = std::fs::read_to_string(temp_file) {
                    if let Ok(temp_millidegrees) = content.trim().parse::<i32>() {
                        return Some(temp_millidegrees as f32 / 1000.0);
                    }
                }
            }
        }
        None
    }
}
