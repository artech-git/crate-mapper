import type {
	SubgraphResponse,
	SearchResult,
	CrateInfoResponse,
	HealthResponse,
	ChainResponse,
	ImpactResponse,
	ClustersResponse
} from './types';

const BASE = '/api';

export async function getHealth(): Promise<HealthResponse> {
	const res = await fetch(`${BASE}/health`);
	if (!res.ok) throw new Error(`Health check failed: ${res.status}`);
	return res.json();
}

export async function searchCrates(q: string, limit = 20): Promise<SearchResult[]> {
	if (!q.trim()) return [];
	const res = await fetch(`${BASE}/search?q=${encodeURIComponent(q)}&limit=${limit}`);
	if (!res.ok) throw new Error(`Search failed: ${res.status}`);
	return res.json();
}

export async function getCrateInfo(name: string): Promise<CrateInfoResponse> {
	const res = await fetch(`${BASE}/crate/${encodeURIComponent(name)}`);
	if (!res.ok) throw new Error(`Crate not found: ${name}`);
	return res.json();
}

export async function getCrateSubgraph(
	name: string,
	depth = 2
): Promise<SubgraphResponse> {
	const res = await fetch(
		`${BASE}/crate/${encodeURIComponent(name)}/subgraph?depth=${depth}`
	);
	if (!res.ok) throw new Error(`Subgraph not found: ${name}`);
	return res.json();
}

export async function getChain(source: string, target: string): Promise<ChainResponse> {
	const res = await fetch(
		`${BASE}/chain/${encodeURIComponent(source)}/${encodeURIComponent(target)}`
	);
	if (!res.ok) throw new Error(`Chain lookup failed: ${res.status}`);
	return res.json();
}

export async function getImpact(name: string): Promise<ImpactResponse> {
	const res = await fetch(`${BASE}/crate/${encodeURIComponent(name)}/impact`);
	if (!res.ok) throw new Error(`Impact analysis failed: ${name}`);
	return res.json();
}

export async function getClusters(): Promise<ClustersResponse> {
	const res = await fetch(`${BASE}/clusters`);
	if (!res.ok) throw new Error(`Clusters fetch failed: ${res.status}`);
	return res.json();
}
