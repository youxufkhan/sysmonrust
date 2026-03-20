<script lang="ts">
	import { cn } from '$lib/utils';
	import { getThresholdColor } from '$lib/utils/thresholds';

	let {
		data = [],
		color = '#3b82f6',
		maxValue: customMaxValue,
		formatY = (v: number) => v.toString(),
		colorMode = 'threshold',
		class: className
	}: {
		data: number[];
		color?: string;
		maxValue?: number;
		formatY?: (v: number) => string;
		colorMode?: 'threshold' | 'static' | 'accent';
		class?: string;
	} = $props();

	// Calculate derived values
	const maxValue = $derived(customMaxValue ?? (data.length > 0 ? Math.max(...data, 1) : 100));
	const lastValue = $derived(data.length > 0 ? data[data.length - 1] : 0);
	const lastValuePercent = $derived(maxValue > 0 ? (lastValue / maxValue) * 100 : 0);

	// Active color based on mode
	const activeColor = $derived(
		colorMode === 'threshold' ? getThresholdColor(lastValuePercent) : color
	);

	// SVG dimensions
	const width = 160;
	const height = 60;
	const padding = { top: 5, right: 5, bottom: 5, left: 28 };

	// Generate smooth curve path data
	const pathData = $derived.by(() => {
		if (data.length < 2) return '';

		const chartWidth = width - padding.left - padding.right;
		const chartHeight = height - padding.top - padding.bottom;

		const points = data.map((value, i) => ({
			x: padding.left + (i / (data.length - 1)) * chartWidth,
			y: padding.top + chartHeight - (value / maxValue) * chartHeight
		}));

		let path = `M ${points[0].x} ${points[0].y}`;
		for (let i = 1; i < points.length; i++) {
			const prev = points[i - 1];
			const curr = points[i];
			const cpx = (prev.x + curr.x) / 2;
			path += ` C ${cpx} ${prev.y}, ${cpx} ${curr.y}, ${curr.x} ${curr.y}`;
		}
		return path;
	});

	// Create filled area path
	const areaPath = $derived.by(() => {
		if (data.length < 2) return '';
		const bottomY = height - padding.bottom;
		return pathData + ` L ${width - padding.right} ${bottomY} L ${padding.left} ${bottomY} Z`;
	});

	// Unique gradient ID
	const gradientId = $derived(`sparkline-${Math.random().toString(36).slice(2, 9)}`);
</script>

<div class={cn('h-full min-h-[80px] w-full', className)}>
	{#if data.length >= 2}
		<svg viewBox="0 0 {width} {height}" preserveAspectRatio="none" class="w-full h-full">
			<defs>
				<linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
					<stop offset="0%" stop-color={activeColor} stop-opacity="0.8" />
					<stop offset="100%" stop-color={activeColor} stop-opacity="0.1" />
				</linearGradient>
			</defs>
			<path d={areaPath} fill="url(#{gradientId})" />
			<path d={pathData} fill="none" stroke={activeColor} stroke-width="1.5" />
			<text
				x={padding.left - 6}
				y={padding.top + 8}
				text-anchor="end"
				class="text-[11px] fill-muted-foreground"
			>
				{formatY(maxValue)}
			</text>
		</svg>
	{:else}
		<div class="h-full w-full flex items-center justify-center text-muted-foreground text-sm">
			No data
		</div>
	{/if}
</div>
