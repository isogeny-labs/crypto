"""Freeze four cross-cubic anchor fans using exact F_p quartic cuts.

The anchors are the first roots of four distinct frozen query cubics.
For each anchor, select the first eight globally distinct residual endpoints
whose partner-cubic bundle is complete and whose endpoint has another
complete cubic, hence an endpointwise known-scalar relation.
"""

import hashlib
import json
from collections import Counter
from pathlib import Path

from screen_residual_point_reuse import quartic_cuts

HERE = Path(__file__).resolve().parent
ANCHORS = (0, 3, 6, 9)
ENDPOINTS_PER_ANCHOR = 8


def load(name):
    return json.loads((HERE / name).read_text())


def sha(name):
    return hashlib.sha256((HERE / name).read_bytes()).hexdigest()


def main():
    source_name = "type_i_known_scalar_query_cross_fan.json"
    plane_name = "type_i_plane_quartic.json"
    source, plane = load(source_name), load(plane_name)
    p = int(source["field_prime"])
    assert p == int(plane["p"]) and p % 4 == 3
    roots = [tuple(map(int, value)) for value in source["endpoint_points"]]
    assert len(roots) == 81 and len({anchor // 3 for anchor in ANCHORS}) == 4
    cut = quartic_cuts(p, plane["quartic_coefficients"], roots)
    used = set(roots)
    selected = []
    anchor_rows = []
    total_status = Counter()
    for anchor in ANCHORS:
        parent_edges = sorted(
            (edge for edge in source["edges"]
             if 3 * edge["query_index"] + edge["root_index"] == anchor
             and edge["selected_endpoint"] // 3 != anchor // 3),
            key=lambda edge: edge["selected_endpoint"])
        assert len({edge["selected_endpoint"] for edge in parent_edges}) == len(parent_edges)
        rows = []
        statuses = Counter()
        for edge in parent_edges:
            partner = edge["selected_endpoint"]
            residuals = [tuple(map(int, point)) for point in edge["residual_points"]]
            for side, endpoint in enumerate(residuals):
                other_residual = residuals[1 - side]
                edges = {}
                for root_index in range(81):
                    if root_index == anchor:
                        status = "certified_parent_line"
                        outputs = tuple(sorted((roots[partner], other_residual)))
                    elif root_index == partner:
                        status = "certified_parent_line"
                        outputs = tuple(sorted((roots[anchor], other_residual)))
                    else:
                        status, outputs = cut(root_index, endpoint)
                    statuses[status] += 1
                    if outputs is not None:
                        edges[root_index] = outputs
                complete = [i for i in range(27)
                            if all(3 * i + root in edges for root in range(3))]
                eligible = len(complete) >= 2 and partner // 3 in complete
                if eligible:
                    assert roots[anchor] in {
                        point for root in range(3)
                        for point in edges[3 * (partner // 3) + root]}
                row = {"anchor_root": anchor, "partner_root": partner,
                       "residual_side": side, "endpoint_point": list(endpoint),
                       "complete_cubics": complete, "eligible": eligible}
                rows.append(row)
        assert sum(statuses.values()) == len(rows) * 81
        total_status.update(statuses)
        eligible_count = sum(row["eligible"] for row in rows)
        taken = []
        for row in rows:
            point = tuple(row["endpoint_point"])
            if not row["eligible"] or point in used:
                continue
            taken.append(row)
            used.add(point)
            if len(taken) == ENDPOINTS_PER_ANCHOR:
                break
        assert len(taken) == ENDPOINTS_PER_ANCHOR
        selected.extend(taken)
        anchor_rows.append({"anchor_root": anchor,
                            "parent_cross_cubic_lines": len(parent_edges),
                            "candidate_residual_endpoints": len(rows),
                            "eligible_endpoints": eligible_count,
                            "line_status_counts": dict(sorted(statuses.items())),
                            "selected_partner_sides": [
                                [row["partner_root"], row["residual_side"]]
                                for row in taken]})
    assert len(selected) == 32 and len(used) == 81 + 32
    output = {
        "status": "verified_four_anchor_field_screen",
        "candidate_id": None,
        "source_sha256": sha("screen_multi_anchor.py"),
        "input_sha256": {name: sha(name) for name in
                         (source_name, plane_name, "screen_residual_point_reuse.py")},
        "anchors": list(ANCHORS),
        "selection_rule": "For anchors 0,3,6,9 in order, choose the first eight lexicographic partner-root/residual-side eligible endpoints not already selected; eligibility requires a complete partner cubic and at least one other complete cubic.",
        "endpoints_per_anchor": ENDPOINTS_PER_ANCHOR,
        "selected_endpoints": selected,
        "anchor_summaries": anchor_rows,
        "line_status_counts": dict(sorted(total_status.items())),
    }
    (HERE / "type_i_four_anchor_field_screen.json").write_text(
        json.dumps(output, indent=2, sort_keys=True) + "\n")
    print("FOUR_ANCHOR_FIELD_SCREEN_PASS", json.dumps({
        "eligible_counts": [row["eligible_endpoints"] for row in anchor_rows],
        "selected_partner_sides": [row["selected_partner_sides"] for row in anchor_rows],
        "line_status_counts": output["line_status_counts"]}, sort_keys=True))


if __name__ == "__main__":
    main()
