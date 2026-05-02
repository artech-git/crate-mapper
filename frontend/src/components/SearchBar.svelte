<script lang="ts">
	import { searchCrates } from '$lib/api/client';
	import { graphStore } from '$lib/stores/graph.svelte';
	import { formatDownloads } from '$lib/utils/format';
	import type { SearchResult } from '$lib/api/types';

	let query = $state('');
	let results: SearchResult[] = $state([]);
	let showDropdown = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout>;
	let inputEl: HTMLInputElement;

	function handleInput() {
		clearTimeout(debounceTimer);
		if (!query.trim()) {
			results = [];
			showDropdown = false;
			return;
		}
		debounceTimer = setTimeout(async () => {
			try {
				results = await searchCrates(query, 10);
				showDropdown = results.length > 0;
			} catch {
				results = [];
			}
		}, 300);
	}

	function selectResult(name: string) {
		query = name;
		showDropdown = false;
		graphStore.loadSubgraph(name);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && query.trim()) {
			showDropdown = false;
			graphStore.loadSubgraph(query.trim());
		}
		if (e.key === 'Escape') {
			showDropdown = false;
		}
	}

	function handleBlur() {
		// Delay to allow click on dropdown item
		setTimeout(() => {
			showDropdown = false;
		}, 200);
	}
</script>

<div class="relative w-full max-w-xl">
	<div class="relative">
		<svg
			class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[var(--color-text-muted)]"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
			/>
		</svg>
		<input
			bind:this={inputEl}
			bind:value={query}
			oninput={handleInput}
			onkeydown={handleKeydown}
			onblur={handleBlur}
			onfocus={() => {
				if (results.length > 0) showDropdown = true;
			}}
			type="text"
			placeholder="Search for crates (e.g., serde, tokio, axum)..."
			class="w-full bg-[var(--color-bg-secondary)] border border-[var(--color-border)] rounded-lg pl-10 pr-4 py-2.5 text-[var(--color-text)] placeholder:text-[var(--color-text-muted)] focus:outline-none focus:border-[var(--color-accent)] transition-colors"
		/>
	</div>

	{#if showDropdown}
		<div
			class="absolute top-full mt-1 w-full bg-[var(--color-bg-panel)] border border-[var(--color-border)] rounded-lg shadow-xl z-50 max-h-80 overflow-y-auto"
		>
			{#each results as result}
				<button
					class="w-full text-left px-4 py-3 hover:bg-[var(--color-bg-secondary)] transition-colors border-b border-[var(--color-border)] last:border-b-0"
					onclick={() => selectResult(result.name)}
				>
					<div class="flex items-center justify-between">
						<span class="font-medium text-[var(--color-text)]">{result.name}</span>
						<span class="text-xs text-[var(--color-text-muted)]"
							>v{result.version} &middot; {formatDownloads(result.downloads)} downloads</span
						>
					</div>
					{#if result.description}
						<p class="text-sm text-[var(--color-text-muted)] mt-0.5 truncate">
							{result.description}
						</p>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>
