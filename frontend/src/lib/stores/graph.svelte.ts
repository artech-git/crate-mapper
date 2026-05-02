import { getCrateSubgraph, getCrateInfo, getChain, getImpact, getClusters } from '$lib/api/client';
import type {
	SubgraphResponse,
	CrateInfoResponse,
	ChainResponse,
	ImpactResponse,
	ClustersResponse
} from '$lib/api/types';

interface NodePosition {
	x: number;
	y: number;
}

export type DepKindFilter = 'Normal' | 'Dev' | 'Build';

class GraphStore {
	subgraph: SubgraphResponse | null = $state(null);
	positions: Map<string, NodePosition> = $state(new Map());
	selectedCrate: string | null = $state(null);
	crateInfo: CrateInfoResponse | null = $state(null);
	depth: number = $state(2);
	loading: boolean = $state(false);
	error: string | null = $state(null);

	// Phase 2: Chain / Pathfinding
	chain: ChainResponse | null = $state(null);
	chainSource: string = $state('');
	chainTarget: string = $state('');
	chainLoading: boolean = $state(false);

	// Phase 2: Impact / Blast Radius
	impact: ImpactResponse | null = $state(null);
	impactLoading: boolean = $state(false);

	// Phase 2: Clusters
	clusters: ClustersResponse | null = $state(null);
	showClusters: boolean = $state(false);
	highlightedCluster: number | null = $state(null);

	// Phase 2: Filters
	depKindFilters: Set<DepKindFilter> = $state(new Set(['Normal', 'Dev', 'Build']));
	categoryFilters: Set<string> = $state(new Set());

	nodeCount: number = $derived(this.subgraph?.nodes.length ?? 0);
	edgeCount: number = $derived(this.subgraph?.edges.length ?? 0);

	// Set of node IDs in the chain path
	chainNodeIds: Set<string> = $derived.by(() => {
		if (!this.chain?.found) return new Set<string>();
		return new Set(this.chain.path.map((n) => n.name));
	});

	// Chain edge set (source->target pairs that form the path)
	chainEdgeKeys: Set<string> = $derived.by(() => {
		if (!this.chain?.found || this.chain.path.length < 2) return new Set<string>();
		const keys = new Set<string>();
		for (let i = 0; i < this.chain.path.length - 1; i++) {
			keys.add(`${this.chain.path[i].name}->${this.chain.path[i + 1].name}`);
		}
		return keys;
	});

	// Get cluster color for a node (if clusters are active)
	clusterColorMap: Map<string, string> = $derived.by(() => {
		const map = new Map<string, string>();
		if (!this.clusters || !this.showClusters) return map;
		for (const [name, clusterId] of Object.entries(this.clusters.assignments)) {
			const cluster = this.clusters.clusters.find((c) => c.id === clusterId);
			if (cluster) {
				map.set(name, cluster.color);
			}
		}
		return map;
	});

	// Available categories from current subgraph
	availableCategories: string[] = $derived.by(() => {
		if (!this.subgraph) return [];
		const cats = new Set<string>();
		for (const node of this.subgraph.nodes) {
			for (const cat of node.categories) {
				cats.add(cat);
			}
		}
		return Array.from(cats).sort();
	});

	// Filtered edges based on dep kind
	filteredEdges = $derived.by(() => {
		if (!this.subgraph) return [];
		return this.subgraph.edges.filter((e) => this.depKindFilters.has(e.kind));
	});

	// Filtered nodes based on category
	filteredNodeIds: Set<string> = $derived.by(() => {
		if (!this.subgraph) return new Set<string>();
		if (this.categoryFilters.size === 0) {
			// No category filter = show all
			return new Set(this.subgraph.nodes.map((n) => n.id));
		}
		return new Set(
			this.subgraph.nodes
				.filter(
					(n) =>
						n.categories.some((c) => this.categoryFilters.has(c)) ||
						n.id === this.subgraph!.center
				)
				.map((n) => n.id)
		);
	});

	// Get nodes with positions merged in
	nodesWithPositions = $derived.by(() => {
		if (!this.subgraph) return [];
		return this.subgraph.nodes.map((node) => {
			const pos = this.positions.get(node.id);
			return {
				...node,
				x: pos?.x ?? 0,
				y: pos?.y ?? 0
			};
		});
	});

	updatePositions(updates: Array<{ id: string; x: number; y: number }>) {
		const newMap = new Map(this.positions);
		for (const u of updates) {
			newMap.set(u.id, { x: u.x, y: u.y });
		}
		this.positions = newMap;
	}

	async loadSubgraph(name: string) {
		this.loading = true;
		this.error = null;
		// Clear chain/impact when loading new graph
		this.chain = null;
		this.impact = null;
		try {
			const [subgraph, info] = await Promise.all([
				getCrateSubgraph(name, this.depth),
				getCrateInfo(name)
			]);
			this.subgraph = subgraph;
			this.crateInfo = info;
			this.selectedCrate = name;
			this.positions = new Map();
			// Reset category filters when loading new graph
			this.categoryFilters = new Set();
		} catch (e) {
			this.error = e instanceof Error ? e.message : 'Failed to load crate';
		} finally {
			this.loading = false;
		}
	}

	async selectCrate(name: string) {
		try {
			this.crateInfo = await getCrateInfo(name);
			this.selectedCrate = name;
		} catch {
			// Crate info fetch failed, just select it
			this.selectedCrate = name;
		}
	}

	setDepth(depth: number) {
		this.depth = Math.max(1, Math.min(4, depth));
		if (this.selectedCrate) {
			this.loadSubgraph(this.selectedCrate);
		}
	}

	// Phase 2: Find shortest chain
	async findChain(source: string, target: string) {
		this.chainLoading = true;
		this.chainSource = source;
		this.chainTarget = target;
		try {
			this.chain = await getChain(source, target);
		} catch {
			this.chain = { path: [], length: 0, found: false };
		} finally {
			this.chainLoading = false;
		}
	}

	clearChain() {
		this.chain = null;
		this.chainSource = '';
		this.chainTarget = '';
	}

	// Phase 2: Calculate impact
	async calculateImpact(name: string) {
		this.impactLoading = true;
		try {
			this.impact = await getImpact(name);
		} catch {
			this.impact = null;
		} finally {
			this.impactLoading = false;
		}
	}

	clearImpact() {
		this.impact = null;
	}

	// Phase 2: Load clusters
	async loadClusters() {
		try {
			this.clusters = await getClusters();
			this.showClusters = true;
		} catch {
			this.clusters = null;
		}
	}

	toggleClusters() {
		if (this.showClusters) {
			this.showClusters = false;
			this.highlightedCluster = null;
		} else {
			if (!this.clusters) {
				this.loadClusters();
			} else {
				this.showClusters = true;
			}
		}
	}

	highlightCluster(id: number | null) {
		this.highlightedCluster = id;
	}

	// Phase 2: Filter controls
	toggleDepKind(kind: DepKindFilter) {
		const next = new Set(this.depKindFilters);
		if (next.has(kind)) {
			// Don't allow removing all filters
			if (next.size > 1) next.delete(kind);
		} else {
			next.add(kind);
		}
		this.depKindFilters = next;
	}

	toggleCategoryFilter(category: string) {
		const next = new Set(this.categoryFilters);
		if (next.has(category)) {
			next.delete(category);
		} else {
			next.add(category);
		}
		this.categoryFilters = next;
	}

	clearCategoryFilters() {
		this.categoryFilters = new Set();
	}
}

export const graphStore = new GraphStore();
