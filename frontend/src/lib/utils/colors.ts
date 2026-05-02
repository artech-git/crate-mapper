// Stable category-based color palette
const CATEGORY_COLORS: Record<string, string> = {
	'Web programming': '#3b82f6',
	'Network programming': '#6366f1',
	'Asynchronous': '#8b5cf6',
	'Command-line utilities': '#10b981',
	'Database interfaces': '#f59e0b',
	'Cryptography': '#ef4444',
	'Data structures': '#ec4899',
	'Parsing tools': '#14b8a6',
	'Development tools': '#f97316',
	'Embedded development': '#84cc16',
	'Operating systems': '#a855f7',
	'Game development': '#e11d48',
	'Science': '#06b6d4',
	'Encoding': '#d946ef',
	'Filesystem': '#0ea5e9',
	'Concurrency': '#eab308',
	'Mathematics': '#22d3ee',
	'Rust patterns': '#a78bfa',
	'Memory management': '#fb923c',
	'Configuration': '#34d399',
};

// Fallback palette for unknown categories
const FALLBACK_COLORS = [
	'#6366f1', '#8b5cf6', '#a855f7', '#d946ef',
	'#ec4899', '#f43f5e', '#ef4444', '#f97316',
	'#f59e0b', '#eab308', '#84cc16', '#22c55e',
	'#10b981', '#14b8a6', '#06b6d4', '#0ea5e9',
	'#3b82f6', '#2563eb', '#4f46e5', '#7c3aed',
];

export function getCategoryColor(category: string): string {
	if (CATEGORY_COLORS[category]) return CATEGORY_COLORS[category];
	// Deterministic hash-based fallback
	let hash = 0;
	for (let i = 0; i < category.length; i++) {
		hash = category.charCodeAt(i) + ((hash << 5) - hash);
	}
	return FALLBACK_COLORS[Math.abs(hash) % FALLBACK_COLORS.length];
}

export function getNodeColor(depth: number, isCenter: boolean): string {
	if (isCenter) return '#f59e0b'; // Gold for center
	if (depth === 0) return '#f59e0b';
	if (depth <= 1) return '#3b82f6'; // Blue for direct deps
	if (depth <= 2) return '#6366f1'; // Indigo for 2-hop
	return '#8b5cf6'; // Purple for deeper
}

export function getEdgeColor(kind: string): string {
	switch (kind) {
		case 'Normal':
			return 'rgba(100, 116, 139, 0.6)';
		case 'Dev':
			return 'rgba(100, 116, 139, 0.25)';
		case 'Build':
			return 'rgba(100, 116, 139, 0.35)';
		default:
			return 'rgba(100, 116, 139, 0.4)';
	}
}
