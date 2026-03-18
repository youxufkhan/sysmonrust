<script lang="ts">
  import { cn } from "$lib/utils";
  
  let { class: className, children } = $props<{ class?: string; children?: any }>();
</script>

<div class={cn("p-4 rounded-lg border bg-card text-card-foreground shadow-sm", className)}>
  {@render children?.()}
</div>
