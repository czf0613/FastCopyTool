use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub cpu_cores: usize,
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
}

lazy_static! {
    static ref OS_NAME: String = {
        let sys = System::new_all();
        let os_type = sys.name().unwrap_or("Unknown".into());
        let os_version = sys.os_version().unwrap_or("Unknown".into());

        return format!("{} {}", os_type, os_version);
    };
}

#[tauri::command]
pub async fn get_sys_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    return SystemInfo {
        os: OS_NAME.to_string(),
        cpu_cores: sys.cpus().len(),
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
    };
}
