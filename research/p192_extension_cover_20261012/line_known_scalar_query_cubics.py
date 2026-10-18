"""Census rational quartic points in frozen known-scalar query cubics.

The query point is Q=[-2a]G for each recorded scalar a. The positive
and half-point input cubics are constructed by the checked inverse-cover
routine; rational roots are enumerated over Fp, not over its cubic extension.
"""

import hashlib
import json
import time
from collections import Counter, defaultdict
from pathlib import Path

from sage.all import GF, Integer, PolynomialRing

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()


def prelude(name):
    path = HERE / name
    source = path.read_text()
    marker = "start = time.monotonic_ns()"
    assert source.count(marker) == 1
    namespace = {"__file__": str(path), "__name__": name[:-3] + "_prelude"}
    exec(compile(source.split(marker)[0], str(path), "exec"), namespace)
    return namespace


def main():
    source_path = HERE / "type_i_line_guided_collector.json"
    guide = json.loads(source_path.read_text())
    quotient = prelude("line_quotient_cubic_probe.py")
    E, L, r, p = (quotient[name] for name in ("E", "L", "r", "p"))
    O, residual, quartic = (quotient[name] for name in ("O", "residual", "quartic"))
    G = E(L(quotient["dt"].gm.gx), L(quotient["dt"].gm.gy))
    assert r * G == O and str(p) == guide["field_prime"]
    assert str(r) == guide["subgroup_order"]
    inv2 = Integer(pow(2, -1, int(r)))
    F = GF(p)
    R0 = PolynomialRing(F, "z")
    selected_paths = (HERE / "type_i_line_base_points.tsv",
                      HERE / "type_i_line_new_points.tsv")
    selected_geometry = set()
    for index, path in enumerate(selected_paths):
        for row in path.read_text().splitlines():
            parts = list(map(int, row.split()))
            selected_geometry.add(tuple(parts[1:] if index == 0 else parts))
    assert len(selected_geometry) == 9925

    def points(encoded):
        h = R0([F(int(c[0])) for c in encoded["residual_x_polynomial"]])
        y_coeffs = [L(int(c[0])) for c in encoded["residual_y_polynomial"]]
        assert h.degree() == 3 and h.gcd(h.derivative()).degree() == 0
        rational = []
        for root, multiplicity in h.roots():
            assert multiplicity == 1
            xx = L(root)
            yy = sum(y_coeffs[i] * xx**i for i in range(len(y_coeffs)))
            assert yy**p == yy and quartic(yy)(xx) == 0
            rational.append([str(int(root)), str(int(yy.polynomial()[0]))])
        assert len(rational) in (0, 1, 3)
        return rational

    rows = []
    failures = []
    point_sources = defaultdict(list)
    count = Counter()
    start = time.monotonic_ns()
    for law, scalars in (("uniform", guide["uniform_scalars"]),
                         ("character_guided", guide["guided_scalars"])):
        assert len(scalars) == 64
        for index, encoded_scalar in enumerate(scalars):
            a = Integer(encoded_scalar)
            assert 0 < a < r
            Q = -2 * a * G
            H = inv2 * Q
            assert Q != O and H != O and 2 * H == Q
            row = {
                "law": law, "query_index": index, "scalar": str(a),
                "quotient": [str(int(Q[0].polynomial()[0])),
                             str(int(Q[1].polynomial()[0]))],
                "half_quotient": [str(int(H[0].polynomial()[0])),
                                  str(int(H[1].polynomial()[0]))],
            }
            try:
                for role, point in (("positive", Q), ("half", H)):
                    cubic = residual(point)
                    roots = points(cubic)
                    row[role] = {"cubic": cubic, "rational_points": roots,
                                 "rational_point_count": len(roots)}
                    count[f"{law}_{role}_{len(roots)}"] += 1
                    for coords in roots:
                        key = tuple(map(int, coords))
                        point_sources[key].append([law, index, role])
                row["status"] = "verified_query_cubic_pair"
            except Exception as error:
                row["status"] = "bounded_constructor_failure"
                row["error_type"] = type(error).__name__
                row["error"] = str(error)[:300]
                failures.append([law, index, row["error_type"], row["error"]])
            rows.append(row)
            if len(rows) % 16 == 0:
                print("QUERY_CUBIC_PROGRESS", len(rows), 128, flush=True)

    points_seen = set(point_sources)
    out = {
        "status": "bounded_known_scalar_query_cubic_root_census",
        "candidate_id": None,
        "field_prime": str(p), "subgroup_order": str(r),
        "queries": len(rows), "verified_pairs": len(rows) - len(failures),
        "failures": failures, "rational_root_counts": dict(sorted(count.items())),
        "distinct_rational_points": len(points_seen),
        "selected_geometry_hits": sorted([list(point) for point in
                                          points_seen & selected_geometry]),
        "cross_cubic_repeats": [
            {"point": list(point), "sources": sources}
            for point, sources in sorted(point_sources.items())
            if len(sources) > 1],
        "rows": rows,
        "elapsed_ns_unisolated": time.monotonic_ns() - start,
        "source_sha256": sha(__file__),
        "input_sha256": {path.name: sha(path) for path in
                         [source_path, *selected_paths,
                          HERE / "type_i_plane_quartic.json",
                          HERE / "line_quotient_cubic_probe.py"]},
        "runtime_sha256": sha(HERE / "type_i_known_scalar_query_cubic_runtime_info.json"),
        "boundary": "Exact rational-root eligibility of 128 frozen known-scalar query cubics and their subgroup halves. Root counts alone do not provide a relation or useful rank; query-dependent projected cuts require a separate check.",
    }
    path = HERE / "type_i_known_scalar_query_cubic_roots.json"
    path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("QUERY_CUBIC_ROOT_CENSUS", json.dumps({
        "verified_pairs": out["verified_pairs"],
        "rational_root_counts": out["rational_root_counts"],
        "distinct_rational_points": out["distinct_rational_points"],
        "selected_geometry_hits": len(out["selected_geometry_hits"]),
        "cross_cubic_repeats": len(out["cross_cubic_repeats"]),
        "elapsed_s_unisolated": round(out["elapsed_ns_unisolated"] / 1e9, 3),
    }, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
