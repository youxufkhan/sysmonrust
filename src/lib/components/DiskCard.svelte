<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Progress from './ui/progress.svelte';
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
      <Sparkline data={$diskHistory} color={getThresholdColor(usedPercent)} width={120} height={40} />
    </div>
    <div class="space-y-3 mt-3">
      {#each disks as disk}
        {@const usedPercent = (disk.used / disk.total) * 100}
        <div>
          <div class="flex justify-between text-sm mb-1">
            <span class="font-medium">{disk.name}</span>
            <span class="text-muted-foreground">{formatPercent(usedPercent)}</span>
          </div>
          <Progress value={usedPercent} class="h-2" />
          <div class="flex justify-between text-xs text-muted-foreground mt-1">
            <span>{formatBytes(disk.used)} used</span>
            <span>{formatBytes(disk.available)} free</span>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="animate-pulse">Loading...</div>
  {/if}
</MetricCard>
