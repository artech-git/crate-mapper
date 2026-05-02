<script lang="ts">
	import SearchBar from '../components/SearchBar.svelte';
	import GraphCanvas from '../components/GraphCanvas.svelte';
	import CratePanel from '../components/CratePanel.svelte';
	import FilterBar from '../components/FilterBar.svelte';
	import PathFinder from '../components/PathFinder.svelte';
	import { graphStore } from '$lib/stores/graph.svelte';
</script>

<div class="h-screen flex flex-col overflow-hidden">
	<!-- Top bar -->
	<header
		class="flex items-center gap-4 px-4 py-3 bg-[var(--color-bg-panel)] border-b border-[var(--color-border)] shrink-0"
	>
		<div class="flex items-center gap-2">
			<svg class="w-6 h-6 text-[var(--color-accent)]" viewBox="0 0 24 24" fill="currentColor">
				<path
					d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"
				/>
			</svg>
			<h1 class="text-lg font-bold text-[var(--color-text)]">CrateMapper</h1>
		</div>

		<SearchBar />

		{#if graphStore.subgraph}
			<div class="flex items-center gap-3 ml-auto shrink-0">
				<!-- Path Finder -->
				<PathFinder />

				<!-- Depth selector -->
				<div class="flex items-center gap-2">
					<label class="text-xs text-[var(--color-text-muted)]">Depth:</label>
					<div class="flex gap-1">
						{#each [1, 2, 3, 4] as d}
							<button
								class="w-7 h-7 text-xs rounded {graphStore.depth === d
									? 'bg-[var(--color-accent)] text-white'
									: 'bg-[var(--color-bg-secondary)] text-[var(--color-text-muted)] hover:bg-[var(--color-border)]'} transition-colors"
								onclick={() => graphStore.setDepth(d)}
							>
								{d}
							</button>
						{/each}
					</div>
				</div>
			</div>
		{/if}
	</header>

	<!-- Main content -->
	<div class="flex flex-1 overflow-hidden">
		<!-- Left sidebar: Filters -->
		<FilterBar />

		<!-- Graph area -->
		<main class="flex-1 relative">
			<GraphCanvas />
		</main>

		<!-- Right sidebar: Crate info -->
		<CratePanel />
	</div>
</div>
