import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface CpuMetrics {
  global: number;
  per_core: number[];
  frequency: number;
}

export interface MemoryMetrics {
  total: number;
  used: number;
  available: number;
  swap_total: number;
  swap_used: number;
}

export interface NetworkMetrics {
  rx_speed: number;
  tx_speed: number;
  total_rx: number;
  total_tx: number;
}

export interface DiskMetrics {
  name: string;
  total: number;
  used: number;
  available: number;
}

export interface GpuMetrics {
  name: string;
  gpu_type: string;
  utilization: number;
  memory_used: number;
  memory_total: number;
  temperature: number;
}

export interface TemperatureMetrics {
  cpu: number;
}

export interface SystemMetrics {
  cpu: CpuMetrics;
  memory: MemoryMetrics;
  network: NetworkMetrics;
  disk: DiskMetrics[];
  gpu: GpuMetrics | null;
  temperature: TemperatureMetrics | null;
}

function createMetricsStore() {
  const { subscribe, set } = writable<SystemMetrics | null>(null);
  let interval: ReturnType<typeof setInterval> | null = null;
  
  return {
    subscribe,
    start: (intervalMs: number = 1000) => {
      const fetchMetrics = async () => {
        try {
          const data = await invoke<SystemMetrics>('get_system_metrics');
          set(data);
        } catch (e) {
          console.error('Failed to fetch metrics:', e);
        }
      };
      
      fetchMetrics();
      interval = setInterval(fetchMetrics, intervalMs);
    },
    stop: () => {
      if (interval) {
        clearInterval(interval);
        interval = null;
      }
    }
  };
}

export const metrics = createMetricsStore();
