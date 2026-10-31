"""Independent reciprocal-line, group-equation, and rank replay."""

import hashlib
import json
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import GX, GY, add, mul, neg, on_curve

HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


source = load("type_i_known_scalar_query_cross_fan.json")
transfer = load("type_i_known_scalar_query_transfer.json")
replay = load("type_i_known_scalar_query_transfer_replay.json")
reduced = load("type_i_triple_star_character_reduced96.json")
base = load("type_i_quotient_base_probe.json")
plane = load("type_i_plane_quartic.json")
screen = load("type_i_triple_star_screen.json")
guide = load("type_i_line_guided_collector.json")
new = load("type_i_line_full_new_quotient_points.json")

assert source["status"] == "verified_known_scalar_query_cross_fan_screen"
assert source["candidate_id"] is None
assert source["source_sha256"] == sha(HERE / "line_known_scalar_query_cross_fan.py")
assert source["runtime_sha256"] == sha(HERE / "type_i_known_scalar_query_cross_fan_runtime_info.json")
for name in source["input_sha256"]:
    assert source["input_sha256"][name] == sha(HERE / name)
p, r = int(source["field_prime"]), int(source["subgroup_order"])
h = int(base["candidate_cofactor"])
G = (GX, GY)
assert on_curve(G, p) and mul(r, G, p) is None
assert p == int(plane["p"]) == int(guide["field_prime"])
assert r == int(guide["subgroup_order"])
assert int(source["cofactor_mod_r"]) == h % r
assert int(replay["known_scalar_difference_multiplier_mod_r"]) == (2 * h) % r
assert source["queries"] == len(transfer["rows"]) == 27
endpoints = list(range(81))
assert source["endpoints"] == 81

terms = [(*map(int, label.split(",")[:2]), int(value))
         for label, value in plane["quartic_coefficients"].items()]


def quartic(point):
    x, y = point
    return sum(c * pow(x, a, p) * pow(y, b, p)
               for a, b, c in terms) % p


geometry = [tuple(map(int, P)) for P in source["endpoint_points"]]
selected_q = [tuple(map(int, Q)) for Q in source["endpoint_quotients"]]
assert len(geometry) == len(selected_q) == 81
S = tuple(map(int, screen["line_section_constant"]))
C = tuple(map(int, replay["shared_constant"]))
fixed = {tuple(map(int, row["quotient"])) for row in reduced["rows"]}
assert len(fixed) == 96

queries = []
for item in transfer["rows"]:
    a = int(item["scalar"])
    Q = tuple(map(int, item["quotient"]))
    pivots = [tuple(map(int, P)) for P in item["points"]]
    pivot_q = [tuple(map(int, P)) for P in item["projected_points"]]
    assert len(pivots) == len(pivot_q) == 3
    assert Q == mul((-2 * a) % r, G, p)
    sigma = None
    for P, V in zip(pivots, pivot_q):
        assert quartic(P) == 0 and on_curve(V, p)
        sigma = add(sigma, V, p)
    assert sigma == tuple(map(int, item["projected_sum"]))
    assert add(sigma, mul((2 * h) % r, Q, p), p) == C
    queries.append((a, Q, sigma, pivots, pivot_q))
assert geometry == [P for query in queries for P in query[3]]
assert selected_q == [P for query in queries for P in query[4]]
assert len(set(geometry)) == 81

edge = {}
occurrences = defaultdict(list)
for item in source["edges"]:
    i, root, u = item["query_index"], item["root_index"], item["selected_endpoint"]
    key = (i, root, u)
    assert key not in edge and 0 <= i < 27 and 0 <= root < 3 and u in endpoints
    P, U = queries[i][3][root], geometry[u]
    assert P != U and P[0] != U[0] and quartic(U) == 0
    slope = (U[1] - P[1]) * pow((U[0] - P[0]) % p, -1, p) % p
    residuals = [tuple(map(int, R)) for R in item["residual_points"]]
    columns = [tuple(map(int, R)) for R in item["projected_residuals"]]
    assert len(residuals) == len(set(residuals)) == 2
    assert len(columns) == len(set(columns)) == 2
    total = add(queries[i][4][root], selected_q[u], p)
    for side, (R, V) in enumerate(zip(residuals, columns)):
        assert R not in (P, U)
        assert (R[1] - P[1] - slope * (R[0] - P[0])) % p == 0
        assert quartic(R) == 0 and on_curve(V, p)
        total = add(total, V, p)
        occurrences[V].append([i, root, u, side])
    assert total == S
    edge[key] = columns

assert len(edge) == source["split_edges"]
assert source["candidate_lines"] == 27 * 3 * 81
assert sum(source["line_status_counts"].values()) == source["candidate_lines"]
assert source["line_status_counts"]["split"] == len(edge)
assert source["line_status_counts"]["coincident"] == 81
assert len(occurrences) == source["distinct_projected_columns"]
assert sum(map(len, occurrences.values())) == source["projected_column_appearances"]
assert [{"quotient": list(V), "occurrences": where}
        for V, where in sorted(occurrences.items()) if len(where) > 1] == source["projected_repeats"]
assert sorted([list(V) for V in set(occurrences) & fixed]) == source["fixed96_column_hits"]

# A line through two query roots is symmetric under swapping pivot and
# endpoint. The two projected residuals therefore recur at its reverse edge.
reciprocal_edges = 0
for (i, root, u), values in edge.items():
    reverse = (u // 3, u % 3, 3 * i + root)
    assert reverse in edge and set(values) == set(edge[reverse])
    reciprocal_edges += 1
assert reciprocal_edges == len(edge)

bundles = {}
for i, (_, _, sigma, _, _) in enumerate(queries):
    for u in endpoints:
        if not all((i, root, u) in edge for root in range(3)):
            continue
        columns = [V for root in range(3) for V in edge[i, root, u]]
        total = None
        for V in columns:
            total = add(total, V, p)
        expected = mul(3, S, p)
        expected = add(expected, neg(sigma, p), p)
        expected = add(expected, neg(mul(3, selected_q[u], p), p), p)
        assert total == expected
        bundles[i, u] = columns
assert len(bundles) == source["complete_three_cut_bundles"]

columns = sorted(occurrences)
column_index = {V: index for index, V in enumerate(columns)}
rows = []
recorded = []
for u in endpoints:
    present = [i for i in range(27) if (i, u) in bundles]
    for i, j in combinations(present, 2):
        left, right = bundles[i, u], bundles[j, u]
        ai, Qi = queries[i][:2]
        aj, Qj = queries[j][:2]
        scalar = (-4 * h * (ai - aj)) % r
        total = None
        coeffs = Counter()
        for V in left:
            total = add(total, V, p)
            coeffs[column_index[V]] += 1
        for V in right:
            total = add(total, neg(V, p), p)
            coeffs[column_index[V]] -= 1
        assert total == mul(scalar, G, p)
        assert total == mul((2 * h) % r, add(Qi, neg(Qj, p), p), p)
        row = {index: coefficient % r for index, coefficient in coeffs.items()
               if coefficient % r}
        rows.append(row)
        recorded.append({"endpoint": u, "query_indices": [i, j],
                         "known_scalar_mod_r": str(scalar),
                         "nonzero_columns": len(row)})
assert recorded == source["known_scalar_rows"]

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
        inv = pow(row[lead], -1, r)
        basis[lead] = {index: value * inv % r for index, value in row.items()}
assert len(basis) == source["known_scalar_row_rank"]

active = {index for row in rows for index in row}
row_column_endpoints = defaultdict(set)
for row, record in zip(rows, recorded):
    for index in row:
        row_column_endpoints[columns[index]].add(record["endpoint"])
active_cross_endpoint = {V for V, used_at in row_column_endpoints.items()
                         if len(used_at) > 1}
active_repeated = {columns[index] for index in active
                   if len(occurrences[columns[index]]) > 1}
out = {"status": "independent_known_scalar_query_cross_fan_replay_pass",
       "split_edges": len(edge), "complete_three_cut_bundles": len(bundles),
       "known_scalar_rows": len(rows), "exact_rank": len(basis),
       "distinct_projected_columns": len(columns),
       "active_relation_columns": len(active),
       "active_repeated_columns": len(active_repeated),
       "active_cross_endpoint_columns": len(active_cross_endpoint),
       "reciprocal_edges": reciprocal_edges,
       "projected_repeats": len(source["projected_repeats"]),
       "fixed96_column_hits": len(source["fixed96_column_hits"]),
       "source_sha256": sha(__file__), "screen_sha256": sha(HERE / "type_i_known_scalar_query_cross_fan.json")}
path = HERE / "type_i_known_scalar_query_cross_fan_replay.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print("QUERY_CROSS_FAN_REPLAY_PASS", json.dumps({
    "split_edges": len(edge), "bundles": len(bundles),
    "rows": len(rows), "rank": len(basis),
    "active_columns": len(active),
    "cross_endpoint_columns": len(active_cross_endpoint),
    "repeats": out["projected_repeats"], "fixed96_hits": out["fixed96_column_hits"]
}, sort_keys=True))
