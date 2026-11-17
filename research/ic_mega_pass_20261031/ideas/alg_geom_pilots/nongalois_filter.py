"""Pilot for MP-alg_geom-4: canonical partial section of a non-Galois (S_3) cover of E/F_p
as a representation-dependent label, tested as a sum-correlated filter.

Cover X: w^3 + w = h(P) with h = x (odd p, p = 2 mod 3 avoided? no: any odd p; generic S_3 monodromy).
U = {P : w^3 + w - x(P) has exactly one root in F_p}; tau(P) = that root.
Filter statistic: for pairs (P,Q) in U with P+Q in U, bucket c(P)=tau(P) mod M etc.
Measure the single-bucket merge gain G(M) = M * max_{c1,c2,c3} Pr[c3 | c1,c2] over observed
triples versus a hash null (random relabelling of tau).  Also linear-dependence test:
rank/agreement of tau(P+Q) with F_p-linear/low-degree functions of (tau(P), tau(Q), x(P), x(Q)).
"""
import random, sys, json, os, math
import numpy as np

def legendre(a, p):
    return pow(a % p, (p - 1) // 2, p)

def sqrt_mod(a, p):
    a %= p
    if a == 0: return 0
    if p % 4 == 3:
        r = pow(a, (p + 1) // 4, p)
        return r if r * r % p == a else None
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2; s += 1
    z = 2
    while legendre(z, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1:
            tt = tt * tt % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r

class EC:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b
    def add(self, P, Q):
        p = self.p
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            lam = (3 * P[0] * P[0] + self.a) * pow(2 * P[1], p - 2, p) % p
        else:
            lam = (Q[1] - P[1]) * pow(Q[0] - P[0], p - 2, p) % p
        x3 = (lam * lam - P[0] - Q[0]) % p
        y3 = (lam * (P[0] - x3) - P[1]) % p
        return (x3, y3)
    def mul(self, k, P):
        R = None
        while k:
            if k & 1: R = self.add(R, P)
            P = self.add(P, P); k >>= 1
        return R
    def order_naive(self):
        # count points by Legendre sweep
        p = self.p; cnt = 1
        for x in range(p):
            v = (x * x * x + self.a * x + self.b) % p
            if v == 0: cnt += 1
            elif legendre(v, p) == 1: cnt += 2
        return cnt
    def points(self):
        p = self.p; pts = []
        for x in range(p):
            v = (x * x * x + self.a * x + self.b) % p
            if v == 0: pts.append((x, 0))
            elif legendre(v, p) == 1:
                y = sqrt_mod(v, p); pts.append((x, y)); pts.append((x, p - y))
        return pts

def cubic_roots_table(p):
    """for each t in F_p, roots of w^3 + w - t : build by evaluating w -> w^3+w"""
    tab = [[] for _ in range(p)]
    for w in range(p):
        tab[(w * w * w + w) % p].append(w)
    return tab

def main(p, seed, M_list=(2, 4, 8, 16), npairs=200000):
    rng = random.Random(seed)
    # prime-order curve
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b ** 2) % p == 0: continue
        E = EC(p, a, b)
        N = E.order_naive()
        # want prime N
        if N > 2 and all(N % q for q in range(2, int(N ** 0.5) + 1)):
            break
    pts = E.points()
    G = pts[0]
    # log table via enumeration
    log = {}
    Q = None
    for k in range(N):
        log[Q] = k
        Q = E.add(Q, G)
    tab = cubic_roots_table(p)
    # tau(P) = unique root of w^3+w = x(P) if exactly one root
    tau = {}
    for P in pts:
        r = tab[P[0]]
        if len(r) == 1:
            tau[P] = r[0]
    dens = len(tau) / len(pts)
    # sample pairs P,Q in U with P+Q in U
    U = list(tau.keys())
    trip = []
    for _ in range(npairs):
        P = rng.choice(U); Q = rng.choice(U)
        S = E.add(P, Q)
        if S in tau:
            trip.append((tau[P], tau[Q], tau[S], P[0], Q[0], S[0]))
    trip = np.array(trip, dtype=np.int64)
    res = {"p": p, "N": N, "density_U": dens, "n_triples": int(len(trip))}
    # single-bucket gain for tau mod M (and for x mod M as a sanity baseline, and random relabel null)
    perm = np.array(rng.sample(range(p), p))
    for M in M_list:
        def gain(c1, c2, c3):
            key = c1 * M * M + c2 * M + c3
            cnt = np.bincount(key, minlength=M ** 3).reshape(M, M, M).astype(float)
            pair = cnt.sum(axis=2, keepdims=True)
            with np.errstate(invalid="ignore", divide="ignore"):
                cond = np.where(pair > 0, cnt / pair, 0)
            # weight by pair count to avoid tiny-sample maxima: require pair count >= 30
            cond = np.where(pair >= 30, cond, 0)
            return float(M * cond.max())
        g_tau = gain(trip[:, 0] % M, trip[:, 1] % M, trip[:, 2] % M)
        g_x = gain(trip[:, 3] % M, trip[:, 4] % M, trip[:, 5] % M)
        g_null = gain(perm[trip[:, 0]] % M, perm[trip[:, 1]] % M, perm[trip[:, 2]] % M)
        res[f"G_tau_M{M}"] = round(g_tau, 4)
        res[f"G_x_M{M}"] = round(g_x, 4)
        res[f"G_null_M{M}"] = round(g_null, 4)
    # low-degree agreement: does tau(P+Q) agree with any polynomial of degree<=2 in (tau(P),tau(Q)) mod p
    # beyond chance? Estimate: for random pairs, count fraction with tau(S) == f(tau(P),tau(Q)) for f in a
    # small family (sum, product, sum of squares); chance ~ 1/p each.
    t1, t2, t3 = trip[:, 0], trip[:, 1], trip[:, 2]
    fam = {"sum": (t1 + t2) % p, "diff": (t1 - t2) % p, "prod": (t1 * t2) % p,
           "sumsq": (t1 * t1 + t2 * t2) % p}
    for k, v in fam.items():
        res[f"agree_{k}"] = int((v == t3).sum())
    res["agree_chance_expect"] = round(len(trip) / p, 3)
    # Chebotarev check: fraction of x with 1 root should be ~1/2, 3 roots ~1/6, 0 roots ~1/3
    nroots = np.array([len(tab[P[0]]) for P in pts])
    res["frac_roots_0_1_3"] = [round(float((nroots == k).mean()), 4) for k in (0, 1, 3)]
    return res

if __name__ == "__main__":
    out = []
    for p in (10007, 20011, 40009):
        for seed in (1, 2):
            r = main(p, seed)
            print(json.dumps(r), flush=True)
            out.append(r)
    json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "RESULTS_nongalois.json"), "w"), indent=1)
