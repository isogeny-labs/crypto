#!/usr/bin/env python3
"""Exact structure of the M31 curves (pure Python, no external CAS).

Every fact is computed two independent ways where a second way exists; the
JSON records both and an `agree` flag. Randomness: SplitMix64 seeded from the
frozen manifest (seed 0x4d33315f73747275, stream names below)."""
import json, sys, hashlib
from math import isqrt

p = 2**31 - 1
CURVES = {
    "B677": dict(a=847514846, b=2087444724, n=2147446831,
                 G=(625037715, 2100627251), Q=(976979610, 1228979761)),
    "M31_minus3_21": dict(a=p - 3, b=21, n=2147487637),
}
MASTER = 0x4d33315f73747275

class SM:
    def __init__(self, stream):
        h = hashlib.sha256(f"{MASTER:016x}/{stream}".encode()).digest()
        self.s = int.from_bytes(h[:8], "little")
    def next(self):
        self.s = (self.s + 0x9e3779b97f4a7c15) & (2**64 - 1)
        z = self.s
        z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) & (2**64 - 1)
        z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) & (2**64 - 1)
        return z ^ (z >> 31)

def is_prime(m):
    if m < 2: return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if m % q == 0: return m == q
    d, s = m - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):  # deterministic < 3.3e24
        x = pow(a, d, m)
        if x in (1, m - 1): continue
        for _ in range(s - 1):
            x = x * x % m
            if x == m - 1: break
        else: return False
    return True

def factor(m):
    m = abs(m); f = {}; d = 2
    while d * d <= m:
        while m % d == 0: f[d] = f.get(d, 0) + 1; m //= d
        d += 1 if d == 2 else 2
    if m > 1: f[m] = f.get(m, 0) + 1
    return f

def leg(x, q):
    x %= q
    if x == 0: return 0
    return 1 if pow(x, (q - 1) // 2, q) == 1 else -1

def sqrt_mod(x):  # p = 3 mod 4
    r = pow(x % p, (p + 1) // 4, p)
    return r if r * r % p == x % p else None

# --- affine group law on y^2 = x^3 + a x + b over F_p ---
def add(P, Q, a):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], p - 2, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], p - 2, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)

def mul(k, P, a):
    R = None
    while k:
        if k & 1: R = add(R, P, a)
        P = add(P, P, a); k >>= 1
    return R

def rand_point(a, b, rng):
    while True:
        x = rng.next() % p
        y = sqrt_mod(x**3 + a * x + b)
        if y is not None and y: return (x, y)

def bsgs_order_in_hasse(P, a):
    """Method B for #E: all m in the Hasse interval with mP = O (BSGS, no use of n)."""
    lo = p + 1 - 2 * isqrt(p) - 2; hi = p + 1 + 2 * isqrt(p) + 2
    width = hi - lo; m = isqrt(width) + 1
    baby = {}; R = None
    for j in range(m):
        baby.setdefault(R, j); R = add(R, P, a)
    mP = mul(m, P, a); negmP = None if mP is None else (mP[0], (-mP[1]) % p)
    S = mul(lo, P, a); S = None if S is None else (S[0], (-S[1]) % p)  # -lo*P
    hits = []
    for i in range(m + 1):
        # want lo + i*m + j with (lo+i*m+j)P = O  <=>  jP = -(lo+i*m)P
        if S in baby:
            c = lo + i * m + baby[S]
            if c <= hi: hits.append(c)
        S = add(S, negmP, a)
    return sorted(set(hits))

# --- polynomials over F_p (lists low->high) ---
def ptrim(f):
    while f and f[-1] == 0: f.pop()
    return f
def pmod(f, g):
    f = f[:]; inv = pow(g[-1], p - 2, p)
    while len(f) >= len(g):
        c = f[-1] * inv % p; s = len(f) - len(g)
        for i, gi in enumerate(g): f[s + i] = (f[s + i] - c * gi) % p
        ptrim(f)
    return f
def pmulmod(f, g, m):
    r = [0] * (len(f) + len(g) - 1) if f and g else []
    for i, fi in enumerate(f):
        for j, gj in enumerate(g): r[i + j] = (r[i + j] + fi * gj) % p
    return pmod(ptrim(r), m)
def pgcd(f, g):
    f, g = ptrim(f[:]), ptrim(g[:])
    while g: f, g = g, pmod(f, g)
    inv = pow(f[-1], p - 2, p); return [c * inv % p for c in f]
def n_roots(f):
    """Number of distinct F_p-roots: deg gcd(f, x^p - x)."""
    x = [0, 1]; r = [1]; base = x; e = p
    while e:
        if e & 1: r = pmulmod(r, base, f)
        base = pmulmod(base, base, f); e >>= 1
    r = r + [0] * (2 - len(r)); r[1] = (r[1] - 1) % p
    return len(pgcd(f, ptrim(r))) - 1

def psi3(a, b):  # 3x^4 + 6ax^2 + 12bx - a^2
    return ptrim([(-a * a) % p, 12 * b % p, 6 * a % p, 0, 3])

def twist(a, b):
    c = next(c for c in range(2, 100) if leg(c, p) == -1)
    return a * c * c % p, b * c**3 % p

def three_torsion_rank_random(a, b, order, rng, trials=40):
    """Method B for the 3-volcano level: size of E(F_p)[3] from random points."""
    v = 0; m = order
    while m % 3 == 0: m //= 3; v += 1
    pts = set()
    for _ in range(trials):
        R = mul(m, rand_point(a, b, rng), a)
        while R is not None and mul(3, R, a) is not None: R = mul(3, R, a)
        if R is not None: pts.add(R)
    return len(pts) + 1  # including O

def mult_order(x, n):
    k = n - 1
    for q, e in factor(n - 1).items():
        for _ in range(e):
            if pow(x, k // q, n) == 1: k //= q
    return k

def analyse(name, c):
    a, b, n = c["a"], c["b"], c["n"]
    rng = SM(f"structure/{name}")
    out = {"curve": name, "p": p, "a": a, "b": b}
    disc = (4 * a**3 + 27 * b * b) % p
    out["nonsingular"] = disc != 0
    # j: method A (1728*4a^3/(4a^3+27b^2)); method B (c4^3/Delta)
    jA = 1728 * 4 * a**3 * pow(disc, p - 2, p) % p
    c4 = -48 * a % p; Delta = -16 * disc % p
    jB = c4**3 * pow(Delta, p - 2, p) % p
    out["j"] = {"A": jA, "B": jB, "agree": jA == jB, "is_0_or_1728": jA in (0, 1728)}
    # #E: A = claimed n verified; B = BSGS over the Hasse window, n not used
    G = c.get("G") or rand_point(a, b, rng)
    A_ok = is_prime(n) and abs(p + 1 - n) <= 2 * isqrt(p) + 1 and mul(n, G, a) is None and G is not None
    hits = None
    for _ in range(3):
        h = bsgs_order_in_hasse(rand_point(a, b, rng), a)
        hits = h if hits is None else sorted(set(hits) & set(h))
    out["order"] = {"n": n, "A_claimed_n_prime_in_hasse_and_kills_G": A_ok,
                    "B_bsgs_hasse_candidates": hits, "agree": A_ok and hits == [n]}
    t = p + 1 - n; D = t * t - 4 * p
    fD = factor(D); sq = 1; core = -1
    for q, e in fD.items():
        sq *= q ** (e // 2); core *= q ** (e % 2)
    if core % 4 != 1: dK = 4 * core; f_pi = sq // 2
    else: dK = core; f_pi = sq
    assert f_pi * f_pi * dK == D
    out["trace"] = t
    out["frobenius_disc"] = {"D": D, "factorization": {str(k): v for k, v in fD.items()},
                             "factor_check_product": D == -eval("*".join(f"{k}**{v}" for k, v in fD.items())),
                             "all_factors_prime": all(is_prime(q) for q in fD),
                             "d_K": dK, "conductor_Zpi": f_pi}
    # End level at each prime l | f_pi (here only l = 3)
    lv = {}
    for l in factor(f_pi):
        assert l == 3
        rootsA = n_roots(psi3(a, b))
        ta, tb = twist(a, b)
        tw_order = 2 * p + 2 - n
        rankB = three_torsion_rank_random(ta, tb, tw_order, rng)
        # crater (End contains (pi-alpha)/3) <=> pi acts as scalar on E[3] <=> 4 rational 3-kernels
        crater_A = rootsA == 4
        # t = 1 mod 3 => the only scalar with 2a = t, a^2 = p (mod 3) is a = -1, i.e. E'(F_p)[3] full
        assert t % 3 == 1
        crater_B = rankB == 9
        lv["3"] = {"A_rational_roots_of_psi3": rootsA, "B_twist_3torsion_size": rankB,
                   "t_mod_3": t % 3,
                   "level": "crater (End = O_K, conductor 1)" if crater_A else "floor (End = Z[pi], conductor 3)",
                   "agree": crater_A == crater_B}
    out["end_ring_level"] = lv
    out["End"] = ("O_K (conductor 1)" if all(v["level"].startswith("crater") for v in lv.values()) else "Z[pi] (conductor 3)") + f", K = Q(sqrt({dK}))"
    out["smallest_nonscalar_endomorphism_degree_lower_bound"] = (abs(dK) + 1) // 4
    out["aut_group_order"] = 2 if jA not in (0, 1728) else (6 if jA == 0 else 4)
    # rational l-isogenies, l <= 47 prime, l not | D: number of roots of x^2 - t x + p mod l
    iso = {}
    for l in [q for q in range(2, 48) if is_prime(q)]:
        if D % l: iso[str(l)] = sum(1 for r in range(l) if (r * r - t * r + p) % l == 0)
        else: iso[str(l)] = "l | D: ramified/conductor prime (1 or l+1; see end_ring_level for l | f)"
    # second method for l = 2: roots of x^3 + ax + b
    iso_check2 = n_roots(ptrim([b % p, a % p, 0, 1]))
    out["rational_l_isogenies"] = {"by_frobenius_charpoly_mod_l": iso, "l2_cubic_roots": iso_check2,
                                   "l2_agree": iso_check2 == iso["2"]}
    # embedding degree
    kA = mult_order(p % n, n)
    kB_small = next((k for k in range(1, 5001) if pow(p, k, n) == 1), None)
    out["embedding_degree"] = {"A_order_of_p_mod_n": kA, "A_check_pow": pow(p, kA, n) == 1,
                               "B_no_k_le_5000": kB_small is None, "log2_k": kA.bit_length(),
                               "factor_n_minus_1": {str(k): v for k, v in factor(n - 1).items()}}
    out["anomalous"] = {"n_eq_p": n == p, "trace_eq_1": t == 1}
    tw = 2 * p + 2 - n
    out["twist"] = {"order": tw, "factorization": {str(k): v for k, v in factor(tw).items()}}
    if "Q" in c:
        out["Q_on_curve"] = (c["Q"][1]**2 - c["Q"][0]**3 - a * c["Q"][0] - b) % p == 0
        out["Q_eq_987654321_G"] = mul(987654321, c["G"], a) == c["Q"]
    return out

def j0_class():
    """j = 0 curves y^2 = x^3 + b over F_p (p = 1 mod 3): the six twists, prime-order ones."""
    assert p % 3 == 1
    # Cornacchia-free: 4p = t^2 + 3v^2 by search on v
    sols = [(isqrt(4 * p - 3 * v * v), v) for v in range(1, isqrt(4 * p // 3) + 1)
            if isqrt(4 * p - 3 * v * v) ** 2 == 4 * p - 3 * v * v]
    t0, v0 = sols[0]
    traces = sorted({s * t0 for s in (1, -1)} | {s * (t0 + 3 * v0) // 2 for s in (1, -1)} | {s * (t0 - 3 * v0) // 2 for s in (1, -1)})
    orders = {tr: p + 1 - tr for tr in traces}
    rng = SM("structure/j0")
    found = {}
    for b in range(1, 200):
        P = rand_point(0, b, rng)
        cand = [o for o in orders.values() if mul(o, P, 0) is None]
        if len(cand) == 1: found.setdefault(cand[0], b)
    prime_ones = []
    for o, b in sorted(found.items()):
        if is_prime(o):
            hits = None
            for _ in range(2):
                h = bsgs_order_in_hasse(rand_point(0, b, rng), 0)
                hits = h if hits is None else sorted(set(hits) & set(h))
            beta = pow(next(g for g in range(2, 50) if pow(g, (p - 1) // 3, p) != 1), (p - 1) // 3, p)
            prime_ones.append({"b": b, "n": o, "trace": p + 1 - o, "bsgs_hasse": hits,
                               "verified_two_ways": hits == [o],
                               "beta_cube_root_of_unity": beta,
                               "lambda_eigen": None})
    # j = 1728 is supersingular for p = 3 mod 4: check #E = p + 1 for y^2 = x^3 + x
    h1728 = bsgs_order_in_hasse(rand_point(1, 0, rng), 1)
    return {"cornacchia_4p": [t0, v0], "traces": traces,
            "orders": {str(k): {"n": v, "factor": {str(a): e for a, e in factor(v).items()}, "b_min": found.get(v)} for k, v in orders.items()},
            "prime_order_curves": prime_ones,
            "j1728": {"p_mod_4": p % 4, "supersingular": True, "x3_plus_x_bsgs": h1728,
                      "order_p_plus_1": p + 1, "factor": {str(a): e for a, e in factor(p + 1).items()}, "embedding_degree": 2}}

if __name__ == "__main__":
    res = {"tool": "m31_curve_structure/structure.py", "master_seed": f"{MASTER:#018x}",
           "curves": [analyse(k, v) for k, v in CURVES.items()], "j0_class": j0_class()}
    json.dump(res, sys.stdout, indent=1); print()
