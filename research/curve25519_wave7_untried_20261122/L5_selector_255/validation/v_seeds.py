#!/usr/bin/env python3
"""Validator: (1) the frame L values and iid-surrogate counts of every arm F cell follow the PROTOCOL seed rule;
(2) PC3 pools 0..P-1 at t = 5, 6 regenerated with the validator's own k20 log table reproduce the recorded realised
gains and the pair U-statistics (the input of the calibration factor f)."""
import sys, os, json, glob, re, random, math
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from v_common import BASE, CURV, seed_rule, gain_U

OUT = sys.argv[1]; NP = int(sys.argv[2]) if len(sys.argv) > 2 else 10
res = {'frames_seed_rule': {}}
PROBS = [1 / 8, 1 / 4, 1 / 4, 1 / 4, 1 / 8]
for f in sorted(glob.glob(os.path.join(BASE, 'data', 'frames', '*.npz'))):
    cur, t = re.match(r'(.+)_t(\d+)\.npz', os.path.basename(f)).groups(); t = int(t)
    z = np.load(f); rows = z['rows']; N = len(rows) - 1; p = CURV[cur][0]
    rng = np.random.default_rng(seed_rule('frames', cur, t))
    Ls = np.concatenate([[0], rng.integers(0, p, size=N)])
    rho = rows[1:, 1].mean() / 2 ** t
    srng = np.random.default_rng(seed_rule('surrogate', cur, t))
    cs = srng.binomial(1 << t, rho, size=N + 1)
    ms = np.array([srng.multinomial(c, PROBS) for c in cs])
    res['frames_seed_rule'][f'{cur}_t{t}'] = dict(L_equal=bool(np.array_equal(Ls, rows[:, 0])), rho_equal=bool(abs(rho - float(z['rho'])) < 1e-15),
                                                  surrogate_equal=bool(np.array_equal(ms, z['surrogate_counts'])))

# own k20 log table (same construction as v_census.py)
p, A, n = CURV['k20']; r = n // 8
def add(P1, Q1):
    if P1 is None: return Q1
    if Q1 is None: return P1
    u1, v1 = P1; u2, v2 = Q1
    if u1 == u2:
        if (v1 + v2) % p == 0: return None
        lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1 % p, -1, p) % p
    else:
        lam = (v2 - v1) * pow((u2 - u1) % p, -1, p) % p
    u3 = (lam * lam - A - u1 - u2) % p
    return (u3, (lam * (u1 - u3) - v1) % p)
g = tuple(next(d for d in json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'curves_out.json'))) if d['curve'] == 'k20')['gen'])
logtab = np.zeros(p, dtype=np.int64); Pt = None
for k in range(1, n // 2 + 1):
    Pt = add(Pt, g); logtab[Pt[0]] = k
A3 = A * pow(3, -1, p) % p
def frame(L, t):
    us = ((L + np.arange(1 << t)) - A3) % p
    k = logtab[us]; logs = np.unique(k[k > 0])
    S = np.concatenate([logs, (-logs) % n])
    sums = (S[:, None] + S[None, :]) % n
    ok = (sums % 8 == 0) & (sums != 0)
    J = int(np.triu(ok).sum()); cov = int(np.unique(sums[ok]).size)
    return len(logs), J, cov
pools = json.load(open(os.path.join(BASE, 'data', 'pc3_pools.json')))
pc3 = {}
for cell in pools['cells']:
    t = cell['t']; rng = np.random.default_rng(seed_rule('pc3pools', t)); K = pools['K']
    mine = []; ust = []
    P_ = []
    for q in range(NP):
        Ls = rng.integers(0, p, size=K)
        R = np.array([(int(L),) + frame(int(L), t) for L in Ls])     # L c J cov
        P_.append(R)
        best = sorted(range(K), key=lambda i: (-R[i, 2], -R[i, 1], R[i, 0]))[0]
        mine.append(R[best, 3] / R[:, 3].mean())
    for q in range(0, NP - 1, 2):
        R = np.vstack([P_[q], P_[q + 1]])
        ust.append(gain_U(R[:, 2], [R[:, 2], R[:, 1], -R[:, 0]], K))
    pc3[t] = dict(pools=NP, realised_max_abs_diff=float(np.max(np.abs(np.array(mine) - np.array(cell['realised'][:NP])))),
                  ustat_max_abs_diff=float(np.max(np.abs(np.array(ust) - np.array(cell['ustat_J'][:len(ust)])))))
res['pc3_regen'] = pc3
json.dump(res, open(OUT, 'w'), indent=1)
print(json.dumps(dict(all_L=all(v['L_equal'] for v in res['frames_seed_rule'].values()),
                      all_rho=all(v['rho_equal'] for v in res['frames_seed_rule'].values()),
                      all_surr=all(v['surrogate_equal'] for v in res['frames_seed_rule'].values()),
                      cells=len(res['frames_seed_rule']), pc3=pc3)))
