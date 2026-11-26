"""Full small-coset y-product on the rational degree-5 P-192 endpoint.

Run one marking per process, for example:
  sage -python -u research/p192_ray_character_20261123/horizontal_short_coset_screen.sage \
    --m 19112 --scalar 2

The output is a raw JSON record. Scalar labels are public known-multiple
controls. The sign computation itself uses only the endpoint point.
"""

import argparse
import hashlib
import json
import platform
import sys
import time
from pathlib import Path

from sage.all import EllipticCurve, GF

P = 6277101735386680763835789423207666416083908700390324961279
R = 6277101735386680763835789423176059013767194773182842284081
E = EllipticCurve(GF(P), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1])
G = E(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)


class EDSWindow:
    """Compute a division-polynomial term with a bounded recursion window."""

    def __init__(self, point):
        x, y = int(point[0]), int(point[1])
        a, b = int(point.curve().a4()), int(point.curve().a6())
        x2 = x * x % P
        x3 = x2 * x % P
        x4 = x2 * x2 % P
        x6 = x3 * x3 % P
        self.inverse_two_y = pow(2 * y % P, -1, P)
        self.small = [
            0, 1, 2 * y % P,
            (3 * x4 + 6 * a * x2 + 12 * b * x - a * a) % P,
            4 * y * (x6 + 5 * a * x4 + 20 * b * x3 - 5 * a * a * x2
                     - 4 * a * b * x - 8 * b * b - a * a * a) % P,
        ]
        for j in range(5, 13):
            self.small.append(self._from_values(j, self.small, 0))

    def _from_values(self, j, values, first):
        get = lambda index: values[index - first]
        q = j // 2
        if j & 1:
            return (get(q + 2) * pow(get(q), 3, P)
                    - get(q - 1) * pow(get(q + 1), 3, P)) % P
        return (get(q) * (get(q + 2) * pow(get(q - 1), 2, P)
                          - get(q - 2) * pow(get(q + 1), 2, P))
                * self.inverse_two_y) % P

    def _window(self, center):
        first = max(0, center - 4)
        if center <= 8:
            return first, self.small[first:center + 5]
        parent_first, parent = self._window(center // 2)
        return first, [self._from_values(j, parent, parent_first)
                       for j in range(first, center + 5)]

    def psi(self, index):
        assert 1 <= index < R
        if index < len(self.small):
            return self.small[index]
        first, values = self._window(index)
        return values[index - first]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--m", type=int, choices=(40, 19112, 95560), required=True)
    parser.add_argument("--scalar", type=int, required=True)
    parser.add_argument("--start", type=int, default=0, help="first sorted representative index")
    parser.add_argument("--stop", type=int, help="exclusive final sorted representative index")
    args = parser.parse_args()
    assert 1 <= args.scalar < R
    n = (R - 1) // 2
    assert n % args.m == 0 and (n // args.m) % 2 == 1
    h = pow(3, n // args.m, R)
    assert pow(h, args.m, R) == R - 1
    representatives = sorted({min(pow(h, j, R), R - pow(h, j, R)) for j in range(args.m)})
    assert len(representatives) == args.m
    stop = args.m if args.stop is None else args.stop
    assert 0 <= args.start < stop <= args.m
    phi = E.isogenies_prime_degree(5)[0]
    endpoint = phi(G)
    assert R * endpoint == endpoint.curve()(0)
    marking = args.scalar * endpoint
    window = EDSWindow(marking)
    product = 1
    start_wall, start_cpu = time.perf_counter(), time.process_time()
    for i, j in enumerate(representatives[args.start:stop]):
        term = window.psi(2 * j)
        if i < 4:
            assert term * pow(window.psi(j), -4, P) % P == 2 * int((j * marking)[1]) % P
        product = product * term % P
        if (i + 1) % 10000 == 0:
            print(f"progress scalar={args.scalar} m={args.m} factors={i+1}", file=sys.stderr, flush=True)
    residue = pow(product, (P - 1) // 2, P)
    assert residue in (1, P - 1)
    source = Path(__file__)
    print(json.dumps({
        "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
        "host": platform.platform(),
        "m": args.m, "scalar": args.scalar,
        "start": args.start, "stop": stop, "factor_count": stop - args.start,
        "degree": 5, "endpoint_j": str(endpoint.curve().j_invariant()),
        "representative_count": len(representatives),
        "direct_eds_checks": 4,
        "product_sign": 1 if residue == 1 else -1,
        "wall_s": time.perf_counter() - start_wall,
        "cpu_s": time.process_time() - start_cpu,
    }, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
