#!/usr/bin/env python3
"""LEAD-TRF-one-point-self-pairing toy protocol. stdlib only. See PROTOCOL.md."""
import json, random, sys, time
from math import gcd

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def factor(n):
    f, q = {}, 2
    while q*q <= n:
        while n % q == 0: f[q]=f.get(q,0)+1; n//=q
        q += 1 if q == 2 else 2
    if n > 1: f[n]=f.get(n,0)+1
    return f

def mult_order(a, n):
    o = n-1
    for q in factor(n-1):
        while o % q == 0 and pow(a, o//q, n) == 1: o//=q
    return o

def chi_table(p):
    sq = [0]*p
    for y in range(1,p): sq[y*y % p] = 1
    return sq  # sq[v]=1 iff v nonzero square

def count_points(p, a, b, sq):
    t = 0
    for x in range(p):
        v = (x*x*x + a*x + b) % p
        t += 1 if sq[v] else (0 if v == 0 else -1)
    return p + 1 + t

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1]+Q[1]) % p == 0: return None
        l = (3*P[0]*P[0]+a) * pow(2*P[1], -1, p) % p
    else:
        l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
    x = (l*l-P[0]-Q[0]) % p
    return (x, (l*(P[0]-x)-P[1]) % p)

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def rand_point(p, a, b, rng):
    while True:
        x = rng.randrange(p); v = (x**3 + a*x + b) % p
        if v and pow(v, (p-1)//2, p) == 1:
            y = sqrt_mod(v, p)
            return (x, y)

def sqrt_mod(v, p):
    if p % 4 == 3: return pow(v, (p+1)//4, p)
    q, s = p-1, 0
    while q % 2 == 0: q//=2; s+=1
    z = 2
    while pow(z,(p-1)//2,p) != p-1: z += 1
    m, c, t, r = s, pow(z,q,p), pow(v,q,p), pow(v,(q+1)//2,p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        bb = pow(c, 1 << (m-i-1), p)
        m, c, t, r = i, bb*bb % p, t*bb*bb % p, r*bb % p
    return r

def miller(P, r, X, a, p):
    """f_{r,P}(X) in F_p; raises ZeroDivisionError on degenerate evaluation."""
    f, T = 1, P
    for bit in bin(r)[3:]:
        l = (3*T[0]*T[0]+a) * pow(2*T[1], -1, p) % p
        T2 = add(T, T, a, p)
        num = (X[1]-T[1]-l*(X[0]-T[0])) % p
        den = (X[0]-T2[0]) % p
        f = f*f % p * num % p * pow(den, -1, p) % p
        T = T2
        if bit == '1':
            if T[0] == P[0]:
                if (T[1]+P[1]) % p == 0:  # T=-P, sum is O
                    f = f * ((X[0]-T[0]) % p) % p; T = None; break
            l = (P[1]-T[1]) * pow(P[0]-T[0], -1, p) % p
            T3 = add(T, P, a, p)
            num = (X[1]-T[1]-l*(X[0]-T[0])) % p
            den = (X[0]-T3[0]) % p if T3 else 1
            f = f * num % p * pow(den, -1, p) % p
            T = T3
    if f == 0: raise ZeroDivisionError
    return f

def tate(P, Q, r, a, b, p, rng):
    """reduced Tate pairing T(P,Q) = (f_P(Q+R)/f_P(R))^((p-1)/r), p = 1 mod r."""
    for _ in range(50):
        R = rand_point(p, a, b, rng)
        QR = add(Q, R, a, p)
        if QR is None: continue
        try:
            v = miller(P, r, QR, a, p) * pow(miller(P, r, R, a, p), -1, p) % p
        except (ZeroDivisionError, ValueError):
            continue
        return pow(v, (p-1)//r, p)
    return None

def positive_control(rng, n_target, pmax):
    """k=1 curves (r | p-1, r | #E, E(F_p)[r] cyclic): self-pairing nontrivial?"""
    out = dict(n=0, nontrivial=0, bilinear_ok=0, bilinear_tested=0)
    tries = 0
    while out["n"] < n_target and tries < 4000:
        tries += 1
        r = rng.randrange(60, pmax//2)
        if not is_prime(r): continue
        m = rng.randrange(2, 9, 2); p = r*m+1
        if not is_prime(p) or p < 200: continue
        sq = chi_table(p)
        for _ in range(60):
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            N = count_points(p, a, b, sq)
            if N % r or N % (r*r) == 0 or N == p: continue
            P = mul(N//r, rand_point(p, a, b, rng), a, p)
            if P is None: continue
            t = tate(P, P, r, a, b, p, rng)
            if t is None: continue
            out["n"] += 1
            out["nontrivial"] += (t != 1)
            t2 = tate(P, mul(2, P, a, p), r, a, b, p, rng)
            if t2 is not None:
                out["bilinear_tested"] += 1
                out["bilinear_ok"] += (t2 == pow(t, 2, p))
            break
    return out

def null_population(rng, n_target, plo, phi):
    rows, tries = [], 0
    while len(rows) < n_target and tries < 200000:
        tries += 1
        p = rng.randrange(plo, phi)
        if not is_prime(p): continue
        sq = chi_table(p)
        for _ in range(8):
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            N = count_points(p, a, b, sq)
            if N > 1000 and N != p and is_prime(N):
                k = mult_order(p, N)
                rows.append(dict(k=k, ratio=k/(N-1), trivial_target=(gcd(N, p-1) == 1)))
                if len(rows) >= n_target: break
    return rows

def median(v):
    v = sorted(v); return v[len(v)//2]

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    seed = 20261110
    n_null, n_pos = (6, 3) if smoke else (240, 60)
    plo, phi = (1 << 11, 1 << 12) if smoke else (1 << 12, 1 << 16)
    pc = positive_control(random.Random(seed+1), n_pos, 1 << 11 if smoke else 1 << 14)
    rows = null_population(random.Random(seed+2), n_null, plo, phi)
    h = len(rows)//2
    A, B = rows[:h], rows[h:]
    medA = median([x["ratio"] for x in A]) if A else None
    medB = median([x["ratio"] for x in B]) if B else None
    med = median([x["ratio"] for x in rows]) if rows else None
    min_k = min((x["k"] for x in rows), default=None)
    low = sum(1 for x in rows if x["k"] <= 20)
    triv = sum(1 for x in rows if x["trivial_target"])
    pos_frac = pc["nontrivial"]/pc["n"] if pc["n"] else 0.0
    pos_ok = pc["n"] >= (2 if smoke else 30) and pos_frac >= 0.9 and pc["bilinear_ok"] == pc["bilinear_tested"]
    aa_ok = bool(A and B) and abs(medA-medB) <= 0.15
    enough = len(rows) >= (4 if smoke else 100)
    if not (pos_ok and aa_ok and enough): verdict = "inconclusive"
    elif low > 0.05*len(rows) or med < 0.1 or triv < len(rows): verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    print(json.dumps(dict(lead="LEAD-TRF-one-point-self-pairing", smoke=smoke, seeds=dict(pos=seed+1, null=seed+2),
        params=dict(n_null=n_null, n_pos=n_pos, p_range=[plo, phi]),
        statistic=dict(median_k_over_Nminus1=med, min_k=min_k, n_k_le_20=low, n_trivial_pairing_target=triv, n_null=len(rows)),
        controls=dict(positive_k1=pc, positive_nontrivial_frac=pos_frac, aa_median_halves=[medA, medB],
                      null="random prime-order curves"),
        verdict=verdict, seconds=round(time.time()-t0, 2))))
main()
