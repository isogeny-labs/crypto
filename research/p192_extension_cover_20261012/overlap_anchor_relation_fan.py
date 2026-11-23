"""Project overlap-selected cross-cubic anchor fans and measure joined rank.

Each of four roots from different query cubics supplies eight selected residual
endpoints from split cross-cubic lines. The corresponding partner bundles
contain that anchor root as an exact residual, forcing four recurring
projected columns in one known-scalar relation system.
"""

import hashlib
import json
import time
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from analyze_cross_endpoint_query_triples import rank_sparse
from four_anchor_receipt import load_four_anchor_receipt
from line_known_scalar_query_cross_fan import prelude

HERE = Path(__file__).resolve().parent
RUNTIME = HERE / "reports/run-20261027-overlap-anchor/runtime-info.json"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


def main():
    assert load("reports/run-20261027-overlap-anchor/runtime-info.json")["status"] == "verified"
    source = load("type_i_known_scalar_query_cross_fan.json")
    screen_all = load("type_i_overlap_anchor_field_screen.json")
    transfer = load("type_i_known_scalar_query_transfer.json")
    replay = load("type_i_known_scalar_query_transfer_replay.json")
    baseline = load("type_i_known_scalar_query_incidence_capacity.json")
    base = load("type_i_quotient_base_probe.json")
    prior = load_four_anchor_receipt()
    old_endpoint_ids = {tuple(map(int, point)): 81 + index
                        for index, point in enumerate(prior["new_endpoint_geometry"])}
    assert len(old_endpoint_ids) == 32
    old_edges = {(edge["query_index"], edge["root_index"], edge["selected_endpoint"]): edge
                 for edge in prior["new_split_edges"]}
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

    anchors = screen_all["anchors"]
    assert anchors == [0, 3, 6, 9] and len(screen_all["selected_endpoints"]) == 32
    assert len({root_quotients[a] for a in anchors}) == 4
    parent_edges = {(3 * edge["query_index"] + edge["root_index"],
                     edge["selected_endpoint"]): edge for edge in source["edges"]
                    if 3 * edge["query_index"] + edge["root_index"] in anchors
                    and edge["selected_endpoint"] // 3 != edge["query_index"]}
    endpoint_geometry = list(root_geometry)
    endpoint_quotients = list(root_quotients)
    known_edges = {}
    selection = []
    for selected in screen_all["selected_endpoints"]:
        anchor = selected["anchor_root"]
        partner = selected["partner_root"]
        side = selected["residual_side"]
        edge = parent_edges[anchor, partner]
        residuals = [tuple(map(int, P)) for P in edge["residual_points"]]
        projected = [tuple(map(int, Q)) for Q in edge["projected_residuals"]]
        assert residuals[0] != residuals[1]
        R, T = residuals[side], residuals[1 - side]
        qR, qT = projected[side], projected[1 - side]
        assert list(R) == selected["endpoint_point"] and R not in endpoint_geometry
        u = len(endpoint_geometry)
        endpoint_geometry.append(R)
        endpoint_quotients.append(qR)
        selection.append({"endpoint": u, "anchor_root": anchor,
                          "partner_root": partner, "parent_residual_side": side})
        for root, other in ((anchor, partner), (partner, anchor)):
            known_edges[root // 3, root % 3, u] = (
                (root_geometry[other], T), (root_quotients[other], qT))
    assert len(selection) == 32 and len(set(endpoint_geometry)) == len(endpoint_geometry) == 113
    reused_endpoints = sum(point in old_endpoint_ids for point in endpoint_geometry[81:])
    assert reused_endpoints == 15
    start = time.monotonic_ns()
    counts = Counter()
    edges = {}
    for u in range(81, 113):
        R, qR = endpoint_geometry[u], group(endpoint_quotients[u])
        old_u = old_endpoint_ids.get(R)
        if old_u is not None:
            assert tuple(map(int, prior["new_endpoint_quotients"][old_u - 81])) == endpoint_quotients[u]
        for i in range(27):
            for root in range(3):
                key = (i, root, u)
                P = root_geometry[3 * i + root]
                if key in known_edges:
                    residuals, columns = known_edges[key]
                    counts["certified_parent_line"] += 1
                    origin = "certified_parent_line"
                elif old_u is not None and (i, root, old_u) in old_edges:
                    old = old_edges[i, root, old_u]
                    residuals = tuple(tuple(map(int, point)) for point in old["residual_points"])
                    columns = tuple(tuple(map(int, point)) for point in old["projected_residuals"])
                    assert old["origin"] == "new_split"
                    counts["reused_split"] += 1
                    origin = "reused_split"
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
                    assert old_u is None
                    columns = tuple(project(T) for T in residuals)
                    counts["new_split"] += 1
                    origin = "new_split"
                assert group(root_quotients[3 * i + root]) + qR + sum(
                    (group(Q) for Q in columns), E0(0)) == S
                edges[key] = {"query_index": i, "root_index": root,
                              "selected_endpoint": u,
                              "residual_points": [list(P) for P in residuals],
                              "projected_residuals": [list(Q) for Q in columns],
                              "origin": origin}
        print("OVERLAP_ANCHOR_PROGRESS", u - 80, 32, len(edges), flush=True)
    assert sum(counts.values()) == 32 * 81
    assert counts["new_split"] + counts["reused_split"] == 1333

    all_edges = {(e["query_index"], e["root_index"], e["selected_endpoint"]):
                 tuple(tuple(map(int, Q)) for Q in e["projected_residuals"])
                 for e in source["edges"]}
    for key, edge in edges.items():
        assert key not in all_edges
        all_edges[key] = tuple(tuple(map(int, Q)) for Q in edge["projected_residuals"])
    bundles = {}
    by_endpoint = defaultdict(list)
    for u in range(113):
        for i in range(27):
            if not all((i, root, u) in all_edges for root in range(3)):
                continue
            values = tuple(Q for root in range(3) for Q in all_edges[i, root, u])
            total = sum((group(Q) for Q in values), E0(0))
            sigma = group(transfer["rows"][i]["projected_sum"])
            assert total + sigma + 3 * group(endpoint_quotients[u]) == 3 * S
            bundles[i, u] = values
            by_endpoint[u].append(i)

    active_endpoints = [u for u in range(113) if len(by_endpoint[u]) >= 2]
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
    edge_shards = []
    for first in range(81, 113):
        name = f"overlap_anchor_edges_{first}.json"
        shard = [edges[key] for key in sorted(edges) if key[2] == first]
        path = HERE / name
        path.write_text(json.dumps(shard, sort_keys=True, separators=(",", ":")) + "\n")
        assert path.stat().st_size < 64 * 1024
        edge_shards.append({"file": name, "first_endpoint": first,
                            "last_endpoint": first, "edges": len(shard),
                            "sha256": sha(path)})
    out = {
        "status": "verified_overlap_anchor_relation_fan",
        "candidate_id": None,
        "field_prime": str(p), "subgroup_order": str(r),
        "source_sha256": sha(__file__),
        "runtime_sha256": sha(RUNTIME),
        "input_sha256": {name: sha(HERE / name) for name in (
            "type_i_overlap_anchor_field_screen.json",
            "type_i_known_scalar_query_cross_fan.json",
            "type_i_known_scalar_query_transfer.json",
            "type_i_known_scalar_query_transfer_replay.json",
            "type_i_known_scalar_query_incidence_capacity.json",
            "type_i_quotient_base_probe.json",
            "type_i_four_anchor_relation_fan.json",
            "four_anchor_receipt.py")},
        "selection_rule": screen_all["selection_rule"],
        "anchors": anchors,
        "selected_parent_lines": [[item["anchor_root"], item["partner_root"]]
                                  for item in selection],
        "selected_residual_endpoints": selection,
        "new_endpoint_geometry": [list(P) for P in endpoint_geometry[81:]],
        "new_endpoint_quotients": [list(Q) for Q in endpoint_quotients[81:]],
        "new_candidate_lines": 32 * 81,
        "reused_endpoint_count": reused_endpoints,
        "new_projected_endpoint_count": 32 - reused_endpoints,
        "new_line_status_counts": dict(sorted(counts.items())),
        "edge_shards": edge_shards,
        "all_complete_bundles": len(bundles),
        "active_complete_bundles": len(active_bundles),
        "new_endpoints_with_rows": sum(u >= 81 for u in active_endpoints),
        "active_endpoints": len(active_endpoints),
        "endpointwise_rows": len(rows), "exact_rank": rank,
        "active_columns": len(columns),
        "new_columns_vs_baseline": len(set(columns) - old_active),
        "max_column_bundle_degree": max(degree.values()),
        "columns_with_degree_at_least_eight": sum(value >= 8 for value in degree.values()),
        "column_bundle_degree_histogram": dict(sorted((str(k), v)
                                                    for k, v in Counter(degree.values()).items())),
        "max_column_appearance_count": max(appearances.values()),
        "anchor_root_active_bundle_hits": {
            str(anchor): [[i, u] for (i, u), values in sorted(active_bundles.items())
                          if root_quotients[anchor] in values]
            for anchor in anchors},
        "bundle_unique_support_histogram": dict(sorted((str(k), v)
                                                    for k, v in Counter(len(set(values)) for values in active_bundles.values()).items())),
        "elapsed_ns_unisolated": time.monotonic_ns() - start,
    }
    assert all(len(out["anchor_root_active_bundle_hits"][str(anchor)]) >= 8
               for anchor in anchors)
    assert out["columns_with_degree_at_least_eight"] >= 4
    path = HERE / "type_i_overlap_anchor_relation_fan.json"
    path.write_text(json.dumps(out, sort_keys=True, separators=(",", ":")) + "\n")
    print("OVERLAP_ANCHOR_RELATION_FAN_PASS", json.dumps({key: out[key] for key in (
        "new_line_status_counts", "active_complete_bundles", "endpointwise_rows",
        "exact_rank", "active_columns", "max_column_bundle_degree",
        "columns_with_degree_at_least_eight",
        "bundle_unique_support_histogram")}, sort_keys=True))


if __name__ == "__main__":
    main()
