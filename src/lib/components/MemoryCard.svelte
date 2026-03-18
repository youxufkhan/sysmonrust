<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import { metrics, memoryHistory } from '$lib/stores/metrics';
  import { formatBytes, formatPercent } from '$lib/utils/formatters';
  import { getThresholdColor } from '$lib/utils/thresholds';
  import Sparkline from '$lib/components/Sparkline.svelte';
  
  let mem = $derived($metrics?.memory);
  
  let memPercent = $derived(mem ? (mem.used / mem.total) * 100 : 0);
</script>

<MetricCard>
  <h3 class="text-sm font-semibold mb-2">Memory</h3>
  
  {#if mem}
    <div class="text-3xl font-bold">{formatPercent(memPercent)}</div>
    <div class="mt-2">
      <Sparkline data={$memoryHistory} color={getThresholdColor(memPercent)} />
    </div>
    <div class="text-sm space-y-1 mt-3">
      <div class="flex justify-between">
        <span class="text-muted-foreground">Used</span>
        <span>{formatBytes(mem.used)}</span>
      </div>
      <div class="flex justify-between">
        <span class="text-muted-foreground">Total</span>
        <span>{formatBytes(mem.total)}</span>
      </div>
      {#if mem.swap_total > 0}
        <div class="border-t pt-2 mt-2">
          <div class="flex justify-between">
            <span class="text-muted-foreground">Swap</span>
            <span>{formatBytes(mem.swap_used)} / {formatBytes(mem.swap_total)}</span>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="animate-pulse">Loading...</div>
  {/if}
</MetricCard>
