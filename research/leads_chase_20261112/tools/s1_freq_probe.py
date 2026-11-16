#!/usr/bin/env python3
"""S1 follow-up (post hoc, exploratory): frequency of departures from the large-char closure at (k, D) = (6, 4) and
(7, 4) among unit instances over small primes with v_2(l - 1) = 1, and over 40961 as the large-char control.
Departure = dim V_4(6) != 605 or dim V_4(7) != 829 (large-char reference, instance-independent); records target order."""
import sys, json, hashlib, numpy as np
sys.path.insert(0, '/home/user/ml-cryptanalysis/scripts'); sys.path.insert(0, 'tools')
import ic_bounds_growth_lib as G
from unit_test import resultant_unit, s3_coeffs
def gen(p, a, b, t, k):
    N = 1 << k; g = {}
    for (e1, e2), v in s3_coeffs(p, a, b, t).items():
        key = (e1 % N, e2 % N); g[key] = (g.get(key, 0) + v) % p
    return {e: v for e, v in g.items() if v}
def tgt_order(p, a, b, t):
    f = (t ** 3 + a * t + b) % p
    y = next((y for y in range(p) if y * y % p == f), None) if p < 5000 else None
    if y is None: return None
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2 and (y1 + y2) % p == 0: return None
        l = ((3 * x1 * x1 + a) * pow(2 * y1, p - 2, p) if P == Q else (y2 - y1) * pow(x2 - x1, p - 2, p)) % p
        x3 = (l * l - x1 - x2) % p; return (x3, (l * (x1 - x3) - y1) % p)
    R = (t, y); Q, n = R, 1
    while Q is not None: Q = add(Q, R); n += 1
    return n
out = open('s1/freq_probe.jsonl', 'a')
for l, n_inst in ((19, 30), (23, 30), (43, 30), (107, 20), (227, 20)):
    got = 0; i = 0; dep = 0
    while got < n_inst:
        i += 1; rng = np.random.default_rng(int(hashlib.sha256(f'freq-{l}-{i}'.encode()).hexdigest()[:8], 16))
        a, b, t = (int(x) for x in rng.integers(1, l, size=3))
        if (4 * a ** 3 + 27 * b ** 2) % l == 0 or len(s3_coeffs(l, a, b, t)) != 9: continue
        if (t ** 3 + a * t + b) % l == 0: continue                    # skip order-2 targets (known degenerate)
        if not resultant_unit(l, a, b, t, 7)['unit']: continue
        rec = dict(p=l, a=a, b=b, t=t, target_order=tgt_order(l, a, b, t))
        for k in (6, 7):
            L = G.Layout(k, 4); B, piv = G.closure(L, gen(l, a, b, t, k), l)
            rec[f'dimV4_k{k}'] = len(piv); rec[f'one_k{k}'] = bool(L.idx[(0, 0)] in set(int(c) for c in piv))
        rec['departs'] = rec['dimV4_k6'] != 605 or rec['dimV4_k7'] != 829
        dep += rec['departs']; got += 1
        out.write(json.dumps(rec) + '\n'); out.flush()
    print(f'l={l}: {dep}/{n_inst} unit instances (non-order-2 targets) depart from the large-char closure at k = 6 or 7', flush=True)
