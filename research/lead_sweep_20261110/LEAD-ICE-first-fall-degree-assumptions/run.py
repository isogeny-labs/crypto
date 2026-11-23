#!/usr/bin/env python3
"""LEAD-ICE-first-fall-degree-assumptions: stdlib toy. See PROTOCOL.md."""
import json, random, sys, time, itertools
IRR = {4: 0b10011, 5: 0b100101, 6: 0b1000011, 7: 0b10000011, 8: 0b100011011, 10: 0b10000001001}
DMAX = 5  # D_solve censored at DMAX+1

def padd(a, b): return a ^ b            # polys are frozenset-free: python sets
def pmul(a, b):
    r = set()
    for x in a:
        for y in b:
            r ^= {x | y}
    return r
def fmul(A, B, n):
    prod = [set() for _ in range(2 * n - 1)]
    for i in range(n):
        for j in range(n):
            if A[i] and B[j]: prod[i + j] ^= pmul(A[i], B[j])
    irr = IRR[n]
    for d in range(2 * n - 2, n - 1, -1):
        if prod[d]:
            for k in range(n):
                if (irr >> k) & 1: prod[d - n + k] ^= prod[d]
            prod[d] = set()
    return prod[:n]
def fadd(A, B): return [a ^ b for a, b in zip(A, B)]
def const(c, n): return [({0} if (c >> i) & 1 else set()) for i in range(n)]
def semaev3(x1, x2, x3, a6, n):
    p12, p13, p23 = fmul(x1, x2, n), fmul(x1, x3, n), fmul(x2, x3, n)
    s = fadd(fadd(p12, p13), p23)
    s2 = fmul(s, s, n)
    return fadd(fadd(s2, fmul(p12, x3, n)), const(a6, n))

def macaulay_stats(eqs, nv):
    """return (D_ff, D_solve): first D with rank<min(rows,cols); first D with rank==cols."""
    dff = dsol = None
    for D in range(2, DMAX + 1):
        cols = [m for m in range(1 << nv) if bin(m).count("1") <= D]
        idx = {m: i for i, m in enumerate(cols)}
        mults = [m for m in cols if bin(m).count("1") <= D - 2]
        piv = {}
        rank = rows = 0
        for f in eqs:
            for m in mults:
                v = 0
                for t in f: v ^= 1 << idx[m | t]
                rows += 1
                while v:
                    h = v.bit_length() - 1
                    if h in piv: v ^= piv[h]
                    else: piv[h] = v; rank += 1; break
        if dff is None and rank < min(rows, len(cols)): dff = D
        if dsol is None and rank == len(cols): dsol = D
        if dff is not None and dsol is not None: break
    return (dff or DMAX + 1), (dsol or DMAX + 1)

def structured(n, npr, rng):
    nv = 2 * npr
    def coords(off):  # x in V = span(1,t,..,t^{npr-1}) with Boolean variables
        return [({1 << (off + i)} if i < npr else set()) for i in range(n)]
    xr = const(rng.randrange(1, 1 << n), n)
    a6 = rng.randrange(1, 1 << n)
    eqs = semaev3(coords(0), coords(npr), xr, a6, n)
    return [e for e in eqs if e], nv
def random_quad(neq, nv, rng):
    mons = [0] + [1 << i for i in range(nv)] + [(1 << i) | (1 << j) for i in range(nv) for j in range(i)]
    return [{m for m in mons if rng.random() < .5} for _ in range(neq)], nv
def planted(neq, nv, rng):
    eqs, _ = random_quad(neq, nv, rng)
    eqs[1] = set(eqs[0])  # planted duplicate: forced rank deficiency at D=2
    return eqs, nv

def arm(gen, seeds, **kw):
    out = []
    for s in seeds:
        eqs, nv = gen(random.Random(s), **kw)
        out.append(macaulay_stats(eqs, nv))
    return out
def mean(xs): return sum(xs) / len(xs)

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    cfgs = [(5, 2, 3)] if smoke else [(6, 3, 30), (8, 4, 30), (10, 5, 8)]
    res, diffs = [], []
    for n, npr, cnt in cfgs:
        if time.time() - t0 > 420: break
        base = 1000 * n
        S = arm(lambda r, n=n, npr=npr: structured(n, npr, r), range(base, base + cnt))
        nv = 2 * npr
        neq = n
        R1 = arm(lambda r: random_quad(neq, nv, r), range(base + 100, base + 100 + cnt))
        R2 = arm(lambda r: random_quad(neq, nv, r), range(base + 200, base + 200 + cnt))
        P = arm(lambda r: planted(neq, nv, r), range(base + 300, base + 300 + max(3, cnt // 3)))
        row = dict(n=n, nprime=npr, nvars=nv, neq=neq, count=cnt,
                   structured_Dsolve=mean([x[1] for x in S]), null_Dsolve=mean([x[1] for x in R1]),
                   null2_Dsolve=mean([x[1] for x in R2]),
                   structured_Dff=mean([x[0] for x in S]), null_Dff=mean([x[0] for x in R1]),
                   planted_Dff=mean([x[0] for x in P]), null_Dff_AA=mean([x[0] for x in R2]))
        row["delta_Dff"] = row["structured_Dff"] - row["null_Dff"]
        row["delta_Dsolve"] = row["structured_Dsolve"] - row["null_Dsolve"]
        row["AA_gap"] = abs(row["null_Dff"] - row["null_Dff_AA"])
        res.append(row)
    ok_pos = all(r["planted_Dff"] <= 3.0 for r in res)
    ok_aa = all(r["AA_gap"] <= 0.5 for r in res)
    if not (ok_pos and ok_aa): verdict = "inconclusive"
    elif all(r["delta_Dff"] <= -1.0 for r in res): verdict = "consistent_with_prediction"
    elif all(r["delta_Dff"] > -0.5 for r in res): verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps(dict(lead="LEAD-ICE-first-fall-degree-assumptions", smoke=smoke, seeds="base=1000*n; arms +0,+100,+200,+300",
        parameters=dict(DMAX=DMAX, field="GF(2^n)", S3="x1^2x2^2+x1^2x3^2+x2^2x3^2+x1x2x3+a6 (from memory)"),
        rows=res, positive_control_ok=ok_pos, aa_ok=ok_aa, verdict=verdict,
        wall_s=round(time.time() - t0, 2))))
main()
