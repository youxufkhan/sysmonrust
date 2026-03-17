<script lang="ts">
  import { cn } from "$lib/utils";
  let { class: className, children } = $props<{ class?: string; children?: any }>();
</script>

<h3 class={cn("text-2xl font-semibold leading-none tracking-tight", className)}>
  {@render children?.()}
</h3>
