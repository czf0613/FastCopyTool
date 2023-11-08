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

export async function getReadSpeed(path: string): Promise<number> {
    return await invoke<number>('get_4k_read_speed', {
        path
    })
}

export async function getWriteSpeed(path: string): Promise<number> {
    return await invoke<number>('get_4k_write_speed', {
        path
    })
}

export async function getReadDelay(path: string): Promise<number> {
    return await invoke<number>('get_read_delay', {
        path
    })
}

export async function getWriteDelay(path: string): Promise<number> {
    return await invoke<number>('get_write_delay', {
        path
    })
}