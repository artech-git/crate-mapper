export interface SubgraphNode {
	id: string;
	name: string;
	description: string;
	downloads: number;
	version: string;
	depth: number;
	categories: string[];
	// Computed by force simulation
	x?: number;
	y?: number;
	vx?: number;
	vy?: number;
}

export interface SubgraphEdge {
	source: string;
	target: string;
	kind: 'Normal' | 'Dev' | 'Build';
	optional: boolean;
}

export interface SubgraphResponse {
	nodes: SubgraphNode[];
	edges: SubgraphEdge[];
	center: string;
	depth: number;
}

export interface SearchResult {
	name: string;
	description: string;
	downloads: number;
	version: string;
	keywords: string[];
}

export interface DepInfo {
	name: string;
	kind: 'Normal' | 'Dev' | 'Build';
	optional: boolean;
	version_req: string;
	downloads: number;
}

export interface CrateInfoResponse {
	name: string;
	description: string;
	downloads: number;
	version: string;
	repository: string | null;
	keywords: string[];
	categories: string[];
	created_at: string;
	updated_at: string;
	direct_deps: DepInfo[];
	direct_dependents: DepInfo[];
	dep_count: number;
	dependent_count: number;
}

export interface HealthResponse {
	status: string;
	nodes: number;
	edges: number;
}

// Phase 2: Pathfinding
export interface ChainNode {
	name: string;
	downloads: number;
}

export interface ChainResponse {
	path: ChainNode[];
	length: number;
	found: boolean;
}

// Phase 2: Impact / Blast Radius
export interface DepthBucket {
	depth: number;
	count: number;
}

export interface ImpactedCrate {
	name: string;
	downloads: number;
	depth: number;
}

export interface ImpactResponse {
	crate_name: string;
	direct_dependents: number;
	total_impacted: number;
	by_depth: DepthBucket[];
	top_dependents: ImpactedCrate[];
}

// Phase 2: Clustering
export interface Cluster {
	id: number;
	name: string;
	color: string;
	crate_count: number;
	top_crates: string[];
}

export interface ClustersResponse {
	clusters: Cluster[];
	assignments: Record<string, number>;
}
