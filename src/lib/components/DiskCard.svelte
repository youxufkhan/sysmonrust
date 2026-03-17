<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Progress from './ui/progress.svelte';
  import { metrics } from '$lib/stores/metrics';
  import { formatBytes, formatPercent } from '$lib/utils/formatters';
  
  let disks = $derived($metrics?.disk ?? []);
</script>

<MetricCard>
  <h3 class="text-sm font-semibold mb-2">Disk</h3>
  
  {#if disks.length > 0}
    <div class="space-y-4">
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
