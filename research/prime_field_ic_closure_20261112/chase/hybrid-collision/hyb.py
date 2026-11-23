#!/usr/bin/env python3
"""Toy: collision hybrids of index calculus (IC) and Pollard rho on prime-order
curves over F_p, 20-24 bits. Pure Python. Every number is [pcc:unverified].

Arms (all share one walk engine: van Oorschot-Wiener DP walks from aP+bQ):
  rho     : DP collision only.
  IC-G    : explicit sum table K over factor base F (V points, combos of <= m
            signed terms); every walk point is checked against K; each hit is a
            row  sum eps*c_i f_i - b x = a  (x = log Q).  Gaudry-style: stop
            at the (V+1)-th hit (guaranteed dependency).
  IC-1dep : same stream, stop at the first hit after which x is determined
            (incremental elimination; first dependency in the f-columns).
  HYB-U   : same stream, stop at min(first dependency, first DP collision).
Costs: group ops (1 GAE each) + native ops (lookups, DP tests/stores, table
inserts, LA mults mod n) priced at w in {0, 0.1, 1}.
"""
import random, sys, json, math, time

# ---------------- field / curve -----------------
def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

class Curve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b; s.ops = 0
    def add(s, P, Q):
        s.ops += 1
        if P is None: return Q
        if Q is None: return P
        p = s.p
        x1,y1 = P; x2,y2 = Q
        if x1 == x2:
            if (y1+y2) % p == 0: return None
            l = (3*x1*x1 + s.a) * pow(2*y1, -1, p) % p
        else:
            l = (y2-y1) * pow(x2-x1, -1, p) % p
        x3 = (l*l - x1 - x2) % p
        return (x3, (l*(x1-x3) - y1) % p)
    def neg(s, P): return None if P is None else (P[0], (-P[1]) % s.p)
    def mul(s, k, P):  # not counted (setup / verification only)
        R = None; ops0 = s.ops
        k %= s.n if hasattr(s,'n') else (1<<62)
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        s.ops = ops0
        return R
    def lift(s, x):
        p = s.p; r = (x*x*x + s.a*x + s.b) % p
        if r == 0: return (x, 0)
        if pow(r, (p-1)//2, p) != 1: return None
        y = sqrt_mod(r, p)
        return (x, min(y, p-y))

def sqrt_mod(a, p):
    if p % 4 == 3: return pow(a, (p+1)//4, p)
    # Tonelli-Shanks
    q, s = p-1, 0
    while q % 2 == 0: q//=2; s+=1
    z = 2
    while pow(z,(p-1)//2,p) != p-1: z += 1
    m, c, t, r = s, pow(z,q,p), pow(a,q,p), pow(a,(q+1)//2,p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        b = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, b*b % p, t*b*b % p, r*b % p
    return r

def gen_curve(bits, rng):
    """Random prime-order curve: find N in Hasse interval with N*P = O, N prime."""
    while True:
        p = rng.randrange(1 << (bits-1), 1 << bits) | 1
        if not is_prime(p): continue
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        E = Curve(p, a, b)
        P = None
        while P is None: P = E.lift(rng.randrange(p))
        lo, hi = p+1-2*math.isqrt(p)-2, p+1+2*math.isqrt(p)+2
        # BSGS for k in [lo,hi] with kP = O
        w = hi - lo
        m = math.isqrt(w) + 1
        baby = {}; R = None
        for j in range(m):
            baby.setdefault(R, j); R = E.add(R, P)
        G = E.mul(m, P); Y = E.mul(lo, P); cands = []
        for i in range(m+1):
            if Y in baby:
                k = lo + i*m - baby[Y]
                if lo <= k <= hi: cands.append(k)
            Y = E.add(Y, G)
        cands = sorted(set(cands))
        if len(cands) == 1 and is_prime(cands[0]) and cands[0] > 4*math.isqrt(p)+4:
            E.n = cands[0]; E.ops = 0
            assert E.mul(E.n, P) is None
            return E, P

# ---------------- incremental elimination mod n -----------------
class LA:
    """Rows over vars 0..V-1 (factor-base logs) and V (= x = log Q). Pivot row
    invariant: all non-pivot vars of a stored row are > its pivot var."""
    def __init__(s, V, n): s.V, s.n, s.piv, s.mults, s.x = V, n, {}, 0, None
    def add_row(s, row, const):
        n = s.n; row = {k: v % n for k, v in row.items() if v % n}; const %= n
        while True:
            fvars = [k for k in row if k < s.V]
            if not fvars: break
            v = min(fvars)
            if v in s.piv:
                prow, pc = s.piv[v]; c = row[v]
                for k, val in prow.items():
                    nv = (row.get(k, 0) - c*val) % n; s.mults += 1
                    if nv: row[k] = nv
                    else: row.pop(k, None)
                const = (const - c*pc) % n; s.mults += 1
            else:
                inv = pow(row[v], -1, n); s.mults += len(row) + 1
                s.piv[v] = ({k: val*inv % n for k, val in row.items()}, const*inv % n)
                return False
        cx = row.get(s.V, 0)
        if cx:
            s.x = const * pow(cx, -1, n) % n; s.mults += 2
            return True
        return False

# ---------------- factor base + sum table -----------------
def build_table(E, V, m, include_P=None):
    """F = first V points with smallest x (interval factor base), optionally
    with P prepended (known log 1). Table key x -> (y, coeff tuple).
    Returns table, F, free relations [(coeffs_dict, const)]."""
    F = []
    if include_P is not None: F.append(include_P)
    x = 0
    while len(F) < V:
        Pt = E.lift(x); x += 1
        if Pt is not None and Pt[1] != 0 and (include_P is None or Pt[0] != include_P[0]): F.append(Pt)
    ops0 = E.ops
    table = {}; free = []; inserts = 0
    # level-wise: entries (point, last_idx, last_sign, coeff tuple)
    level = [(None, -1, 0, ())]
    for k in range(1, m+1):
        nxt = []
        for (Y, li, ls, co) in level:
            for i in range(max(li, 0), V):
                for sg in ((1,) if k == 1 else (1, -1)):
                    if i == li and sg != ls: continue
                    Z = E.add(Y, F[i] if sg == 1 else E.neg(F[i]))
                    c2 = co + ((i, sg),)
                    nxt.append((Z, i, sg, c2))
                    if Z is None: continue
                    inserts += 1
                    if Z[0] in table:
                        y0, c0 = table[Z[0]]
                        eps = 1 if y0 == Z[1] else -1
                        # Z = sum c2 = eps * sum c0  -> free FB relation
                        r = {}
                        for (j, s_) in c2: r[j] = r.get(j, 0) + s_
                        for (j, s_) in c0: r[j] = r.get(j, 0) - eps*s_
                        free.append(r)
                    else:
                        table[Z[0]] = (Z[1], c2)
        level = nxt
    tops = E.ops - ops0
    return table, F, free, tops, inserts

# ---------------- walk engine -----------------
def run_instance(E, P, Q, x_true, rng, V, m, r=128, theta_bits=5, neg=False,
                 include_P=False, max_steps=None, use_table=True):
    n, p = E.n, E.p
    E.ops = 0
    res = {}
    if use_table:
        table, F, free, tops, inserts = build_table(E, V, m, P if include_P else None)
        la = LA(V, n)
        if include_P: la.add_row({0: 1}, 1)
        for rrow in free: la.add_row(rrow, 0)
        res.update(table_ops=tops, table_inserts=inserts, table_size=len(table), free_rel=len(free))
    else:
        table, la, tops, inserts = {}, None, 0, 0
        res.update(table_ops=0, table_inserts=0, table_size=0, free_rel=0)
    mults = [(rng.randrange(n), rng.randrange(n)) for _ in range(r)]
    M = [E.add(E.mul(c, P), E.mul(d, Q)) for c, d in mults]
    E.ops = tops
    dmask = (1 << theta_bits) - 1
    dps = {}
    steps = 0; lookups = 0; dptests = 0; dpstores = 0; hits = 0
    ev = {}  # event -> (steps, ops, lookups, dptests, dpstores, la_mults, hits)
    def snap():
        return dict(steps=steps, gops=E.ops, lookups=lookups, dptests=dptests,
                    dpstores=dpstores, la_mults=(la.mults if la else 0), hits=hits,
                    inserts=inserts)
    if la is not None and la.x is not None: ev['dep'] = snap()
    maxlen = 20 << theta_bits
    cap = max_steps or 60 * math.isqrt(n)
    def canon(W, a, b):
        if neg and W is not None and W[1] > p - W[1]:
            return (W[0], p - W[1]), (-a) % n, (-b) % n
        return W, a, b
    while steps < cap:
        a, b = rng.randrange(n), rng.randrange(n)
        W = E.add(E.mul(a, P), E.mul(b, Q))  # start: E.add counts 1 op (mul uncounted)
        W, a, b = canon(W, a, b)
        L = 0; hist = []
        while L < maxlen and steps < cap:
            if W is None: break
            j = W[0] % r
            Wn = E.add(W, M[j]); an, bn = (a + mults[j][0]) % n, (b + mults[j][1]) % n
            Wn, an, bn = canon(Wn, an, bn)
            if neg and Wn is not None and Wn[0] % r == j:  # look-ahead vs fruitless 2-cycles
                j2 = (j + 1) % r
                Wn = E.add(W, M[j2]); an, bn = (a + mults[j2][0]) % n, (b + mults[j2][1]) % n
                Wn, an, bn = canon(Wn, an, bn)
            W, a, b = Wn, an, bn; steps += 1; L += 1
            if W is None: break
            if neg:  # fruitless 2-/4-cycle detection, escape by doubling the min-x cycle point
                hist.append((W, a, b))
                if len(hist) > 5: hist.pop(0)
                cyc = None
                if len(hist) >= 3 and hist[-3][0] == W: cyc = hist[-3:-1]
                elif len(hist) >= 5 and hist[-5][0] == W: cyc = hist[-5:-1]
                if cyc:
                    Wm, am, bm = min(cyc, key=lambda t: t[0][0])
                    W = E.add(Wm, Wm); a, b = 2 * am % n, 2 * bm % n
                    W, a, b = canon(W, a, b); hist = []; steps += 1
                    if W is None: break
            # sum-table check
            if use_table:
                lookups += 1
                t = table.get(W[0])
                if t is not None:
                    hits += 1
                    y0, co = t; eps = 1 if y0 == W[1] else -1
                    row = {}
                    for (i, s_) in co: row[i] = row.get(i, 0) + eps*s_
                    row[V] = -b
                    ok = la.add_row(row, a)
                    if ok and 'dep' not in ev:
                        ev['dep'] = snap()
                        assert la.x == x_true, "LA solution wrong"
                    if hits == V + 1 and 'gaudry' not in ev:
                        ev['gaudry'] = snap()
            # DP check
            dptests += 1
            if (W[0] >> 3) & dmask == 0:
                dpstores += 1
                key = W[0] if neg else W
                prev = dps.get(key)
                if prev is not None and 'rho' not in ev:
                    a0, b0, y0 = prev
                    s_ = 1 if (not neg or y0 == W[1]) else -1
                    # a + b x = s(a0 + b0 x)
                    den = (b - s_*b0) % n
                    if den:
                        xs = (s_*a0 - a) * pow(den, -1, n) % n
                        assert xs == x_true
                        ev['rho'] = snap()
                else:
                    dps[key] = (a, b, W[1])
                break
        need = ['rho'] + (['dep', 'gaudry'] if use_table else [])
        if all(k in ev for k in need): break
    res['events'] = ev
    res['cap_hit'] = steps >= cap
    return res

def cost(s, w):
    if s is None: return None
    native = s['lookups'] + s['dptests'] + s['dpstores'] + s['la_mults'] + s['inserts']
    return s['gops'] + w * native

if __name__ == '__main__':
    pass
