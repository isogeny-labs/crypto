"""Freeze a cross-cubic anchor fan using exact quartic cuts over F_p.

For an anchored root A and a partner root B in another query cubic, the
split line A+B+R+T=S gives a certified cut from B at endpoint R with A as
one residual.  This screen chooses eight endpoints whose partner-cubic
bundle is complete and whose endpoint contributes at least one row.
"""

import hashlib
import json
from collections import Counter
from pathlib import Path

from screen_residual_point_reuse import quartic_cuts

HERE = Path(__file__).resolve().parent
ANCHOR = 0
SELECT = 8


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
    roots = [tuple(map(int, point)) for point in source["endpoint_points"]]
    cut = quartic_cuts(p, plane["quartic_coefficients"], roots)
    parent_edges = sorted((edge for edge in source["edges"]
                           if edge["query_index"] == ANCHOR // 3
                           and edge["root_index"] == ANCHOR % 3
                           and edge["selected_endpoint"] // 3 != ANCHOR // 3),
                          key=lambda edge: edge["selected_endpoint"])
    assert len({edge["selected_endpoint"] for edge in parent_edges}) == len(parent_edges)
    candidate_rows = []
    counts = Counter()
    for edge in parent_edges:
        partner = edge["selected_endpoint"]
        residuals = [tuple(map(int, point)) for point in edge["residual_points"]]
        for side, endpoint in enumerate(residuals):
            other_residual = residuals[1 - side]
            edge_points = {}
            for root_index, root in enumerate(roots):
                if root_index == ANCHOR:
                    status, outputs = "certified_parent_line", tuple(sorted((roots[partner], other_residual)))
                elif root_index == partner:
                    status, outputs = "certified_parent_line", tuple(sorted((roots[ANCHOR], other_residual)))
                else:
                    status, outputs = cut(root_index, endpoint)
                counts[status] += 1
                if outputs is not None:
                    edge_points[root_index] = outputs
            complete = [i for i in range(27)
                        if all(3 * i + root in edge_points for root in range(3))]
            active = len(complete) >= 2
            parent_complete = partner // 3 in complete
            anchor_hit = active and parent_complete
            assert not anchor_hit or roots[ANCHOR] in set(
                point for root in range(3)
                for point in edge_points[3 * (partner // 3) + root])
            candidate_rows.append({
                "partner_root": partner, "residual_side": side,
                "endpoint_point": list(endpoint),
                "complete_cubics": complete,
                "active": active, "partner_bundle_complete": parent_complete,
                "anchor_point_in_partner_bundle": anchor_hit})
    assert sum(counts.values()) == 81 * len(candidate_rows)
    eligible = [row for row in candidate_rows
                if row["active"] and row["partner_bundle_complete"]]
    selected = eligible[:SELECT]
    assert len(selected) == SELECT
    assert len({tuple(row["endpoint_point"]) for row in selected}) == SELECT
    output = {
        "status": "verified_cross_cubic_anchor_screen",
        "boundary": "Exact plane-quartic split and point recurrence; quotient projection and subgroup equations are checked by the Sage producer and integer-coordinate replay.",
        "source_sha256": sha("screen_cross_cubic_anchor.py"),
        "input_sha256": {name: sha(name) for name in (source_name, plane_name,
                                                       "screen_residual_point_reuse.py")},
        "anchor_root": ANCHOR,
        "selection_rule": "First eight lexicographic partner-root/residual-side endpoints with at least two complete cubics, including the partner cubic.",
        "parent_cross_cubic_lines": len(parent_edges),
        "candidate_residual_endpoints": len(candidate_rows),
        "line_status_counts": dict(sorted(counts.items())),
        "eligible_endpoints": len(eligible),
        "selected_endpoints": selected,
        "candidate_rows": candidate_rows,
    }
    path = HERE / "type_i_cross_cubic_anchor_screen.json"
    path.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")
    print("CROSS_CUBIC_ANCHOR_SCREEN_PASS", json.dumps({key: output[key]
          for key in ("parent_cross_cubic_lines", "candidate_residual_endpoints",
                      "eligible_endpoints", "line_status_counts", "selected_endpoints")},
          sort_keys=True))


if __name__ == "__main__":
    main()
