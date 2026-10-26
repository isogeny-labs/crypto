"""Exact base-field split screen for every same-cubic residual endpoint.

Each selected endpoint already has two certified parent-cubic cuts. This
screen checks the remaining third-root cut without computing quotient images,
then identifies roots that could recur in at least three complete bundles.
"""

import hashlib
import json
from collections import Counter, defaultdict
from math import comb
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((HERE / name).read_text())


def main():
    source = load("type_i_known_scalar_query_cross_fan.json")
    plane = load("type_i_plane_quartic.json")
    pilot = load("type_i_residual_endpoint_clique_pilot.json")
    p = int(source["field_prime"])
    assert p == int(plane["p"]) and p % 4 == 3
    terms = [(tuple(map(int, label.split(",")[:2])), int(value) % p)
             for label, value in plane["quartic_coefficients"].items()]

    def taylor_at(x, y):
        xp, yp = [pow(x, i, p) for i in range(5)], [pow(y, i, p) for i in range(5)]
        out = [[0] * (k + 1) for k in range(5)]
        for (a, b), value in terms:
            for ix in range(a + 1):
                for iy in range(b + 1):
                    out[ix + iy][iy] = (out[ix + iy][iy] + value * comb(a, ix) *
                                      comb(b, iy) * xp[a - ix] * yp[b - iy]) % p
        assert out[0][0] == 0
        return out

    def evaluate(coefficients, z):
        value = 0
        for coefficient in reversed(coefficients):
            value = (value * z + coefficient) % p
        return value

    def status(P, R):
        d = (R[0] - P[0]) % p
        if not d:
            return "vertical_or_coincident"
        slope = (R[1] - P[1]) * pow(d, -1, p) % p
        taylor = taylor_at(*P)
        c1, c2, c3, A = (evaluate(taylor[k], slope) for k in range(1, 5))
        if not A:
            return "degree_below_four"
        B, D = (c3 + d * A) % p, (c2 + d * (c3 + d * A)) % p
        if not D or (c1 + d * D) % p:
            return "tangent_or_invalid"
        discr = (B * B - 4 * A * D) % p
        if not discr:
            return "double_residual"
        return "split" if pow(discr, (p - 1) // 2, p) == 1 else "nonsplit"

    same = {}
    for edge in source["edges"]:
        i, a, u = edge["query_index"], edge["root_index"], edge["selected_endpoint"]
        if u // 3 == i and a != u % 3:
            same.setdefault((i, *sorted((a, u % 3))), edge)
    assert len(same) == 43
    roots = [tuple(map(int, P)) for P in source["endpoint_points"]]
    rows = []
    histogram = Counter()
    forced_root_endpoints = defaultdict(list)
    for line_number, (i, a, b) in enumerate(sorted(same)):
        edge = same[i, a, b]
        c = ({0, 1, 2} - {a, b}).pop()
        for side in (0, 1):
            R = tuple(map(int, edge["residual_points"][side]))
            outcome = status(roots[3 * i + c], R)
            u = 81 + 2 * line_number + side
            if u < 113:
                present = any(e["query_index"] == i and e["root_index"] == c and
                              e["selected_endpoint"] == u for e in pilot["new_split_edges"])
                assert present == (outcome == "split")
                assert list(R) == pilot["new_endpoint_geometry"][u - 81]
            histogram[outcome] += 1
            rows.append({"endpoint": u, "parent_cubic": i, "parent_roots": [a, b],
                         "third_root": c, "parent_residual_side": side,
                         "third_cut_status": outcome,
                         "residual_point": list(R)})
            if outcome == "split":
                for root in (a, b):
                    forced_root_endpoints[3 * i + root].append(u)
    assert len(rows) == 86
    max_degree = max(map(len, forced_root_endpoints.values()))
    top = sorted(root for root, users in forced_root_endpoints.items()
                 if len(users) == max_degree)
    out = {"status": "verified_same_cubic_residual_third_cut_screen",
           "source_sha256": sha(__file__),
           "input_sha256": {name: sha(HERE / name) for name in (
               "type_i_known_scalar_query_cross_fan.json",
               "type_i_plane_quartic.json",
               "type_i_residual_endpoint_clique_pilot.json")},
           "candidate_parent_lines": len(same), "candidate_residual_endpoints": len(rows),
           "third_cut_status_counts": dict(sorted(histogram.items())),
           "eligible_parent_bundles": histogram["split"],
           "max_forced_root_incidence_in_parent_bundles": max_degree,
           "max_incidence_root_indices": top,
           "top_root_endpoint_sets": {str(root): forced_root_endpoints[root] for root in top},
           "endpoint_rows": rows}
    path = HERE / "type_i_same_cubic_residual_third_cut_screen.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("SAME_CUBIC_THIRD_CUT_PASS", json.dumps({key: out[key] for key in (
        "third_cut_status_counts", "eligible_parent_bundles",
        "max_forced_root_incidence_in_parent_bundles",
        "top_root_endpoint_sets")}, sort_keys=True))


if __name__ == "__main__":
    main()
