<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { searchCrates } from '$lib/api/client';
	import { formatDownloads } from '$lib/utils/format';
	import type { SearchResult } from '$lib/api/types';

	let sourceQuery = $state('');
	let targetQuery = $state('');
	let sourceResults: SearchResult[] = $state([]);
	let targetResults: SearchResult[] = $state([]);
	let showSourceDropdown = $state(false);
	let showTargetDropdown = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout>;
	let isOpen = $state(false);

	async function searchDebounced(
		query: string,
		setter: (results: SearchResult[]) => void,
		showSetter: (show: boolean) => void
	) {
		clearTimeout(debounceTimer);
		if (!query.trim()) {
			setter([]);
			showSetter(false);
			return;
		}
		debounceTimer = setTimeout(async () => {
			try {
				const results = await searchCrates(query, 5);
				setter(results);
				showSetter(results.length > 0);
			} catch {
				setter([]);
			}
		}, 300);
	}

	function handleSourceInput() {
		searchDebounced(
			sourceQuery,
			(r) => (sourceResults = r),
			(s) => (showSourceDropdown = s)
		);
	}

	function handleTargetInput() {
		searchDebounced(
			targetQuery,
			(r) => (targetResults = r),
			(s) => (showTargetDropdown = s)
		);
	}

	function selectSource(name: string) {
		sourceQuery = name;
		showSourceDropdown = false;
	}

	function selectTarget(name: string) {
		targetQuery = name;
		showTargetDropdown = false;
	}

	function findPath() {
		if (sourceQuery.trim() && targetQuery.trim()) {
			graphStore.findChain(sourceQuery.trim(), targetQuery.trim());
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			findPath();
		}
	}

	function swapCrates() {
		const tmp = sourceQuery;
		sourceQuery = targetQuery;
		targetQuery = tmp;
	}
</script>

<div class="relative">
	<button
		class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs rounded transition-colors {isOpen
			? 'bg-[var(--color-accent)] text-white'
			: 'bg-[var(--color-bg-secondary)] text-[var(--color-text-muted)] hover:text-[var(--color-text)]'}"
		onclick={() => {
			isOpen = !isOpen;
			if (!isOpen) graphStore.clearChain();
		}}
	>
		<svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M13 7l5 5m0 0l-5 5m5-5H6"
			/>
		</svg>
		Path Finder
	</button>

	{#if isOpen}
		<div
			class="absolute top-full mt-2 right-0 w-72 bg-[var(--color-bg-panel)] border border-[var(--color-border)] rounded-lg shadow-xl z-50 p-3"
		>
			<h3 class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
				Shortest Dependency Chain
			</h3>

			<!-- Source input -->
			<div class="relative mb-2">
				<input
					bind:value={sourceQuery}
					oninput={handleSourceInput}
					onkeydown={handleKeydown}
					onfocus={() => {
						if (sourceResults.length > 0) showSourceDropdown = true;
					}}
					onblur={() => setTimeout(() => (showSourceDropdown = false), 200)}
					placeholder="From crate..."
					class="w-full bg-[var(--color-bg-secondary)] border border-[var(--color-border)] rounded px-2.5 py-1.5 text-sm text-[var(--color-text)] placeholder:text-[var(--color-text-muted)] focus:outline-none focus:border-[var(--color-accent)]"
				/>
				{#if showSourceDropdown}
					<div
						class="absolute top-full mt-0.5 w-full bg-[var(--color-bg-panel)] border border-[var(--color-border)] rounded shadow-lg z-50 max-h-40 overflow-y-auto"
					>
						{#each sourceResults as r}
							<button
								class="w-full text-left px-2.5 py-1.5 text-sm hover:bg-[var(--color-bg-secondary)] text-[var(--color-text)]"
								onmousedown={() => selectSource(r.name)}
							>
								{r.name}
							</button>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Swap button -->
			<div class="flex justify-center mb-2">
				<button
					class="p-1 rounded hover:bg-[var(--color-bg-secondary)] text-[var(--color-text-muted)]"
					onclick={swapCrates}
					title="Swap source and target"
				>
					<svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"
						/>
					</svg>
				</button>
			</div>

			<!-- Target input -->
			<div class="relative mb-3">
				<input
					bind:value={targetQuery}
					oninput={handleTargetInput}
					onkeydown={handleKeydown}
					onfocus={() => {
						if (targetResults.length > 0) showTargetDropdown = true;
					}}
					onblur={() => setTimeout(() => (showTargetDropdown = false), 200)}
					placeholder="To crate..."
					class="w-full bg-[var(--color-bg-secondary)] border border-[var(--color-border)] rounded px-2.5 py-1.5 text-sm text-[var(--color-text)] placeholder:text-[var(--color-text-muted)] focus:outline-none focus:border-[var(--color-accent)]"
				/>
				{#if showTargetDropdown}
					<div
						class="absolute top-full mt-0.5 w-full bg-[var(--color-bg-panel)] border border-[var(--color-border)] rounded shadow-lg z-50 max-h-40 overflow-y-auto"
					>
						{#each targetResults as r}
							<button
								class="w-full text-left px-2.5 py-1.5 text-sm hover:bg-[var(--color-bg-secondary)] text-[var(--color-text)]"
								onmousedown={() => selectTarget(r.name)}
							>
								{r.name}
							</button>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Find button -->
			<button
				class="w-full py-1.5 px-3 rounded text-sm font-medium transition-colors {sourceQuery &&
				targetQuery
					? 'bg-[var(--color-accent)] text-white hover:bg-[var(--color-accent-hover)]'
					: 'bg-[var(--color-bg-secondary)] text-[var(--color-text-muted)] cursor-not-allowed'}"
				onclick={findPath}
				disabled={!sourceQuery || !targetQuery || graphStore.chainLoading}
			>
				{graphStore.chainLoading ? 'Finding...' : 'Find Path'}
			</button>

			<!-- Results -->
			{#if graphStore.chain}
				<div class="mt-3 pt-3 border-t border-[var(--color-border)]">
					{#if graphStore.chain.found}
						<p class="text-xs text-[var(--color-text-muted)] mb-2">
							Chain length: <span class="text-[var(--color-accent)] font-semibold"
								>{graphStore.chain.length}</span
							>
							{graphStore.chain.length === 1 ? 'hop' : 'hops'}
						</p>
						<div class="space-y-0.5">
							{#each graphStore.chain.path as node, i}
								<div class="flex items-center gap-1.5">
									{#if i > 0}
										<svg
											class="w-3 h-3 text-[var(--color-accent)] shrink-0"
											fill="none"
											viewBox="0 0 24 24"
											stroke="currentColor"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M19 14l-7 7m0 0l-7-7m7 7V3"
											/>
										</svg>
									{:else}
										<span class="w-3"></span>
									{/if}
									<button
										class="text-sm text-[var(--color-text)] hover:text-[var(--color-accent)] transition-colors"
										onclick={() => graphStore.selectCrate(node.name)}
									>
										{node.name}
									</button>
									<span class="text-xs text-[var(--color-text-muted)] ml-auto"
										>{formatDownloads(node.downloads)}</span
									>
								</div>
							{/each}
						</div>
					{:else}
						<p class="text-xs text-[var(--color-text-muted)]">
							No dependency chain found between these crates.
						</p>
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>
