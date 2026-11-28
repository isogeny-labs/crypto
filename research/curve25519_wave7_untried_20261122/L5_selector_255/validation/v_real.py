#!/usr/bin/env python3
"""Validator: independent relabelling of real-Curve25519 frames with the validator's own affine (u, v) arithmetic
and its own 8-torsion reference (no x-only ladder), regenerating the lead's frames from the PROTOCOL seed rule.
Checks: R1 rows (t = 6: first M6 frames, t = 8: first M8), R1c column counts (t = 12, 16), R2 frames 0 and 1
(counts and trials per half), the smoke R2 frame-0 overlap, and the parity identity on every label computed."""
import sys, os, json, random, time
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from v_common import BASE, seed_rule

P = 2 ** 255 - 19; A = 486662
R = 2 ** 252 + 27742317777372353535851937790883648493
M6, M8, M12, M16, NR2 = (int(x) for x in sys.argv[2:7]) if len(sys.argv) > 6 else (100, 10, 30, 3, 2)
OUT = sys.argv[1]
SQ = pow(2, (P - 1) // 4, P)

def leg(a):
    a %= P
    if a == 0: return 0
    return 1 if pow(a, (P - 1) // 2, P) == 1 else -1

def sqrt5(a):
    a %= P
    x = pow(a, (P + 3) // 8, P)
    if x * x % P != a: x = x * SQ % P
    assert x * x % P == a
    return x

def add(P1, Q1):
    if P1 is None: return Q1
    if Q1 is None: return P1
    u1, v1 = P1; u2, v2 = Q1
    if u1 == u2:
        if (v1 + v2) % P == 0: return None
        lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1 % P, -1, P) % P
    else:
        lam = (v2 - v1) * pow((u2 - u1) % P, -1, P) % P
    u3 = (lam * lam - A - u1 - u2) % P
    return (u3, (lam * (u1 - u3) - v1) % P)

def mul(k, Pt):
    Rr = None
    for b in bin(k)[2:]:
        Rr = add(Rr, Rr)
        if b == '1': Rr = add(Rr, Pt)
    return Rr

def f(u): return (u * u % P * u + A * u * u + u) % P

# own 8-torsion reference
rr = random.Random(55555)
while True:
    u = rr.randrange(2, P)
    if leg(f(u)) != 1: continue
    T = mul(R, (u, sqrt5(f(u))))
    if T is None: continue
    T2 = add(T, T)
    if T2 is None or T2[0] == 0: continue
    break
uT1 = T[0]; uT3 = mul(3, T)[0]; uT2 = T2[0]
assert add(T2, T2) == (0, 0)
NL = [0]; PAR = [0]
def label(u):
    v = sqrt5(f(u)); Q = mul(R, (u, v)); NL[0] += 1
    if Q is None: c = 0
    elif Q[0] == 0: c = 4
    elif Q[0] == uT2: c = 2
    elif Q[0] == uT1: c = 1
    elif Q[0] == uT3: c = 3
    else: raise SystemExit('label outside E[8]')
    PAR[0] += int((1 if c in (0, 2, 4) else -1) != leg(u))
    return c

def frame(L, t, labels=True):
    m = [0] * 5; c = 0
    for y in range(1 << t):
        u = (L + y) % P
        if u == 0: continue
        if leg(f(u)) != 1: continue
        c += 1
        if labels: m[label(u)] += 1
    return c, m

Z = np.load(os.path.join(BASE, 'data', 'real25519.npz'))
S = np.load(os.path.join(BASE, 'smoke', 'real25519.npz'))
res = {}; t0 = time.time()
for t, M in ((6, M6), (8, M8)):
    rng = random.Random(seed_rule('real', 'r1', t))
    Ls = [rng.randrange(P) for _ in range(M)]
    data = Z[f'r1_t{t}']
    exact = 0; swap = 0; c_eq = 0; mism = []
    for i, L in enumerate(Ls):
        c, m = frame(L, t)
        d = data[i].tolist()
        c_eq += int(c == d[0])
        if [c] + m == d: exact += 1
        elif [c, m[0], m[3], m[2], m[1], m[4]] == d: swap += 1
        else: mism.append(dict(i=i, mine=[c] + m, data=d))
    res[f'r1_t{t}'] = dict(frames=M, c_equal=c_eq, rows_equal=exact, rows_equal_with_class13_swap=swap, mismatches=len(mism), first=mism[:3])
for t, M in ((12, M12), (16, M16)):
    rng = random.Random(seed_rule('real', 'r1c', t)); data = Z[f'r1c_t{t}']; eq = 0
    for i in range(M):
        L = rng.randrange(P); c, _ = frame(L, t, labels=False); eq += int(c == int(data[i]))
    res[f'r1c_t{t}'] = dict(frames=M, equal=eq)
# R2: frames 0..NR2-1 with the lead's sampling order; also the smoke overlap (first 64 labels of frame 0)
rng = random.Random(seed_rule('real', 'r2', 128))
r2 = []
smoke_cnt = S['r2_counts'][0].tolist(); smoke_tri = S['r2_trials'][0].tolist(); smoke_n = int(S['r2_counts'][0].sum())
for fidx in range(NR2):
    L = rng.randrange(P); got = 0; cnt = [[0] * 5, [0] * 5]; tri = [0, 0]; snap = None
    while got < 4096:
        u = (L + rng.randrange(1 << 128)) % P
        tri[got % 2] += 1
        if u == 0 or leg(f(u)) != 1: continue
        c = label(u); cnt[got % 2][c] += 1; got += 1
        if fidx == 0 and got == smoke_n: snap = ([x[:] for x in cnt], tri[:])
    d_cnt = Z['r2_counts'][fidx].tolist(); d_tri = Z['r2_trials'][fidx].tolist()
    sw = lambda h: [h[0], h[3], h[2], h[1], h[4]]
    ent = dict(frame=fidx, trials_equal=tri == d_tri, counts_equal=cnt == d_cnt, counts_equal_with_swap=[sw(cnt[0]), sw(cnt[1])] == d_cnt)
    if fidx == 0 and snap is not None:
        # the smoke run stopped after smoke_n labels; its trial counter stops at the smoke_n-th success
        ent['smoke_frame0_counts_equal'] = snap[0] == smoke_cnt or [sw(snap[0][0]), sw(snap[0][1])] == smoke_cnt
        ent['smoke_frame0_trials_equal'] = snap[1] == smoke_tri
    r2.append(ent)
res['r2'] = r2
res['labels_computed'] = NL[0]; res['parity_mismatches'] = PAR[0]; res['seconds'] = round(time.time() - t0, 1)
json.dump(res, open(OUT, 'w'), indent=1)
print(json.dumps(res, indent=1))
