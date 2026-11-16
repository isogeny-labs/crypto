"""Certify incidence and rank-capacity bounds for reciprocal query bundles.

The recorded rows are differences between two complete three-cut bundles
at one endpoint. Each bundle has six projected residual columns. The
endpointwise row space has dimension at most bundles minus endpoints.
"""

import hashlib
import json
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


source_path = HERE / "type_i_known_scalar_query_cross_fan.json"
replay_path = HERE / "type_i_known_scalar_query_cross_fan_replay.json"
source, replay = load(source_path.name), load(replay_path.name)
assert source["status"] == "verified_known_scalar_query_cross_fan_screen"
assert replay["status"] == "independent_known_scalar_query_cross_fan_replay_pass"
assert replay["screen_sha256"] == sha(source_path)
assert source["source_sha256"] == sha(HERE / "line_known_scalar_query_cross_fan.py")
assert replay["source_sha256"] == sha(HERE / "verify_known_scalar_query_cross_fan.py")
r = int(source["subgroup_order"])

edges = {}
for item in source["edges"]:
    key = (item["query_index"], item["root_index"], item["selected_endpoint"])
    assert key not in edges
    values = tuple(map(tuple, item["projected_residuals"]))
    assert len(values) == len(set(values)) == 2
    edges[key] = values
assert len(edges) == source["split_edges"] == 3194
for (i, root, u), values in edges.items():
    reverse = (u // 3, u % 3, 3 * i + root)
    assert reverse in edges and set(values) == set(edges[reverse])

bundles = {}
by_endpoint = defaultdict(list)
for u in range(81):
    for i in range(27):
        if not all((i, root, u) in edges for root in range(3)):
            continue
        values = tuple(value for root in range(3)
                       for value in edges[i, root, u])
        assert len(values) == len(set(values)) == 6
        bundles[i, u] = values
        by_endpoint[u].append(i)
assert len(bundles) == source["complete_three_cut_bundles"] == 258

active_endpoints = sorted(u for u, indices in by_endpoint.items()
                          if len(indices) >= 2)
active = {(i, u): bundles[i, u] for u in active_endpoints
          for i in by_endpoint[u]}
counts = Counter(value for values in active.values() for value in values)
degrees = Counter(counts.values())
assert set(degrees) <= {1, 2}
assert sum(degree * count for degree, count in degrees.items()) == 6 * len(active)
assert len(active) == 250 and len(active_endpoints) == 69
assert len(counts) == replay["active_relation_columns"] == 1316
assert degrees[2] == replay["active_cross_endpoint_columns"] == 184
assert degrees[1] == 1132
expected_row_keys = {
    (u, i, j) for u in active_endpoints
    for i, j in combinations(by_endpoint[u], 2)
}
recorded_row_keys = {
    (row["endpoint"], *row["query_indices"])
    for row in source["known_scalar_rows"]
}
assert len(source["known_scalar_rows"]) == len(recorded_row_keys) == 370
assert recorded_row_keys == expected_row_keys
assert all(row["nonzero_columns"] == 12
           for row in source["known_scalar_rows"])


def rows_for(endpoints):
    columns = sorted({value for u in endpoints if len(by_endpoint[u]) >= 2
                      for i in by_endpoint[u] for value in bundles[i, u]})
    index = {value: position for position, value in enumerate(columns)}
    rows = []
    for u in sorted(endpoints):
        for i, j in combinations(by_endpoint[u], 2):
            coefficients = Counter()
            for value in bundles[i, u]:
                coefficients[index[value]] += 1
            for value in bundles[j, u]:
                coefficients[index[value]] -= 1
            rows.append({position: coefficient % r
                         for position, coefficient in coefficients.items()
                         if coefficient % r})
    return columns, rows


def rank_sparse_reverse(rows):
    basis = {}
    for raw in rows:
        row = dict(raw)
        for lead, vector in sorted(basis.items(), reverse=True):
            factor = row.pop(lead, 0)
            if not factor:
                continue
            for index, value in vector.items():
                if index == lead:
                    continue
                updated = (row.get(index, 0) - factor * value) % r
                if updated:
                    row[index] = updated
                else:
                    row.pop(index, None)
        if row:
            lead = max(row)
            inverse = pow(row[lead], -1, r)
            basis[lead] = {index: value * inverse % r
                           for index, value in row.items()}
    return len(basis)


order = sorted(range(81), key=lambda u: (-(len(by_endpoint[u]) - 1), u))
panels = []
for width in (1, 4, 8, 16, 32, 69, 81):
    endpoints = order[:width]
    columns, rows = rows_for(endpoints)
    bundle_count = sum(len(by_endpoint[u]) for u in endpoints
                       if len(by_endpoint[u]) >= 2)
    endpoint_count = sum(len(by_endpoint[u]) >= 2 for u in endpoints)
    rank = rank_sparse_reverse(rows)
    assert rank == bundle_count - endpoint_count
    assert 3 * bundle_count <= len(columns)
    panels.append({"endpoint_budget": width, "endpoints": endpoints,
                   "complete_bundles_in_rows": bundle_count,
                   "endpoints_with_rows": endpoint_count,
                   "known_scalar_rows": len(rows), "exact_rank": rank,
                   "active_columns": len(columns),
                   "column_to_rank_ratio_decimal": f"{len(columns) / rank:.6f}" if rank else None})

assert panels[-1]["exact_rank"] == replay["exact_rank"] == 181
assert panels[-1]["exact_rank"] == source["known_scalar_row_rank"]
assert panels[-1]["active_columns"] == replay["active_relation_columns"]
assert panels[3]["exact_rank"] == 67 and panels[3]["active_columns"] == 482

N, R, B = len(active), replay["exact_rank"], len(counts)
assert N == R + len(active_endpoints)
assert 6 * N == degrees[1] + 2 * degrees[2]
assert 6 * N <= 2 * B
required_reuse = (6 * N + R - 1) // R
assert required_reuse == 9
assert (required_reuse - 1) * R < 6 * N

out = {
    "status": "verified_reciprocal_query_incidence_capacity",
    "candidate_id": None,
    "source_sha256": sha(Path(__file__)),
    "cross_fan_sha256": sha(source_path),
    "cross_fan_replay_sha256": sha(replay_path),
    "split_directed_edges": len(edges),
    "all_complete_bundles": len(bundles),
    "active_complete_bundles": N,
    "active_endpoints": len(active_endpoints),
    "active_columns": B,
    "column_bundle_multiplicity_histogram": {str(k): v for k, v in sorted(degrees.items())},
    "incidences": 6 * N,
    "max_column_bundle_multiplicity": max(degrees),
    "known_scalar_rows": len(source["known_scalar_rows"]),
    "exact_rank": R,
    "endpointwise_rank_ceiling": N - len(active_endpoints),
    "pairwise_reuse_column_lower_bound": 3 * N,
    "column_to_rank_ratio_decimal": f"{B / R:.6f}",
    "necessary_max_multiplicity_for_closure_at_frozen_rank": required_reuse,
    "max_rank_endpoint_panels": panels,
    "selection_rule": "Sort endpoints by descending complete-bundle count minus one, breaking ties by ascending root index; take the first k.",
    "boundary": "This certificate bounds the frozen family of rows formed by differences of complete three-cut bundles. New row types or geometries can change the incidence and rank premises.",
}
path = HERE / "type_i_known_scalar_query_incidence_capacity.json"
path.write_text(json.dumps(out, sort_keys=True, indent=2) + "\n")
print("QUERY_INCIDENCE_CAPACITY_PASS", json.dumps({
    "bundles": N, "endpoints": len(active_endpoints), "columns": B,
    "rank": R, "max_multiplicity": max(degrees),
    "necessary_closure_multiplicity": required_reuse,
    "best_rank_16_endpoints": panels[3]["exact_rank"],
    "columns_16_endpoints": panels[3]["active_columns"],
}, sort_keys=True))
