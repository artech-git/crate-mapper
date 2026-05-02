import * as d3 from 'd3-force';

interface SimNode extends d3.SimulationNodeDatum {
	id: string;
	downloads: number;
	depth: number;
	isCenter: boolean;
}

interface SimLink extends d3.SimulationLinkDatum<SimNode> {
	source: string | SimNode;
	target: string | SimNode;
	kind: string;
}

let simulation: d3.Simulation<SimNode, SimLink> | null = null;

self.onmessage = (event: MessageEvent) => {
	const { type, nodes, links, config } = event.data;

	switch (type) {
		case 'init': {
			if (simulation) simulation.stop();

			const simNodes: SimNode[] = nodes.map((n: any) => ({
				...n,
				isCenter: n.id === config.center
			}));

			const simLinks: SimLink[] = links.map((l: any) => ({
				source: l.source,
				target: l.target,
				kind: l.kind
			}));

			const nodeRadius = (d: SimNode) => 4 + Math.log10(d.downloads + 1) * 2;

			simulation = d3
				.forceSimulation<SimNode>(simNodes)
				.force(
					'link',
					d3
						.forceLink<SimNode, SimLink>(simLinks)
						.id((d) => d.id)
						.distance(80)
				)
				.force('charge', d3.forceManyBody<SimNode>().strength(-200))
				.force('center', d3.forceCenter(config.width / 2, config.height / 2))
				.force(
					'collide',
					d3.forceCollide<SimNode>().radius((d) => nodeRadius(d) + 4)
				)
				.alphaDecay(0.02)
				.on('tick', () => {
					self.postMessage({
						type: 'tick',
						nodes: simNodes.map((n) => ({
							id: n.id,
							x: n.x!,
							y: n.y!
						}))
					});
				})
				.on('end', () => {
					self.postMessage({ type: 'end' });
				});
			break;
		}

		case 'reheat': {
			if (simulation) {
				if (config) {
					const charge = simulation.force('charge') as d3.ForceManyBody<SimNode>;
					if (charge && config.strength) charge.strength(config.strength);

					const link = simulation.force('link') as d3.ForceLink<SimNode, SimLink>;
					if (link && config.distance) link.distance(config.distance);
				}
				simulation.alpha(0.5).restart();
			}
			break;
		}

		case 'stop': {
			if (simulation) simulation.stop();
			break;
		}

		case 'drag': {
			if (simulation) {
				const node = simulation.nodes().find((n) => n.id === event.data.nodeId);
				if (node) {
					node.fx = event.data.x;
					node.fy = event.data.y;
					simulation.alpha(0.3).restart();
				}
			}
			break;
		}

		case 'dragend': {
			if (simulation) {
				const node = simulation.nodes().find((n) => n.id === event.data.nodeId);
				if (node) {
					node.fx = null;
					node.fy = null;
				}
			}
			break;
		}
	}
};
