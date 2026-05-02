<script lang="ts">
	import { graphStore } from '$lib/stores/graph.svelte';
	import { formatDownloads } from '$lib/utils/format';

	const info = $derived(graphStore.crateInfo);
</script>

{#if info}
	<div
		class="w-80 shrink-0 bg-[var(--color-bg-panel)] border-l border-[var(--color-border)] overflow-y-auto"
	>
		<div class="p-4">
			<!-- Header -->
			<div class="mb-4">
				<h2 class="text-lg font-bold text-[var(--color-text)]">{info.name}</h2>
				<div class="flex items-center gap-2 mt-1">
					<span class="text-xs px-2 py-0.5 bg-[var(--color-accent)]/20 text-[var(--color-accent)] rounded"
						>v{info.version}</span
					>
					<span class="text-xs text-[var(--color-text-muted)]"
						>{formatDownloads(info.downloads)} downloads</span
					>
				</div>
			</div>

			<!-- Description -->
			{#if info.description}
				<p class="text-sm text-[var(--color-text-muted)] mb-4 leading-relaxed">
					{info.description}
				</p>
			{/if}

			<!-- Links -->
			{#if info.repository}
				<a
					href={info.repository}
					target="_blank"
					rel="noopener noreferrer"
					class="inline-flex items-center gap-1 text-xs text-[var(--color-accent)] hover:text-[var(--color-accent-hover)] mb-4"
				>
					<svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
						<path
							d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z"
						/>
						<path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
					</svg>
					Repository
				</a>
			{/if}

			<!-- Categories -->
			{#if info.categories.length > 0}
				<div class="mb-4">
					<h3 class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
						Categories
					</h3>
					<div class="flex flex-wrap gap-1">
						{#each info.categories as cat}
							<span
								class="text-xs px-2 py-0.5 bg-[var(--color-bg-secondary)] text-[var(--color-text-muted)] rounded"
								>{cat}</span
							>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Keywords -->
			{#if info.keywords.length > 0}
				<div class="mb-4">
					<h3 class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
						Keywords
					</h3>
					<div class="flex flex-wrap gap-1">
						{#each info.keywords as kw}
							<span
								class="text-xs px-2 py-0.5 bg-[var(--color-bg-secondary)] text-[var(--color-text-muted)] rounded"
								>{kw}</span
							>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Dependencies -->
			<div class="mb-4">
				<h3 class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
					Dependencies ({info.dep_count})
				</h3>
				<div class="space-y-1 max-h-48 overflow-y-auto">
					{#each info.direct_deps as dep}
						<button
							class="w-full text-left flex items-center justify-between px-2 py-1.5 rounded hover:bg-[var(--color-bg-secondary)] transition-colors"
							onclick={() => graphStore.selectCrate(dep.name)}
							ondblclick={() => graphStore.loadSubgraph(dep.name)}
						>
							<span class="text-sm text-[var(--color-text)]">{dep.name}</span>
							<span class="text-xs text-[var(--color-text-muted)]">
								{#if dep.optional}
									<span class="text-yellow-500">opt</span>
								{/if}
								{dep.version_req}
							</span>
						</button>
					{/each}
				</div>
			</div>

			<!-- Dependents -->
			<div>
				<h3 class="text-xs font-semibold text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
					Dependents ({info.dependent_count})
				</h3>
				<div class="space-y-1 max-h-48 overflow-y-auto">
					{#each info.direct_dependents.slice(0, 20) as dep}
						<button
							class="w-full text-left flex items-center justify-between px-2 py-1.5 rounded hover:bg-[var(--color-bg-secondary)] transition-colors"
							onclick={() => graphStore.selectCrate(dep.name)}
							ondblclick={() => graphStore.loadSubgraph(dep.name)}
						>
							<span class="text-sm text-[var(--color-text)]">{dep.name}</span>
							<span class="text-xs text-[var(--color-text-muted)]"
								>{formatDownloads(dep.downloads)}</span
							>
						</button>
					{/each}
					{#if info.dependent_count > 20}
						<p class="text-xs text-[var(--color-text-muted)] px-2 py-1">
							...and {info.dependent_count - 20} more
						</p>
					{/if}
				</div>
			</div>

			<!-- Metadata -->
			<div class="mt-4 pt-4 border-t border-[var(--color-border)]">
				<div class="text-xs text-[var(--color-text-muted)] space-y-1">
					<p>Created: {new Date(info.created_at).toLocaleDateString()}</p>
					<p>Updated: {new Date(info.updated_at).toLocaleDateString()}</p>
				</div>
			</div>
		</div>
	</div>
{/if}
