"""Validator's own helpers (written from PROTOCOL.md definitions; nothing imported from the lead's scripts)."""
import hashlib, math, csv, json, os
import numpy as np

BASE = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L5_selector_255'
CURV = {'k20': (1048573, 926, 1049624), 'k24f': (16777141, 7842, 16784072), 'k24r': (16777213, 318, 16778792)}

def seed_rule(*parts):
    """PROTOCOL.md: first 8 bytes of sha256('L5|' + '|'.join(parts))."""
    h = hashlib.sha256(('L5|' + '|'.join(str(x) for x in parts)).encode()).digest()
    return int.from_bytes(h[:8], 'big')

_W = {}
def max_weights(N, K):
    """Exact P(rank i (1..N, ascending) is the max of a uniform K-subset) = C(i-1, K-1)/C(N, K), via big ints."""
    if (N, K) in _W: return _W[(N, K)]
    tot = math.comb(N, K)
    w = np.zeros(N)
    c = 1  # C(K-1, K-1)
    for i in range(K, N + 1):
        w[i - 1] = c / tot
        c = c * i // (i - K + 1)   # C(i, K-1) from C(i-1, K-1)
    _W[(N, K)] = w
    return w

def order_stable(keys):
    """Ascending order by keys (most significant first) using successive stable argsorts (LSD radix style)."""
    keys = [np.asarray(k) for k in keys]
    idx = np.arange(len(keys[0]))
    for k in reversed(keys):
        idx = idx[np.argsort(k[idx], kind='stable')]
    return idx

def gain_U(values, keys, K):
    values = np.asarray(values, float)
    o = order_stable(keys)
    w = max_weights(len(values), K)
    return float(np.dot(w, values[o]) / values.mean())

def gain_U_py(values, keys, K):
    """Pure-Python cross-check: sort tuples."""
    N = len(values)
    tup = sorted(range(N), key=lambda i: tuple(float(k[i]) for k in keys))
    w = max_weights(N, K)
    s = sum(w[j] * float(values[tup[j]]) for j in range(N))
    return s / (sum(float(v) for v in values) / N)

def J_of_counts(m0, m1, m2, m3, m4):
    c = m0 + m1 + m2 + m3 + m4
    return m1 * m1 + m2 * m2 + m3 * m3 + m0 * (2 * m0 + 1) + m4 * (2 * m4 + 1) - c

def S1_of_counts(m0, m1, m2, m3, m4):
    c = m0 + m1 + m2 + m3 + m4
    e = 2 * (m0 + m2 + m4); o = 2 * (m1 + m3)
    return e * (e + 1) // 2 + o * (o + 1) // 2 - c

def read_gains_csv():
    out = {}
    with open(os.path.join(BASE, 'analysis', 'gains.csv')) as f:
        rd = csv.DictReader(f)
        for r in rd:
            key = (r['arm'], r['curve'], int(r['t']), r['selector'], r['value'], int(r['K']))
            out[key] = r
    return out
