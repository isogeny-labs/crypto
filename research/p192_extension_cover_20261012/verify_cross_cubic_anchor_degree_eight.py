"""Independent integer-coordinate replay of the cross-cubic anchor fan."""

import hashlib
import json
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from analyze_cross_endpoint_query_triples import rank_sparse
from verify_adaptive_private_columns import GX, GY, add, mul, neg, on_curve

HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


def pt(raw):
    return tuple(map(int, raw))


def main():
    source, result = (load(name) for name in (
        "type_i_known_scalar_query_cross_fan.json",
        "type_i_cross_cubic_anchor_degree_eight.json"))
    transfer = load("type_i_known_scalar_query_transfer.json")
    replay = load("type_i_known_scalar_query_transfer_replay.json")
    base = load("type_i_quotient_base_probe.json")
    screen = load("type_i_triple_star_screen.json")
    plane = load("type_i_plane_quartic.json")
    assert result["status"] == "verified_cross_cubic_anchor_degree_eight"
    assert result["source_sha256"] == sha(HERE / "cross_cubic_anchor_degree_eight.py")
    assert result["runtime_sha256"] == sha(HERE / "reports/run-20261024-anchor-eight/runtime-info.json")
    for name, expected in result["input_sha256"].items():
        assert expected == sha(HERE / name)
    p, r, h = int(result["field_prime"]), int(result["subgroup_order"]), int(base["candidate_cofactor"])
    assert p == int(plane["p"]) and r == int(source["subgroup_order"])
    G, S, C = (GX, GY), pt(screen["line_section_constant"]), pt(replay["shared_constant"])
    assert on_curve(G, p) and mul(r, G, p) is None
    terms = [(a, b, int(coefficient)) for label, coefficient in
             plane["quartic_coefficients"].items()
             for a, b in [map(int, label.split(",")[:2])]]

    def quartic(P):
        x, y = P
        return sum(coefficient * pow(x, a, p) * pow(y, b, p)
                   for a, b, coefficient in terms) % p

    def sum_points(values):
        total = None
        for Q in values:
            total = add(total, Q, p)
        return total

    anchor = result["anchor_root"]
    screen_all = load("type_i_cross_cubic_anchor_screen.json")
    assert anchor == screen_all["anchor_root"] == 0
    selected = screen_all["selected_endpoints"]
    parent = {edge["selected_endpoint"]: edge for edge in source["edges"]
              if edge["query_index"] == anchor // 3
              and edge["root_index"] == anchor % 3
              and edge["selected_endpoint"] // 3 != anchor // 3}
    assert result["selected_parent_lines"] == [[anchor, row["partner_root"]]
                                               for row in selected]
    geometry = [pt(P) for P in source["endpoint_points"]]
    quotients = [pt(Q) for Q in source["endpoint_quotients"]]
    expected_selection = []
    for row in selected:
        partner, side = row["partner_root"], row["residual_side"]
        edge = parent[partner]
        u = len(geometry)
        geometry.append(pt(edge["residual_points"][side]))
        quotients.append(pt(edge["projected_residuals"][side]))
        assert list(geometry[-1]) == row["endpoint_point"]
        expected_selection.append({"endpoint": u, "anchor_root": anchor,
                                   "partner_root": partner,
                                   "parent_residual_side": side})
    assert len(geometry) == 89 and len(set(geometry)) == 89
    assert expected_selection == result["selected_residual_endpoints"]
    assert [list(P) for P in geometry[81:]] == result["new_endpoint_geometry"]
    assert [list(Q) for Q in quotients[81:]] == result["new_endpoint_quotients"]
    assert all(quartic(P) == 0 and on_curve(Q, p)
               for P, Q in zip(geometry, quotients))

    edges = {}
    for edge in source["edges"]:
        key = edge["query_index"], edge["root_index"], edge["selected_endpoint"]
        assert key not in edges
        edges[key] = tuple(pt(Q) for Q in edge["projected_residuals"])
    new_status = Counter()
    for edge in result["new_split_edges"]:
        i, root, u = edge["query_index"], edge["root_index"], edge["selected_endpoint"]
        key = i, root, u
        assert key not in edges and 0 <= i < 27 and 0 <= root < 3 and 81 <= u < 89
        P, R = geometry[3 * i + root], geometry[u]
        assert P != R and P[0] != R[0]
        slope = (R[1] - P[1]) * pow((R[0] - P[0]) % p, -1, p) % p
        residuals = [pt(T) for T in edge["residual_points"]]
        columns = [pt(Q) for Q in edge["projected_residuals"]]
        assert len(residuals) == len(columns) == 2
        assert len(set(residuals)) == 2 and all(T not in (P, R) for T in residuals)
        assert all(quartic(T) == 0 and (T[1] - P[1] - slope * (T[0] - P[0])) % p == 0
                   for T in residuals)
        assert all(on_curve(Q, p) for Q in columns)
        assert sum_points([quotients[3 * i + root], quotients[u], *columns]) == S
        origin = edge["origin"]
        assert origin in ("certified_parent_line", "new_split")
        new_status[origin] += 1
        if origin == "certified_parent_line":
            selection = expected_selection[u - 81]
            partner = selection["partner_root"]
            side = selection["parent_residual_side"]
            root_index = 3 * i + root
            assert root_index in (anchor, partner)
            other_root = partner if root_index == anchor else anchor
            parent_edge = parent[partner]
            other_residual = pt(parent_edge["residual_points"][1 - side])
            other_quotient = pt(parent_edge["projected_residuals"][1 - side])
            assert set(residuals) == {geometry[other_root], other_residual}
            assert set(columns) == {quotients[other_root], other_quotient}
        edges[key] = tuple(columns)
    assert len(result["new_split_edges"]) == len(set(
        (e["query_index"], e["root_index"], e["selected_endpoint"])
        for e in result["new_split_edges"]))
    assert new_status["certified_parent_line"] == 16
    assert new_status["new_split"] == result["new_line_status_counts"]["new_split"]

    bundles = {}
    by_endpoint = defaultdict(list)
    for u in range(89):
        for i in range(27):
            if not all((i, root, u) in edges for root in range(3)):
                continue
            values = tuple(Q for root in range(3) for Q in edges[i, root, u])
            total = sum_points(values)
            sigma = pt(transfer["rows"][i]["projected_sum"])
            assert sum_points([total, sigma, mul(3, quotients[u], p)]) == mul(3, S, p)
            bundles[i, u] = values
            by_endpoint[u].append(i)
    assert len(bundles) == result["all_complete_bundles"]
    assert all(by_endpoint[81 + index] == row["complete_cubics"]
               for index, row in enumerate(selected))
    active = {(i, u): bundles[i, u] for u in range(89)
              if len(by_endpoint[u]) >= 2 for i in by_endpoint[u]}
    columns = sorted({Q for values in active.values() for Q in values})
    index = {Q: k for k, Q in enumerate(columns)}
    scalars = [int(row["scalar"]) for row in transfer["rows"]]
    rows = []
    for u in range(89):
        for i, j in combinations(by_endpoint[u], 2):
            left, right = bundles[i, u], bundles[j, u]
            delta = add(sum_points(left), neg(sum_points(right), p), p)
            scalar = (-4 * h * (scalars[i] - scalars[j])) % r
            assert delta == mul(scalar, G, p)
            coeffs = Counter(index[Q] for Q in left)
            coeffs.subtract(index[Q] for Q in right)
            rows.append({k: v % r for k, v in coeffs.items() if v % r})
    degree = Counter(Q for values in active.values() for Q in set(values))
    appearances = Counter(Q for values in active.values() for Q in values)
    hist = dict(sorted((str(k), v) for k, v in Counter(degree.values()).items()))
    support_hist = dict(sorted((str(k), v) for k, v in Counter(
        len(set(values)) for values in active.values()).items()))
    rank = rank_sparse(rows, r)
    assert len(rows) == result["endpointwise_rows"]
    assert rank == result["exact_rank"]
    assert len(active) == result["active_complete_bundles"]
    assert len(columns) == result["active_columns"]
    assert max(degree.values()) == result["max_column_bundle_degree"]
    assert max(appearances.values()) == result["max_column_appearance_count"]
    assert hist == result["column_bundle_degree_histogram"]
    assert support_hist == result["bundle_unique_support_histogram"]
    hits = [[i, u] for (i, u), values in sorted(active.items())
            if quotients[anchor] in values]
    assert hits == result["anchor_root_active_bundle_hits"] and len(hits) >= 8
    assert all([row["partner_root"] // 3, 81 + index] in hits
               for index, row in enumerate(selected))
    out = {"status": "independent_cross_cubic_anchor_degree_eight_replay_pass",
           "source_sha256": sha(__file__),
           "pilot_sha256": sha(HERE / "type_i_cross_cubic_anchor_degree_eight.json"),
           "edges_checked": len(result["new_split_edges"]),
           "rows_checked": len(rows), "exact_rank": rank,
           "active_columns": len(columns),
           "max_column_bundle_degree": max(degree.values()),
           "anchor_root_active_bundle_hits": hits}
    path = HERE / "type_i_cross_cubic_anchor_degree_eight_replay.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("CROSS_CUBIC_ANCHOR_DEGREE_EIGHT_REPLAY_PASS", json.dumps(out, sort_keys=True))


if __name__ == "__main__":
    main()
