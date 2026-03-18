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
          updateHistory(data);
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

const MAX_HISTORY_POINTS = 60;

export const historyStore = {
  cpu: [] as number[],
  memory: [] as number[],
  networkRx: [] as number[],
  networkTx: [] as number[],
  disk: [] as number[],
  gpu: [] as number[],
};

export function pushHistory(store: number[], value: number) {
  if (store.length >= MAX_HISTORY_POINTS) {
    store.shift();
  }
  store.push(value);
}

function updateHistory(metrics: SystemMetrics) {
  pushHistory(historyStore.cpu, metrics.cpu.global);
  
  const memUsedPct = metrics.memory.total > 0
    ? (metrics.memory.used / metrics.memory.total) * 100
    : 0;
  pushHistory(historyStore.memory, memUsedPct);
  
  pushHistory(historyStore.networkRx, metrics.network.rx_speed);
  pushHistory(historyStore.networkTx, metrics.network.tx_speed);
  
  if (metrics.disk.length > 0) {
    const diskUsedPct = metrics.disk[0].total > 0
      ? (metrics.disk[0].used / metrics.disk[0].total) * 100
      : 0;
    pushHistory(historyStore.disk, diskUsedPct);
  }
  
  if (metrics.gpu) {
    pushHistory(historyStore.gpu, metrics.gpu.utilization);
  }
}
