import type { SubgraphNode, SubgraphEdge } from '$lib/api/types';
import { getNodeColor, getEdgeColor } from '$lib/utils/colors';
import { nodeRadius } from '$lib/utils/format';
interface RenderNode extends SubgraphNode {
	x: number;
	y: number;
}

interface RenderContext {
	ctx: CanvasRenderingContext2D;
	nodes: RenderNode[];
	edges: SubgraphEdge[];
	center: string;
	hoveredNode: string | null;
	selectedNode: string | null;
	transform: { x: number; y: number; k: number };
	width: number;
	height: number;
	// Phase 2 additions
	chainNodeIds?: Set<string>;
	chainEdgeKeys?: Set<string>;
	clusterColorMap?: Map<string, string>;
	highlightedCluster?: number | null;
	clusterAssignments?: Record<string, number>;
	filteredNodeIds?: Set<string>;
	filteredEdges?: SubgraphEdge[];
}

export function render(rc: RenderContext) {
	const {
		ctx,
		nodes,
		center,
		hoveredNode,
		selectedNode,
		transform,
		width,
		height,
		chainNodeIds,
		chainEdgeKeys,
		clusterColorMap,
		highlightedCluster,
		clusterAssignments,
		filteredNodeIds
	} = rc;

	// Use filtered edges if provided, otherwise fall back to all edges
	const edges = rc.filteredEdges ?? rc.edges;

	const hasChain = chainNodeIds && chainNodeIds.size > 0;
	const hasClusterHighlight = highlightedCluster != null && clusterAssignments;

	ctx.save();
	ctx.clearRect(0, 0, width, height);

	// Apply zoom/pan transform
	ctx.translate(transform.x, transform.y);
	ctx.scale(transform.k, transform.k);

	// Build node position lookup
	const nodeMap = new Map<string, RenderNode>();
	for (const n of nodes) {
		nodeMap.set(n.id, n);
	}

	// Draw edges
	for (const edge of edges) {
		const source = nodeMap.get(edge.source);
		const target = nodeMap.get(edge.target);
		if (!source || !target) continue;

		// Skip edges for nodes not in filtered set
		if (
			filteredNodeIds &&
			(!filteredNodeIds.has(edge.source) || !filteredNodeIds.has(edge.target))
		) {
			continue;
		}

		const isHighlighted =
			hoveredNode === edge.source ||
			hoveredNode === edge.target ||
			selectedNode === edge.source ||
			selectedNode === edge.target;

		const isChainEdge =
			hasChain &&
			chainEdgeKeys &&
			(chainEdgeKeys.has(`${edge.source}->${edge.target}`) ||
				chainEdgeKeys.has(`${edge.target}->${edge.source}`));

		ctx.beginPath();
		ctx.moveTo(source.x, source.y);
		ctx.lineTo(target.x, target.y);

		if (edge.kind === 'Dev') {
			ctx.setLineDash([4, 4]);
		} else if (edge.kind === 'Build') {
			ctx.setLineDash([8, 4]);
		} else {
			ctx.setLineDash([]);
		}

		if (isChainEdge) {
			ctx.strokeStyle = '#f59e0b';
			ctx.lineWidth = 3;
			ctx.globalAlpha = 1;
		} else if (hasChain) {
			// Dim non-chain edges when chain is active
			ctx.strokeStyle = getEdgeColor(edge.kind);
			ctx.lineWidth = 0.5;
			ctx.globalAlpha = 0.15;
		} else if (isHighlighted) {
			ctx.strokeStyle = 'rgba(148, 163, 184, 0.8)';
			ctx.lineWidth = 1.5;
			ctx.globalAlpha = 1;
		} else {
			ctx.strokeStyle = getEdgeColor(edge.kind);
			ctx.lineWidth = 0.8;
			ctx.globalAlpha = 1;
		}

		ctx.stroke();
		ctx.setLineDash([]);
		ctx.globalAlpha = 1;
	}

	// Dim non-connected nodes when hovering
	const connectedToHover = new Set<string>();
	if (hoveredNode) {
		connectedToHover.add(hoveredNode);
		for (const edge of edges) {
			if (edge.source === hoveredNode) connectedToHover.add(edge.target);
			if (edge.target === hoveredNode) connectedToHover.add(edge.source);
		}
	}

	// Draw nodes
	for (const node of nodes) {
		// Skip nodes not in filtered set
		if (filteredNodeIds && !filteredNodeIds.has(node.id)) {
			continue;
		}

		const r = nodeRadius(node.downloads);
		const isCenter = node.id === center;
		const isHovered = node.id === hoveredNode;
		const isSelected = node.id === selectedNode;
		const isDimmed = hoveredNode !== null && !connectedToHover.has(node.id);
		const isChainNode = hasChain && chainNodeIds!.has(node.id);
		const isClusterDimmed =
			hasClusterHighlight && clusterAssignments![node.name] !== highlightedCluster;

		// Determine alpha
		let alpha = 1;
		if (hasChain && !isChainNode) {
			alpha = 0.12;
		} else if (isDimmed) {
			alpha = 0.15;
		} else if (isClusterDimmed) {
			alpha = 0.15;
		}

		ctx.globalAlpha = alpha;

		// Node circle
		ctx.beginPath();
		ctx.arc(node.x, node.y, isChainNode ? r + 1 : r, 0, Math.PI * 2);

		// Choose color: cluster color if active, otherwise depth-based
		let color: string;
		if (clusterColorMap && clusterColorMap.size > 0) {
			color = clusterColorMap.get(node.name) ?? getNodeColor(node.depth, isCenter);
		} else {
			color = getNodeColor(node.depth, isCenter);
		}
		ctx.fillStyle = color;
		ctx.fill();

		// Chain node golden ring
		if (isChainNode) {
			ctx.beginPath();
			ctx.arc(node.x, node.y, r + 4, 0, Math.PI * 2);
			ctx.strokeStyle = '#f59e0b';
			ctx.lineWidth = 2.5;
			ctx.stroke();
		}

		// Selection/hover ring
		if (isSelected || isHovered) {
			ctx.beginPath();
			ctx.arc(node.x, node.y, r + 3, 0, Math.PI * 2);
			ctx.strokeStyle = isSelected ? '#f59e0b' : '#818cf8';
			ctx.lineWidth = 2;
			ctx.stroke();
		}

		// Glow for center node
		if (isCenter && !hasChain) {
			ctx.beginPath();
			ctx.arc(node.x, node.y, r + 5, 0, Math.PI * 2);
			ctx.strokeStyle = 'rgba(245, 158, 11, 0.4)';
			ctx.lineWidth = 2;
			ctx.stroke();
		}

		// Label for visible nodes (center, hovered, selected, chain, or large enough)
		const showLabel =
			isCenter || isHovered || isSelected || isChainNode || (r > 6 && alpha > 0.5);
		if (showLabel) {
			ctx.fillStyle = alpha < 0.5 ? `rgba(226, 232, 240, ${alpha})` : '#e2e8f0';
			ctx.font = `${isCenter || isHovered || isChainNode ? 'bold ' : ''}${Math.max(10, Math.min(14, r + 2))}px Inter, sans-serif`;
			ctx.textAlign = 'center';
			ctx.textBaseline = 'top';
			ctx.fillText(node.name, node.x, node.y + r + 4);
		}

		ctx.globalAlpha = 1;
	}

	ctx.restore();
}

export function findNodeAtPoint(
	nodes: RenderNode[],
	x: number,
	y: number,
	transform: { x: number; y: number; k: number }
): RenderNode | null {
	// Transform screen coords to graph coords
	const gx = (x - transform.x) / transform.k;
	const gy = (y - transform.y) / transform.k;

	let closest: RenderNode | null = null;
	let closestDist = Infinity;

	for (const node of nodes) {
		const r = nodeRadius(node.downloads) + 4; // Margin for easier clicking
		const dx = gx - node.x;
		const dy = gy - node.y;
		const dist = Math.sqrt(dx * dx + dy * dy);
		if (dist < r && dist < closestDist) {
			closest = node;
			closestDist = dist;
		}
	}

	return closest;
}
