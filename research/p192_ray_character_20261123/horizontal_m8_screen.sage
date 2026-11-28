"""Screen small horizontal-isogeny endpoints for a P-192 m=8 sign.

Run from the repository root:
  sage -python -u research/p192_ray_character_20261123/horizontal_m8_screen.sage \
    > research/p192_ray_character_20261123/HORIZONTAL_M8_SCREEN.jsonl

The first sample is diagnostic; the second uses a fresh seed as a holdout.
All scalar labels below are public known-multiple controls.
"""

import hashlib
import json
import platform
import random
import time
from pathlib import Path

from sage.all import EllipticCurve, GF, kronecker

P = 6277101735386680763835789423207666416083908700390324961279
R = 6277101735386680763835789423176059013767194773182842284081
N = (R - 1) // 2
M = 8
H = pow(3, N // M, R)
assert pow(H, M, R) == R - 1
REPS = sorted({min(pow(H, j, R), R - pow(H, j, R)) for j in range(M)})
assert len(REPS) == M

E = EllipticCurve(GF(P), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1])
G = E(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert R * G == E(0)


def field_sign(value):
    residue = pow(int(value), (P - 1) // 2, P)
    assert residue in (1, P - 1)
    return 1 if residue == 1 else -1


def y_product_sign(points):
    product = GF(P)(1)
    for point in points:
        product *= point[1]
    return field_sign(product)


def doubling_sextic(x, a, b):
    f = x**3 + a * x + b
    derivative = 3 * x * x + a
    return 12 * x * f * derivative - derivative**3 - 8 * f * f


def edge_rows(ell):
    phis = sorted(E.isogenies_prime_degree(ell), key=lambda phi: int(phi.codomain().j_invariant()))
    for index, phi in enumerate(phis):
        target = phi(G)
        curve = target.curve()
        assert R * target == curve(0)
        assert phi(2 * G) == 2 * target and phi(3 * G) == 3 * target
        points = [int(rep) * target for rep in REPS]
        base_sign = y_product_sign(points)
        norm = GF(P)(1)
        for point in points:
            norm *= doubling_sextic(point[0], curve.a4(), curve.a6())
        assert field_sign(norm) == base_sign * y_product_sign([2 * point for point in points])
        edge = {"degree": ell, "index": index, "codomain_j": str(curve.j_invariant()),
                "base_sign": base_sign, "doubling_norm_sign": field_sign(norm)}
        yield phi, points, edge


def mark_row(points, base_sign, scalar, panel):
    observed = base_sign * y_product_sign([scalar * point for point in points])
    expected = int(kronecker(scalar, R))
    return {"panel": panel, "scalar": str(scalar), "feature_sign": observed,
            "scalar_sign": expected, "agree": observed == expected}


def emit(row):
    print(json.dumps(row, sort_keys=True), flush=True)


def main():
    source = Path(__file__)
    emit({"metadata": {"source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
                       "host": platform.platform(), "sage_version": __import__("sage.version", fromlist=["version"]).version,
                       "curve_prime": str(P), "subgroup_order": str(R), "m": M,
                       "reps": [str(v) for v in REPS],
                       "selection_marks": [1, 2, 3, 5, 7, 11, 13],
                       "disjoint_witness_marks": [4, 10, 14, 15, 17, 19, 23, 29, 31],
                       "diagnostic": {"seed": 20261127, "count": 1024},
                       "holdout": {"seed": 20261128, "count": 4096},
                       "direct_point_only_checks": 8}})
    degree5 = None
    for ell in (5, 11, 13):
        for phi, points, edge in edge_rows(ell):
            emit({"edge": edge})
            for scalar in (1, 2, 3, 5, 7, 11, 13):
                emit({"edge": edge, "mark": mark_row(points, edge["base_sign"], scalar, "selection")})
            if ell == 5:
                for scalar in (4, 10, 14, 15, 17, 19, 23, 29, 31):
                    emit({"edge": edge, "mark": mark_row(points, edge["base_sign"], scalar, "disjoint_witness")})
                curve = points[0].curve()
                norm_at_two = GF(P)(1)
                for point in points:
                    norm_at_two *= doubling_sextic((2 * point)[0], curve.a4(), curve.a6())
                feature_two = mark_row(points, edge["base_sign"], 2, "doubling_witness")["feature_sign"]
                feature_four = mark_row(points, edge["base_sign"], 4, "doubling_witness")["feature_sign"]
                assert field_sign(norm_at_two) == feature_two * feature_four == -1
                emit({"doubling_witness": {"edge": edge, "marking_scalar": 2,
                                            "norm_sign": field_sign(norm_at_two),
                                            "feature_two": feature_two, "feature_four": feature_four}})
                degree5 = (phi, points, edge)
    assert degree5 is not None
    phi, points, edge = degree5
    for panel, seed, count in (("diagnostic", 20261127, 1024), ("holdout", 20261128, 4096)):
        rng = random.Random(seed)
        start_wall = time.perf_counter()
        start_cpu = time.process_time()
        agreements = 0
        for index in range(count):
            scalar = rng.randrange(1, R)
            row = mark_row(points, edge["base_sign"], scalar, panel)
            if panel == "holdout" and index < 8:
                public_point = scalar * G
                image = phi(public_point)
                direct_points = [int(rep) * image for rep in REPS]
                assert direct_points == [scalar * point for point in points]
                assert edge["base_sign"] * y_product_sign(direct_points) == row["feature_sign"]
            agreements += row["agree"]
            emit({"edge": edge, "mark": row})
        emit({"summary": {"panel": panel, "seed": seed, "count": count,
                          "agreements": agreements, "correlation": (2 * agreements - count) / count,
                          "wall_s": time.perf_counter() - start_wall,
                          "cpu_s": time.process_time() - start_cpu}})


if __name__ == "__main__":
    main()
