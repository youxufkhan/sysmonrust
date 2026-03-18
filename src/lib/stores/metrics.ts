import { writable, get } from 'svelte/store';
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

const MAX_HISTORY_POINTS = 60;

export const cpuHistory = writable<number[]>([]);
export const memoryHistory = writable<number[]>([]);
export const networkRxHistory = writable<number[]>([]);
export const networkTxHistory = writable<number[]>([]);
export const diskHistory = writable<number[]>([]);
export const gpuHistory = writable<number[]>([]);

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

function updateHistory(metrics: SystemMetrics) {
  const cpu = get(cpuHistory);
  cpuHistory.set([...cpu.slice(-(MAX_HISTORY_POINTS - 1)), metrics.cpu.global]);
  
  const memUsedPct = metrics.memory.total > 0
    ? (metrics.memory.used / metrics.memory.total) * 100
    : 0;
  const mem = get(memoryHistory);
  memoryHistory.set([...mem.slice(-(MAX_HISTORY_POINTS - 1)), memUsedPct]);
  
  const rx = get(networkRxHistory);
  networkRxHistory.set([...rx.slice(-(MAX_HISTORY_POINTS - 1)), metrics.network.rx_speed]);
  
  const tx = get(networkTxHistory);
  networkTxHistory.set([...tx.slice(-(MAX_HISTORY_POINTS - 1)), metrics.network.tx_speed]);
  
  if (metrics.disk.length > 0) {
    const diskUsedPct = metrics.disk[0].total > 0
      ? (metrics.disk[0].used / metrics.disk[0].total) * 100
      : 0;
    const disk = get(diskHistory);
    diskHistory.set([...disk.slice(-(MAX_HISTORY_POINTS - 1)), diskUsedPct]);
  }
  
  if (metrics.gpu) {
    const gpu = get(gpuHistory);
    gpuHistory.set([...gpu.slice(-(MAX_HISTORY_POINTS - 1)), metrics.gpu.utilization]);
  }
}
