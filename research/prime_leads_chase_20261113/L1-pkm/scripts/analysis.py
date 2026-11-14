#!/usr/bin/env python3
"""Slope fits and implied exponents per the frozen rule. Input: results.csv with
m,t,D_lo,D_hi (D_hi empty = censored upper), source. Uses the PC closure values (instrument 1)."""
import csv, math, itertools, json, sys
def H(x): return 0 if x <= 0 or x >= 1 else -x*math.log2(x)-(1-x)*math.log2(1-x)
def Hinv(y):
    lo, hi = 0.0, 0.5
    for _ in range(100):
        mid = (lo+hi)/2
        if H(mid) < y: lo = mid
        else: hi = mid
    return lo
def E(m, s, w):
    den = m+1-w*m*H(min(s, 0.5))
    return 2/den if den > 2 else 1.0   # >=1: no gain (capped)
rows = [r for r in csv.DictReader(open(sys.argv[1])) if r['instrument'] == sys.argv[2]]
out = {}
for m in sorted({int(r['m']) for r in rows}):
    R = sorted([r for r in rows if int(r['m']) == m], key=lambda r: int(r['t']))
    exact = [(int(r['t'])*m, int(r['D_lo'])) for r in R if r['D_hi'] and r['D_lo'] == r['D_hi']]
    ex2 = [(N, D) for N, D in exact if N >= 2*m]
    bounds = [(int(r['t'])*m, int(r['D_lo']), (int(r['D_hi']) if r['D_hi'] else None)) for r in R]
    a1, a2 = Hinv(1-3/m) if m > 3 else 0.0, Hinv((1-3/m)/2) if m > 3 else 0.0
    res = dict(m=m, cells=[dict(N=N, D_lo=lo, D_hi=hi) for N, lo, hi in bounds], alpha1=a1, alpha2=a2)
    def ls(pts):
        if len(pts) < 2: return None
        n = len(pts); mx = sum(p[0] for p in pts)/n; my = sum(p[1] for p in pts)/n
        sxx = sum((p[0]-mx)**2 for p in pts)
        return sum((p[0]-mx)*(p[1]-my) for p in pts)/sxx if sxx else None
    res['slope_ls_t>=2'] = ls(ex2); res['slope_ls_all'] = ls(exact)
    # interval from pairwise slopes among t>=2 cells using bounds
    b2 = [b for b in bounds if b[0] >= 2*m]
    lo_s, hi_s = [], []
    for (N1, l1, h1), (N2, l2, h2) in itertools.combinations(b2, 2):
        lo_s.append((l2-h1)/(N2-N1) if h1 is not None else -math.inf)
        hi_s.append((h2-l1)/(N2-N1) if h2 is not None else math.inf)
    if lo_s:
        res['sigma_lo'] = min(lo_s)   # protocol: min/max of all pairwise slopes, bounds entering as bounds
        res['sigma_hi'] = max(hi_s)
    # last-step slope
    if len(bounds) >= 2:
        (N1, l1, h1), (N2, l2, h2) = bounds[-2], bounds[-1]
        res['last_step'] = dict(lo=(l2-(h1 if h1 is not None else math.inf))/(N2-N1) if h1 is not None else None,
                                hi=((h2-l1)/(N2-N1)) if h2 is not None else None)
    dN = max((b[0] for b in b2), default=0) - min((b[0] for b in b2), default=0)
    res['dN_max'] = dN
    verdict = 'INCONCLUSIVE'
    if len([b for b in b2 if b[2] is not None and b[1] == b[2]]) >= 2 and 'sigma_lo' in res:
        if res['sigma_lo'] - 1/dN >= a1: verdict = 'KILL'
        elif res['sigma_hi'] + 1/dN < a1: verdict = 'SURVIVE'
    res['verdict_frozen_rule'] = verdict
    for key in ['slope_ls_t>=2', 'slope_ls_all']:
        s = res[key]
        if s is not None: res['E1_'+key], res['E2_'+key] = E(m, s, 1), E(m, s, 2)
    for key in ['sigma_lo', 'sigma_hi']:
        if key in res and math.isfinite(res[key]): res['E1_'+key], res['E2_'+key] = E(m, res[key], 1), E(m, res[key], 2)
    res['E1_one_per_tstep'] = E(m, 1/m, 1); res['E2_one_per_tstep'] = E(m, 1/m, 2)
    out[m] = res
json.dump(out, open(sys.argv[3], 'w'), indent=1, default=str)
for m, r in out.items():
    print(m, {k: (round(v, 3) if isinstance(v, float) else v) for k, v in r.items() if k != 'cells'})
