"""Compare exact LP projection fill on the frozen B677 relation stream.

The weighted shortest-path forest is the unit-demand, uncapacitated min-cost
flow solution from each component root. All three orders consume identical
point-checked source rows; only LP pivot admission order changes.
"""

import heapq
import hashlib
import json
import time
from collections import deque
from pathlib import Path

from flint import nmod_mat

from verify_mitm_graph import B677_INPUT, N, ROOT, dickson1024

RECEIPT = ROOT / "evidence" / "mitm-b677-control-01" / "forty-cap-full-cost.json"
OUT = ROOT / "evidence" / "mitm-b677-control-01" / "graph-order-comparison.json"
FB, LP, TARGET, RHS = 509, 168, 677, 678


def modular_inverse(x):
    return pow(x, -1, N)


def project(rows, order):
    pivots = {}
    projected = []
    ops = 0
    began = time.perf_counter_ns()
    for idx in order:
        row = rows[idx].copy()
        for col in range(FB, TARGET):
            coefficient = row.get(col, 0)
            if not coefficient:
                continue
            pivot = pivots.get(col)
            if pivot is None:
                factor = modular_inverse(coefficient)
                pivots[col] = {j: (v * factor) % N for j, v in row.items()}
                break
            for j, v in pivot.items():
                value = (row.get(j, 0) - coefficient * v) % N
                if value:
                    row[j] = value
                else:
                    row.pop(j, None)
                ops += 1
        else:
            assert all(j < FB or j >= TARGET for j in row)
            projected.append(row)
    projected_nnz = sum(len(row) for row in projected)
    projected_max_nnz = max(map(len, projected))
    reduced, projected_rank = nmod_mat(
        [[row.get(j, 0) for j in list(range(FB)) + [TARGET, RHS]]
         for row in projected], N
    ).rref()
    target_only = [int(reduced[i, FB + 1]) for i in range(len(projected))
                   if all(int(reduced[i, j]) == 0 for j in range(FB))
                   and int(reduced[i, FB]) == 1]
    return {
        "lp_rank": len(pivots),
        "projected_rows": len(projected),
        "projected_rank_augmented": projected_rank,
        "target_only_rows": target_only,
        "projected_nnz": projected_nnz,
        "projected_max_nnz": projected_max_nnz,
        "modular_elimination_updates": ops,
        "projection_wall_ns": time.perf_counter_ns() - began,
    }


def forest(adjacency, weights, mode):
    seen = set()
    selected = []
    roots = [LP] + list(range(LP))
    for root in roots:
        if root in seen:
            continue
        seen.add(root)
        if mode == "bfs":
            queue = deque([root])
            while queue:
                u = queue.popleft()
                for v, idx in sorted(adjacency[u], key=lambda edge: edge[1]):
                    if v not in seen:
                        seen.add(v)
                        selected.append(idx)
                        queue.append(v)
        else:
            heap = [(0, root, -1)]
            distance = {root: 0}
            settled = set()
            while heap:
                cost, u, parent = heapq.heappop(heap)
                if u in settled or cost != distance[u]:
                    continue
                settled.add(u)
                seen.add(u)
                if parent >= 0:
                    selected.append(parent)
                for v, idx in adjacency[u]:
                    next_cost = cost + weights[idx]
                    if v not in settled and next_cost < distance.get(v, 1 << 60):
                        distance[v] = next_cost
                        heapq.heappush(heap, (next_cost, v, idx))
    return selected


def mst(edges, weights):
    parent = list(range(LP + 1))

    def find(u):
        while parent[u] != u:
            parent[u] = parent[parent[u]]
            u = parent[u]
        return u

    out = []
    for u, v, idx in sorted(edges, key=lambda edge: (weights[edge[2]], edge[2])):
        a, b = find(u), find(v)
        if a != b:
            parent[a] = b
            out.append(idx)
    return out


def main():
    receipt = json.loads(RECEIPT.read_text())
    fixture_line = next(line for line in B677_INPUT.read_text().splitlines()
                        if line.startswith("B677_INPUT "))
    fixture = json.loads(fixture_line.split(" ", 1)[1])
    points = fixture["public_base"]
    base = [i for i, (x, _) in enumerate(points)
            if dickson1024(x) == fixture["dickson_terminal"]]
    lp = [i for i in range(677) if i not in set(base)]
    assert (len(base), len(lp)) == (FB, LP)
    columns = {i: col for col, i in enumerate(base + lp)}
    graph_nodes = {i: col - FB for i, col in columns.items() if col >= FB}
    rows, edges, weights = [], [], {}
    adjacency = [[] for _ in range(LP + 1)]
    for relation in receipt["relation_rows"]:
        terms = relation["terms"]
        lp_terms = [i for i, _ in terms if i in graph_nodes]
        if len(lp_terms) > 2:
            continue
        query = receipt["queries"][relation["label"]]
        row = {columns[i]: sign % N for i, sign in terms}
        row[TARGET] = (-query["b"]) % N
        row[RHS] = query["a"]
        idx = len(rows)
        rows.append(row)
        if lp_terms:
            u = graph_nodes[lp_terms[0]]
            v = graph_nodes[lp_terms[1]] if len(lp_terms) == 2 else LP
            edges.append((u, v, idx))
            weights[idx] = 4 - len(lp_terms)
            adjacency[u].append((v, idx))
            adjacency[v].append((u, idx))
    assert len(rows) == 669 and len(edges) == 448
    selected = {
        "stream": [],
        "bfs_forest": forest(adjacency, weights, "bfs"),
        "weighted_mst": mst(edges, weights),
        "unit_min_cost_paths": forest(adjacency, weights, "dijkstra"),
    }
    result = {}
    for name, leading in selected.items():
        leading_set = set(leading)
        order = leading + [i for i in range(len(rows)) if i not in leading_set]
        metrics = project(rows, order)
        assert (metrics["lp_rank"], metrics["projected_rows"],
                metrics["projected_rank_augmented"]) == (162, 507, 506), (name, metrics)
        assert metrics["target_only_rows"] == [987654321], (name, metrics)
        result[name] = {"forest_edges": len(leading), **metrics}
    output = {
        "curve": receipt["curve"],
        "public_target": receipt["public_target"],
        "source_rows": len(rows),
        "source_receipt_sha256": hashlib.sha256(RECEIPT.read_bytes()).hexdigest(),
        "graph_edges": len(edges),
        "graph_nodes_including_root": LP + 1,
        "weight": "factor-base leaves per partial row",
        "methods": result,
        "selected_by_minimum_projected_nnz": min(
            result, key=lambda name: result[name]["projected_nnz"]),
    }
    OUT.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output, indent=2))


if __name__ == "__main__":
    main()
