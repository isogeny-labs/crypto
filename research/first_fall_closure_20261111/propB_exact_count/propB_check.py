#!/usr/bin/env python3
"""Exhaustive/sampled check of the exact count in Proposition B (see NOTE.md).
Builds the n descended Boolean polynomials of S3(X1,X2,x3) literally (symbolic multilinear
arithmetic with F-valued coefficients, x_i^2 = x_i), then computes dim(span ∩ R_{<=1})."""
import json, random, sys, time
from gf2n import GF, IRRED, poly_str

def lin_basis(elems, n):
    """F2-basis of the span of elems (ints)."""
    piv = {}; basis = []
    for e in elems:
        v = e
        while v:
            h = v.bit_length() - 1
            if h in piv: v ^= piv[h]
            else: piv[h] = v; basis.append(e); break
    return basis

def rank(rows):
    piv = {}; r = 0
    for v in rows:
        while v:
            h = v.bit_length() - 1
            if h in piv: v ^= piv[h]
            else: piv[h] = v; r += 1; break
    return r

# --- symbolic F-valued multilinear polynomials: dict {monomial bitmask over 2n' vars: F element}
def padd(P, Q):
    R = dict(P)
    for m, c in Q.items():
        c2 = R.get(m, 0) ^ c
        if c2: R[m] = c2
        else: R.pop(m, None)
    return R
def pmul(F, P, Q):
    R = {}
    for m1, c1 in P.items():
        for m2, c2 in Q.items():
            m = m1 | m2                      # x_i^2 = x_i
            c = R.get(m, 0) ^ F.mul(c1, c2)
            if c: R[m] = c
            else: R.pop(m, None)
    return R
def pconst(c): return {0: c} if c else {}

def descend(F, Vb, x3, b):
    """return list of n Boolean polynomials (each a set of monomial bitmasks)."""
    np_ = len(Vb)
    X1 = {1 << j: Vb[j] for j in range(np_)}
    X2 = {1 << (np_ + j): Vb[j] for j in range(np_)}
    c3 = pconst(x3)
    e = padd(padd(pmul(F, X1, X2), pmul(F, X1, c3)), pmul(F, X2, c3))
    S = padd(padd(pmul(F, e, e), pmul(F, pmul(F, X1, X2), c3)), pconst(b))
    polys = []
    for i in range(F.n):
        polys.append({m for m, c in S.items() if c >> i & 1})
    return polys

def count_deg_le1(polys):
    mons = sorted({m for p in polys for m in p} | {0})
    idx = {m: k for k, m in enumerate(mons)}
    def vec(p, only_high=False):
        v = 0
        for m in p:
            if only_high and bin(m).count("1") <= 1: continue
            v |= 1 << idx[m]
        return v
    full = [vec(p) for p in polys]
    high = [vec(p, True) for p in polys]
    rk, rh = rank(full), rank(high)
    one = 1 << idx[0]
    one_in_span = rank(full + [one]) == rk
    return rk - rh, one_in_span, rk

def fixed_field_basis(F, npr):
    V = [a for a in range(F.size) if F.frob(a, npr) == a]
    assert len(V) == 1 << npr
    Vs = set(V)
    for a in V[:40]:                       # closure checks (full set is cheap for n<=12)
        for c in V[:40]:
            assert a ^ c in Vs and F.mul(a, c) in Vs
    Vb = lin_basis(V, F.n)
    assert len(Vb) == npr
    return V, Vs, Vb

def predicted(F, Vs, npr, x3, b):
    if x3 in Vs:
        return 1 + (0 if b in Vs else 1), (b not in Vs)
    t = F.tr(F.mul(b, F.inv(F.sq(x3))))
    return npr - 1 + t, bool(t)

def run_n(npr, rng, b_exhaustive, nb_sample, x3_sample=None):
    n = 2 * npr; F = GF(n)
    V, Vs, Vb = fixed_field_basis(F, npr)
    x3s = list(range(1, F.size))
    if x3_sample and x3_sample < len(x3s):
        x3s = [x for x in V if x][:] + rng.sample(x3s, x3_sample)
        x3s = sorted(set(x3s))
    stats = dict(n=n, nprime=npr, modulus=poly_str(F.m), pairs=0, agree=0, disagree=[],
                 const_agree=0, by_class={})
    t0 = time.time()
    for x3 in x3s:
        if b_exhaustive: bs = list(range(F.size))
        else:
            bs = {0, 1, rng.choice([v for v in V if v]), rng.choice([v for v in V if v])}
            while len(bs) < nb_sample + 4: bs.add(rng.randrange(1, F.size))
            # force both trace classes outside V
            for want in (0, 1):
                for _ in range(200):
                    c = rng.randrange(1, F.size)
                    if F.tr(F.mul(c, F.inv(F.sq(x3)))) == want: bs.add(c); break
            bs = sorted(bs)
        for b in bs:
            cnt, one, rk = count_deg_le1(descend(F, Vb, x3, b))
            pc, pone = predicted(F, Vs, npr, x3, b)
            cls = ("x3inV" if x3 in Vs else "x3notV") + ("_binV" if b in Vs else "_bnotV") + "_Tr%d" % (F.tr(F.mul(b, F.inv(F.sq(x3)))) if x3 else -1)
            d = stats["by_class"].setdefault(cls, dict(pairs=0, agree=0, counts={}))
            d["pairs"] += 1; d["counts"][cnt] = d["counts"].get(cnt, 0) + 1
            stats["pairs"] += 1
            ok = (cnt == pc); okc = (one == pone)
            stats["agree"] += ok; stats["const_agree"] += okc; d["agree"] += ok
            if not ok or not okc:
                stats["disagree"].append(dict(x3=x3, b=b, measured=cnt, predicted=pc, one=one, pred_one=pone))
    stats["seconds"] = round(time.time() - t0, 1)
    stats["exhaustive_x3"] = len(x3s) == F.size - 1
    stats["exhaustive_b"] = bool(b_exhaustive)
    return stats

def quad_independence(npr):
    """Lemma: the n' bit-forms of s2 = X1*X2 are linearly independent (as bilinear forms)."""
    n = 2 * npr; F = GF(n); V, Vs, Vb = fixed_field_basis(F, npr)
    # coordinates of V-elements w.r.t. Vb: solve by brute force table
    coord = {}
    for mask in range(1 << npr):
        e = 0
        for j in range(npr):
            if mask >> j & 1: e ^= Vb[j]
        coord[e] = mask
    # bilinear form m: matrix bit (j,k) = m-th coordinate of Vb[j]*Vb[k]
    rows = []
    for m in range(npr):
        v = 0
        for j in range(npr):
            for k in range(npr):
                if coord[F.mul(Vb[j], Vb[k])] >> m & 1: v |= 1 << (j * npr + k)
        rows.append(v)
    return rank(rows) == npr

if __name__ == "__main__":
    rng = random.Random(20261111)
    plan = {2: (True, 0, None), 3: (True, 0, None), 4: (True, 0, None),
            5: (False, 6, None), 6: (False, 4, None)}
    out = []
    for npr in (2, 3, 4, 5, 6):
        ex, nb, xs = plan[npr]
        s = run_n(npr, rng, ex, nb, xs); s["quad_independence_lemma"] = quad_independence(npr)
        out.append(s)
        print(npr, 2 * npr, s["modulus"], "pairs", s["pairs"], "agree", s["agree"], "const_agree", s["const_agree"],
              "disagree", len(s["disagree"]), "t=%ss" % s["seconds"], "lemma", s["quad_independence_lemma"], flush=True)
    json.dump(out, open("propB_check_results.json", "w"), indent=1)
