"""Construct 12-column known-scalar rows from split query-cubic line fans.

For each of the 27 distinct fully split query cubics, test lines from its
three rational roots to the 16 frozen endpoints. Two complete three-cut
bundles at one endpoint cancel both that endpoint and the cubic constant.
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
    transfer_path = HERE / "type_i_known_scalar_query_transfer.json"
    replay_path = HERE / "type_i_known_scalar_query_transfer_replay.json"
    selection_path = HERE / "type_i_triple_star_character_selection.json"
    reduced_path = HERE / "type_i_triple_star_character_reduced96.json"
    base_path = HERE / "type_i_quotient_base_probe.json"
    transfer, replay, selection, reduced, base = (
        json.loads(path.read_text()) for path in
        (transfer_path, replay_path, selection_path, reduced_path, base_path))
    assert transfer["queries_checked"] == replay["queries_checked"] == 27
    endpoints = selection["optimized"]["endpoints"]
    assert len(endpoints) == 16 and endpoints == reduced["selected_endpoints"]
    ns = prelude("line_residual_pivot_three.py", "start_ns = time.monotonic_ns()")
    p, r, h = int(ns["p"]), int(ns["dt"].r), int(base["candidate_cofactor"])
    assert str(p) == transfer["field_prime"]
    assert str(r) == transfer["subgroup_order"]
    assert str((2 * h) % r) == replay["known_scalar_difference_multiplier_mod_r"]
    F = ns["dt"].gm.F
    E6, E0, inverse = ns["E6"], ns["E0"], ns["inverse"]
    trace3, scalar3, anchor_trace3 = (ns[name] for name in
                                      ("trace3", "scalar3", "anchor_trace3"))
    S, selected_q = ns["S"], ns["selected_q"]
    taylor_at, evaluate = ns["taylor_at"], ns["evaluate"]
    G = E0(F(ns["dt"].gm.gx), F(ns["dt"].gm.gy))
    assert r * G == E0(0)
    C = E0(F(replay["shared_constant"][0]),
           F(replay["shared_constant"][1]))
    fixed = {tuple(map(int, row["quotient"])) for row in reduced["rows"]}
    assert len(fixed) == 96

    def projected(point):
        cover, ys = inverse.invert_affine(F(point[0]), F(point[1]))
        image = inverse.quotient_image(E6, cover, ys, 0)
        Q = scalar3 * (trace3(image) - anchor_trace3)
        assert r * Q == E0(0)
        key = ns["projector"].key(Q)
        assert key is not None
        return Q, tuple(map(int, key))

    queries = []
    for item in transfer["rows"]:
        a = int(item["scalar"])
        Q = E0(F(int(item["quotient"][0])), F(int(item["quotient"][1])))
        sigma = E0(F(item["projected_sum"][0]),
                   F(item["projected_sum"][1]))
        assert Q == -2 * a * G and sigma + (2 * h) * Q == C
        assert len(item["points"]) == len(item["projected_points"]) == 3
        pivots = [E0(F(point[0]), F(point[1]))
                  for point in item["projected_points"]]
        assert sum(pivots, E0(0)) == sigma
        queries.append((a, Q, sigma, pivots, [tuple(map(int, P)) for P in item["points"]]))

    counts = Counter()
    edges = {}
    occurrences = defaultdict(list)
    start = time.monotonic_ns()
    for query_index, (_, _, _, pivots, plane_points) in enumerate(queries):
        for root_index, pivot in enumerate(plane_points):
            pivot_q = pivots[root_index]
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
                D = (c2 + d * B) % p
                if not D or (c1 + d * D) % p:
                    counts["tangent_or_invalid"] += 1
                    continue
                discr = (B * B - 4 * A * D) % p
                if not discr:
                    counts["double_residual"] += 1
                    continue
                if pow(discr, (p - 1) // 2, p) != 1:
                    counts["nonsplit"] += 1
                    continue
                sq = pow(discr, (p + 1) // 4, p)
                den = pow(2 * A, -1, p)
                roots = [(-B + sq) * den % p, (-B - sq) * den % p]
                residuals = [((pivot[0] + z) % p,
                              (pivot[1] + slope * z) % p) for z in roots]
                assert len(set(residuals)) == 2
                projected_residuals = [projected(P) for P in residuals]
                assert (pivot_q + selected_q[endpoint] +
                        projected_residuals[0][0] + projected_residuals[1][0] == S)
                keys = [item[1] for item in projected_residuals]
                assert keys[0] != keys[1]
                edge_key = (query_index, root_index, endpoint)
                edges[edge_key] = {
                    "query_index": query_index, "root_index": root_index,
                    "selected_endpoint": endpoint,
                    "residual_points": [list(P) for P in residuals],
                    "projected_residuals": [list(key) for key in keys],
                }
                for side, key in enumerate(keys):
                    occurrences[key].append([query_index, root_index, endpoint, side])
                counts["split"] += 1
        print("QUERY_RELATION_FAN_PROGRESS", query_index + 1, 27,
              counts["split"], flush=True)
    assert sum(counts.values()) == 27 * 3 * 16

    bundles = {}
    for query_index, (_, _, sigma, _, _) in enumerate(queries):
        for endpoint in endpoints:
            if not all((query_index, root, endpoint) in edges for root in range(3)):
                continue
            keys = [tuple(Q) for root in range(3)
                    for Q in edges[query_index, root, endpoint]["projected_residuals"]]
            total = E0(0)
            for key in keys:
                total += E0(F(key[0]), F(key[1]))
            assert total + sigma + 3 * selected_q[endpoint] == 3 * S
            bundles[query_index, endpoint] = keys

    columns = sorted(occurrences)
    column_index = {key: i for i, key in enumerate(columns)}
    rows = []
    pivots = {}
    for endpoint in endpoints:
        present = [i for i in range(len(queries)) if (i, endpoint) in bundles]
        for i, j in combinations(present, 2):
            ai, Qi = queries[i][0], queries[i][1]
            aj, Qj = queries[j][0], queries[j][1]
            left, right = bundles[i, endpoint], bundles[j, endpoint]
            group_left = sum((E0(F(x), F(y)) for x, y in left), E0(0))
            group_right = sum((E0(F(x), F(y)) for x, y in right), E0(0))
            scalar = (-4 * h * (ai - aj)) % r
            assert group_left - group_right == (2 * h) * (Qi - Qj)
            assert group_left - group_right == scalar * G
            sparse = Counter()
            for key in left:
                sparse[column_index[key]] += 1
            for key in right:
                sparse[column_index[key]] -= 1
            row = {index: value % r for index, value in sparse.items()
                   if value % r}
            rows.append({
                "endpoint": endpoint, "query_indices": [i, j],
                "known_scalar_mod_r": str(scalar),
                "nonzero_columns": len(row),
            })
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
                inverse_lead = pow(row[lead], -1, r)
                pivots[lead] = {index: value * inverse_lead % r
                                for index, value in row.items()}

    repeats = [{"quotient": list(key), "occurrences": where}
               for key, where in sorted(occurrences.items()) if len(where) > 1]
    fixed_hits = sorted([list(key) for key in set(columns) & fixed])
    out = {
        "status": "verified_known_scalar_query_twelve_column_relation_screen",
        "candidate_id": None,
        "field_prime": str(p), "subgroup_order": str(r),
        "cofactor_mod_r": str(h % r),
        "queries": len(queries), "endpoints": endpoints,
        "candidate_lines": 27 * 3 * 16,
        "line_status_counts": dict(sorted(counts.items())),
        "split_edges": len(edges), "complete_three_cut_bundles": len(bundles),
        "projected_column_appearances": 2 * len(edges),
        "distinct_projected_columns": len(columns),
        "projected_repeats": repeats, "fixed96_column_hits": fixed_hits,
        "known_scalar_rows": rows, "known_scalar_row_rank": len(pivots),
        "edges": [edges[key] for key in sorted(edges)],
        "elapsed_ns_unisolated": time.monotonic_ns() - start,
        "source_sha256": sha(__file__),
        "input_sha256": {path.name: sha(path) for path in
                         (transfer_path, replay_path, selection_path,
                          reduced_path, base_path,
                          HERE / "line_residual_pivot_three.py")},
        "runtime_sha256": sha(HERE / "type_i_known_scalar_query_relation_runtime_info.json"),
        "boundary": "Verified known-scalar 12-column rows from complete three-cut query bundles. Rank is measured over the new and fixed projected columns; factor-base closure, relation yield for unseen targets, linear algebra, descent, and total cost remain separate.",
    }
    path = HERE / "type_i_known_scalar_query_relation_screen.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("QUERY_RELATION_SCREEN_PASS", json.dumps({
        "queries": len(queries), "split_edges": len(edges),
        "bundles": len(bundles), "known_scalar_rows": len(rows),
        "rank": len(pivots), "distinct_columns": len(columns),
        "projected_repeats": len(repeats), "fixed96_hits": len(fixed_hits),
        "elapsed_s_unisolated": round(out["elapsed_ns_unisolated"] / 1e9, 3),
    }, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
