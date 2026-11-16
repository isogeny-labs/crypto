#!/usr/bin/env python3
"""S1 crossing, small cells (growth lib): arms (a) control mod 12289, (b) one generator mod l, (c) {g, sigma_l g},
(d) mod 12289 at k = 6 with h / sigma_s g / x1 g.  PROTOCOL.md section S1.  Writes JSONL records."""
import sys, json, hashlib, time, os, argparse
import numpy as np
ap = argparse.ArgumentParser(); ap.add_argument('--lib', required=True); ap.add_argument('--out', required=True)
ap.add_argument('--cells', default='all'); a = ap.parse_args()
sys.path.insert(0, a.lib); sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ic_bounds_growth_lib as G
from unit_test import resultant_unit, brute_zeros, s3_coeffs

def seed_of(s): return int(hashlib.sha256(s.encode()).hexdigest()[:8], 16)
def law(k): return (k + 14) // 4
def sigma(g, m, k):
    N = 1 << k; out = {}
    for (e1, e2), v in g.items():
        key = ((e1 * m) % N, (e2 * m) % N); out[key] = (out.get(key, 0) + v)
    return {e: v for e, v in out.items() if v}
def chain_gen(p, a_, b_, t, k):
    N = 1 << k; g = {}
    for (e1, e2), v in s3_coeffs(p, a_, b_, t).items():
        key = (e1 % N, e2 % N); g[key] = (g.get(key, 0) + v) % p
    return {e: v for e, v in g.items() if v}
def closure_multi(L, gs, p):
    """closure of several inputs (copy of G.closure with a stacked start)."""
    k, D = L.k, L.D
    shifts = [(i, j) for i in range(2) for j in range(k)]
    perms = {s: L.shift_perm(*s) for s in shifts}
    rows = []
    for g in gs:
        if max(G.popcount(e1) + G.popcount(e2) for (e1, e2) in g) > D: return None, None
        rows.append(L.vec(g, p))
    B, piv = G.rref(np.vstack(rows), p)
    processed = set()
    while True:
        todo = [c for c in piv if L.wt[c] < D and c not in processed][:256]
        if not todo: break
        pos = {c: r for r, c in enumerate(piv)}
        new = B[[pos[c] for c in todo]]; processed.update(todo); prods = []
        for s in shifts:
            pm = perms[s]; Y = np.zeros_like(new); src = np.nonzero(pm >= 0)[0]
            Y[:, pm[src]] = new[:, src]; prods.append(Y)
        B, piv, _ = G.merge(B, piv, np.vstack(prods), p)
    return B, piv
def run_cell(rec, p, gs, k, D):
    L = G.Layout(k, D); t0 = time.time()
    B, piv = closure_multi(L, gs, p)
    if B is None:
        rec.update(skipped='input weight above D'); return rec
    one = L.idx[(0, 0)] in set(int(c) for c in piv)
    bad = G.verify_closed(L, B, piv, p)
    rec.update(columns=L.n, rank=len(piv), profile=G.profile(L, piv), one_in_V=bool(one), sweep_failures=int(bad),
               seconds=round(time.time() - t0, 2))
    return rec
def nine_nonzero(p, a_, b_, t):
    c = s3_coeffs(p, a_, b_, t); return len(c) == 9
def mod_l_instances(l, n=4):
    out = []; i = 0
    while len(out) < n:
        i += 1; rng = np.random.default_rng(seed_of(f's1-{l}-{i}'))
        a_, b_, t = (int(x) for x in rng.integers(1, l, size=3))
        if (4 * a_**3 + 27 * b_**2) % l == 0 or not nine_nonzero(l, a_, b_, t): continue
        u = resultant_unit(l, a_, b_, t, 10)
        if not u['unit']: continue
        out.append(dict(p=l, a=a_, b=b_, t=t, draw=i, unit_k10=True))
    return out
def leg(x, p): return pow(x % p, (p - 1) // 2, p)
def p12289_instances(n=4):
    p = 12289; out = []
    for a_ in (1, 2, 3, 5):
        t = next(t for t in range(2, p) if leg(t, p) == p - 1 and (t**3 + a_ * t) % p and leg(t**3 + a_ * t, p) == 1)
        bz = brute_zeros(p, a_, 0, t, 10); out.append(dict(p=p, a=a_, b=0, t=t, zeros_k10=bz['zeros']))
        if len(out) == n: break
    return out
def common_zero_free(p, g, h, k):
    """brute force on rational mu_N^2 (2^k | p-1): no common zero of g and h."""
    N = 1 << k
    hgen = next(pow(x, (p - 1) // N, p) for x in range(2, p) if pow(pow(x, (p - 1) // N, p), N // 2, p) != 1)
    H = np.array([pow(hgen, e, p) for e in range(N)], dtype=np.int64)
    def ev(poly):
        v = np.zeros((N, N), dtype=np.int64)
        for (e1, e2), c in poly.items():
            v = (v + c * (np.power(H[:, None], e1) % p) * (np.power(H[None, :], e2) % p)) % p
        return v
    return not np.any((ev(g) == 0) & (ev(h) == 0))
out = open(a.out, 'a')
def emit(r):
    r['utc'] = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()); out.write(json.dumps(r) + '\n'); out.flush(); print(json.dumps(r), flush=True)
LS = (19, 43, 107, 227)
inst_l = {l: mod_l_instances(l) for l in LS}; inst_p = p12289_instances()
emit(dict(event='instances', mod_l=inst_l, p12289=inst_p, note='Fermat moduli 17, 97, 257 admit no unit instance (0/60 draws each, k = 6 and 10): F_l^x = mu_{l-1} lies inside mu_N; redesign of 2026-11-12 03:55 UTC, PROTOCOL.md S1 amendment'))
# arm (a): control mod 12289, one generator, k = 4..9 D <= 4, D = 5 for k <= 7
for ins in inst_p[:2]:
    for k in range(4, 10):
        for D in (3, 4, 5):
            if D == 5 and k > 7: continue
            g = chain_gen(ins['p'], ins['a'], ins['b'], ins['t'], k)
            emit(run_cell(dict(arm='a', **ins, k=k, D=D, law=law(k), inputs=['g']), ins['p'], [g], k, D))
# arms (b), (c) mod l
for l, lst in inst_l.items():
    for ins in lst:
        for k in range(4, 10):
            for D in (3, 4, 5):
                if D == 5 and k > 7: continue
                g = chain_gen(l, ins['a'], ins['b'], ins['t'], k); s = sigma(g, l, k)
                emit(run_cell(dict(arm='b', **ins, k=k, D=D, law=law(k), inputs=['g']), l, [g], k, D))
                sw = max(G.popcount(e1) + G.popcount(e2) for e1, e2 in s)
                if sw <= D:
                    emit(run_cell(dict(arm='c', **ins, k=k, D=D, law=law(k), inputs=['g', f'sigma_{l} g'], sigma_weight=sw), l, [g, s], k, D))
                else:
                    emit(dict(arm='c', **ins, k=k, D=D, law=law(k), inputs=['g', f'sigma_{l} g'], sigma_weight=sw, skipped='sigma weight above D'))
# arm (d) at k = 6 mod 12289: d1 {g,h}, d2 {g, sigma_s g}, d0 {g, x1 g}
k = 6; p = 12289
for ins in inst_p:
    g = chain_gen(p, ins['a'], ins['b'], ins['t'], k)
    i = 0
    while True:
        i += 1; rng = np.random.default_rng(seed_of(f's1-h-{i}'))
        h = {}
        for e1 in range(3):
            for e2 in range(e1, 3):
                v = int(rng.integers(1, p)); h[(e1, e2)] = v; h[(e2, e1)] = v
        if common_zero_free(p, g, h, k): break
    s = 1 + (1 << (k - 2)); sg = sigma(g, s, k); d2ok = common_zero_free(p, g, sg, k)
    x1g = {((e1 + 1) % (1 << k), e2): v for (e1, e2), v in g.items()}
    for D in (3, 4, 5):
        emit(run_cell(dict(arm='d1', **ins, k=k, D=D, law=law(k), inputs=['g', 'h'], h_draw=i), p, [g, h], k, D))
        emit(run_cell(dict(arm='d2', **ins, k=k, D=D, law=law(k), inputs=['g', f'sigma_{s} g'], d2_common_zero_free=d2ok), p, [g, sg], k, D))
        emit(run_cell(dict(arm='d0', **ins, k=k, D=D, law=law(k), inputs=['g', 'x1 g']), p, [g, x1g], k, D))
emit(dict(event='done'))
