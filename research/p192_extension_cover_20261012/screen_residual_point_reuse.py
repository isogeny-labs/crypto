"""Screen exact quartic-point recurrence across all 86 residual endpoints.

This base-field screen has no quotient projection.  Equal residual points
necessarily give equal projected columns, so its degrees are certified lower
bounds on projected-column degrees for the same selected bundles.
"""

import hashlib
import json
from collections import Counter, defaultdict
from math import comb
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load(name):
    return json.loads((HERE / name).read_text())


def digest(name):
    return hashlib.sha256((HERE / name).read_bytes()).hexdigest()


def quartic_cuts(prime, coefficients, roots):
    terms = [(tuple(map(int, key.split(",")[:2])), int(value) % prime)
             for key, value in coefficients.items()]
    taylor = {}
    for root_index, (x, y) in enumerate(roots):
        xp = [pow(x, exponent, prime) for exponent in range(5)]
        yp = [pow(y, exponent, prime) for exponent in range(5)]
        table = [[0] * (order + 1) for order in range(5)]
        for (a, b), value in terms:
            for ix in range(a + 1):
                for iy in range(b + 1):
                    order = ix + iy
                    table[order][iy] = (table[order][iy] + value * comb(a, ix) *
                                        comb(b, iy) * xp[a - ix] * yp[b - iy]) % prime
        assert table[0][0] == 0
        taylor[root_index] = table

    def evaluate(coefficients, slope):
        value = 0
        for coefficient in reversed(coefficients):
            value = (value * slope + coefficient) % prime
        return value

    def cut(root_index, point):
        x, y = roots[root_index]
        d = (point[0] - x) % prime
        if not d:
            return "vertical_or_coincident", None
        slope = (point[1] - y) * pow(d, -1, prime) % prime
        c1, c2, c3, a = (evaluate(taylor[root_index][order], slope)
                         for order in range(1, 5))
        if not a:
            return "degree_below_four", None
        b = (c3 + d * a) % prime
        c = (c2 + d * b) % prime
        if not c or (c1 + d * c) % prime:
            return "tangent_or_invalid", None
        discriminant = (b * b - 4 * a * c) % prime
        if not discriminant:
            return "double_residual", None
        if pow(discriminant, (prime - 1) // 2, prime) != 1:
            return "nonsplit", None
        square_root = pow(discriminant, (prime + 1) // 4, prime)
        denominator = pow(2 * a, -1, prime)
        shifts = [(-b + sign * square_root) * denominator % prime
                  for sign in (1, -1)]
        residuals = tuple(sorted(((x + shift) % prime,
                                  (y + slope * shift) % prime)
                                 for shift in shifts))
        assert len(set(residuals)) == 2
        return "split", residuals

    return cut


def main():
    source_name = "type_i_known_scalar_query_cross_fan.json"
    plane_name = "type_i_plane_quartic.json"
    screen_name = "type_i_same_cubic_residual_third_cut_screen.json"
    source, plane, screen = (load(name) for name in (source_name, plane_name,
                                                      screen_name))
    prime = int(source["field_prime"])
    assert prime == int(plane["p"]) and prime % 4 == 3
    roots = [tuple(map(int, point)) for point in source["endpoint_points"]]
    assert len(roots) == 81 and len(screen["endpoint_rows"]) == 86
    cut = quartic_cuts(prime, plane["quartic_coefficients"], roots)
    baseline_edges = {}
    for edge in source["edges"]:
        key = edge["query_index"], edge["root_index"], edge["selected_endpoint"]
        baseline_edges[key] = tuple(sorted(tuple(map(int, point))
                                           for point in edge["residual_points"]))

    same = {}
    for edge in source["edges"]:
        i, a, u = edge["query_index"], edge["root_index"], edge["selected_endpoint"]
        if u // 3 == i and a != u % 3:
            same.setdefault((i, *sorted((a, u % 3))), edge)
    assert len(same) == 43
    edges = dict(baseline_edges)
    counts = Counter()
    endpoint_points = {}
    for row in screen["endpoint_rows"]:
        u, i = row["endpoint"], row["parent_cubic"]
        a, b = row["parent_roots"]
        side = row["parent_residual_side"]
        point = tuple(map(int, row["residual_point"]))
        endpoint_points[u] = point
        parent_edge = same[i, a, b]
        residuals = [tuple(map(int, value)) for value in parent_edge["residual_points"]]
        assert point == residuals[side]
        other_residual = residuals[1 - side]
        for root, other in ((a, b), (b, a)):
            edges[i, root, u] = tuple(sorted((roots[3 * i + other], other_residual)))
            counts["certified_parent_line"] += 1
        for root_index in range(81):
            key = root_index // 3, root_index % 3, u
            if key in edges:
                continue
            status, residual_points = cut(root_index, point)
            counts[status] += 1
            if residual_points is not None:
                edges[key] = residual_points
        assert sum(counts.values()) == (u - 80) * 81
    assert sorted(endpoint_points) == list(range(81, 167))

    bundles = {}
    by_endpoint = defaultdict(list)
    for u in range(167):
        for i in range(27):
            keys = [(i, root, u) for root in range(3)]
            if not all(key in edges for key in keys):
                continue
            values = tuple(point for key in keys for point in edges[key])
            bundles[i, u] = values
            by_endpoint[u].append(i)
    active = {key: values for key, values in bundles.items()
              if len(by_endpoint[key[1]]) >= 2}
    degree = Counter(point for values in active.values() for point in set(values))
    appearance = Counter(point for values in active.values() for point in values)
    top_degree = max(degree.values())
    highest = sorted(point for point, value in degree.items() if value == top_degree)
    endpoint_degree = Counter(u for (i, u), values in active.items()
                              for point in set(values) if point in highest)
    output = {
        "status": "verified_residual_point_reuse_screen",
        "boundary": "Exact equal plane-quartic points imply equal projected columns; projection may identify more points.",
        "source_sha256": digest("screen_residual_point_reuse.py"),
        "input_sha256": {name: digest(name) for name in
                         (source_name, plane_name, screen_name)},
        "candidate_endpoints": len(endpoint_points),
        "candidate_lines": len(endpoint_points) * len(roots),
        "line_status_counts": dict(sorted(counts.items())),
        "complete_bundles": len(bundles),
        "active_bundles": len(active),
        "active_endpoints": sum(len(items) >= 2 for items in by_endpoint.values()),
        "active_distinct_points": len(degree),
        "max_point_bundle_degree": top_degree,
        "max_point_appearance_count": max(appearance.values()),
        "point_degree_histogram": dict(sorted((str(k), value) for k, value
                                               in Counter(degree.values()).items())),
        "highest_degree_points": [
            {"point": list(point), "degree": degree[point],
             "bundle_keys": [list(key) for key, values in sorted(active.items())
                             if point in values]}
            for point in highest],
        "highest_point_endpoint_counts": dict(sorted((str(u), count)
                                                     for u, count in endpoint_degree.items())),
        "endpoint_rows": [
            {"endpoint": u, "point": list(endpoint_points[u]),
             "complete_cubics": by_endpoint[u]}
            for u in sorted(endpoint_points)],
    }
    path = HERE / "type_i_residual_point_reuse_screen.json"
    path.write_text(json.dumps(output, sort_keys=True, indent=2) + "\n")
    print("RESIDUAL_POINT_REUSE_SCREEN_PASS", json.dumps({name: output[name]
          for name in ("line_status_counts", "complete_bundles", "active_bundles",
                       "active_distinct_points", "max_point_bundle_degree",
                       "highest_point_endpoint_counts")}, sort_keys=True))


if __name__ == "__main__":
    main()
