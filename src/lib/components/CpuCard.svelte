<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Badge from './ui/badge.svelte';
  import { metrics, cpuHistory } from '$lib/stores/metrics';
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
      <Sparkline data={$cpuHistory} color={getThresholdColor(cpu.global)} />
    </div>
  {:else}
    <div class="animate-pulse">Loading...</div>
  {/if}
</MetricCard>
