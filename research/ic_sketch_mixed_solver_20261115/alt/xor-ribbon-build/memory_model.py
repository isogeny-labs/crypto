#!/usr/bin/env python3
"""Memory-budget model for the xor/ribbon lane. All figures [pcc:unverified] model outputs.
Incumbent cost model: setup = F^2 GAE, trials = 0.68 F / h(F), h(F) = F^2 / ((p+1)/2),
GAE/trial c_t = 1.0479 (p24 incremental-gauss record: relations phase 50875 GAE / 48550 trials).
Structure bits/key from raw_output.txt (measured) or formulas. native probes/lookup priced at w."""
import math, sys
CT = 1.0479
def hit(F, p): return min(1.0, F*(F-1)/((p+1)/2))
def vbits(F): return 2*math.ceil(math.log2(max(F,2))) + 1
def structs(F, p):
    v = vbits(F); lp = math.ceil(math.log2((p+1)/2))
    f = 12
    return {  # name: (bits/key, probes/lookup, extra_gae_fn(trials,F,h))
     "exact HashMap(u32,u32) [measured 91-146 b/key, use 124]": (124, 1, lambda T,F,h: 0),
     "exact packed (x + value)": (lp+v, 1, lambda T,F,h: 0),
     "ribbon f=12 + value (1.10 overhead), verify 1 GAE per candidate": (1.10*(f+v), 32.5, lambda T,F,h: T*(h+2**-f)*1.0),
     "xor f=12 + value (1.23), verify 1 GAE per candidate": (1.23*(f+v), 3, lambda T,F,h: T*(h+2**-f)*1.0),
     "xor f=12 membership only; resolve (i,j) by F adds per candidate": (1.23*f, 3, lambda T,F,h: T*(h+2**-f)*F),
    }
def best(p, Mbits, name):
    b, pr, ex = structs(2,p)[name] if False else (None,None,None)
def total(F, p, S, w, M):
    b, pr, ex = S
    h = hit(F, p); T = 0.68*F/h
    return F*F + CT*T + ex(T,F,h) + w*T*pr, T, h
def solve(p, Mbits, name, w):
    # F bounded by memory: keys ~F(F-1) * bits(F) <= M
    lo, hi = 8, int(2*math.sqrt((p+1)/2))+8
    bestv = None
    # geometric scan then refine
    Fs = sorted(set(int(8*1.02**i) for i in range(0, 1500) if 8*1.02**i < hi))
    for F in Fs:
        S = structs(F, p)[name]
        if F*(F-1)*S[0] > Mbits: break
        t, T, h = total(F, p, S, w, Mbits)
        if bestv is None or t < bestv[0]: bestv = (t, F, T, h)
    return bestv
def fmt(x): return f"{x:.3g}"
if __name__ == "__main__":
    curves = [("p24", 16777199), ("p28", 268435399), ("p40", 2**40-87), ("p56", 2**56-5), ("p83", 2**83-1)]
    budgets = [("32KiB(L1)", 32*1024*8), ("1MiB(L2)", 2**20*8), ("32MiB(L3)", 32*2**20*8), ("16GiB", 16*2**30*8), ("1TiB", 2**40*8)]
    names = list(structs(100, 2**24).keys())
    for w in (0.0, 0.1, 1.0):
        print(f"\n##### w = {w} GAE per native probe #####")
        for cn, p in curves:
            # unconstrained optimum for reference (exact packed, huge memory)
            unc = solve(p, 1e30, names[1], w)
            print(f"\n{cn}: unconstrained exact-packed optimum F*={unc[1]} total={fmt(unc[0])} GAE (trials={fmt(unc[2])}, h={fmt(unc[3])}); keys*bits at F* = {fmt(unc[1]*(unc[1]-1)*structs(unc[1],p)[names[1]][0]/8)} bytes")
            for bn, Mb in budgets:
                row = []
                base = solve(p, Mb, names[1], w)
                for nm in names:
                    r = solve(p, Mb, nm, w)
                    row.append("-" if r is None else f"F={r[1]} tot={fmt(r[0])} ({base[0]/r[0]:.2f}x vs exact-packed)" if base else f"F={r[1]} tot={fmt(r[0])}")
                print(f"  budget {bn:10s}: " + " | ".join(f"[{i}] {x}" for i, x in enumerate(row)))
    print("\nstructures:"); [print(f"  [{i}] {n}") for i, n in enumerate(names)]
