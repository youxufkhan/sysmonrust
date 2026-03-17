<script lang="ts">
  import { onMount } from 'svelte';
  import { metrics } from "$lib/stores/metrics";
  import CpuCard from "$lib/components/CpuCard.svelte";
  import MemoryCard from "$lib/components/MemoryCard.svelte";
  import NetworkCard from "$lib/components/NetworkCard.svelte";
  import DiskCard from "$lib/components/DiskCard.svelte";
  import GpuCard from "$lib/components/GpuCard.svelte";
  
  onMount(() => {
    metrics.start(1000);
    return () => metrics.stop();
  });
</script>

<div class="p-4 min-h-screen bg-background text-foreground">
  <h1 class="text-xl font-bold mb-4">sysmonrust</h1>
  <div class="grid grid-cols-2 gap-4">
    <CpuCard />
    <MemoryCard />
    <NetworkCard />
    <DiskCard />
    <GpuCard />
  </div>
</div>
