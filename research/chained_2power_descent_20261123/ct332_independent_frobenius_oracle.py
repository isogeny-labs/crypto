#!/usr/bin/env python3
"""CT332 bounded abstract 2-adic Frobenius/Krylov regression; NOT a Jacobian solver.

Independent of the CT331 induction: cycle permutations induce the four-dimensional
F_2-module of even subsets of six roots modulo complement. A chosen integral
companion lift of the (5)(1) action is checked modulo 4 and 8. This specific
lift is an abstract control, NOT claimed to be the Frobenius of a curve.
"""
import hashlib
import itertools
import json
import sys


def pairvector(i, j):
    def v(k):
        return (1 << (k - 1)) if k <= 4 else (15 if k == 5 else 0)
    return v(i) ^ v(j)


def f_columns(cycles):
    perm = list(range(7))
    for cyc in cycles:
        for a, b in zip(cyc, cyc[1:] + cyc[:1]):
            perm[a] = b
    return [pairvector(perm[i], perm[6]) for i in range(1, 5)]


def act(cols, x):
    y = 0
    for j in range(4):
        if x >> j & 1:
            y ^= cols[j]
    return y


def rank4(vectors):
    pivots = []
    for x in vectors:
        for p in pivots:
            x = min(x, x ^ p)
        if x:
            pivots.append(x)
            pivots.sort(reverse=True)
    return len(pivots)


def krylov_rank(cols, seed):
    vs = []
    v = seed
    for _ in range(4):
        vs.append(v)
        v = act(cols, v)
    return rank4(vs)


def orbits(cols):
    pending = set(range(1, 16))
    orbit_sizes = []
    while pending:
        seed = min(pending)
        seen = []
        v = seed
        while v not in seen:
            if v == 0:
                raise AssertionError("Frobenius action not invertible")
            seen.append(v)
            v = act(cols, v)
        if v != seed:
            raise AssertionError("orbit not closed at seed")
        for v in seen:
            pending.remove(v)
        orbit_sizes.append(len(seen))
    return sorted(orbit_sizes)


def det4_mod(a, modulus):
    total = 0
    for p in itertools.permutations(range(4)):
        inversions = sum(p[i] > p[j] for i in range(4) for j in range(i + 1, 4))
        prod = 1
        for i in range(4):
            prod = (prod * a[i][p[i]]) % modulus
        total += (-1 if inversions % 2 else 1) * prod
    return total % modulus


def lift_companion_mul(v, n):
    # Integral companion of x^4+x^3+x^2+x+1; order five modulo 2^n.
    return (-(v[3]) % n,
            (v[0] - v[3]) % n,
            (v[1] - v[3]) % n,
            (v[2] - v[3]) % n)


def enumerate_primitive(n):
    primitive = 0
    odd_det = 0
    odd_residues = set()
    for v in itertools.product(range(n), repeat=4):
        if not any(t % 2 for t in v):
            continue
        primitive += 1
        vs = [v]
        for _ in range(3):
            vs.append(lift_companion_mul(vs[-1], n))
        m = [[vs[col][row] for col in range(4)] for row in range(4)]
        d = det4_mod(m, n)
        if d & 1:
            odd_det += 1
        else:
            raise AssertionError(("singular Krylov lift", n, v, d))
        odd_residues.add(d % 2)
    return {"primitive_vectors": primitive, "odd_krylov_determinants": odd_det,
            "determinant_parities": sorted(odd_residues)}


def main():
    types = {
        "5+1": [(1, 2, 3, 4, 5)],
        "3+3": [(1, 2, 3), (4, 5, 6)],
        "6": [(1, 2, 3, 4, 5, 6)],
        "identity_mutant": [],
    }
    controls = {}
    for name, cycles in types.items():
        cols = f_columns(cycles)
        ranks = [krylov_rank(cols, x) for x in range(1, 16)]
        controls[name] = {"columns_binary": [format(x, "04b") for x in cols],
                          "orbit_sizes": orbits(cols),
                          "rank_histogram": {str(i): ranks.count(i) for i in range(1, 5)},
                          "min_rank": min(ranks)}
    if controls["5+1"]["orbit_sizes"] != [5, 5, 5]:
        raise AssertionError("incorrect 5+1 cycle control")
    if controls["5+1"]["min_rank"] != 4:
        raise AssertionError("5+1 expected irreducible action")
    for name in ("3+3", "6", "identity_mutant"):
        if controls[name]["min_rank"] == 4:
            raise AssertionError(("negative control unexpectedly cyclic", name))
    # Check the chosen integral lift really reduces to the six-root (5)(1) action.
    for n in (4, 8):
        for j in range(4):
            unit = tuple(int(i == j) for i in range(4))
            nxt = lift_companion_mul(unit, n)
            bit_col = sum((x & 1) << k for k, x in enumerate(nxt))
            if bit_col != f_columns(types["5+1"])[j]:
                raise AssertionError(("lift reduction mismatch", n, j))
            w = unit
            for _ in range(5):
                w = lift_companion_mul(w, n)
            if w != unit:
                raise AssertionError(("lift is not order-five", n, j))
    lifts = {str(n): enumerate_primitive(n) for n in (4, 8)}
    if lifts["4"]["primitive_vectors"] != 240 or lifts["8"]["primitive_vectors"] != 3840:
        raise AssertionError("incomplete lift enumeration")
    out = {"scope": "abstract finite-module structural regression only",
           "source_sha256": hashlib.sha256(open(__file__, "rb").read()).hexdigest(),
           "mod2_controls": controls,
           "companion_lift": lifts,
           "verified": True,
           "limitations": ["not genuine Jacobian Frobenius modulo 4/8",
                           "not group-scheme or Lean proof", "not ECDLP timing or transfer"]}
    print(json.dumps(out, sort_keys=True, indent=2))


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        sys.exit(1)
