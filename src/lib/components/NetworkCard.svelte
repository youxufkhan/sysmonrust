<script lang="ts">
  import MetricCard from './MetricCard.svelte';
  import Badge from './ui/badge.svelte';
  import { metrics, networkRxHistory, networkTxHistory } from '$lib/stores/metrics';
  import { formatSpeed, formatBytes } from '$lib/utils/formatters';
  import Sparkline from '$lib/components/Sparkline.svelte';
  
  let net = $derived($metrics?.network);
</script>

<MetricCard>
  <h3 class="text-sm font-semibold mb-2">Network</h3>
  
  {#if net}
    <div class="space-y-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-green-500">↓</span>
          <span class="text-muted-foreground text-sm">Download</span>
        </div>
        <Badge variant="secondary">{formatSpeed(net.rx_speed)}</Badge>
      </div>
      <Sparkline data={$networkRxHistory} color="#22c55e" width={120} height={40} />
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-blue-500">↑</span>
          <span class="text-muted-foreground text-sm">Upload</span>
        </div>
        <Badge variant="secondary">{formatSpeed(net.tx_speed)}</Badge>
      </div>
      <Sparkline data={$networkTxHistory} color="#3b82f6" width={120} height={40} />
      <div class="border-t pt-2 mt-2 text-sm">
        <div class="flex justify-between text-muted-foreground">
          <span>Total down</span>
          <span>{formatBytes(net.total_rx)}</span>
        </div>
        <div class="flex justify-between text-muted-foreground">
          <span>Total up</span>
          <span>{formatBytes(net.total_tx)}</span>
        </div>
      </div>
    </div>
  {:else}
    <div class="animate-pulse">Loading...</div>
  {/if}
</MetricCard>
