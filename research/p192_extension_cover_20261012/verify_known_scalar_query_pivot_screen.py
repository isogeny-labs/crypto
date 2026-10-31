"""Independent quartic, subgroup, and modular-rank replay of query pivots."""

import hashlib
import json
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from verify_adaptive_private_columns import add, on_curve

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
source_path = HERE / "type_i_known_scalar_query_pivot_screen.json"
roots_path = HERE / "type_i_known_scalar_query_cubic_roots.json"
selection_path = HERE / "type_i_triple_star_character_selection.json"
reduced_path = HERE / "type_i_triple_star_character_reduced96.json"
guide_path = HERE / "type_i_line_guided_collector.json"
new_path = HERE / "type_i_line_full_new_quotient_points.json"
plane_path = HERE / "type_i_plane_quartic.json"
screen_path = HERE / "type_i_triple_star_screen.json"
source, roots, selection, reduced, guide, new, plane, screen = (
    json.loads(path.read_text()) for path in
    (source_path, roots_path, selection_path, reduced_path,
     guide_path, new_path, plane_path, screen_path))
assert source["source_sha256"] == sha(HERE / "line_known_scalar_query_pivot_screen.py")
assert source["runtime_sha256"] == sha(HERE / "type_i_known_scalar_query_pivot_runtime_info.json")
for path in (roots_path, selection_path, reduced_path,
             HERE / "line_residual_pivot_three.py"):
    assert source["input_sha256"][path.name] == sha(path)
assert source["status"] == "verified_known_scalar_query_pivot_line_screen"
p, r = int(source["field_prime"]), int(source["subgroup_order"])
assert p == int(plane["p"]) == int(guide["field_prime"])
assert r == int(guide["subgroup_order"])
endpoints = selection["optimized"]["endpoints"]
assert source["selected_endpoints"] == endpoints
assert source["candidate_lines"] == 256

quartic_terms = [(*map(int, label.split(",")[:2]), int(value))
                 for label, value in plane["quartic_coefficients"].items()]


def quartic(P):
    x, y = P
    return sum(value * pow(x, a, p) * pow(y, b, p)
               for a, b, value in quartic_terms) % p


old_geometry = [tuple(map(int, line.split()[1:])) for line in
                (HERE / "type_i_line_base_points.tsv").read_text().splitlines()]
new_geometry = [tuple(map(int, line.split())) for line in
                (HERE / "type_i_line_new_points.tsv").read_text().splitlines()]
geometry = old_geometry + new_geometry
selected_q = [tuple(map(int, item)) for item in guide["base_points_fp"] + new]
assert len(geometry) == len(selected_q) == 9925
fixed = {tuple(map(int, row["quotient"])) for row in reduced["rows"]}
assert len(fixed) == 96
S = tuple(map(int, screen["line_section_constant"]))

root_lookup = {(row["law"], row["query_index"]): row for row in roots["rows"]}
pivots = source["selected_pivots"]
assert len(pivots) == len({row["scalar"] for row in pivots}) == 16
for pivot in pivots:
    row = root_lookup[pivot["law"], pivot["query_index"]]
    assert row["scalar"] == pivot["scalar"]
    assert list(map(int, row["positive"]["rational_points"][0])) == pivot["point"]
    assert pivot["positive_cubic_rational_points"] == row["positive"]["rational_point_count"]
    assert quartic(pivot["point"]) == 0
    assert on_curve(tuple(pivot["quotient_point"]), p)

edge = {}
occurrences = defaultdict(list)
for item in source["edges"]:
    pivot_index, endpoint = item["pivot_index"], item["selected_endpoint"]
    key = (pivot_index, endpoint)
    assert key not in edge and 0 <= pivot_index < 16 and endpoint in endpoints
    P, U = pivots[pivot_index]["point"], geometry[endpoint]
    assert P != U and quartic(P) == quartic(U) == 0
    assert P[0] != U[0]
    slope = (U[1] - P[1]) * pow((U[0] - P[0]) % p, -1, p) % p
    residuals = [tuple(map(int, point)) for point in item["residual_points"]]
    columns = [tuple(map(int, point)) for point in item["projected_residuals"]]
    assert len(residuals) == len(set(residuals)) == 2
    assert len(columns) == len(set(columns)) == 2
    for side, (point, column) in enumerate(zip(residuals, columns)):
        assert point not in (tuple(P), tuple(U))
        assert (point[1] - P[1] - slope * (point[0] - P[0])) % p == 0
        assert quartic(point) == 0 and on_curve(column, p)
        occurrences[column].append([pivot_index, endpoint, side])
    total = None
    for Q in (tuple(pivots[pivot_index]["quotient_point"]),
              selected_q[endpoint], *columns):
        total = add(total, Q, p)
    assert total == S
    edge[key] = columns
assert len(edge) == source["split_edges"]
assert len(occurrences) == source["distinct_projected_columns"]
assert sum(map(len, occurrences.values())) == source["projected_column_appearances"]
assert sorted([list(key) for key in set(occurrences) & fixed]) == source["fixed96_column_hits"]
repeats = [{"quotient": list(key), "occurrences": where}
           for key, where in sorted(occurrences.items()) if len(where) > 1]
assert repeats == source["projected_repeats"]
assert sum(source["line_status_counts"].values()) == 256
assert source["line_status_counts"]["split"] == len(edge)

columns = sorted(occurrences)
column_index = {key: i for i, key in enumerate(columns)}
rows = []
for a, b in combinations(range(16), 2):
    common = [u for u in endpoints if (a, u) in edge and (b, u) in edge]
    for u, v in combinations(common, 2):
        coefficients = Counter()
        positive = negative = None
        for pivot_index, endpoint, sign in ((a, u, 1), (b, v, 1),
                                            (a, v, -1), (b, u, -1)):
            for Q in edge[pivot_index, endpoint]:
                coefficients[column_index[Q]] += sign
                if sign == 1:
                    positive = add(positive, Q, p)
                else:
                    negative = add(negative, Q, p)
        assert positive == negative
        rows.append([coefficients[i] % r for i in range(len(columns))])
assert len(rows) == source["rectangles"]

parent = list(range(32))


def root(index):
    while parent[index] != index:
        parent[index] = parent[parent[index]]
        index = parent[index]
    return index


for pivot_index, endpoint in edge:
    a, b = root(pivot_index), root(16 + endpoints.index(endpoint))
    parent[a] = b
components = len({root(index) for index in range(32)})
cycle_dimension = len(edge) - 32 + components
assert components == 1

basis = {}
for raw in rows:
    row = raw[:]
    for lead, vector in sorted(basis.items()):
        if row[lead]:
            factor = row[lead]
            row = [(value - factor * other) % r
                   for value, other in zip(row, vector)]
    lead = next((i for i, value in enumerate(row) if value), None)
    if lead is not None:
        inverse = pow(row[lead], -1, r)
        basis[lead] = [value * inverse % r for value in row]
assert len(basis) == source["rectangle_rank"]
assert len(basis) == cycle_dimension == 115

out = {
    "status": "independent_known_scalar_query_pivot_replay_pass",
    "candidate_lines": 256, "split_edges": len(edge),
    "distinct_projected_columns": len(columns),
    "projected_repeats": len(repeats),
    "fixed96_column_hits": len(source["fixed96_column_hits"]),
    "rectangles": len(rows), "exact_rank": len(basis),
    "incidence_components": components,
    "incidence_cycle_dimension": cycle_dimension,
    "source_sha256": sha(__file__),
    "screen_sha256": sha(source_path),
    "quartic_sha256": sha(plane_path),
}
path = HERE / "type_i_known_scalar_query_pivot_replay.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
print("QUERY_PIVOT_REPLAY_PASS", json.dumps({
    "split_edges": len(edge), "fixed96_hits": out["fixed96_column_hits"],
    "rectangles": len(rows), "rank": len(basis),
}, sort_keys=True), flush=True)
