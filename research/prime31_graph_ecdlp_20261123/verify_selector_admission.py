"""Independent public-input checks for the B677 RR5 selector hypotheses.

The result certifies selector admission conditions. It does not run an
algebraic extractor or measure relation-finding cost.
"""

import hashlib
import json
from math import isqrt

from flint import nmod_poly

from verify_mitm_graph import A, B, G, N, P, Q, ROOT, scalar


SOURCE = ROOT / "evidence/rr5-b677-prerequisite-01/structured-positive.stdout"


def is_prime(n):
    if n < 2:
        return False
    for d in range(2, isqrt(n) + 1):
        if n % d == 0:
            return False
    return True


def roots_and_lifts(poly):
    roots = poly.roots()
    assert len(roots) == poly.degree()
    assert all(multiplicity == 1 for _, multiplicity in roots)
    assert poly.gcd(poly.derivative()).degree() == 0
    xs = {int(root) for root, _ in roots}
    assert len(xs) == poly.degree()
    liftable = set()
    for x in xs:
        value = (x * x * x + A * x + B) % P
        assert value != 0
        if pow(value, (P - 1) // 2, P) == 1:
            liftable.add(x)
    return xs, liftable


def main():
    source = SOURCE.read_bytes()
    fixture = json.loads(next(
        line.split(" ", 1)[1] for line in source.decode().splitlines()
        if line.startswith("B677_INPUT ")
    ))
    assert fixture["curve"] == {
        "a": A, "b": B, "g": list(G), "n": N, "p": P,
    }
    assert is_prime(N) and N % 2 == 1 and G is not None
    assert (G[1] * G[1] - G[0] * G[0] * G[0] - A * G[0] - B) % P == 0
    assert scalar(N, G) is None
    trace = P + 1 - N
    assert trace * trace <= 4 * P
    assert 2 * N > P + 1 and (2 * N - P - 1) ** 2 > 4 * P

    x = nmod_poly([0, 1], P)
    dickson = x
    for _ in range(10):
        dickson = dickson * dickson - 2
    dickson -= fixture["dickson_terminal"]
    y7 = nmod_poly([1], P)
    f7 = (x * x * x + A * x + B) ** 7
    for square in fixture["public_coset_squares"]:
        y7 *= f7 - square
    assert (dickson.degree(), y7.degree(), dickson.gcd(y7).degree()) == (1024, 168, 0)
    assert dickson[dickson.degree()] == y7[y7.degree()] == 1
    d_roots, d_lifts = roots_and_lifts(dickson)
    y_roots, y_lifts = roots_and_lifts(y7)
    assert len(d_lifts) == 509 and len(y_lifts) == 168
    assert y_roots == y_lifts and d_roots.isdisjoint(y_roots)

    base = [tuple(point) for point in fixture["public_base"]]
    base_x = {point[0] for point in base}
    assert len(base) == len(base_x) == 677
    assert all(y != 0 and (y * y - x0 * x0 * x0 - A * x0 - B) % P == 0
               for x0, y in base)
    assert base_x == d_lifts | y_lifts
    targets = {
        "positive": tuple(fixture["public_target"]),
        "public_Q": Q,
    }
    for point in targets.values():
        x0, y = point
        assert y != 0 and (y * y - x0 * x0 * x0 - A * x0 - B) % P == 0
        assert x0 not in d_roots and x0 not in y_roots

    print(json.dumps({
        "fixture_sha256": hashlib.sha256(source).hexdigest(),
        "curve": fixture["curve"],
        "full_odd_curve_order": N,
        "dickson": {"degree": 1024, "simple_field_roots": len(d_roots),
                    "liftable_roots": len(d_lifts), "nonliftable_roots": len(d_roots - d_lifts)},
        "y7": {"degree": 168, "simple_field_roots": len(y_roots),
               "liftable_roots": len(y_lifts)},
        "selector_gcd_degree": 0,
        "fixture_equals_liftable_selector": True,
        "targets_outside_selector": list(targets),
        "scope": "public selector admission only; no extraction or cost result",
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
