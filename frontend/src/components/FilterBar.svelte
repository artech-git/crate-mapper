<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import type { DepKindFilter } from '$lib/stores/graph.svelte';

	const depKinds: { kind: DepKindFilter; label: string; style: string }[] = [
		{ kind: 'Normal', label: 'Normal', style: 'border-solid' },
		{ kind: 'Dev', label: 'Dev', style: 'border-dashed' },
		{ kind: 'Build', label: 'Build', style: 'border-dotted' }
	];
</script>

{#if graphStore.subgraph}
	<div
		class="w-56 shrink-0 bg-[var(--color-bg-panel)] border-r border-[var(--color-border)] overflow-y-auto"
	>
		<div class="p-3">
			<!-- Dependency Kind Filters -->
			<div class="mb-4">
				<h3
					class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2"
				>
					Dependency Kind
				</h3>
				<div class="space-y-1">
					{#each depKinds as { kind, label, style }}
						<button
							class="w-full flex items-center gap-2 px-2 py-1.5 rounded text-sm transition-colors {graphStore.depKindFilters.has(
								kind
							)
								? 'bg-[var(--color-bg-secondary)] text-[var(--color-text)]'
								: 'text-[var(--color-text-muted)] hover:bg-[var(--color-bg-secondary)]/50'}"
							onclick={() => graphStore.toggleDepKind(kind)}
						>
							<span
								class="w-5 h-0 border-t-2 {style} {graphStore.depKindFilters.has(kind)
									? 'border-[var(--color-accent)]'
									: 'border-[var(--color-text-muted)]'}"
							></span>
							{label}
							{#if graphStore.depKindFilters.has(kind)}
								<svg class="w-3 h-3 ml-auto text-[var(--color-accent)]" fill="currentColor" viewBox="0 0 20 20">
									<path
										fill-rule="evenodd"
										d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
										clip-rule="evenodd"
									/>
								</svg>
							{/if}
						</button>
					{/each}
				</div>
			</div>

			<!-- Cluster Toggle -->
			<div class="mb-4">
				<h3
					class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2"
				>
					Visualization
				</h3>
				<button
					class="w-full flex items-center gap-2 px-2 py-1.5 rounded text-sm transition-colors {graphStore.showClusters
						? 'bg-[var(--color-bg-secondary)] text-[var(--color-text)]'
						: 'text-[var(--color-text-muted)] hover:bg-[var(--color-bg-secondary)]/50'}"
					onclick={() => graphStore.toggleClusters()}
				>
					<span
						class="w-3 h-3 rounded-full {graphStore.showClusters
							? 'bg-[var(--color-accent)]'
							: 'bg-[var(--color-text-muted)]/30'}"
					></span>
					Cluster Colors
				</button>
			</div>

			<!-- Category Filters -->
			{#if graphStore.availableCategories.length > 0}
				<div class="mb-4">
					<div class="flex items-center justify-between mb-2">
						<h3
							class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider"
						>
							Categories
						</h3>
						{#if graphStore.categoryFilters.size > 0}
							<button
								class="text-xs text-[var(--color-accent)] hover:text-[var(--color-accent-hover)]"
								onclick={() => graphStore.clearCategoryFilters()}
							>
								Clear
							</button>
						{/if}
					</div>
					<div class="space-y-0.5 max-h-64 overflow-y-auto">
						{#each graphStore.availableCategories as cat}
							<button
								class="w-full text-left flex items-center gap-2 px-2 py-1 rounded text-xs transition-colors {graphStore.categoryFilters.has(
									cat
								)
									? 'bg-[var(--color-bg-secondary)] text-[var(--color-text)]'
									: 'text-[var(--color-text-muted)] hover:bg-[var(--color-bg-secondary)]/50'}"
								onclick={() => graphStore.toggleCategoryFilter(cat)}
							>
								{#if graphStore.categoryFilters.has(cat)}
									<svg class="w-3 h-3 text-[var(--color-accent)] shrink-0" fill="currentColor" viewBox="0 0 20 20">
										<path
											fill-rule="evenodd"
											d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
											clip-rule="evenodd"
										/>
									</svg>
								{:else}
									<span class="w-3 h-3 shrink-0"></span>
								{/if}
								<span class="truncate">{cat}</span>
							</button>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Impact Button -->
			{#if graphStore.selectedCrate}
				<div class="mb-4">
					<h3
						class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2"
					>
						Analysis
					</h3>
					<button
						class="w-full text-left flex items-center gap-2 px-2 py-1.5 rounded text-sm transition-colors text-[var(--color-text-muted)] hover:bg-[var(--color-bg-secondary)] hover:text-[var(--color-text)]"
						onclick={() => graphStore.calculateImpact(graphStore.selectedCrate!)}
						disabled={graphStore.impactLoading}
					>
						<svg class="w-4 h-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M13 10V3L4 14h7v7l9-11h-7z"
							/>
						</svg>
						{graphStore.impactLoading ? 'Calculating...' : `Blast Radius`}
					</button>
					{#if graphStore.impact}
						<button
							class="w-full text-left px-2 py-1 text-xs text-[var(--color-text-muted)] hover:text-[var(--color-accent)]"
							onclick={() => graphStore.clearImpact()}
						>
							Clear impact view
						</button>
					{/if}
				</div>
			{/if}

			<!-- Impact Results (inline) -->
			{#if graphStore.impact}
				<div
					class="p-2 rounded bg-[var(--color-bg-secondary)] border border-[var(--color-border)]"
				>
					<h4 class="text-xs font-semibold text-[var(--color-accent)] mb-1">
						{graphStore.impact.crate_name} Impact
					</h4>
					<div class="text-xs text-[var(--color-text-muted)] space-y-0.5">
						<p>
							Direct: <span class="text-[var(--color-text)]"
								>{graphStore.impact.direct_dependents}</span
							>
						</p>
						<p>
							Total: <span class="text-[var(--color-text)]"
								>{graphStore.impact.total_impacted}</span
							>
						</p>
						{#if graphStore.impact.by_depth.length > 0}
							<div class="mt-1 space-y-0.5">
								{#each graphStore.impact.by_depth as bucket}
									<div class="flex items-center gap-1">
										<span class="w-12">Depth {bucket.depth}:</span>
										<div
											class="h-2 rounded bg-[var(--color-accent)]/60"
											style="width: {Math.max(
												8,
												(bucket.count / graphStore.impact.total_impacted) * 100
											)}px"
										></div>
										<span>{bucket.count}</span>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}
