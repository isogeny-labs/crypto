#!/usr/bin/env python3
"""Paired-translate (sibling) toy: counted field ops per candidate for IC
(m=2 negation-folded pair table, r-adding walk targets) and for rho-neg,
single vs paired x(R+-J) vs Montgomery-batched across L lanes.

Every field operation actually performed is tallied (I, M, S, A).  Table
probes, guard probes, inserts and LA row ops are tallied separately.
All results are [pcc:unverified] development-toy numbers.
"""
import random, sys, json, math

# ---------------------------------------------------------------- counting
class T:
    def __init__(s): s.c = dict(I=0, M=0, S=0, A=0, adds=0, dbls=0, second=0,
                                probe=0, guard=0, insert=0, rowop=0, cand=0, lift=0,
                                restart=0, cyc2=0, cap_restart=0)
    def __getitem__(s, k): return s.c[k]
    def inc(s, k, v=1): s.c[k] += v

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def sqrt_mod(a, p):
    a %= p
    if a == 0: return 0
    if pow(a, (p-1)//2, p) != 1: return None
    if p % 4 == 3: return pow(a, (p+1)//4, p)
    # Tonelli-Shanks
    q, s = p-1, 0
    while q % 2 == 0: q//=2; s+=1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        b = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, b*b % p, t*b*b % p, r*b % p
    return r

def count_points(p, a, b):
    n = 1
    for x in range(p):
        f = (x*x*x + a*x + b) % p
        if f == 0: n += 1
        else:
            n += 2 if pow(f, (p-1)//2, p) == 1 else 0
    return n

def find_curve(bits, seed):
    rng = random.Random(seed)
    while True:
        p = rng.randrange(1 << (bits-1), 1 << bits) | 1
        if p % 4 != 3 or not is_prime(p): continue
        for _ in range(40):
            a, b = rng.randrange(p), rng.randrange(1, p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            n = count_points(p, a, b)
            if is_prime(n): return p, a, b, n

# ---------------------------------------------------------------- curve (uncounted helpers for setup)
INF = None
class Curve:
    def __init__(s, p, a, b, n): s.p, s.a, s.b, s.n = p, a, b, n
    def add_u(s, P, Q):
        p = s.p
        if P is INF: return Q
        if Q is INF: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return INF
            l = (3*P[0]*P[0] + s.a) * pow(2*P[1], -1, p) % p
        else:
            l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
        x = (l*l - P[0] - Q[0]) % p
        return (x, (l*(P[0]-x) - P[1]) % p)
    def mul_u(s, k, P):
        R = INF; k %= s.n
        while k:
            if k & 1: R = s.add_u(R, P)
            P = s.add_u(P, P); k >>= 1
        return R
    def neg(s, P): return INF if P is INF else (P[0], (-P[1]) % s.p)
    # counted affine add, inversion supplied (for batching)
    def add_with_inv(s, t, P, Q, inv):
        p = s.p
        dy = (Q[1]-P[1]) % p; t.inc('A')
        l = dy*inv % p; t.inc('M')
        x = (l*l - P[0] - Q[0]) % p; t.inc('S'); t.inc('A', 2)
        y = (l*(P[0]-x) - P[1]) % p; t.inc('M'); t.inc('A', 2)
        return (x, y)
    def dbl_counted(s, t, P):
        p = s.p
        t.inc('dbls'); t.inc('I'); t.inc('M', 2); t.inc('S', 2); t.inc('A', 6)
        return s.add_u(P, P)

def charge_scalar(t, n, count=1):
    """Price one [a]G+[b]Q restart as 3*bits affine additions (GAE and field ops)."""
    k = 3 * n.bit_length() * count
    t.inc('adds', k); t.inc('I', k); t.inc('M', 2*k); t.inc('S', k); t.inc('A', 6*k)

def inv_c(t, p, v):
    t.inc('I'); return pow(v, -1, p)

def batch_inv(t, p, vals):
    """Montgomery's trick: 3(L-1) M + 1 I."""
    L = len(vals)
    if L == 1: return [inv_c(t, p, vals[0])]
    pre = [vals[0]]
    for v in vals[1:]: pre.append(pre[-1]*v % p); t.inc('M')
    acc = inv_c(t, p, pre[-1])
    out = [0]*L
    for i in range(L-1, 0, -1):
        out[i] = acc*pre[i-1] % p; t.inc('M')
        acc = acc*vals[i] % p; t.inc('M')
    out[0] = acc
    return out

# ---------------------------------------------------------------- IC: m=2 folded pair table
def factor_base(E, N):
    pts = []
    x = 0
    while len(pts) < N:
        f = (x**3 + E.a*x + E.b) % E.p
        y = sqrt_mod(f, E.p)
        if y is not None and f != 0: pts.append((x, y))
        x += 1
    return pts

def build_pair_table(E, t, fb):
    """Negation folded: key x(P_i +- P_j), i<=j, as the incumbent (about N(N+1) adds)."""
    table = {}
    N = len(fb)
    for i in range(N):
        for j in range(i, N):
            for sj in (1, -1):
                Pj = fb[j] if sj == 1 else E.neg(fb[j])
                if i == j and sj == -1: continue  # identity
                S = E.add_u(fb[i], Pj); t.inc('adds')
                # price as one affine add / double
                if i == j: t.inc('I'); t.inc('M', 2); t.inc('S', 2); t.inc('A', 6)
                else: t.inc('I'); t.inc('M', 2); t.inc('S'); t.inc('A', 6)
                if S is INF: continue
                t.inc('insert')
                table.setdefault(S[0], (i, j, sj, S[1]))
    return table

def solve_mod(rows, nunk, n, t):
    """rows: list of (dict col->coef, rhs).  Unknowns 0..nunk-1; return solution of unknown nunk-1 (d)."""
    # Gaussian elimination mod n on dense rows
    M = []
    for cols, rhs in rows:
        r = [0]*(nunk+1)
        for c, v in cols.items(): r[c] = (r[c] + v) % n
        r[nunk] = rhs % n
        M.append(r)
    piv_row = 0; where = [-1]*nunk
    for col in range(nunk):
        sel = None
        for r in range(piv_row, len(M)):
            if M[r][col]: sel = r; break
        if sel is None: continue
        M[piv_row], M[sel] = M[sel], M[piv_row]
        iv = pow(M[piv_row][col], -1, n)
        M[piv_row] = [v*iv % n for v in M[piv_row]]
        for r in range(len(M)):
            if r != piv_row and M[r][col]:
                f = M[r][col]
                M[r] = [(a - f*b) % n for a, b in zip(M[r], M[piv_row])]
                t.inc('rowop', nunk+1)
        where[col] = piv_row; piv_row += 1
    return where, M

def ic_run(E, G, Q, d_true, N, paired, L, seed, nj=16, excess=8):
    """Returns tally T, success flag.  Unknowns: x_0..x_{N-1} (logs of base pts), d."""
    rng = random.Random(seed)
    t = T(); p, n = E.p, E.n
    fb = factor_base(E, N)
    table = build_pair_table(E, t, fb)
    setup_adds = t['adds']
    # jumps
    jumps = []
    for _ in range(nj):
        al, be = rng.randrange(n), rng.randrange(1, n)
        J = E.add_u(E.mul_u(al, G), E.mul_u(be, Q))
        jumps.append((J, al, be))
    charge_scalar(t, n, nj)
    # lanes
    lanes = []
    for _ in range(L):
        a0, b0 = rng.randrange(n), rng.randrange(1, n)
        lanes.append([E.add_u(E.mul_u(a0, G), E.mul_u(b0, Q)), a0, b0, -1])
    charge_scalar(t, n, L)
    seen = set(); rows = []; rowkeys = set(); dup_rows = 0; minus_rows = 0; skipped_copy = 0
    nunk = N + 1
    restarts = 0
    def try_cand(x, yfun, a, b, is_minus):
        nonlocal dup_rows, minus_rows
        t.inc('cand'); t.inc('guard')
        if x in seen: return 'seen'
        seen.add(x)
        t.inc('probe')
        hit = table.get(x)
        if hit is None: return
        i, j, sj, ys = hit
        y = yfun()  # lift y only on a hit (minus children: 1 M)
        sgn = 1 if y == ys else -1
        cols = {}
        cols[i] = cols.get(i, 0) + sgn
        cols[j] = cols.get(j, 0) + sgn*sj
        # relation: a + b d = sum  ->  sum - b*d = a
        cols[N] = (-b) % n
        key = (tuple(sorted(cols.items())), a % n)
        if key in rowkeys: dup_rows += 1; return
        rowkeys.add(key)
        rows.append((cols, a))
        if is_minus: minus_rows += 1
    max_steps = 50 * n
    steps = 0
    pre = dict(t.c)
    while len(rows) < nunk + excess and steps < max_steps:
        # one synchronized round over L lanes
        hs = [hash((R[0] * 2654435761) >> 7) % nj for R, _, _, _ in lanes]
        dxs = []
        for (R, a, b, prevh), h in zip(lanes, hs):
            J = jumps[h][0]
            dxs.append((J[0]-R[0]) % p); t.inc('A')
        if any(v == 0 for v in dxs):
            for k, v in enumerate(dxs):
                if v == 0:
                    a0, b0 = rng.randrange(n), rng.randrange(1, n)
                    lanes[k] = [E.add_u(E.mul_u(a0, G), E.mul_u(b0, Q)), a0, b0, -1]
                    charge_scalar(t, n); restarts += 1
            continue
        if L == 1: invs = [inv_c(t, p, dxs[0])]
        else: invs = batch_inv(t, p, dxs)
        for k, ((R, a, b, prevh), h, inv) in enumerate(zip(lanes, hs, invs)):
            J, al, be = jumps[h]
            if paired:
                if h == prevh:
                    skipped_copy += 1  # R - J_h is the previous walk point
                else:
                    sy = (J[1] + R[1]) % p; t.inc('A')
                    lm = (-sy) * inv % p; t.inc('M'); t.inc('A')
                    xm = (lm*lm - R[0] - J[0]) % p; t.inc('S'); t.inc('A', 2)
                    t.inc('second')
                    def ym(R=R, xm=xm, lm=lm):
                        t.inc('M'); t.inc('A', 2); t.inc('lift')
                        return (lm*(R[0]-xm) - R[1]) % p
                    try_cand(xm, ym, a - al, b - be, True)
            Rn = E.add_with_inv(t, R, J, inv); t.inc('adds')
            an, bn = (a + al) % n, (b + be) % n
            lanes[k] = [Rn, an, bn, h]
            if try_cand(Rn[0], lambda Rn=Rn: Rn[1], an, bn, False) == 'seen':
                restarts += 1
                a0, b0 = rng.randrange(n), rng.randrange(1, n)
                lanes[k] = [E.add_u(E.mul_u(a0, G), E.mul_u(b0, Q)), a0, b0, -1]
                charge_scalar(t, n)
            steps += 1
    where, M = solve_mod(rows, nunk, n, t)
    ok = False
    if where[N] >= 0:
        # d determined only if row has no other free unknowns: check
        r = M[where[N]]
        others = [c for c in range(N) if r[c] and where[c] < 0]
        if not others:
            ok = (r[nunk] % n == d_true % n)
    t.c['steps'] = steps; t.c['rows'] = len(rows); t.c['dup_rows'] = dup_rows
    t.c['minus_rows'] = minus_rows; t.c['skipped_copy'] = skipped_copy
    t.c['setup_adds'] = setup_adds; t.c['restarts'] = restarts
    for k in ('I', 'M', 'S', 'A', 'adds'):
        t.c['pre_' + k] = pre[k]
    return t, ok

# ---------------------------------------------------------------- rho with negation map
def canon(E, P):
    return P if P[1] <= E.p - P[1] else E.neg(P)

def rho_run(E, G, Q, d_true, L, theta_bits, side, seed, nj=16, store_all=False):
    """Parallel rho-neg with DPs (x mod 2^theta_bits == 0), L lanes.
    side=True: also form x(R - J_h) from the shared inversion and store it if DP.
    store_all: every point is stored (theta=1), the full-storage birthday walk."""
    rng = random.Random(seed)
    t = T(); p, n = E.p, E.n
    mask = (1 << theta_bits) - 1
    jumps = []
    for _ in range(nj):
        while True:
            al, be = rng.randrange(n), rng.randrange(1, n)
            Jp = E.add_u(E.mul_u(al, G), E.mul_u(be, Q))
            if Jp is not INF: break
        jumps.append((Jp, al, be))
    charge_scalar(t, n, nj)
    def fresh():
        while True:
            a0, b0 = rng.randrange(n), rng.randrange(1, n)
            R = E.add_u(E.mul_u(a0, G), E.mul_u(b0, Q))
            if R is not INF: break
        charge_scalar(t, n); t.inc('restart')
        s = 1 if canon(E, R) == R else -1
        return [canon(E, R), s*a0 % n, s*b0 % n, None, 0]
    lanes = [fresh() for _ in range(L)]
    pre = dict(t.c)
    dp = {}  # x -> (a, b, y)
    def isdp(x): return store_all or (x & mask) == 0
    def check(x, y, a, b):
        t.inc('probe')
        if x in dp:
            a2, b2, y2 = dp[x]
            if y2 != y: a2, b2 = (-a2) % n, (-b2) % n
            if (b - b2) % n == 0: return 'self'
            return (a2 - a) * pow(b - b2, -1, n) % n
        dp[x] = (a, b, y); t.inc('insert')
        return None
    steps = 0; maxlen = 40 << theta_bits if not store_all else 10**9
    while steps < 200 * int(math.isqrt(n)) + 10**5:
        hs = [hash((R[0] * 2654435761) >> 7) % nj for R, *_ in lanes]
        dxs = []
        for (R, *_), h in zip(lanes, hs):
            dxs.append((jumps[h][0][0] - R[0]) % p); t.inc('A')
        # points with equal x (R = +-J) are vanishingly rare; guard anyway
        if any(v == 0 for v in dxs):
            for k, v in enumerate(dxs):
                if v == 0: lanes[k] = fresh()
            continue
        invs = [inv_c(t, p, dxs[0])] if L == 1 else batch_inv(t, p, dxs)
        for k, ((R, a, b, prev, ln), h, inv) in enumerate(zip(lanes, hs, invs)):
            J, al, be = jumps[h]
            if side:
                sy = (J[1] + R[1]) % p; t.inc('A')
                lm = (-sy) * inv % p; t.inc('M'); t.inc('A')
                xm = (lm*lm - R[0] - J[0]) % p; t.inc('S'); t.inc('A', 2)
                t.inc('second')
                if isdp(xm):
                    ym = (lm*(R[0]-xm) - R[1]) % p; t.inc('M'); t.inc('A', 2); t.inc('lift')
                    res = check(xm, ym, (a - al) % n, (b - be) % n)
                    if res is not None and res != 'self':
                        t.c['steps'] = steps
                        for kk in ('I', 'M', 'S', 'A', 'adds'): t.c['pre_' + kk] = pre[kk]
                        return t, res == d_true % n
            Rn = E.add_with_inv(t, R, J, inv); t.inc('adds'); steps += 1
            an, bn = (a + al) % n, (b + be) % n
            if canon(E, Rn) != Rn: Rn, an, bn = E.neg(Rn), (-an) % n, (-bn) % n
            # fruitless 2-cycle: Rn equals the point before R
            if prev is not None and Rn == prev:
                t.inc('cyc2')
                Rm = min(R, Rn)
                am, bm = (a, b) if Rm == R else (an, bn)
                Rn = E.dbl_counted(t, Rm); an, bn = 2*am % n, 2*bm % n
                if canon(E, Rn) != Rn: Rn, an, bn = E.neg(Rn), (-an) % n, (-bn) % n
            lanes[k] = [Rn, an, bn, R, ln + 1]
            if isdp(Rn[0]):
                res = check(Rn[0], Rn[1], an, bn)
                if res == 'self': lanes[k] = fresh()
                elif res is not None:
                    t.c['steps'] = steps
                    for kk in ('I', 'M', 'S', 'A', 'adds'): t.c['pre_' + kk] = pre[kk]
                    return t, res == d_true % n
            elif ln + 1 > maxlen:
                t.inc('cap_restart'); lanes[k] = fresh()
    t.c['steps'] = steps
    return t, False

# ---------------------------------------------------------------- pricing
def bitop(b):
    return dict(A=13*b, M=12*b*b + 3*b, S=12*b*b + 3*b, I=26*b*b, probe=4*b)

def wordram(b):
    s = (b + 63)//64; base = 128*(s+1); add = base; mul = b*(2*add + 64) + base
    return dict(A=add, M=mul, S=mul, I=2*b*(mul + 64) + base, probe=4*add)

def field_price(t, pr):
    return sum(t[k]*pr[k] for k in ('A', 'M', 'S', 'I'))

def gae(t, w, extra_gae=0):
    # 1 GAE per group add/double; second x-only point, probes/guards/inserts and row ops at w
    native = t['second'] + t['probe'] + t['guard'] + t['insert'] + t['rowop'] + t['lift']
    return t['adds'] + t['dbls'] + extra_gae + w * native

if __name__ == '__main__':
    pass
