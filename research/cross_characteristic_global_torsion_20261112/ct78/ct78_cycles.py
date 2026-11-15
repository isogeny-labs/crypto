#!/usr/bin/env python3
"""CT78: Frobenius cycle partitions on nonzero odd torsion, by enumeration."""
from collections import Counter
import json

PRIMES = (3, 5, 7, 11, 13)
TRACES = (-2, -1, 0, 1, 2)
WINNERS = {3: (0, 1, 2, ((1, 2), (2, 3))),
           5: (-2, 1, 4, ((1, 4), (4, 5))),
           7: (1, 3, 21, ((3, 2), (21, 2))),
           11: (-1, 5, 10, ((5, 2), (10, 11))),
           13: (2, 3, 12, ((3, 4), (12, 13)))}


def check(condition, error):
    if not condition:
        raise ArithmeticError(error)


def frobenius(vector, trace, ell):
    x, y = vector
    return ((-2 * y) % ell, (x + trace * y) % ell)


def record(ell, trace):
    unseen = {(x, y) for x in range(ell) for y in range(ell)}
    unseen.remove((0, 0))
    cycles = Counter()
    while unseen:
        start = min(unseen)
        v = start
        length = 0
        while True:
            check(v in unseen, "orbit overlap / unexpected cycle")
            unseen.remove(v)
            length += 1
            v = frobenius(v, trace, ell)
            if v == start:
                break
        cycles[length] += 1
    full = 1
    for length in cycles:
        # Pure-integer LCM; independent JS uses matrix period instead.
        from math import lcm
        full = lcm(full, length)
    some = min(cycles)
    fixed = sum(degree * count for degree, count in cycles.items() if some % degree == 0)
    check(sum(degree * count for degree, count in cycles.items()) == ell**2-1, "point partition")
    check(fixed in (ell-1, ell**2-1), "unexpected nonzero fixed subspace")
    check(frobenius((0, 0), trace, ell) == (0, 0), "zero not fixed")
    return {
        "cycles": [[degree, cycles[degree]] for degree in sorted(cycles)],
        "ell": ell, "f_full": full, "f_some": some,
        "fixed_at_some": fixed, "total_nonzero": ell**2-1,
        "trace": trace, "type": "cycle",
    }


def main():
    for ell in PRIMES:
        rs = [record(ell, trace) for trace in TRACES]
        for r in rs:
            print(json.dumps(r, sort_keys=True, separators=(",", ":")))
        best = min(rs, key=lambda r: (r["f_some"], r["trace"], r["f_full"]))
        t, f, full, cycles = WINNERS[ell]
        check((best["trace"], best["f_some"], best["f_full"],
               tuple(map(tuple, best["cycles"]))) == (t, f, full, cycles),
              "CT77 winner data changed")
        check(best["fixed_at_some"] == ell-1, "winner fixed-space not rank one")
        print(json.dumps({"ell":ell,"trace":t,"f_some":f,"f_full":full,
                          "fixed_at_some":best["fixed_at_some"],
                          "total_nonzero":ell**2-1,
                          "cycles":best["cycles"],"type":"winner"},
                         sort_keys=True,separators=(",", ":")))


if __name__ == "__main__":
    main()
