<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import { metrics, diskHistory } from '$lib/stores/metrics';
  import { formatBytes, formatPercent } from '$lib/utils/formatters';
  import { getThresholdColor } from '$lib/utils/thresholds';
  import Sparkline from '$lib/components/Sparkline.svelte';
  
  let disks = $derived($metrics?.disk ?? []);
  let firstDiskPercent = $derived(disks.length > 0 ? (disks[0].used / disks[0].total) * 100 : 0);
</script>

<MetricCard>
  <h3 class="text-sm font-semibold mb-2">Disk</h3>
  
  {#if disks.length > 0}
    {@const usedPercent = (disks[0].used / disks[0].total) * 100}
    <div class="text-3xl font-bold">{formatPercent(usedPercent)}</div>
    <div class="mt-2">
      <Sparkline data={$diskHistory} color={getThresholdColor(usedPercent)} />
    </div>
    <div class="space-y-2 mt-3">
      {#each disks as disk}
        {@const usedPercent = (disk.used / disk.total) * 100}
        <div class="flex justify-between text-sm">
          <span class="font-medium">{disk.name}</span>
          <span class="text-muted-foreground">{formatPercent(usedPercent)} ({formatBytes(disk.used)} used, {formatBytes(disk.available)} free)</span>
        </div>
      {/each}
    </div>
  {:else}
    <div class="animate-pulse">Loading...</div>
  {/if}
</MetricCard>
