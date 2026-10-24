"""Screen targeted same-cubic residual endpoints for degree-four recurrence.

Four split lines selected by the complete third-cut screen each
contribute their two residual points as new endpoints.  At each endpoint we
cut all 81 frozen query roots, reuse the two certified parent-line cuts,
and form every available complete six-appearance query bundle.
"""

import hashlib
import json
import time
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from analyze_cross_endpoint_query_triples import rank_sparse
from line_known_scalar_query_cross_fan import prelude

HERE = Path(__file__).resolve().parent
RUNTIME = HERE / "reports/run-20261024-high-degree/runtime-info.json"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


def main():
    assert load("reports/run-20261024-high-degree/runtime-info.json")["status"] == "verified"
    source = load("type_i_known_scalar_query_cross_fan.json")
    screen_all = load("type_i_same_cubic_residual_third_cut_screen.json")
    transfer = load("type_i_known_scalar_query_transfer.json")
    replay = load("type_i_known_scalar_query_transfer_replay.json")
    baseline = load("type_i_known_scalar_query_incidence_capacity.json")
    base = load("type_i_quotient_base_probe.json")
    assert source["status"] == "verified_known_scalar_query_cross_fan_screen"
    assert baseline["cross_fan_sha256"] == sha(HERE / "type_i_known_scalar_query_cross_fan.json")
    ns = prelude("line_residual_pivot_three.py", "start_ns = time.monotonic_ns()")
    p, r, h = int(ns["p"]), int(ns["dt"].r), int(base["candidate_cofactor"])
    assert p == int(source["field_prime"]) and r == int(source["subgroup_order"])
    F, E6, E0 = ns["dt"].gm.F, ns["E6"], ns["E0"]
    S = ns["S"]
    taylor_at, evaluate = ns["taylor_at"], ns["evaluate"]
    inverse = ns["inverse"]
    trace3, scalar3, anchor_trace3 = (ns[name] for name in
                                      ("trace3", "scalar3", "anchor_trace3"))
    G = E0(F(ns["dt"].gm.gx), F(ns["dt"].gm.gy))
    C = E0(F(int(replay["shared_constant"][0])), F(int(replay["shared_constant"][1])))
    assert r * G == E0(0)

    def group(key):
        return E0(F(int(key[0])), F(int(key[1])))

    def project(point):
        cover, ys = inverse.invert_affine(F(point[0]), F(point[1]))
        image = inverse.quotient_image(E6, cover, ys, 0)
        Q = scalar3 * (trace3(image) - anchor_trace3)
        assert r * Q == E0(0)
        key = ns["projector"].key(Q)
        assert key is not None
        return tuple(map(int, key))

    scalars = [int(item["scalar"]) for item in transfer["rows"]]
    assert len(scalars) == 27
    root_geometry = [tuple(map(int, P)) for P in source["endpoint_points"]]
    root_quotients = [tuple(map(int, Q)) for Q in source["endpoint_quotients"]]
    assert len(root_geometry) == len(root_quotients) == 81
    for i, item in enumerate(transfer["rows"]):
        Q = group(item["quotient"])
        sigma = group(item["projected_sum"])
        assert Q == -2 * scalars[i] * G and sigma + 2 * h * Q == C

    same = {}
    for edge in source["edges"]:
        i, a, u = edge["query_index"], edge["root_index"], edge["selected_endpoint"]
        if u // 3 != i or u % 3 == a:
            continue
        key = (i, *sorted((a, u % 3)))
        if key in same:
            old = same[key]
            assert {tuple(P) for P in old["residual_points"]} == {
                tuple(P) for P in edge["residual_points"]}
        else:
            same[key] = edge
    assert len(same) == 43
    top_sets = screen_all["top_root_endpoint_sets"]
    assert set(top_sets) == {"58", "78"}
    selected_original_endpoints = sorted({u for users in top_sets.values() for u in users})
    original_line_numbers = sorted({(u - 81) // 2 for u in selected_original_endpoints})
    assert original_line_numbers == [29, 30, 41, 42]
    chosen_lines = [sorted(same)[number] for number in original_line_numbers]
    endpoint_geometry = list(root_geometry)
    endpoint_quotients = list(root_quotients)
    known_edges = {}
    selection = []
    for original_line_number, (i, a, b) in zip(original_line_numbers, chosen_lines):
        edge = same[i, a, b]
        residuals = [tuple(map(int, P)) for P in edge["residual_points"]]
        projected = [tuple(map(int, Q)) for Q in edge["projected_residuals"]]
        assert residuals[0] != residuals[1]
        for side in (0, 1):
            R, T = residuals[side], residuals[1 - side]
            qR, qT = projected[side], projected[1 - side]
            assert R not in endpoint_geometry
            u = len(endpoint_geometry)
            endpoint_geometry.append(R)
            endpoint_quotients.append(qR)
            selection.append({"endpoint": u, "original_candidate_endpoint":
                              81 + 2 * original_line_number + side,
                              "parent_cubic": i,
                              "parent_roots": [a, b], "parent_residual_side": side})
            for root, other in ((a, b), (b, a)):
                known_edges[i, root, u] = (
                    (root_geometry[3 * i + other], T),
                    (root_quotients[3 * i + other], qT))
    assert len(selection) == 8 and len(set(endpoint_geometry)) == len(endpoint_geometry) == 89
    assert sorted(item["original_candidate_endpoint"] for item in selection) == selected_original_endpoints
    start = time.monotonic_ns()
    counts = Counter()
    edges = {}
    for u in range(81, 89):
        R, qR = endpoint_geometry[u], group(endpoint_quotients[u])
        for i in range(27):
            for root in range(3):
                key = (i, root, u)
                P = root_geometry[3 * i + root]
                if key in known_edges:
                    residuals, columns = known_edges[key]
                    counts["certified_parent_line"] += 1
                else:
                    d = (R[0] - P[0]) % p
                    if not d:
                        counts["vertical_or_coincident"] += 1
                        continue
                    slope = (R[1] - P[1]) * pow(d, -1, p) % p
                    taylor = taylor_at(*P)
                    c1, c2, c3, A = (evaluate(taylor[k], slope) for k in range(1, 5))
                    if not A:
                        counts["degree_below_four"] += 1
                        continue
                    B, D = (c3 + d * A) % p, (c2 + d * (c3 + d * A)) % p
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
                    sq, den = pow(discr, (p + 1) // 4, p), pow(2 * A, -1, p)
                    shifts = [(-B + sq) * den % p, (-B - sq) * den % p]
                    residuals = tuple(((P[0] + z) % p,
                                       (P[1] + slope * z) % p) for z in shifts)
                    assert len(set(residuals)) == 2
                    columns = tuple(project(T) for T in residuals)
                    counts["new_split"] += 1
                assert group(root_quotients[3 * i + root]) + qR + sum(
                    (group(Q) for Q in columns), E0(0)) == S
                edges[key] = {"query_index": i, "root_index": root,
                              "selected_endpoint": u,
                              "residual_points": [list(P) for P in residuals],
                              "projected_residuals": [list(Q) for Q in columns],
                              "origin": "certified_parent_line" if key in known_edges else "new_split"}
        print("RESIDUAL_ENDPOINT_PROGRESS", u - 80, 8, len(edges), flush=True)
    assert sum(counts.values()) == 8 * 81

    all_edges = {(e["query_index"], e["root_index"], e["selected_endpoint"]):
                 tuple(tuple(map(int, Q)) for Q in e["projected_residuals"])
                 for e in source["edges"]}
    for key, edge in edges.items():
        assert key not in all_edges
        all_edges[key] = tuple(tuple(map(int, Q)) for Q in edge["projected_residuals"])
    bundles = {}
    by_endpoint = defaultdict(list)
    for u in range(89):
        for i in range(27):
            if not all((i, root, u) in all_edges for root in range(3)):
                continue
            values = tuple(Q for root in range(3) for Q in all_edges[i, root, u])
            total = sum((group(Q) for Q in values), E0(0))
            sigma = group(transfer["rows"][i]["projected_sum"])
            assert total + sigma + 3 * group(endpoint_quotients[u]) == 3 * S
            bundles[i, u] = values
            by_endpoint[u].append(i)

    active_endpoints = [u for u in range(89) if len(by_endpoint[u]) >= 2]
    active_bundles = {(i, u): bundles[i, u] for u in active_endpoints for i in by_endpoint[u]}
    columns = sorted({Q for values in active_bundles.values() for Q in values})
    index = {Q: j for j, Q in enumerate(columns)}
    rows = []
    for u in active_endpoints:
        for i, j in combinations(by_endpoint[u], 2):
            left, right = bundles[i, u], bundles[j, u]
            scalar = (-4 * h * (scalars[i] - scalars[j])) % r
            assert sum((group(Q) for Q in left), E0(0)) - sum(
                (group(Q) for Q in right), E0(0)) == scalar * G
            coefficients = Counter(index[Q] for Q in left)
            coefficients.subtract(index[Q] for Q in right)
            rows.append({k: value % r for k, value in coefficients.items() if value % r})
    rank = rank_sparse(rows, r)
    degree = Counter(Q for values in active_bundles.values() for Q in set(values))
    appearances = Counter(Q for values in active_bundles.values() for Q in values)
    old_active = set()
    for u in range(81):
        if len(by_endpoint[u]) >= 2:
            for i in by_endpoint[u]:
                old_active.update(bundles[i, u])
    assert len(old_active) == baseline["active_columns"] == 1316
    assert rank >= baseline["exact_rank"] == 181
    out = {
        "status": "verified_residual_endpoint_high_degree",
        "candidate_id": None,
        "field_prime": str(p), "subgroup_order": str(r),
        "source_sha256": sha(__file__),
        "runtime_sha256": sha(RUNTIME),
        "input_sha256": {name: sha(HERE / name) for name in (
            "type_i_same_cubic_residual_third_cut_screen.json",
            "type_i_known_scalar_query_cross_fan.json",
            "type_i_known_scalar_query_transfer.json",
            "type_i_known_scalar_query_transfer_replay.json",
            "type_i_known_scalar_query_incidence_capacity.json",
            "type_i_quotient_base_probe.json")},
        "selection_rule": "Both residuals from every parent line for query roots with four complete parent bundles in the frozen third-cut screen.",
        "selected_original_candidate_endpoints": selected_original_endpoints,
        "selected_root_indices": sorted(map(int, top_sets)),
        "selected_parent_lines": [list(key) for key in chosen_lines],
        "selected_residual_endpoints": selection,
        "new_endpoint_geometry": [list(P) for P in endpoint_geometry[81:]],
        "new_endpoint_quotients": [list(Q) for Q in endpoint_quotients[81:]],
        "new_candidate_lines": 8 * 81,
        "new_line_status_counts": dict(sorted(counts.items())),
        "new_split_edges": [edges[key] for key in sorted(edges)],
        "all_complete_bundles": len(bundles),
        "active_complete_bundles": len(active_bundles),
        "new_endpoints_with_rows": sum(u >= 81 for u in active_endpoints),
        "active_endpoints": len(active_endpoints),
        "endpointwise_rows": len(rows), "exact_rank": rank,
        "active_columns": len(columns),
        "new_columns_vs_baseline": len(set(columns) - old_active),
        "max_column_bundle_degree": max(degree.values()),
        "column_bundle_degree_histogram": dict(sorted((str(k), v)
                                                    for k, v in Counter(degree.values()).items())),
        "max_column_appearance_count": max(appearances.values()),
        "bundle_unique_support_histogram": dict(sorted((str(k), v)
                                                    for k, v in Counter(len(set(values)) for values in active_bundles.values()).items())),
        "elapsed_ns_unisolated": time.monotonic_ns() - start,
    }
    path = HERE / "type_i_residual_endpoint_high_degree.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("RESIDUAL_ENDPOINT_HIGH_DEGREE_PASS", json.dumps({key: out[key] for key in (
        "new_line_status_counts", "active_complete_bundles", "endpointwise_rows",
        "exact_rank", "active_columns", "max_column_bundle_degree",
        "bundle_unique_support_histogram")}, sort_keys=True))


if __name__ == "__main__":
    main()
