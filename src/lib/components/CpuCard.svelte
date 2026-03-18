<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Progress from './ui/progress.svelte';
  import Badge from './ui/badge.svelte';
  import { metrics, historyStore } from '$lib/stores/metrics';
  import { formatPercent, formatMHz } from '$lib/utils/formatters';
  import { getThresholdColor } from '$lib/utils/thresholds';
  import Sparkline from '$lib/components/Sparkline.svelte';
  
  let cpu = $derived($metrics?.cpu);
</script>

<MetricCard class="col-span-2">
  <div class="flex items-center justify-between mb-2">
    <h3 class="text-sm font-semibold">CPU</h3>
    {#if cpu}
      <Badge variant="secondary">{formatMHz(cpu.frequency)}</Badge>
    {/if}
  </div>
  
  {#if cpu}
    <div class="text-3xl font-bold">{formatPercent(cpu.global)}</div>
    <div class="mt-2">
      <Sparkline data={historyStore.cpu} color={getThresholdColor(cpu.global)} width={120} height={40} />
    </div>
    <div class="grid grid-cols-4 gap-2 mt-3">
      {#each cpu.per_core as coreUsage, i}
        <div class="text-xs">
          <div class="flex justify-between mb-1">
            <span class="text-muted-foreground">Core {i}</span>
          </div>
          <Progress value={coreUsage} class="h-2" />
        </div>
      {/each}
    </div>
  {:else}
    <div class="animate-pulse">Loading...</div>
  {/if}
</MetricCard>
