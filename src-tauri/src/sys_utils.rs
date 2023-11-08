use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
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

const FILE_TEST_BATCH: i32 = 200;
pub static SYSTEM_READ_SPEED: AtomicU64 = AtomicU64::new(1);
pub static SYSTEM_WRITE_SPEED: AtomicU64 = AtomicU64::new(1);
pub static SYSTEM_READ_DELAY: AtomicU64 = AtomicU64::new(1);
pub static SYSTEM_WRITE_DELAY: AtomicU64 = AtomicU64::new(1);

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

// 计算每秒钟能写入多少字节
#[tauri::command]
pub async fn get_4k_write_speed(path: String) -> u64 {
    let path = PathBuf::from(path);
    let test_path = path.join(".copy_test");
    std::fs::create_dir_all(test_path.clone()).unwrap();

    // 做一千次写入，然后来计算平均速度
    let mut total_size = 0u64;
    let mut total_time_micro = 0u64;

    for i in 0..FILE_TEST_BATCH {
        let (size, time) = random_write_file_at_path(test_path.join(format!("{}.bin", i)));
        total_size += size;
        total_time_micro += time;

        // 此时故意去读取一下别的目录，让磁盘的磁头移开，以此来模拟极端的使用场景
        let _ = std::fs::read(test_path.join("0.bin"));
    }

    recursive_delete(test_path);
    let speed = total_size * 1000000 / total_time_micro;
    SYSTEM_WRITE_SPEED.store(speed, Ordering::SeqCst);

    return speed;
}

#[tauri::command]
pub async fn get_4k_read_speed(path: String) -> u64 {
    let path = PathBuf::from(path);
    let test_path = path.join(".copy_test");
    std::fs::create_dir_all(test_path.clone()).unwrap();

    // 随便乱生成一些文件，然后读取它们
    for i in 0..FILE_TEST_BATCH {
        let (_, _) = random_write_file_at_path(test_path.join(format!("{}.bin", i)));
    }

    let mut rng = rand::thread_rng();
    let mut total_size = 0u64;
    let mut total_time_micro = 0u64;

    for _ in 0..FILE_TEST_BATCH {
        let file_index = rng.next_u64() % FILE_TEST_BATCH as u64;
        let start_time = Instant::now();

        if let Ok(buffer) = std::fs::read(test_path.join(format!("{}.bin", file_index))) {
            let end_time = start_time.elapsed().as_micros();
            total_size += buffer.len() as u64;
            total_time_micro += end_time as u64;
        }
    }

    recursive_delete(test_path);
    let speed = total_size * 1000000 / total_time_micro;
    SYSTEM_READ_SPEED.store(speed, Ordering::SeqCst);

    return speed;
}

// 返回一个元组，第一项为写入的数据量，第二项为花费的时间（微秒），即可统计写入的速度
fn random_write_file_at_path(path: PathBuf) -> (u64, u64) {
    let mut rng = rand::thread_rng();
    let file_size = rng.next_u64() % 3072 + 1024;

    let mut buf = vec![0u8; file_size as usize];
    rng.fill_bytes(&mut buf);

    let start_time = Instant::now();
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(&buf).unwrap();
    let end_time = start_time.elapsed().as_micros();

    return (file_size, end_time as u64);
}

fn recursive_delete(path: PathBuf) {
    if path.is_dir() {
        for entry in std::fs::read_dir(path.clone()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            recursive_delete(path);
        }

        std::fs::remove_dir(path).unwrap();
    } else {
        std::fs::remove_file(path).unwrap();
    }
}

#[tauri::command]
pub async fn get_read_delay(path: String) -> f64 {
    let read_path = PathBuf::from(path);
    let time_micro = read_first_byte_time(&read_path);
    SYSTEM_READ_DELAY.store(time_micro, Ordering::SeqCst);

    return time_micro as f64 / 1000.0;
}

fn read_first_byte_time(path: &PathBuf) -> u64 {
    if !path.is_dir() {
        return timing_read_first_byte(path);
    } else {
        for entry in std::fs::read_dir(path.clone()).unwrap() {
            let entry_path = entry.unwrap().path();

            if !entry_path.is_dir() {
                return timing_read_first_byte(&entry_path);
            } else {
                return read_first_byte_time(&entry_path);
            }
        }

        return 1;
    }
}

fn timing_read_first_byte(path: &PathBuf) -> u64 {
    let mut buffer = [0u8; 1];

    let start_time = Instant::now();
    let mut file = File::open(path).unwrap();
    let _ = file.read_exact(&mut buffer);

    let end_time = start_time.elapsed().as_micros();
    return end_time as u64;
}

#[tauri::command]
pub async fn get_write_delay(path: String) -> f64 {
    let write_path = PathBuf::from(path);
    let test_path = write_path.join(".copy_test.bin");
    let mut file = File::create(test_path.clone()).unwrap();
    let buffer = [0u8; 1];

    let start_time = Instant::now();
    let _ = file.write(&buffer);
    let end_time = start_time.elapsed().as_micros() as u64;

    SYSTEM_WRITE_DELAY.store(end_time, Ordering::SeqCst);
    let _ = std::fs::remove_file(test_path);

    return end_time as f64 / 1000.0;
}