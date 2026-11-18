#!/usr/bin/env python3
"""pkm-cost-rules: reconcile s < m-3 (closure) with c < (m-1)/2 (10-31 note) for the PKM Kummer chain.
All outputs [pcc:unverified]. Pure desk computation; no solver runs. Usage: python3 -I rules.py OUTDIR"""
import json, math, sys
from math import comb, log2

OUT = sys.argv[1]
H = lambda x: 0.0 if x <= 0 or x >= 1 else -x * log2(x) - (1 - x) * log2(1 - x)
FCr = lambda x: (1 + x) * H(x / (1 + x))  # log2 C(N+D,D)/N with D = xN

def inv(f, target, lo=1e-12, hi=0.5):
    for _ in range(200):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if f(mid) < target else (lo, mid)
    return (lo + hi) / 2

# ---------- exponent models (units: log_r; B = r^beta, per-attempt cost B^s) ----------
def E_uniform(m, s, beta, la, floor, one_output=False):
    att = 1 - (m - 1) * beta  # log_r of B/mu = r m!/B^(m-1)
    if one_output:
        att = max(beta, att)
    elif floor:
        att = max(0.0, att)
    return max(la * beta, att + s * beta)

def E_mixed(m, s, beta, n, la):
    # columns B = r^beta, total tower bits N = n log2 r with beta <= n <= m beta, per-attempt 2^{(s/m) N}
    att = max(0.0, 1 + beta - n)
    return max(la * beta, att + (s / m) * n)

def closed_floor(m, s, la):
    if s <= la:
        return la / (m - 1 + la - s)
    return max(la, s) / (m - 1)

def closed_nofloor(m, s, la):
    if s >= m - 1:
        return 1.0
    return la / (m - 1 + la - s)

import numpy as np
BETAS = np.arange(1, 20001) / 20000.0

def numin(f):
    return float(np.min(f(BETAS)))

def E_uniform(m, s, beta, la, floor, one_output=False):
    att = 1 - (m - 1) * beta
    if one_output:
        att = np.maximum(beta, att)
    elif floor:
        att = np.maximum(0.0, att)
    return np.maximum(la * beta, att + s * beta)

res = {"K1": [], "K2": [], "K3": {}, "thresholds": {}}
k1_maxdev = 0.0
k1_set_equal_closure = []
for m in range(3, 17):
    for la in (1, 2):
        thr = {"floor": None, "nofloor": None, "one_output": None}
        prev = {"floor": True, "nofloor": True, "one_output": True}
        for si in range(0, int((m - 1) * 100) + 1):
            s = si / 100
            ef = numin(lambda b: E_uniform(m, s, b, la, True))
            en = numin(lambda b: E_uniform(m, s, b, la, False))
            eo = numin(lambda b: E_uniform(m, s, b, la, True, True))
            dev = max(abs(ef - closed_floor(m, s, la)), abs(en - closed_nofloor(m, s, la)))
            k1_maxdev = max(k1_maxdev, dev)
            for key, e in (("floor", ef), ("nofloor", en), ("one_output", eo)):
                if prev[key] and not e < 0.5:
                    thr[key] = s
                    prev[key] = False
        res["thresholds"][f"m{m}_la{la}"] = thr
        # beat-rho set under floor equals {s < m-3}?  (thr = first s on grid with E >= 1/2)
        if la == 2 and m >= 6 and thr["floor"] is not None and abs(thr["floor"] - (m - 3)) < 0.011:
            k1_set_equal_closure.append(m)
res["K1"] = {"max_abs_dev_numeric_vs_closed": k1_maxdev, "m>=6 where floored beat-set == {s<m-3}": k1_set_equal_closure}

# K2: mixed depths
k2_maxgain = 0.0
NB = 4000
for m in (4, 6, 8, 10, 12):
    for la in (1, 2):
        for s in [x / 4 for x in range(0, 4 * (m - 1) + 1)]:
            uni = numin(lambda b: E_uniform(m, s, b, la, True))
            b = (np.arange(1, NB + 1) / NB)[:, None]
            j = (np.arange(0, 401) / 400)[None, :]
            n = b + (m * b - b) * j
            best = float(np.min(np.maximum(la * b, np.maximum(0.0, 1 + b - n) + (s / m) * n)))
            k2_maxgain = max(k2_maxgain, uni - best)
res["K2"] = {"max(uniform_floored - mixed_optimum)": k2_maxgain}

# K3: reproduce the closure's implied exponents with 2/(m+1-s), s = m H(sigma)
k3 = {}
for m, sig, pub in ((6, 1 / 6, 0.645), (8, 1 / 8, 0.430), (7, 1 / 7, 0.518), (4, 1 / 4, 1.140)):
    s = m * H(sig)
    k3[f"m{m}_sigma{sig:.4f}"] = {"s": s, "2/(m+1-s)": 2 / (m + 1 - s), "published": pub,
                                  "floored_quadLA": closed_floor(m, s, 2), "floored_linLA": closed_floor(m, s, 1),
                                  "attempt_exponent_at_nofloor_opt": 1 - (m - 1) / (m + 1 - s)}
res["K3"] = k3

# ---------- slope bars alpha*(m,w): per-unknown slope sigma = D/N ----------
bars = {}
for m in range(4, 17):
    sstar_cl = m - 3
    sstar_q = res["thresholds"][f"m{m}_la2"]["floor"]
    sstar_l = res["thresholds"][f"m{m}_la1"]["floor"]
    sstar_o = res["thresholds"][f"m{m}_la2"]["one_output"]
    row = {"s*_closure": sstar_cl, "s*_floor_quadLA(grid)": sstar_q, "s*_floor_linLA(grid)": sstar_l, "s*_one_output_quadLA(grid)": sstar_o}
    for w in (1, 2):
        for name, sst in (("closure", sstar_cl), ("correct", min(m - 3, (m - 1) / 2)), ("one_output", m / 2 - 1)):
            tgt = sst / (m * w)
            row[f"a*_{name}_ML_w{w}"] = inv(H, tgt) if tgt < 1 else 0.5
            row[f"a*_{name}_FC_w{w}"] = inv(FCr, tgt, hi=5) if tgt < 1 else None
    bars[m] = row
res["bars"] = bars

# ---------- measured D (certified cells only for slopes) ----------
# sources: chase L1 results.csv (p40961 / p16801793 closures), pkm_m8_runs tables.md (P1 mixed ladders, msolve), ic_next10 m=4 P1 ladder
cert = {  # m: [(N, D_exact)]
    4: [(4, 5), (8, 6), (12, 7)],
    5: [(5, 5), (10, 6)],
    6: [(6, 6), (12, 7)],
    7: [(7, 6)],
    8: [(8, 7), (16, 8)],
}
lb = {4: [(16, 7), (20, 8)], 5: [(15, 8)], 6: [(18, 8)], 7: [(14, 8)], 8: [(24, 9)]}
mixed = {  # msolve P1 mixed ladders (exact rungs), first censored rung as lb
    6: ([(6, 6), (7, 6), (8, 6), (9, 6), (10, 6), (11, 7), (12, 7), (13, 7), (14, 7)], (15, 8)),
    7: ([(7, 6), (8, 7), (9, 7), (10, 7), (11, 7)], (12, 8)),
    8: ([(8, 7), (9, 7), (10, 7), (11, 7)], (12, 8)),
}
mixedW = {
    6: [(6, 1688), (7, 2600), (8, 3975), (9, 9391), (10, 14234), (11, 33571), (12, 40004), (13, 35390), (14, 105615)],
    7: [(7, 3479), (8, 12017), (9, 22526), (10, 32665), (11, 35880)],
    8: [(8, 16355), (9, 32639), (10, 8265), (11, 44704)],
}

def ols(pts):
    n = len(pts); mx = sum(x for x, _ in pts) / n; my = sum(y for _, y in pts) / n
    sxx = sum((x - mx) ** 2 for x, _ in pts)
    return sum((x - mx) * (y - my) for x, y in pts) / sxx if sxx else None

def implied(m, sig, w=1):
    s = w * m * H(sig)
    return {"s": s, "E_closure_rule(2/(m+1-s), no floor)": (2 / (m + 1 - s) if s < m + 1 else None),
            "E_correct(floor, quadLA)": closed_floor(m, s, 2), "E_correct(floor, linLA)": closed_floor(m, s, 1)}

meas = {}
for m in range(4, 9):
    rows = []
    c = cert[m]
    for (n0, d0), (n1, d1) in zip(c, c[1:]):
        rows.append(("certified step", n0, n1, (d1 - d0) / (n1 - n0), "value"))
    if len(c) >= 2:
        rows.append(("certified endpoints", c[0][0], c[-1][0], (c[-1][1] - c[0][1]) / (c[-1][0] - c[0][0]), "value"))
    for (nl, dl) in lb.get(m, []):
        n0, d0 = c[-1]
        if nl > n0:
            rows.append(("censored lb vs last certified", n0, nl, (dl - d0) / (nl - n0), "lower bound"))
    if m in mixed:
        ex, cen = mixed[m]
        rows.append(("P1 mixed ladder OLS (msolve)", ex[0][0], ex[-1][0], ols(ex), "value (finite-N fit)"))
        rows.append(("P1 mixed ladder endpoints", ex[0][0], ex[-1][0], (ex[-1][1] - ex[0][1]) / (ex[-1][0] - ex[0][0]), "value"))
    out = []
    for lab, n0, n1, sig, kind in rows:
        b = bars[m]
        out.append({"estimate": lab, "N": [n0, n1], "sigma": sig, "kind": kind,
                    "vs_closure_bar": ("KILL" if sig >= b["a*_closure_ML_w1"] else "below-bar"),
                    "vs_correct_bar": ("KILL" if sig >= b["a*_correct_ML_w1"] else "below-bar"),
                    **implied(m, sig)})
    meas[m] = out
res["measured"] = meas

# s_W: per-attempt exponent read from msolve matrix width over mixed ladders: s_W = m * dlog2W/dN (OLS)
sw = {}
for m, pts in mixedW.items():
    sl = ols([(n, log2(w)) for n, w in pts])
    sw[m] = {"OLS dlog2W/dN": sl, "s_W = m*slope": m * sl, "s*_correct": min(m - 3, (m - 1) / 2), "s*_closure": m - 3}
res["s_W"] = sw

# ---------- m = 8: needed D ----------
def log2M(N, D):
    return log2(sum(comb(N, k) for k in range(0, D + 1)))

def log2FC(N, D, extra=0):
    return log2(comb(N + extra + D, D))

m = 8
need = {}
for L in (31, 64, 128, 160, 256):
    Nopt = m / (m - 1) * L
    Ni = math.floor(Nopt)
    row = {"N_opt=(8/7)log2r": Nopt}
    for w in (1, 2):
        for name, f in (("ML", lambda N, D: log2M(N, D)), ("FC", lambda N, D: log2FC(N, D)), ("FC+u6", lambda N, D: log2FC(N, D, 6))):
            budget = Ni * (m - 1) / (2 * m)
            D = 0
            while D + 1 <= Ni and w * f(Ni, D + 1) < budget:
                D += 1
            row[f"Dmax_{name}_w{w}"] = D
    # closure rule at its own (fractional-attempt) optimum, for contrast: s < m-3 with s = log2M/t, t = N/m
    budget_cl = Ni * (m - 3) / m
    D = 0
    while D + 1 <= Ni and log2M(Ni, D + 1) < budget_cl:
        D += 1
    row["Dmax_closure_rule_ML_w1 (same N)"] = D
    # required slope from the certified anchor D(8,16)=8
    row["sigma_needed_from_anchor(16,8)_ML_w1"] = (row["Dmax_ML_w1"] - 8) / (Ni - 16) if Ni > 16 else None
    row["D_at_Nopt_if_sigma=1/8_from_anchor"] = 8 + (Ni - 16) / 8
    need[L] = row

# finite exact optimisation (generous unit: 1 monomial = 1 group op), uniform & mixed N, quad LA m*B^2, floor, m! yield factor
def finite(L, w=1, Dfun=None):
    best = None
    for N in range(max(m, L - 8), int(1.6 * L) + 8):
        tmax = math.ceil(N / m)
        B = 2 ** tmax
        att = max(1.0, math.factorial(m) * 2.0 ** L * B / 2.0 ** N)
        rho = math.sqrt(math.pi * 2.0 ** L / 4)
        # largest D with total < rho
        D = -1
        while D + 1 <= N:
            T = att * 2 ** (w * log2M(N, D + 1)) + m * B * B
            if T < rho:
                D += 1
            else:
                break
        if best is None or D - 0.0 > best[1] - 0.0 or (D == best[1] and N > best[0]):
            if D >= 0:
                best = (N, D, att)
    return best

fin = {}
for L in (31, 64, 128, 256):
    b = finite(L)
    fin[L] = {"N_best": b[0] if b else None, "Dmax": b[1] if b else None, "attempts": b[2] if b else None}
res["m8_need"] = need
res["m8_finite"] = fin
json.dump(res, open(f"{OUT}/results.json", "w"), indent=1, default=str)

# ---------- printed summary ----------
P = print
P("K1 max |numeric - closed form| =", f"{k1_maxdev:.2e}", "; m>=6 with floored beat-set == {s<m-3}:", k1_set_equal_closure)
P("K2 max gain of mixed depths over uniform floored optimum =", f"{k2_maxgain:.2e}")
P("K3:")
for k, v in k3.items():
    P(f"  {k}: s={v['s']:.3f} 2/(m+1-s)={v['2/(m+1-s)']:.3f} (published {v['published']}) floored quad={v['floored_quadLA']:.3f} lin={v['floored_linLA']:.3f} attempt-exp at no-floor opt={v['attempt_exponent_at_nofloor_opt']:.3f}")
P("\nthresholds s* (grid, first s with E>=1/2):")
P(" m | closure m-3 | floor quadLA | floor linLA | one-output quadLA | nofloor quadLA")
for m_ in range(4, 17):
    t2 = res["thresholds"][f"m{m_}_la2"]; t1 = res["thresholds"][f"m{m_}_la1"]
    P(f"{m_:2d} | {m_-3:5} | {t2['floor']} | {t1['floor']} | {t2['one_output']} | {t2['nofloor']}")
P("\nslope bars alpha* (multilinear ML / full columns FC), w=1 [w=2]:")
P(" m | closure ML | correct ML | correct FC | one-output ML | closure ML w2 | correct ML w2")
for m_ in range(4, 17):
    b = bars[m_]
    P(f"{m_:2d} | {b['a*_closure_ML_w1']:.4f} | {b['a*_correct_ML_w1']:.4f} | {b['a*_correct_FC_w1']:.4f} | {b['a*_one_output_ML_w1']:.4f} | {b['a*_closure_ML_w2']:.4f} | {b['a*_correct_ML_w2']:.4f}")
P("\nmeasured slopes (w=1, multilinear):")
for m_, rows in meas.items():
    for r in rows:
        P(f" m={m_} {r['estimate']:32s} N {r['N'][0]}->{r['N'][1]} sigma={r['sigma']:.4f} ({r['kind']}) closure:{r['vs_closure_bar']:9s} correct:{r['vs_correct_bar']:9s} s={r['s']:.2f} E_cl={r['E_closure_rule(2/(m+1-s), no floor)']:.3f} E_corr(q)={r['E_correct(floor, quadLA)']:.3f} E_corr(l)={r['E_correct(floor, linLA)']:.3f}")
P("\ns_W from msolve widths (P1 mixed ladders):")
for m_, v in sw.items():
    P(f" m={m_}: dlog2W/dN={v['OLS dlog2W/dN']:.3f} s_W={v['s_W = m*slope']:.2f} vs s*_correct={v['s*_correct']} s*_closure={v['s*_closure']}")
P("\nm=8 needed D at N_opt (asymptotic floored rule, budget N*(m-1)/(2m)):")
for L, r in need.items():
    P(f" log2r={L}: " + ", ".join(f"{k}={(f'{v:.3f}' if isinstance(v, float) else v)}" for k, v in r.items()))
P("\nm=8 finite exact optimisation (T = max(1, m! r B/2^N) * M(N,D) + m B^2 < sqrt(pi r/4)):")
for L, r in fin.items():
    P(f" log2r={L}: {r}")
