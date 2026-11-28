#!/usr/bin/env python3
"""Shared code for lead L5 (frame and seed selectors toward 255 bits).

Selector definitions are copied from research/curve25519_frame_search_20261103/scripts/frame_census.py
(sha256 afcf2cc3...; verbatim copy in scripts/frame_census_orig.py) and from
research/curve25519_descent_skip_20261114/frame_selection.py (one-bit score S1). Nothing is imported
from those directories at run time.

Definitions (frame-search):
  curve       short Weierstrass model x = u + A/3 of Montgomery v^2 = u^3 + A u^2 + u, #E = 8r cyclic
  frame       translation frame (1, L, 0, 1): base = { P : x(P) = L + y, y in [0, 2^t) }
  logs        +-folded full logs in Z/8r of the base points (one per column)
  label       log mod 8 (identifies [r]P among the 8 rational 8-torsion points; no discrete log needed)
  J           signed pairs (unordered, i = j allowed, P + (-P) excluded) whose labels cancel mod 8
  covered     distinct points +-P_i +-P_j in E[r] \\ {O} (the engine's coverage_exact numerator)
  selection   argmax J, ties by cols (desc), then lowest L
  S1          (descent-skip) signed pairs (i <= j, P + (-P) excluded) with label sum even
"""
import hashlib, math, random
import numpy as np

CURVES = {
    # name: (k, p, A, n = #E) ; facts re-verified by scripts/verify_curves.py
    'k20':  (20, 1048573, 926, 1049624),
    'k24f': (24, 16777141, 7842, 16784072),   # brief's "faithful" k = 24 member (wave 6 L1); n checked by verify_curves.py
    'k24r': (24, 16777213, 318, 16778792),    # family-rule k = 24 (frame search, iso-census)
}

def derived_seed(*parts):
    """Seeds by a derived rule: first 8 bytes of sha256('L5|' + '|'.join(parts))."""
    return int(hashlib.sha256(('L5|' + '|'.join(str(x) for x in parts)).encode()).hexdigest()[:16], 16)

# ---- copied from frame_census.py (build), unchanged except for returning n ----
def sqrt_mod(a, p):
    a %= p
    if a == 0: return 0
    if pow(a, (p - 1) // 2, p) != 1: return None
    if p % 8 == 5:
        x = pow(a, (p + 3) // 8, p)
        if x * x % p != a: x = x * pow(2, (p - 1) // 4, p) % p
        return x
    raise ValueError

def build(p, A, n):
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        (u1, v1), (u2, v2) = P, Q
        if u1 == u2:
            if (v1 + v2) % p == 0: return None
            lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1, -1, p) % p
        else:
            lam = (v2 - v1) * pow(u2 - u1, -1, p) % p
        u3 = (lam * lam - A - u1 - u2) % p
        return (u3, (lam * (u1 - u3) - v1) % p)
    def mul(k, P):
        R = None
        while k:
            if k & 1: R = add(R, P)
            P = add(P, P); k >>= 1
        return R
    r = n // 8
    rng = random.Random(1)
    for _try in range(10000):
        u = rng.randrange(2, p); v = sqrt_mod(u * u % p * u + A * u * u + u, p)
        if v is None: continue
        g = (u, v)
        if mul(n // 2, g) is not None and mul(n // r, g) is not None: break
    else:
        raise SystemExit('E(F_p) is not cyclic of order 8r: curve out of scope')
    logu = np.zeros(p, dtype=np.int64)
    P = None
    for k in range(1, n // 2 + 1):
        P = add(P, g)
        if P is None: break
        logu[P[0]] = k
    return logu, r, g, add, mul

# ---- frame statistics ----
def frame_logs_translate(p, A3, logu, L, t):
    """+-folded logs of base points of the translation frame (1, L, 0, 1), as frame_census.frame_logs."""
    x = (L + np.arange(1 << t, dtype=np.int64)) % p
    k = logu[(x - A3) % p]
    return np.unique(k[k > 0])

def class_counts(logs):
    """Folded label classes 0,1,2,3,4 (labels {0},{1,7},{2,6},{3,5},{4}) per column."""
    lab = logs % 8
    fold = np.minimum(lab, 8 - lab)
    return np.bincount(fold, minlength=5)[:5]

def J_from_counts(m):
    """J of frame_census.coverage from the folded class counts (exact when no 2-torsion column)."""
    m0, m1, m2, m3, m4 = (int(v) for v in m)
    c = m0 + m1 + m2 + m3 + m4
    return m1 * m1 + m2 * m2 + m3 * m3 + m0 * (2 * m0 + 1) + m4 * (2 * m4 + 1) - c

def S1_from_counts(m):
    """Descent-skip one-bit score S1 from the folded class counts."""
    c = int(sum(m)); e = 2 * int(m[0] + m[2] + m[4]); o = 2 * int(m[1] + m[3])
    return e * (e + 1) // 2 + o * (o + 1) // 2 - c

def J_exact(logs, n):
    """frame_census.coverage's J (numpy triu), for checks."""
    if len(logs) == 0: return 0
    S = np.concatenate([logs, (-logs) % n]); sums = (S[:, None] + S[None, :]) % n
    sub = (sums % 8 == 0) & (sums != 0)
    return int(np.triu(sub).sum())

def covered_exact(logs, n):
    """Distinct E[r]\\{O} points +-P_i +-P_j, computed by label groups (equal to frame_census covered)."""
    if len(logs) == 0: return 0
    S = np.concatenate([logs, (-logs) % n]); lab = S % 8
    groups = [S[lab == a] for a in range(8)]
    parts = []
    for a in range(5):
        b = (8 - a) % 8
        if len(groups[a]) and len(groups[b]):
            s = np.add.outer(groups[a], groups[b]).ravel() % n
            parts.append(s[s != 0])
    if not parts: return 0
    return int(np.unique(np.concatenate(parts)).size)

# ---- selection estimators ----
def _selection_weights(N, K):
    """P(rank i is the max of a uniform random K-subset of N), i = 1..N ascending."""
    lc = lambda a, b: math.lgamma(a + 1) - math.lgamma(b + 1) - math.lgamma(a - b + 1)
    lCNK = lc(N, K)
    w = np.zeros(N)
    for i in range(K, N + 1):             # rank i (1-based) is the max: C(i-1, K-1) / C(N, K)
        w[i - 1] = math.exp(lc(i - 1, K - 1) - lCNK)
    return w

_WCACHE = {}
def selection_weights(N, K):
    if (N, K) not in _WCACHE: _WCACHE[(N, K)] = _selection_weights(N, K)
    return _WCACHE[(N, K)]

def order_by_key(keys):
    """keys: list of arrays, most significant first, all 'larger is better' (use -L for lowest L).
    Returns indices sorted ascending (best last)."""
    return np.lexsort(tuple(reversed([np.asarray(k) for k in keys])))

def expected_selected(values, keys, K):
    """U-statistic: E[value of argmax-key frame over a uniform random K-subset]."""
    o = order_by_key(keys)
    return float(np.dot(selection_weights(len(o), K), np.asarray(values, dtype=float)[o]))

def gain(values, keys, K):
    return expected_selected(values, keys, K) / float(np.mean(values))
