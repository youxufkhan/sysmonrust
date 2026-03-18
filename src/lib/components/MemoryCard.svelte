<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Progress from './ui/progress.svelte';
  import { metrics, historyStore } from '$lib/stores/metrics';
  import { formatBytes, formatPercent } from '$lib/utils/formatters';
  import { getThresholdColor } from '$lib/utils/thresholds';
  import Sparkline from '$lib/components/Sparkline.svelte';
  
  let mem = $derived($metrics?.memory);
  
  let memPercent = $derived(mem ? (mem.used / mem.total) * 100 : 0);
  let swapPercent = $derived(mem?.swap_total ? (mem.swap_used / mem.swap_total) * 100 : 0);
</script>

<MetricCard>
  <h3 class="text-sm font-semibold mb-2">Memory</h3>
  
  {#if mem}
    <div class="text-3xl font-bold">{formatPercent(memPercent)}</div>
    <div class="mt-2">
      <Sparkline data={historyStore.memory} color={getThresholdColor(memPercent)} width={120} height={40} />
    </div>
    <div class="mt-3">
      <Progress value={memPercent} class="mb-3" />
    </div>
    <div class="text-sm space-y-1">
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
