"""Freeze an exact-point overlap policy for four P-192 query-root anchors.

The search uses only F_p quartic cuts, complete-cubic incidence, and modular
row rank. Quotient projection is a separate checked Sage stage.
"""

import hashlib
import json
import random
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path

from analyze_cross_endpoint_query_triples import rank_sparse
from screen_residual_point_reuse import quartic_cuts

HERE = Path(__file__).resolve().parent
ANCHORS = (0, 3, 6, 9)
FORCED = {(0, 9, 1), (3, 57, 1), (3, 64, 0), (6, 64, 1), (9, 57, 1)}
SEED = 20261027
RANDOM_STARTS = 24


def load(name):
    return json.loads((HERE / name).read_text())


def sha(name):
    return hashlib.sha256((HERE / name).read_bytes()).hexdigest()


def difference_rows(bundles, modulus):
    rows = []
    for i, j in combinations(sorted(bundles), 2):
        counts = Counter(bundles[i])
        counts.subtract(bundles[j])
        rows.append({point: value % modulus for point, value in counts.items()
                     if value % modulus})
    return rows


def main():
    source_name = "type_i_known_scalar_query_cross_fan.json"
    plane_name = "type_i_plane_quartic.json"
    prior_name = "type_i_four_anchor_field_screen.json"
    source, plane, prior = (load(name) for name in
                            (source_name, plane_name, prior_name))
    p, r = int(source["field_prime"]), int(source["subgroup_order"])
    assert p == int(plane["p"]) and p % 4 == 3 and prior["anchors"] == list(ANCHORS)
    roots = [tuple(map(int, point)) for point in source["endpoint_points"]]
    assert len(roots) == 81
    cut = quartic_cuts(p, plane["quartic_coefficients"], roots)
    base_edges = {(edge["query_index"], edge["root_index"], edge["selected_endpoint"]):
                  tuple(tuple(map(int, point)) for point in edge["residual_points"])
                  for edge in source["edges"]}
    base_bundles = defaultdict(dict)
    for u in range(81):
        for i in range(27):
            if all((i, j, u) in base_edges for j in range(3)):
                base_bundles[u][i] = tuple(point for j in range(3)
                                           for point in base_edges[i, j, u])
    active_base = {u: bundle for u, bundle in base_bundles.items()
                   if len(bundle) >= 2}
    base_support = {point for bundle in active_base.values()
                    for values in bundle.values() for point in values}
    base_rows = [row for bundle in active_base.values()
                 for row in difference_rows(bundle, r)]
    base_index = {point: i for i, point in enumerate(sorted(base_support))}
    assert len(base_support) == 1316 and len(base_rows) == 370
    assert rank_sparse([{base_index[P]: v for P, v in row.items()}
                        for row in base_rows], r) == 181

    items, status_counts = [], Counter()
    for anchor in ANCHORS:
        parents = sorted((edge for edge in source["edges"]
                          if 3 * edge["query_index"] + edge["root_index"] == anchor
                          and edge["selected_endpoint"] // 3 != anchor // 3),
                         key=lambda edge: edge["selected_endpoint"])
        for edge in parents:
            partner = edge["selected_endpoint"]
            residuals = [tuple(map(int, point)) for point in edge["residual_points"]]
            for side, endpoint in enumerate(residuals):
                other = residuals[1 - side]
                cuts = {}
                for root_index in range(81):
                    if root_index == anchor:
                        status, outputs = "certified_parent_line", (roots[partner], other)
                    elif root_index == partner:
                        status, outputs = "certified_parent_line", (roots[anchor], other)
                    else:
                        status, outputs = cut(root_index, endpoint)
                    status_counts[status] += 1
                    if outputs is not None:
                        cuts[root_index] = outputs
                complete = [i for i in range(27)
                            if all(3 * i + j in cuts for j in range(3))]
                if len(complete) < 2 or partner // 3 not in complete:
                    continue
                bundles = {i: tuple(point for j in range(3)
                                    for point in cuts[3 * i + j])
                           for i in complete}
                support = {point for values in bundles.values() for point in values}
                rows = difference_rows(bundles, r)
                local_index = {point: i for i, point in enumerate(sorted(support))}
                local_rank = rank_sparse([{local_index[P]: v for P, v in row.items()}
                                          for row in rows], r)
                assert roots[anchor] in bundles[partner // 3]
                items.append({"key": (anchor, partner, side), "point": endpoint,
                              "complete": complete, "support": support,
                              "new": support - base_support, "rows": rows,
                              "local_rank": local_rank})
    assert len(items) == 88
    assert Counter(item["key"][0] for item in items) == {0: 14, 3: 17, 6: 30, 9: 27}
    assert len({item["point"] for item in items}) == 87
    lookup = {item["key"]: item for item in items}
    assert lookup[0, 9, 1]["point"] == lookup[9, 0, 1]["point"]

    cross_points = {point for a, b in combinations(items, 2)
                    if a["key"][0] != b["key"][0] and a["point"] != b["point"]
                    for point in a["new"] & b["new"]}
    assert cross_points == {roots[i] for i in (0, 9, 57, 64)}
    assert [item["key"] for item in items
            if item["key"][0] != 0 and roots[0] in item["new"]] == [(9, 0, 1)]
    assert [item["key"] for item in items
            if item["key"][0] != 9 and roots[9] in item["new"]] == [(0, 9, 1)]

    unique_items, seen = [], set()
    for item in items:
        if item["point"] not in seen:
            unique_items.append(item)
            seen.add(item["point"])
    full_support = base_support | set().union(*(item["support"] for item in unique_items))
    full_index = {point: i for i, point in enumerate(sorted(full_support))}
    full_rows = base_rows + [row for item in unique_items for row in item["rows"]]
    full_rank = rank_sparse([{full_index[P]: v for P, v in row.items()}
                             for row in full_rows], r)
    assert len(unique_items) == 87 and len(full_rows) == 924
    assert full_rank == 425 == 181 + sum(item["local_rank"] for item in unique_items)

    by_anchor = {anchor: sorted((item for item in items if item["key"][0] == anchor),
                                key=lambda item: item["key"])
                 for anchor in ANCHORS}
    prior_keys = [tuple(item[key] for key in
                        ("anchor_root", "partner_root", "residual_side"))
                  for item in prior["selected_endpoints"]]

    def score(keys):
        selected = [lookup[key] for key in keys]
        if len({item["point"] for item in selected}) != len(selected):
            return None
        support = set().union(*(item["new"] for item in selected))
        return len(base_support) + len(support), 181 + sum(
            item["local_rank"] for item in selected)

    def better(a, b):
        left, right = a[0] * b[1], b[0] * a[1]
        return left < right or (left == right and (a[0], -a[1]) < (b[0], -b[1]))

    rng = random.Random(SEED)

    def seed_selection(kind):
        chosen = set(FORCED)
        points = {lookup[key]["point"] for key in chosen}
        for anchor in ANCHORS:
            if kind == "first":
                order = [lookup[key] for key in prior_keys if key[0] == anchor]
            elif kind == "ratio":
                order = sorted(by_anchor[anchor], key=lambda item:
                               (len(item["new"]) / item["local_rank"], item["key"]))
            elif kind == "rank":
                order = sorted(by_anchor[anchor], key=lambda item:
                               (-item["local_rank"], len(item["new"]), item["key"]))
            elif kind == "size":
                order = sorted(by_anchor[anchor], key=lambda item:
                               (len(item["new"]), -item["local_rank"], item["key"]))
            else:
                order = by_anchor[anchor][:]
                rng.shuffle(order)
            for item in order:
                if sum(key[0] == anchor for key in chosen) == 8:
                    break
                if item["key"] not in chosen and item["point"] not in points:
                    chosen.add(item["key"])
                    points.add(item["point"])
        assert len(chosen) == 32 and score(chosen)
        return chosen

    def descend(chosen):
        steps = 0
        while True:
            current, best = score(chosen), None
            for old in sorted(chosen - FORCED):
                for item in by_anchor[old[0]]:
                    new = item["key"]
                    if new in chosen:
                        continue
                    trial = chosen - {old} | {new}
                    value = score(trial)
                    if value is None or not better(value, current):
                        continue
                    if (best is None or better(value, best[0]) or
                        (value == best[0] and (old, new) < best[1:])):
                        best = value, old, new
            if best is None:
                return chosen, steps
            chosen = chosen - {best[1]} | {best[2]}
            steps += 1

    starts = []
    for kind in ("first", "ratio", "rank", "size") + ("random",) * RANDOM_STARTS:
        selected, steps = descend(seed_selection(kind))
        starts.append({"kind": kind, "score": score(selected),
                       "selected": tuple(sorted(selected)), "steps": steps})
    best = starts[0]
    for result in starts[1:]:
        if better(result["score"], best["score"]) or (
                result["score"] == best["score"] and
                result["selected"] < best["selected"]):
            best = result
    assert best["score"] == (2221, 310)
    selected = [lookup[key] for key in best["selected"]]
    shared = {point for a, b in combinations(selected, 2)
              if a["key"][0] != b["key"][0] for point in a["new"] & b["new"]}
    assert shared == {roots[i] for i in (9, 57, 64)}
    summaries = [{"anchor_root": item["key"][0], "partner_root": item["key"][1],
                  "residual_side": item["key"][2], "endpoint_point": list(item["point"]),
                  "complete_cubics": item["complete"], "local_rank": item["local_rank"],
                  "new_exact_support_count": len(item["new"])}
                 for item in selected]
    pool_summary = [{"key": list(item["key"]), "endpoint_point": list(item["point"]),
                     "complete_cubics": item["complete"], "local_rank": item["local_rank"],
                     "new_exact_support_count": len(item["new"])}
                    for item in items]
    output = {
        "status": "verified_overlap_anchor_field_screen", "candidate_id": None,
        "source_sha256": sha("screen_overlap_anchor.py"),
        "input_sha256": {name: sha(name) for name in
                         (source_name, plane_name, prior_name,
                          "screen_residual_point_reuse.py")},
        "anchors": list(ANCHORS), "endpoints_per_anchor": 8,
        "selection_rule": "Force keys (0,9,1),(3,57,1),(3,64,0),(6,64,1),(9,57,1); fill from first/ratio/rank/size and 24 seeded random orders, then best strictly improving single-key swaps by exact-point columns/rank; seed 20261027; tie by columns, rank, and lexicographic keys.",
        "random_seed": SEED, "random_starts": RANDOM_STARTS,
        "forced_keys": [list(key) for key in sorted(FORCED)],
        "candidate_pool": pool_summary, "eligible_candidates": len(items),
        "distinct_candidate_endpoints": len(unique_items),
        "candidate_line_status_counts": dict(sorted(status_counts.items())),
        "full_pool_rows": len(full_rows), "full_pool_rank": full_rank,
        "full_pool_sum_local_rank": sum(item["local_rank"] for item in unique_items),
        "cross_anchor_possible_new_root_indices": [0, 9, 57, 64],
        "conflicting_keys_with_equal_endpoint": [[0, 9, 1], [9, 0, 1]],
        "max_distinct_cross_anchor_new_points": 3,
        "selected_cross_anchor_new_root_indices": [9, 57, 64],
        "search_start_scores": [{"kind": row["kind"], "columns": row["score"][0],
                                 "rank": row["score"][1], "swap_steps": row["steps"]}
                                for row in starts],
        "selected_field_columns": best["score"][0],
        "selected_predicted_rank": best["score"][1],
        "selected_endpoints": summaries,
    }
    (HERE / "type_i_overlap_anchor_field_screen.json").write_text(
        json.dumps(output, indent=2, sort_keys=True) + "\n")
    print("OVERLAP_ANCHOR_FIELD_SCREEN_PASS", json.dumps({
        "eligible": len(items), "unique": len(unique_items),
        "full_pool_rank": full_rank, "selected_columns": best["score"][0],
        "selected_rank": best["score"][1],
        "selected_shared_roots": [9, 57, 64]}, sort_keys=True))


if __name__ == "__main__":
    main()
