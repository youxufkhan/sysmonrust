<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import uPlot from 'uplot';

	export let data: number[] = [];
	export let color: string = '#3b82f6';
	export let width: number = 120;
	export let height: number = 40;

	let container: HTMLDivElement;
	let chart: uPlot | null = null;

	function createChart() {
		if (!container || data.length < 2) return;

		const opts: uPlot.Options = {
			width,
			height,
			cursor: { show: false },
			legend: { show: false },
			scales: {
				x: { time: false },
				y: { auto: true }
			},
			axes: [
				{ show: false },
				{ show: false }
			],
			series: [
				{},
				{
					stroke: color,
					width: 1.5,
					fill: 'transparent',
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

	onMount(() => {
		createChart();
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

	$: if (chart && (width || height)) {
		destroyChart();
		createChart();
	}
</script>

<div bind:this={container} class="sparkline"></div>

<style>
	.sparkline {
		background: transparent;
	}
</style>
