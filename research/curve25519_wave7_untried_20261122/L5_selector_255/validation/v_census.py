#!/usr/bin/env python3
"""Validator: independent frame census.
- builds a full log table with the validator's own affine Montgomery arithmetic and its own generator
  (logs differ from the lead's by a unit mod 8r; J and covered are invariant under that);
- k20: replays the frame-search k = 20 RNG stream (random.Random(20261103 + 20), t = 4, 5, 6; translate and
  mobius draws, mobius coverage not computed = "reduced census") and recomputes the published PC1 numbers;
- verifies a random sample of rows (plus every 2-torsion row) of the lead's data/frames/<curve>_t*.npz;
- counts double-and-add and x-only-ladder operations for [r]P and checks label = log mod 8.
Usage: v_census.py <curve> <out.json> [rows_per_cell]"""
import sys, os, json, random, math, time
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from v_common import BASE, CURV

cur = sys.argv[1]; OUT = sys.argv[2]; NS = int(sys.argv[3]) if len(sys.argv) > 3 else 150
p, A, n = CURV[cur]; r = n // 8
t0 = time.time()

def add(P, Q):
    if P is None: return Q
    if Q is None: return P
    u1, v1 = P; u2, v2 = Q
    if u1 == u2:
        if (v1 + v2) % p == 0: return None
        lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1 % p, -1, p) % p
    else:
        lam = (v2 - v1) * pow((u2 - u1) % p, -1, p) % p
    u3 = (lam * lam - A - u1 - u2) % p
    return (u3, (lam * (u1 - u3) - v1) % p)

def mul(k, P):
    R = None
    for bit in bin(k)[2:]:
        R = add(R, R)
        if bit == '1': R = add(R, P)
    return R

gens = {d['curve']: d['gen'] for d in json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'curves_out.json'))) if 'gen' in d}
g = tuple(gens[cur])
assert mul(n, g) is None and mul(n // 2, g) is not None and mul(n // r, g) is not None
# log table over the Montgomery u coordinate: logtab[u] = k in [1, n/2] with x(kg) = u
logtab = np.zeros(p, dtype=np.int32)
P = None
for k in range(1, n // 2 + 1):
    P = add(P, g)
    logtab[P[0]] = k
assert P == (0, 0)
res = dict(curve=cur, table_seconds=round(time.time() - t0, 1))
A3 = A * pow(3, -1, p) % p

def logs_of_frame_x(L, t):
    """Translation frame in x_W = u + A/3: x = L + y, y in [0, 2^t)."""
    xs = (L + np.arange(1 << t, dtype=np.int64)) % p
    us = (xs - A3) % p
    k = logtab[us].astype(np.int64)
    return np.unique(k[k > 0])

def stats(logs, with_cov):
    S = np.concatenate([logs, (-logs) % n])
    lab = S % 8
    h = np.bincount(lab, minlength=8)
    # unordered pairs (i <= j) of signed points whose labels cancel mod 8
    tot = h[0] * (h[0] + 1) // 2 + h[4] * (h[4] + 1) // 2 + h[1] * h[7] + h[2] * h[6] + h[3] * h[5]
    # minus pairs whose sum is O (log = 0 mod n): count explicitly
    cnt = {}
    for s in S.tolist(): cnt[s] = cnt.get(s, 0) + 1
    zero = 0
    for s, c in cnt.items():
        s2 = (-s) % n
        if s2 == s: zero += c * (c + 1) // 2      # s = -s (2-torsion; s = 0 never occurs)
        elif s < s2: zero += c * cnt.get(s2, 0)
    J = int(tot - zero)
    folded = np.minimum(logs % 8, 8 - logs % 8)
    m = np.bincount(folded, minlength=5)[:5]
    cov = -1
    if with_cov:
        pts = set()
        Sl = S.tolist(); labl = lab.tolist()
        bylab = {a: [s for s, l in zip(Sl, labl) if l == a] for a in range(8)}
        for a in range(8):
            b = (-a) % 8
            if a > b: continue
            A_ = np.array(bylab[a], dtype=np.int64); B_ = np.array(bylab[b], dtype=np.int64)
            if len(A_) == 0 or len(B_) == 0: continue
            sums = (A_[:, None] + B_[None, :]) % n
            pts.update(sums[sums != 0].ravel().tolist())
        cov = len(pts)
    return len(logs), m, J, cov

# ---- PC1 reduced replay (k20 only) ----
if cur == 'k20':
    rng = random.Random(20261103 + 20); K = 2000
    pc1 = {}
    for t in (4, 5, 6):
        Ls = [rng.randrange(p) for _ in range(K)]
        for _ in range(K):                       # mobius draws consumed, coverage not computed
            while True:
                F = tuple(rng.randrange(p) for _ in range(4))
                if (F[0] * F[3] - F[1] * F[2]) % p: break
        if t == 4: continue
        rows = []
        for L in Ls:
            c, m, J, cov = stats(logs_of_frame_x(L, t), True)
            rows.append((L, c, J, cov))
        c0, m0, J0, cov0 = stats(logs_of_frame_x(0, t), True)
        best = sorted(rows, key=lambda x: (-x[2], -x[1], x[0]))[0]
        mean = sum(x[3] for x in rows) / len(rows)
        pc1[t] = dict(selected_L=best[0], selected_cols=best[1], selected_J=best[2], selected_covered=best[3], trans_mean=mean,
                      gain_vs_mean=best[3] / mean, bits_lo0_covered=cov0, bits_lo0_cols=c0, gain_vs_bits_lo0=best[3] / cov0,
                      oracle_best_covered=max(x[3] for x in rows))
    res['PC1_replay'] = pc1

# ---- verify sampled rows of the lead's data ----
ver = {}
rs = np.random.default_rng(777)
for t in range(4, 15):
    f = os.path.join(BASE, 'data', 'frames', f'{cur}_t{t}.npz')
    if not os.path.exists(f): continue
    z = np.load(f); rows = z['rows']; tors = z['tors']
    idx = sorted(set([0] + rs.choice(len(rows), size=min(NS, len(rows)), replace=False).tolist() + np.where(tors == 1)[0].tolist()))
    mism = []; unit_swaps = set()
    for i in idx:
        L = int(rows[i, 0]); with_cov = rows[i, 9] >= 0
        c, m, J, cov = stats(logs_of_frame_x(L, t), with_cov)
        d = rows[i]
        m_ok = (m[0] == d[2] and m[2] == d[4] and m[4] == d[6] and sorted([m[1], m[3]]) == sorted([d[3], d[5]]))
        if m[1] != m[3]: unit_swaps.add(bool(m[1] == d[5] and m[1] != d[3]))
        if not (c == d[1] and m_ok and J == d[7] and (not with_cov or cov == d[9])):
            mism.append(dict(i=i, L=L, mine=[c, m.tolist(), J, cov], data=d.tolist()))
    ver[t] = dict(rows_checked=len(idx), torsion_rows=int(tors.sum()), mismatches=len(mism), first=mism[:3],
                  class13_swapped=sorted(unit_swaps))
res['row_verification'] = ver

# ---- [r]P cost and label = log mod 8 ----
T = mul(r, g)  # order 8, label 1 relative to my generator
E8 = {}
Q = None
for i in range(8):
    E8[Q] = i; Q = add(Q, T)
rc = np.random.default_rng(4242); ok = 0; N = 300
for _ in range(N):
    k = int(rc.integers(1, n)); Pk = mul(k, g)
    ok += int(E8[mul(r, Pk)] == k % 8)
res['label_equals_log_mod8'] = [ok, N]
bl = r.bit_length(); pc = bin(r).count('1')
res['r'] = r; res['r_bits'] = bl; res['r_popcount'] = pc
res['double_and_add_ops'] = (bl - 1) + (pc - 1)
res['double_and_add_over_log2r'] = ((bl - 1) + (pc - 1)) / math.log2(r)
res['xonly_ladder_ops'] = 2 * bl
res['xonly_ladder_over_log2r'] = 2 * bl / math.log2(r)
res['seconds'] = round(time.time() - t0, 1)
json.dump(res, open(OUT, 'w'), indent=1, default=int)
print(json.dumps(res, default=int)[:3000])
