#!/usr/bin/env python3
"""Independent checks for research/pkm_paper_faithful_20261124/.

Three things are re-derived here rather than trusted:

1. the construction: the emitted V really is the x-image of a coset of the
   order-2^t subgroup of the emitted E'/F_p, with the coset and its negative
   disjoint and chi not 2-torsion (PKM Lemma 1);
2. the answer: `recovered` equals the planted scalar `1 + seed mod (r - 1)`,
   recomputed outside the harness;
3. the density table: (deg L)^m/(m! p) against the observed relation rate.

Pure Python; no dependencies.  Run from this directory.
"""
import glob
import json
import math
import os

# roster primes, src/cryptanalysis/research_bench.rs: bench_curves()
ROSTER = {10: 827, 12: 3907, 14: 16187, 16: 65353, 18: 261847}
HERE = os.path.dirname(os.path.abspath(__file__))

# ── 1. The §3.4 construction, from the paper's definition ──────────────
P = 100003
AUX_A, AUX_B, AUX_N = 67190, 45464, 100352
T = 5
TGT_A, TGT_B, X_R = 46244, 7379, 90553
O = None  # point at infinity


def inv(a):
    return pow(a, P - 2, P)


def add(Pt, Q, a=AUX_A):
    if Pt is O:
        return Q
    if Q is O:
        return Pt
    (x1, y1), (x2, y2) = Pt, Q
    if x1 == x2 and (y1 + y2) % P == 0:
        return O
    if Pt == Q:
        lam = (3 * x1 * x1 + a) * inv(2 * y1) % P
    else:
        lam = (y2 - y1) * inv(x2 - x1) % P
    x3 = (lam * lam - x1 - x2) % P
    return (x3, (lam * (x1 - x3) - y1) % P)


def mul(k, Pt, a=AUX_A):
    R = O
    while k:
        if k & 1:
            R = add(R, Pt, a)
        Pt = add(Pt, Pt, a)
        k >>= 1
    return R


def neg(Pt):
    return O if Pt is O else (Pt[0], (-Pt[1]) % P)


def lift(x, a=AUX_A, b=AUX_B):
    rhs = (x * x * x + a * x + b) % P
    y = pow(rhs, (P + 1) // 4, P)  # P = 3 (mod 4)
    return (x, y) if y * y % P == rhs else None


def on(Pt, a=AUX_A, b=AUX_B):
    return Pt is not O and (Pt[1] ** 2 - Pt[0] ** 3 - a * Pt[0] - b) % P == 0


def check_construction():
    rows = [json.loads(l) for l in open(os.path.join(HERE, "pilot_iso.json"))
            if l.strip().startswith("{")]
    v = next(r["tower"]["v"] for r in rows if r.get("p") == P)

    assert len(v) == len(set(v)) == 1 << T, "|V| != 2^t or repeats"
    pts = []
    for x in v:
        Pt = lift(x)
        assert Pt is not None and on(Pt), f"x={x} does not lift to E'"
        pts += [Pt, neg(Pt)]
    assert all(on(q) for q in pts)

    H = None
    for x in range(2, 200):  # E'(F_p)[2^t]: unique of its order here
        Pt = lift(x)
        if Pt is None:
            continue
        cand = mul(AUX_N // (1 << T), Pt)
        if cand is not O and mul(1 << T, cand) is O:
            H = {mul(k, cand) for k in range(1 << T)}
            if len(H) == 1 << T:
                break
            H = None
    assert H is not None, "no point of order 2^t on E'"

    found = None
    for P0 in pts:
        C = {add(P0, h) for h in H}
        nC = {neg(q) for q in C}
        if (C & nC) == set() and (C | nC) == set(pts):
            found = (P0, C)
            break
    assert found is not None, "V is not x(R' + H) for the 2^t-torsion H"
    P0, C = found
    assert sorted(q[0] for q in C) == sorted(v)
    assert add(P0, P0) not in H, "2P0 in H: chi would be 2-torsion"
    print(f"construction: |V|={len(v)} distinct, V = x(P0+H), P0={P0}, "
          f"C n -C = empty, 2P0 not in H")

    R = lift(X_R, TGT_A, TGT_B)
    assert R is not None and on(R, TGT_A, TGT_B)
    rels = []
    for x1 in v:
        P1 = lift(x1, TGT_A, TGT_B)
        if P1 is None:
            continue
        for x2 in v:
            P2 = lift(x2, TGT_A, TGT_B)
            if P2 is None:
                continue
            for s1 in (P1, neg(P1)):
                for s2 in (P2, neg(P2)):
                    if add(s1, s2, TGT_A) == R:
                        rels.append((x1, x2))
    print(f"target x_R={X_R}: {len(rels)} ordered / "
          f"{len({tuple(sorted(r)) for r in rels})} unordered decompositions over V")


# ── 2. The answer, recomputed outside the harness ──────────────────────
def check_answers():
    ok = True
    for f in sorted(glob.glob(os.path.join(HERE, "results", "*.json"))):
        d = json.load(open(f))
        r = d["rows"][0]
        if r.get("recovered") is None:
            print(f"{os.path.basename(f):20s} no recovered scalar ({r.get('exhausted')})")
            continue
        planted = 1 + r["spec"]["seed"] % (r["r"] - 1)
        good = planted == r["recovered"]
        ok &= good
        print(f"{os.path.basename(f):20s} r={r['r']:7d} planted={planted:7d} "
              f"recovered={r['recovered']:7d} {'ok' if good else 'MISMATCH'}")
    return ok


# ── 3. Density table ──────────────────────────────────────────────────
def check_density():
    print(f"\n{'run':20s} {'p':>7s} {'t':>2s} {'m':>2s} {'|F|':>5s} {'pred':>6s} "
          f"{'predF':>6s} {'obs':>6s} {'obs/predF':>9s} {'D':>2s}")
    for f in sorted(glob.glob(os.path.join(HERE, "results", "*.json"))):
        d = json.load(open(f))
        r = d["rows"][0]
        spec = r["spec"]
        t = int(spec["factor_base_params"]["t"])
        m = int(spec["oracle_params"]["m"])
        bits = next(b for b in ROSTER if abs(math.log2(ROSTER[b]) - math.log2(r["r"])) < 0.2)
        p = ROSTER[bits]
        F = r["factor_base"]["signed_points"]
        pred = (2 ** t) ** m / math.factorial(m) / p
        predF = F ** m / math.factorial(m) / p
        obs = r["decomposition"]["hit_rate"]
        D = r["decomposition"]["solver"]["solving_degree_max"]
        print(f"{os.path.basename(f):20s} {p:7d} {t:2d} {m:2d} {F:5d} {pred:6.3f} "
              f"{predF:6.3f} {obs:6.3f} {obs / predF:9.2f} {D:2d}")


if __name__ == "__main__":
    check_construction()
    print()
    ok = check_answers()
    check_density()
    raise SystemExit(0 if ok else 1)
