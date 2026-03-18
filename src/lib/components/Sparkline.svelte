<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import uPlot from 'uplot';

	export let data: number[] = [];
	export let color: string = '#3b82f6';

	let container: HTMLDivElement;
	let chart: uPlot | null = null;
	let localWidth = 200;
	let localHeight = 48;

	function createChart() {
		if (!container || data.length < 2) return;

		const opts: uPlot.Options = {
			width: localWidth,
			height: localHeight,
			cursor: { show: false },
			legend: { show: false },
			scales: {
				x: { time: false },
				y: { auto: true }
			},
			axes: [
				{ show: false },
				{
					show: true,
					size: 40,
					values: (u, vals) => vals.map((v) => v.toFixed(0) + '%'),
					grid: { show: true, stroke: 'rgba(255,255,255,0.05)' },
					ticks: { show: false },
					side: 1
				}
			],
			series: [
				{},
				{
					stroke: color,
					width: 1.5,
					fill: `${color}20`,
					points: { show: false }
				}
			]
		};

		chart = new uPlot(opts, [data.map((v, i) => i), data], container);
	}

	function destroyChart() {
		if (chart) {
			chart.destroy();
			chart = null;
		}
	}

	function updateSize() {
		if (chart && localWidth > 0) {
			chart.setSize({ width: localWidth, height: localHeight });
		}
	}

	onMount(() => {
		createChart();

		const ro = new ResizeObserver((entries) => {
			for (const entry of entries) {
				localWidth = Math.floor(entry.contentRect.width);
				updateSize();
			}
		});
		if (container) ro.observe(container);

		return () => {
			ro.disconnect();
		};
	});

	onDestroy(() => {
		destroyChart();
	});

	$: if (container && data.length >= 2) {
		destroyChart();
		createChart();
	}

	$: if (chart && color) {
		destroyChart();
		createChart();
	}
</script>

<div bind:this={container} class="sparkline w-full"></div>

<style>
	.sparkline {
		background: transparent;
		min-height: 48px;
	}
</style>
