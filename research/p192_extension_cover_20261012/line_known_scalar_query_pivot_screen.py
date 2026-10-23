"""Screen query-root line pivots for projected reuse and cycle rank.

The frozen pilot takes the first 16 distinct scalar queries with a
rational point in the positive cubic and the 16 character-selected
endpoints. Every split line is directly projected on both residual sides.
"""

import hashlib
import json
import time
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()


def prelude(name, marker):
    path = HERE / name
    source = path.read_text()
    assert source.count(marker) == 1
    namespace = {"__file__": str(path), "__name__": name[:-3] + "_prelude"}
    exec(compile(source.split(marker)[0], str(path), "exec"), namespace)
    return namespace


def main():
    roots_path = HERE / "type_i_known_scalar_query_cubic_roots.json"
    selection_path = HERE / "type_i_triple_star_character_selection.json"
    reduced_path = HERE / "type_i_triple_star_character_reduced96.json"
    roots, selection, reduced = (json.loads(path.read_text()) for path in
                                 (roots_path, selection_path, reduced_path))
    assert roots["verified_pairs"] == 128
    endpoints = selection["optimized"]["endpoints"]
    assert len(endpoints) == 16 and reduced["selected_endpoints"] == endpoints
    selected_pivots = []
    used_scalars = set()
    for row in roots["rows"]:
        if row["scalar"] in used_scalars:
            continue
        rational = row["positive"]["rational_points"]
        if rational:
            selected_pivots.append({
                "law": row["law"], "query_index": row["query_index"],
                "scalar": row["scalar"],
                "point": list(map(int, rational[0])),
                "positive_cubic_rational_points": len(rational),
            })
            used_scalars.add(row["scalar"])
        if len(selected_pivots) == 16:
            break
    assert len(selected_pivots) == len(used_scalars) == 16

    ns = prelude("line_residual_pivot_three.py", "start_ns = time.monotonic_ns()")
    p, r = int(ns["p"]), int(ns["dt"].r)
    assert str(p) == roots["field_prime"] and str(r) == roots["subgroup_order"]
    assert len(ns["points"]) == len(ns["selected_q"]) == 9925
    inverse, E6, E0 = ns["inverse"], ns["E6"], ns["E0"]
    trace3, scalar3, anchor_trace3 = (ns[name] for name in
                                      ("trace3", "scalar3", "anchor_trace3"))
    S, selected_q = ns["S"], ns["selected_q"]
    taylor_at, evaluate = ns["taylor_at"], ns["evaluate"]
    F = ns["dt"].gm.F
    fixed_columns = {tuple(map(int, row["quotient"])) for row in reduced["rows"]}
    assert len(fixed_columns) == 96

    def projected(point):
        cover, ys = inverse.invert_affine(F(point[0]), F(point[1]))
        image = inverse.quotient_image(E6, cover, ys, 0)
        Q = scalar3 * (trace3(image) - anchor_trace3)
        assert Q in E0 and r * Q == E0(0)
        key = ns["projector"].key(Q)
        assert key is not None
        return Q, tuple(map(int, key))

    counts = Counter()
    edges = {}
    occurrences = defaultdict(list)
    start = time.monotonic_ns()
    for pivot_index, pivot_info in enumerate(selected_pivots):
        pivot = tuple(pivot_info["point"])
        pivot_q, pivot_key = projected(pivot)
        pivot_info["quotient_point"] = list(pivot_key)
        taylor = taylor_at(*pivot)
        for endpoint in endpoints:
            target = ns["points"][endpoint]
            d = (target[0] - pivot[0]) % p
            if not d:
                counts["vertical"] += 1
                continue
            slope = (target[1] - pivot[1]) * pow(d, -1, p) % p
            c1, c2, c3, A = (evaluate(taylor[k], slope)
                              for k in range(1, 5))
            if not A:
                counts["degree_below_four"] += 1
                continue
            B = (c3 + d * A) % p
            C = (c2 + d * B) % p
            if not C or (c1 + d * C) % p:
                counts["tangent_or_invalid"] += 1
                continue
            discriminant = (B * B - 4 * A * C) % p
            if not discriminant:
                counts["double_residual"] += 1
                continue
            if pow(discriminant, (p - 1) // 2, p) != 1:
                counts["nonsplit"] += 1
                continue
            sq = pow(discriminant, (p + 1) // 4, p)
            den = pow(2 * A, -1, p)
            z_values = [(-B + sq) * den % p, (-B - sq) * den % p]
            residuals = [((pivot[0] + z) % p,
                          (pivot[1] + slope * z) % p) for z in z_values]
            assert len(set(residuals)) == 2
            projected_residuals = [projected(point) for point in residuals]
            assert (pivot_q + selected_q[endpoint] +
                    projected_residuals[0][0] + projected_residuals[1][0] == S)
            keys = [item[1] for item in projected_residuals]
            assert keys[0] != keys[1]
            edge_key = (pivot_index, endpoint)
            edges[edge_key] = {
                "pivot_index": pivot_index, "selected_endpoint": endpoint,
                "residual_points": [list(point) for point in residuals],
                "projected_residuals": [list(key) for key in keys],
            }
            for side, key in enumerate(keys):
                occurrences[key].append([pivot_index, endpoint, side])
            counts["split"] += 1
        print("QUERY_PIVOT_PROGRESS", pivot_index + 1, 16,
              counts["split"], flush=True)
    assert sum(counts.values()) == 16 * 16

    columns = sorted(occurrences)
    column_index = {key: index for index, key in enumerate(columns)}
    rows = []
    rectangle_count = 0
    for a, b in combinations(range(16), 2):
        common = [u for u in endpoints if (a, u) in edges and (b, u) in edges]
        for u, v in combinations(common, 2):
            row = Counter()
            for pivot_index, endpoint, sign in ((a, u, 1), (b, v, 1),
                                                (a, v, -1), (b, u, -1)):
                for key in map(tuple, edges[pivot_index, endpoint]["projected_residuals"]):
                    row[column_index[key]] += sign
            rows.append({index: coefficient % r for index, coefficient in row.items()
                         if coefficient % r})
            rectangle_count += 1

    pivots = {}
    for sparse in rows:
        row = dict(sparse)
        for lead, basis in sorted(pivots.items()):
            if lead in row:
                factor = row.pop(lead)
                for index, value in basis.items():
                    if index == lead:
                        continue
                    updated = (row.get(index, 0) - factor * value) % r
                    if updated:
                        row[index] = updated
                    else:
                        row.pop(index, None)
        if row:
            lead = min(row)
            inverse = pow(row[lead], -1, r)
            pivots[lead] = {index: value * inverse % r
                            for index, value in row.items()}

    repeats = [{"quotient": list(key), "occurrences": where}
               for key, where in sorted(occurrences.items()) if len(where) > 1]
    fixed_hits = sorted([list(key) for key in set(columns) & fixed_columns])
    out = {
        "status": "verified_known_scalar_query_pivot_line_screen",
        "candidate_id": None,
        "field_prime": str(p), "subgroup_order": str(r),
        "selection_policy": "first 16 distinct scalar queries with a positive cubic rational point; first rational root per query",
        "selected_pivots": selected_pivots,
        "selected_endpoints": endpoints,
        "candidate_lines": 256, "line_status_counts": dict(sorted(counts.items())),
        "split_edges": len(edges), "projected_column_appearances": 2 * len(edges),
        "distinct_projected_columns": len(columns),
        "projected_repeats": repeats,
        "fixed96_column_hits": fixed_hits,
        "rectangles": rectangle_count, "rectangle_rank": len(pivots),
        "edges": [edges[key] for key in sorted(edges)],
        "elapsed_ns_unisolated": time.monotonic_ns() - start,
        "source_sha256": sha(__file__),
        "input_sha256": {path.name: sha(path) for path in
                         (roots_path, selection_path, reduced_path,
                          HERE / "line_residual_pivot_three.py")},
        "runtime_sha256": sha(HERE / "type_i_known_scalar_query_pivot_runtime_info.json"),
        "boundary": "Exact query-root pivot line and projected-column pilot. Rectangle rank is a geometry row-space measurement; using a known-scalar query in a relation requires a separately verified divisor-to-quotient transfer and target equation.",
    }
    path = HERE / "type_i_known_scalar_query_pivot_screen.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("QUERY_PIVOT_SCREEN_PASS", json.dumps({
        "split_edges": len(edges), "distinct_projected_columns": len(columns),
        "projected_repeats": len(repeats), "fixed96_column_hits": len(fixed_hits),
        "rectangles": rectangle_count, "rectangle_rank": len(pivots),
        "elapsed_s_unisolated": round(out["elapsed_ns_unisolated"] / 1e9, 3),
    }, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
