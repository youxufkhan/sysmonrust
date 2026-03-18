<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Badge from './ui/badge.svelte';
  import { metrics, gpuHistory } from '$lib/stores/metrics';
  import { formatBytes, formatPercent, formatTemp } from '$lib/utils/formatters';
  import { getThresholdColor } from '$lib/utils/thresholds';
  import Sparkline from '$lib/components/Sparkline.svelte';
  
  let gpu = $derived($metrics?.gpu);
</script>

<MetricCard>
  <div class="flex items-center justify-between mb-2">
    <h3 class="text-sm font-semibold">GPU</h3>
    {#if gpu}
      <Badge variant="outline">{gpu.gpu_type}</Badge>
    {/if}
  </div>
  
  {#if gpu}
    <div class="text-2xl font-bold">{formatPercent(gpu.utilization)}</div>
    <div class="mt-2">
      <Sparkline data={$gpuHistory} color={getThresholdColor(gpu.utilization)} />
    </div>
    <div class="text-sm space-y-1 mt-3">
      <div class="flex justify-between">
        <span class="text-muted-foreground">VRAM</span>
        <span>{formatBytes(gpu.memory_used)} / {formatBytes(gpu.memory_total)}</span>
      </div>
      {#if gpu.temperature > 0}
        <div class="flex justify-between">
          <span class="text-muted-foreground">Temperature</span>
          <span>{formatTemp(gpu.temperature)}</span>
        </div>
      {/if}
    </div>
  {:else}
    <div class="text-muted-foreground text-sm">No GPU detected</div>
  {/if}
</MetricCard>
