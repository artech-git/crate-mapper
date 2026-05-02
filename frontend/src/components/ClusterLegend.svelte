<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
</script>

{#if graphStore.showClusters && graphStore.clusters}
	<div
		class="absolute top-4 left-4 bg-[var(--color-bg-panel)]/90 border border-[var(--color-border)] rounded-lg shadow-lg p-3 max-w-48 z-10"
	>
		<h3
			class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2"
		>
			Clusters
		</h3>
		<div class="space-y-1 max-h-64 overflow-y-auto">
			{#each graphStore.clusters.clusters as cluster}
				<button
					class="w-full flex items-center gap-2 px-1.5 py-1 rounded text-xs transition-colors {graphStore.highlightedCluster ===
					cluster.id
						? 'bg-[var(--color-bg-secondary)]'
						: 'hover:bg-[var(--color-bg-secondary)]/50'}"
					onmouseenter={() => graphStore.highlightCluster(cluster.id)}
					onmouseleave={() => graphStore.highlightCluster(null)}
					onclick={() =>
						graphStore.highlightCluster(
							graphStore.highlightedCluster === cluster.id ? null : cluster.id
						)}
				>
					<span
						class="w-2.5 h-2.5 rounded-full shrink-0"
						style="background-color: {cluster.color}"
					></span>
					<span class="text-[var(--color-text)] truncate">{cluster.name}</span>
					<span class="text-[var(--color-text-muted)] ml-auto shrink-0"
						>{cluster.crate_count}</span
					>
				</button>
			{/each}
		</div>
	</div>
{/if}
