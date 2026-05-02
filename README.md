# CrateMapper

An interactive dependency graph explorer for the Rust crate ecosystem. CrateMapper downloads the full [crates.io](https://crates.io) database dump, builds an in-memory dependency graph (260k+ crates, 1.9M+ edges), and exposes it through a REST API and a D3-powered force-directed visualization frontend.

## Architecture

```
crate-mapper/
├── crates/
│   ├── cm-core      # Shared graph types: CrateNode, DepEdge, CrateGraph, NameIndex
│   ├── cm-ingest    # Download, parse, and snapshot the crates.io DB dump
│   ├── cm-graph     # Graph algorithms (subgraph, chain, impact, clusters)
│   └── cm-server    # Axum HTTP server — wires everything together
└── frontend/        # SvelteKit + D3 visualization UI
```

**Dependency chain:** `cm-core` ← `cm-graph`, `cm-ingest` ← `cm-server`

## Getting Started

### Prerequisites

- Rust (stable, 2021 edition)
- Node.js + npm (for the frontend)
- ~2 GB free disk space (crates.io DB dump is ~1 GB compressed)
- ~2 GB RAM (graph is held in memory)

### Run the backend

```bash
cargo run --release -p cm-server
```

On first start the server will:

1. Download the crates.io DB dump (`~1 GB`) from `https://static.crates.io/db-dump.tar.gz`
2. Parse all CSVs (crates, versions, dependencies, categories, keywords)
3. Build the petgraph dependency graph
4. Save a `graph.bincode` snapshot for fast subsequent startups

Subsequent starts load the snapshot in seconds.

### Run the frontend

```bash
cd frontend
npm install
npm run dev
```

Open [http://localhost:5173](http://localhost:5173).

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `CRATE_MAPPER_PORT` | `3001` | Backend listen port |
| `CRATE_MAPPER_DATA_DIR` | `./data` | Directory for the DB dump and graph snapshot |
| `CRATE_MAPPER_MOCK` | _(unset)_ | Set to any value to skip the download and use built-in mock data |

## API Reference

All endpoints accept `GET` requests and return JSON.

### `GET /api/health`

Returns graph load status and size.

```json
{ "status": "ok", "nodes": 259999, "edges": 1908009 }
```

### `GET /api/search?q=<query>&limit=<n>`

Fuzzy-searches crates by name and description. Results are ranked by match quality (prefix > substring > keyword) then by download count.

| Param | Type | Default | Description |
|---|---|---|---|
| `q` | string | required | Search query |
| `limit` | integer | `20` | Max results (capped at 100) |

### `GET /api/crate/:name`

Returns full metadata for a crate plus its direct dependencies (fetched live from the crates.io API, with graph fallback) and up to 50 direct dependents.

Returns `404` if the crate is not found.

### `GET /api/crate/:name/subgraph?depth=<n>`

Returns a BFS subgraph centred on `:name` up to `depth` hops away.

| Param | Type | Default | Description |
|---|---|---|---|
| `depth` | integer | `2` | Traversal depth (1–4 recommended) |

Returns `404` if the crate is not found.

### `GET /api/crate/:name/impact`

Returns the reverse-dependency impact radius — how many crates depend on `:name` and how far that influence extends through the graph.

Returns `404` if the crate is not found.

### `GET /api/chain/:source/:target`

Finds the shortest dependency path between two crates.

```
GET /api/chain/axum/serde
```

Returns the ordered list of crates in the path, or an empty path if none exists.

### `GET /api/clusters`

Computes and returns tightly-coupled crate clusters across the full ecosystem.

## Data Pipeline & Resilience

The ingest pipeline has several safety guarantees:

- **Atomic downloads** — the DB dump is written to a `.tmp` file and renamed into place only after a complete, successful download. An interrupted download never leaves a partial file that would be mistaken for a valid one.
- **Pre-flight validation** — on startup the existing dump file is checked: minimum size (300 MB), valid gzip header, and a readable first tar entry. If any check fails the file is discarded and re-downloaded.
- **Parse-error retry** — if the dump passes pre-flight but fails mid-parse, the file is evicted and a fresh download is attempted automatically.
- **Atomic snapshot writes** — `graph.bincode` is written to `graph.bincode.tmp` and renamed atomically, so a crash during save never corrupts the snapshot.
- **Corrupt snapshot recovery** — if `graph.bincode` fails to load (bad bincode, truncated file, empty graph), it is deleted and the graph is rebuilt from the dump automatically.
- **24-hour dump cache** — a fresh, valid dump is reused for 24 hours before the next download is triggered.

## Frontend

Built with SvelteKit 5, TailwindCSS 4, and D3 7.

| Script | Command |
|---|---|
| Dev server | `npm run dev` |
| Production build | `npm run build` |
| Type check | `npm run check` |

The UI renders a force-directed graph of crate dependencies. Key features:

- **Search** — find any crate and load its subgraph
- **Depth control** — adjust BFS depth (1–4) to expand or narrow the view
- **Path finder** — highlight the shortest dependency chain between two crates
- **Crate panel** — click any node to see metadata, dependencies, and download stats
- **Cluster view** — visualise tightly-coupled groups in the ecosystem
