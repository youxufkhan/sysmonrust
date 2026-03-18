<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Badge from './ui/badge.svelte';
  import Progress from './ui/progress.svelte';
  import { metrics, historyStore } from '$lib/stores/metrics';
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
    <div class="text-xl font-bold">{gpu.name}</div>
    <div class="text-2xl font-bold mt-1">{formatPercent(gpu.utilization)}</div>
    <div class="mt-2">
      <Sparkline data={historyStore.gpu} color={getThresholdColor(gpu.utilization)} width={120} height={40} />
    </div>
    <div class="space-y-3 mt-3">
      <div class="text-xl font-bold">{gpu.name}</div>
      
      <div>
        <div class="flex justify-between text-sm mb-1">
          <span class="text-muted-foreground">Usage</span>
          <span>{formatPercent(gpu.utilization)}</span>
        </div>
        <Progress value={gpu.utilization} class="h-2" />
      </div>
      
      <div>
        <div class="flex justify-between text-sm mb-1">
          <span class="text-muted-foreground">VRAM</span>
          <span>{formatBytes(gpu.memory_used)} / {formatBytes(gpu.memory_total)}</span>
        </div>
        <Progress value={(gpu.memory_used / gpu.memory_total) * 100} class="h-2" />
      </div>
      
      {#if gpu.temperature > 0}
        <div class="flex justify-between text-sm">
          <span class="text-muted-foreground">Temperature</span>
          <span>{formatTemp(gpu.temperature)}</span>
        </div>
      {/if}
    </div>
  {:else}
    <div class="text-muted-foreground text-sm">No GPU detected</div>
  {/if}
</MetricCard>
