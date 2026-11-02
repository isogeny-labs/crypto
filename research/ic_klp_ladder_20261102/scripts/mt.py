#!/usr/bin/env python3
"""Multi-target shared-table collision search on a toy prime-field curve.

Counts group additions only (1 GAE each).  Arms:
  indep  : T independent single-target searches (T=1 code, run T times)
  shared : T targets, one shared point table, linear solve over GF(r)
  sharedB: shared + one unknown base point P0 subtracted on every step (B=1 column,
           the residual collector's structure)
All walks take steps in <G> only (J_j = c_j G), so trails of different targets can merge.
Every recovered log is checked against the planted value.
"""
import random, sys, json, math

def make_curve(bits, seed):
    rnd = random.Random(seed)
    def is_prime(n):
        if n < 2: return False
        for q in (2,3,5,7,11,13,17,19,23,29,31,37):
            if n % q == 0: return n == q
        d, s = n-1, 0
        while d % 2 == 0: d //= 2; s += 1
        for a in (2,3,5,7,11,13,17,19,23,29,31,37):
            x = pow(a, d, n)
            if x in (1, n-1): continue
            for _ in range(s-1):
                x = x*x % n
                if x == n-1: break
            else: return False
        return True
    while True:
        p = rnd.getrandbits(bits) | (1 << (bits-1)) | 1
        if p % 4 == 3 and is_prime(p): break   # p = 3 mod 4: sqrt by pow
    while True:
        a, b = rnd.randrange(p), rnd.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        # count points naively
        cnt = 1
        e = (p-1)//2
        for x in range(p):
            f = (x*x*x + a*x + b) % p
            if f == 0: cnt += 1
            elif pow(f, e, p) == 1: cnt += 2
        if is_prime(cnt) and cnt != p:
            break
    r = cnt
    # a generator: any non-identity point
    while True:
        x = rnd.randrange(p)
        f = (x*x*x + a*x + b) % p
        if pow(f, e, p) == 1:
            y = pow(f, (p+1)//4, p)
            return p, a, b, r, (x, y)

class EC:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            l = (3*x1*x1 + s.a) * pow(2*y1, -1, p) % p
        else:
            l = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (l*l - x1 - x2) % p
        return (x3, (l*(x1 - x3) - y1) % p)
    def neg(s, P): return None if P is None else (P[0], (-P[1]) % s.p)
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R

class Solver:
    """Incremental Gaussian elimination over GF(r); n unknowns."""
    def __init__(s, n, r): s.n, s.r, s.piv, s.rank = n, r, {}, 0
    def add(s, row, rhs):
        r = s.r
        row = row[:];
        for c in range(s.n):
            if row[c] % r == 0: continue
            if c in s.piv:
                pr, pb = s.piv[c]
                f = row[c]
                row = [(x - f*y) % r for x, y in zip(row, pr)]
                rhs = (rhs - f*pb) % r
            else:
                inv = pow(row[c], -1, r)
                row = [x*inv % r for x in row]; rhs = rhs*inv % r
                s.piv[c] = (row, rhs); s.rank += 1
                return True
        return False
    def solve(s):
        r, n = s.r, s.n
        sol = [0]*n
        for c in sorted(s.piv, reverse=True):
            row, rhs = s.piv[c]
            v = rhs
            for j in range(c+1, n): v = (v - row[j]*sol[j]) % r
            sol[c] = v
        return sol

def run(curve, T, B, seed, L=512, njump=32, cap=40_000_000):
    p, a, b, r, G = curve
    E = EC(p, a, b)
    rnd = random.Random(seed)
    hs = [rnd.randrange(1, r) for _ in range(T)]
    Qs = [E.mul(h, G) for h in hs]
    p0log = rnd.randrange(1, r)
    P0 = E.mul(p0log, G) if B else None
    negP0 = E.neg(P0) if B else None
    cs = [rnd.randrange(1, r) for _ in range(njump)]
    Js = [E.mul(c, G) for c in cs]
    # fused steps J_j - P0 (B=1) are precomputed (setup, not counted: equal for all arms)
    steps = [E.add(J, negP0) if B else J for J in Js]
    startc = [rnd.randrange(1, r) for _ in range(64)]
    startG = [E.mul(c, G) for c in startc]
    n = T + B
    sol = Solver(n, r)
    table = {}
    adds = 0
    # per-target walk state
    st = []
    def new_walk(t, prev):
        nonlocal adds
        if prev is None:
            bb = 1 + rnd.randrange(r-1); aa = rnd.randrange(r)
            pt = E.add(E.mul(aa, G), E.mul(bb, Qs[t])); adds += 2*int(1.5*r.bit_length())
        else:
            k = rnd.randrange(64)
            aa = (prev[1] + startc[k]) % r; bb = prev[2] + 1
            pt = E.add(E.add(prev[0], startG[k]), Qs[t]); adds += 2
        return [pt, aa, bb, 0, 0, t]   # point, a, b, cnt, len, target
    walks = [new_walk(t, None) for t in range(T)]
    base = [list(w) for w in walks]
    rounds = 0
    while sol.rank < n and adds < cap:
        rounds += 1
        for t in range(T):
            w = walks[t]
            pt, aa, bb, cnt, ln, _ = w
            j = (pt[0] * 2654435761 + pt[1]) % njump if pt else 0
            pt = E.add(pt, steps[j]); adds += 1
            aa = (aa + cs[j]) % r
            if B: cnt += 1
            ln += 1
            w[:] = [pt, aa, bb, cnt, ln, t]
            if pt is None:
                walks[t] = new_walk(t, base[t]); base[t] = list(walks[t]); continue
            key = (pt[0], pt[1])
            hit = table.get(key)
            if hit is None:
                table[key] = (t, aa, bb, cnt)
            else:
                t2, a2, b2, c2 = hit
                if (t2, b2) == (t, bb) and a2 == aa and c2 == cnt:
                    # own revisit / identical trail: no information
                    walks[t] = new_walk(t, base[t]); base[t] = list(walks[t]); continue
                # a + b h_t - cnt p = a2 + b2 h_t2 - c2 p
                row = [0]*n
                row[t] = (row[t] + bb) % r
                row[t2] = (row[t2] - b2) % r
                if B: row[T] = (c2 - cnt) % r
                sol.add(row, (a2 - aa) % r)
                walks[t] = new_walk(t, base[t]); base[t] = list(walks[t])
                continue
            if ln >= L:
                walks[t] = new_walk(t, base[t]); base[t] = list(walks[t])
    ok = sol.rank == n
    if ok:
        x = sol.solve()
        assert x[:T] == hs, "recovered logs differ from planted"
        if B: assert x[T] == p0log
    return adds, ok

if __name__ == "__main__":
    bits = int(sys.argv[1]); trials = int(sys.argv[2]); seed0 = int(sys.argv[3]) if len(sys.argv) > 3 else 0
    curve = make_curve(bits, seed=bits*7919)
    p, a, b, r, G = curve
    print(json.dumps({"p": p, "r": r, "sqrt_r": math.isqrt(r)}), flush=True)
    res = {}
    for T in (1, 2, 4, 8, 16, 32, 64):
        for B in (0, 1):
            tot = []; fail = 0
            for s in range(trials):
                ad, ok = run(curve, T, B, seed=seed0 + 1000*T + 17*B + s)
                if ok: tot.append(ad/T)
                else: fail += 1
            m = sum(tot)/len(tot) if tot else float('nan')
            sd = (sum((x-m)**2 for x in tot)/(len(tot)-1))**.5 if len(tot) > 1 else float('nan')
            res[(T, B)] = (m, sd/len(tot)**.5 if len(tot) > 1 else float('nan'), len(tot), fail)
            print(f"T={T:3d} B={B} adds/target = {m:10.1f} +- {res[(T,B)][1]:7.1f}  (n={len(tot)}, failed={fail})", flush=True)
