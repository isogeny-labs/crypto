"""Audit exact plane-quartic point recurrence for the frozen four-anchor fan."""

import hashlib
import json
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from analyze_cross_endpoint_query_triples import rank_sparse
from analyze_residual_endpoint_combined_rows import rank_sparse_min
from screen_residual_point_reuse import quartic_cuts

HERE = Path(__file__).resolve().parent


def load(name):
    return json.loads((HERE / name).read_text())


def sha(name):
    return hashlib.sha256((HERE / name).read_bytes()).hexdigest()


def main():
    source_name = "type_i_known_scalar_query_cross_fan.json"
    plane_name = "type_i_plane_quartic.json"
    screen_name = "type_i_four_anchor_field_screen.json"
    source, plane, screen = (load(name) for name in
                             (source_name, plane_name, screen_name))
    p = int(source["field_prime"])
    roots = [tuple(map(int, point)) for point in source["endpoint_points"]]
    assert len(roots) == 81 and screen["anchors"] == [0, 3, 6, 9]
    cut = quartic_cuts(p, plane["quartic_coefficients"], roots)
    parent = {(3 * edge["query_index"] + edge["root_index"],
               edge["selected_endpoint"]): edge for edge in source["edges"]
              if 3 * edge["query_index"] + edge["root_index"] in screen["anchors"]
              and edge["selected_endpoint"] // 3 != edge["query_index"]}
    edges = {(edge["query_index"], edge["root_index"],
              edge["selected_endpoint"]): tuple(sorted(
                  tuple(map(int, point)) for point in edge["residual_points"]))
             for edge in source["edges"]}
    assert len(edges) == len(source["edges"])
    statuses = Counter()
    for index, item in enumerate(screen["selected_endpoints"]):
        anchor, partner, side = (item[key] for key in
                                 ("anchor_root", "partner_root", "residual_side"))
        endpoint = tuple(item["endpoint_point"])
        parent_edge = parent[anchor, partner]
        residuals = [tuple(map(int, point)) for point in parent_edge["residual_points"]]
        assert endpoint == residuals[side]
        other_residual = residuals[1 - side]
        u = 81 + index
        for root_index in range(81):
            if root_index == anchor:
                status = "certified_parent_line"
                points = tuple(sorted((roots[partner], other_residual)))
            elif root_index == partner:
                status = "certified_parent_line"
                points = tuple(sorted((roots[anchor], other_residual)))
            else:
                status, points = cut(root_index, endpoint)
            statuses[status] += 1
            if points is not None:
                key = root_index // 3, root_index % 3, u
                assert key not in edges
                edges[key] = points
    assert sum(statuses.values()) == 32 * 81
    bundles = {}
    by_endpoint = defaultdict(list)
    for u in range(113):
        for i in range(27):
            keys = [(i, root, u) for root in range(3)]
            if all(key in edges for key in keys):
                bundles[i, u] = tuple(point for key in keys for point in edges[key])
                by_endpoint[u].append(i)
    assert all(by_endpoint[81 + index] == item["complete_cubics"]
               for index, item in enumerate(screen["selected_endpoints"]))
    active = {key: values for key, values in bundles.items()
              if len(by_endpoint[key[1]]) >= 2}
    columns = sorted({point for values in active.values() for point in values})
    index = {point: k for k, point in enumerate(columns)}
    rows = []
    for u in range(113):
        for i, j in combinations(by_endpoint[u], 2):
            counts = Counter(index[point] for point in bundles[i, u])
            counts.subtract(index[point] for point in bundles[j, u])
            rows.append({column: coefficient % int(source["subgroup_order"])
                         for column, coefficient in counts.items() if coefficient})
    point_rank = rank_sparse(rows, int(source["subgroup_order"]))
    assert rank_sparse_min(rows, int(source["subgroup_order"])) == point_rank
    degree = Counter(point for values in active.values() for point in set(values))
    anchor_hits = {str(anchor): [[i, u] for (i, u), values in sorted(active.items())
                                 if roots[anchor] in values]
                   for anchor in screen["anchors"]}
    assert all(len(hits) >= 8 for hits in anchor_hits.values())
    assert len({roots[anchor] for anchor in screen["anchors"]}) == 4
    support = {anchor: {point for (i, u), values in active.items()
                        if u >= 81 and screen["selected_endpoints"][u - 81]["anchor_root"] == anchor
                        for point in values}
               for anchor in screen["anchors"]}
    baseline_support = {point for (i, u), values in active.items()
                        if u < 81 for point in values}
    shared_new = {f"{a},{b}": len((support[a] - baseline_support) &
                                  (support[b] - baseline_support))
                  for a, b in combinations(screen["anchors"], 2)}
    out = {
        "status": "verified_four_anchor_exact_point_incidence",
        "source_sha256": sha("analyze_four_anchor_field_incidence.py"),
        "input_sha256": {name: sha(name) for name in
                         (source_name, plane_name, screen_name,
                          "screen_residual_point_reuse.py")},
        "new_line_status_counts": dict(sorted(statuses.items())),
        "all_complete_bundles": len(bundles),
        "active_complete_bundles": len(active),
        "active_distinct_points": len(degree),
        "endpointwise_rows": len(rows),
        "exact_point_column_rank_upper_bound": point_rank,
        "max_point_bundle_degree": max(degree.values()),
        "points_with_degree_at_least_eight": sum(value >= 8 for value in degree.values()),
        "point_degree_histogram": dict(sorted((str(k), value) for k, value
                                              in Counter(degree.values()).items())),
        "anchor_active_bundle_hits": anchor_hits,
        "shared_new_points_by_anchor_pair": shared_new,
    }
    (HERE / "type_i_four_anchor_field_incidence.json").write_text(
        json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("FOUR_ANCHOR_FIELD_INCIDENCE_PASS", json.dumps({key: out[key]
          for key in ("new_line_status_counts", "active_complete_bundles",
                      "active_distinct_points", "endpointwise_rows",
                      "exact_point_column_rank_upper_bound", "max_point_bundle_degree",
                      "points_with_degree_at_least_eight",
                      "shared_new_points_by_anchor_pair")}, sort_keys=True))


if __name__ == "__main__":
    main()
