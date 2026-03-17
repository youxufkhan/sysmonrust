export function formatBytes(bytes: number, decimals: number = 1): string {
  if (bytes === 0) return '0 B';
  
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

export function formatSpeed(bytesPerSec: number): string {
  return formatBytes(bytesPerSec) + '/s';
}

export function formatPercent(value: number, decimals: number = 0): string {
  return value.toFixed(decimals) + '%';
}

export function formatMHz(mhz: number): string {
  if (mhz >= 1000) {
    return (mhz / 1000).toFixed(2) + ' GHz';
  }
  return mhz + ' MHz';
}

export function formatTemp(celsius: number): string {
  return celsius.toFixed(0) + '°C';
}
