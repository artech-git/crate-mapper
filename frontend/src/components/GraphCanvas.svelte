<script lang="ts">
	import { onMount } from 'svelte';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { render, findNodeAtPoint } from '$lib/graph/renderer';
	import ClusterLegend from './ClusterLegend.svelte';
	import * as d3Zoom from 'd3-zoom';
	import { select } from 'd3-selection';
	let canvas: HTMLCanvasElement;
	let container: HTMLDivElement;
	let worker: Worker | null = null;
	let animationFrame: number | null = null;
	let hoveredNode: string | null = $state(null);
	let transform = $state({ x: 0, y: 0, k: 1 });

	let width = $state(800);
	let height = $state(600);

	onMount(() => {
		const resizeObserver = new ResizeObserver((entries) => {
			for (const entry of entries) {
				width = entry.contentRect.width;
				height = entry.contentRect.height;
				canvas.width = width * window.devicePixelRatio;
				canvas.height = height * window.devicePixelRatio;
				canvas.style.width = `${width}px`;
				canvas.style.height = `${height}px`;
				draw();
			}
		});
		resizeObserver.observe(container);

		// Set up zoom
		const zoom = d3Zoom
			.zoom<HTMLCanvasElement, unknown>()
			.scaleExtent([0.1, 10])
			.on('zoom', (event) => {
				transform = {
					x: event.transform.x,
					y: event.transform.y,
					k: event.transform.k
				};
				draw();
			});

		select(canvas).call(zoom);

		// Set up worker
		worker = new Worker(new URL('$lib/graph/ForceWorker.ts', import.meta.url), {
			type: 'module'
		});

		worker.onmessage = (e) => {
			if (e.data.type === 'tick') {
				graphStore.updatePositions(e.data.nodes);
				draw();
			}
		};

		return () => {
			resizeObserver.disconnect();
			worker?.terminate();
			if (animationFrame) cancelAnimationFrame(animationFrame);
		};
	});

	// React to subgraph changes - send to worker
	$effect(() => {
		const subgraph = graphStore.subgraph;
		if (subgraph && worker) {
			worker.postMessage({
				type: 'init',
				nodes: subgraph.nodes.map((n) => ({
					id: n.id,
					downloads: n.downloads,
					depth: n.depth
				})),
				links: subgraph.edges.map((e) => ({
					source: e.source,
					target: e.target,
					kind: e.kind
				})),
				config: {
					width,
					height,
					center: subgraph.center
				}
			});
		}
	});

	// Re-draw when chain/cluster/filter state changes
	$effect(() => {
		// Touch reactive dependencies to trigger re-draw
		graphStore.chainNodeIds;
		graphStore.chainEdgeKeys;
		graphStore.clusterColorMap;
		graphStore.highlightedCluster;
		graphStore.filteredEdges;
		graphStore.filteredNodeIds;
		draw();
	});

	function draw() {
		if (!canvas) return;
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		ctx.save();
		ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

		render({
			ctx,
			nodes: graphStore.nodesWithPositions as any,
			edges: graphStore.subgraph?.edges ?? [],
			center: graphStore.subgraph?.center ?? '',
			hoveredNode,
			selectedNode: graphStore.selectedCrate,
			transform,
			width,
			height,
			// Phase 2 render context
			chainNodeIds: graphStore.chainNodeIds,
			chainEdgeKeys: graphStore.chainEdgeKeys,
			clusterColorMap: graphStore.clusterColorMap,
			highlightedCluster: graphStore.highlightedCluster,
			clusterAssignments: graphStore.clusters?.assignments,
			filteredNodeIds: graphStore.filteredNodeIds,
			filteredEdges: graphStore.filteredEdges
		});

		ctx.restore();
	}

	function handleMouseMove(e: MouseEvent) {
		const rect = canvas.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;
		const node = findNodeAtPoint(graphStore.nodesWithPositions as any, x, y, transform);
		const newHovered = node?.id ?? null;
		if (newHovered !== hoveredNode) {
			hoveredNode = newHovered;
			canvas.style.cursor = hoveredNode ? 'pointer' : 'default';
			draw();
		}
	}

	function handleClick(e: MouseEvent) {
		const rect = canvas.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;
		const node = findNodeAtPoint(graphStore.nodesWithPositions as any, x, y, transform);
		if (node) {
			graphStore.selectCrate(node.id);
			draw();
		}
	}

	function handleDblClick(e: MouseEvent) {
		const rect = canvas.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const y = e.clientY - rect.top;
		const node = findNodeAtPoint(graphStore.nodesWithPositions as any, x, y, transform);
		if (node) {
			graphStore.loadSubgraph(node.id);
		}
	}
</script>

<div class="relative w-full h-full" bind:this={container}>
	<canvas
		bind:this={canvas}
		onmousemove={handleMouseMove}
		onclick={handleClick}
		ondblclick={handleDblClick}
		class="block w-full h-full"
	></canvas>

	<!-- Cluster legend overlay -->
	<ClusterLegend />

	{#if graphStore.loading}
		<div class="absolute inset-0 flex items-center justify-center bg-black/50">
			<div class="text-lg text-white animate-pulse">Loading graph...</div>
		</div>
	{/if}

	{#if !graphStore.subgraph && !graphStore.loading}
		<div class="absolute inset-0 flex items-center justify-center">
			<div class="text-center text-[var(--color-text-muted)]">
				<p class="text-xl mb-2">Search for a crate to explore</p>
				<p class="text-sm">Try "serde", "tokio", or "axum"</p>
			</div>
		</div>
	{/if}

	{#if graphStore.subgraph}
		<div
			class="absolute bottom-4 left-4 text-xs text-[var(--color-text-muted)] bg-[var(--color-bg-panel)]/80 px-2 py-1 rounded"
		>
			{graphStore.nodeCount} nodes, {graphStore.edgeCount} edges
		</div>
	{/if}
</div>
