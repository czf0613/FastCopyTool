import { invoke } from '@tauri-apps/api/tauri'

export interface SystemInfo {
    os: string,
    cpu_cores: number,
    cpu_usage: number,
    total_memory: number,
    used_memory: number,
}

export async function getSystemInfo(): Promise<SystemInfo> {
    return await invoke<SystemInfo>('get_sys_info')
}