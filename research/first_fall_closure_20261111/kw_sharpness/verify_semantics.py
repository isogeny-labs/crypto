"""Brute-force check of the Semaev property for S_3 and S_4 over small fields GF(2^n).
(x1..xk) such that some points P_i=(x_i,y_i) of E(F_q) sum to O must be zeros of S_k.
The converse holds over the algebraic closure; over F_q we report how many zeros of S_k are not
explained by F_q-rational points (they come from points defined over F_{q^2})."""
import numpy as np, itertools, random, sys
from kwgf import GF
import semaev

def curve_points(F, a2, a6):
    pts = []  # affine points
    for x in range(F.q):
        for y in range(F.q):
            lhs = F.sq(y) ^ F.mul(x, y)
            rhs = F.mul(F.sq(x), x) ^ F.mul(a2, F.sq(x)) ^ a6
            if lhs == rhs: pts.append((x, y))
    return pts

def add(F, a2, P, Q):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if y1 ^ y2 == x1: return None  # Q = -P
        lam = x1 ^ F.mul(y1, F.inv(x1))
        x3 = F.sq(lam) ^ lam ^ a2
        y3 = F.sq(x1) ^ F.mul(lam ^ 1, x3)
        return (x3, y3)
    lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
    x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ a2
    y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
    return (x3, y3)

def eval_poly(F, p, a6, xs):
    """evaluate semaev poly at a6 and array arguments xs=[arr1..arrk] (broadcast)"""
    shape = np.broadcast(*xs).shape
    tot = np.zeros(shape, dtype=np.int64)
    k = len(xs)
    # powers
    pw = []
    for x in xs:
        x = np.broadcast_to(x, shape)
        lst = [np.ones(shape, dtype=np.int64), x.astype(np.int64)]
        for e in range(2, 5): lst.append(F.vmul(lst[-1], x.astype(np.int64)))
        pw.append(lst)
    for mon in p:
        t = np.full(shape, F.pow(a6, mon[0]) if mon[0] else 1, dtype=np.int64)
        for i in range(k): t = F.vmul(t, pw[i][mon[1 + i]])
        tot ^= t
    return tot

def run(n, seed):
    rnd = random.Random(seed)
    F = GF(n)
    while True:
        a2 = rnd.randrange(2); a6 = rnd.randrange(1, F.q)
        pts = curve_points(F, a2, a6)
        if len(pts) > 4: break
    xs_with = {}
    for P in pts: xs_with.setdefault(P[0], []).append(P)
    # S3
    s3 = {k: 1 for k in semaev.S3()}
    q = F.q
    g = np.meshgrid(*[np.arange(q)] * 3, indexing='ij')
    v3 = eval_poly(F, s3, a6, g)
    Z3 = set(zip(*[a.tolist() for a in np.nonzero(v3 == 0)]))
    G3 = set()
    for P in pts:
        for Qp in pts:
            for sgn in (0, 1):
                Rr = add(F, a2, P, Qp)
                if Rr is None: continue
                # P + Q + R' = O  with R' = -(P+Q): x(R') = x(R)
                G3.add((P[0], Qp[0], Rr[0]))
    # include P+Q=O: x1==x2 and x3 anything? (x3 = inf) excluded (affine only)
    missing3 = [t for t in G3 if t not in Z3]
    # S4
    s4 = {k: 1 for k in semaev.S4()}
    g4 = np.meshgrid(*[np.arange(q)] * 4, indexing='ij')
    v4 = eval_poly(F, s4, a6, g4)
    nz4 = int((v4 == 0).sum())
    G4 = set()
    # x-coordinates of sums of two points (all sign choices covered since -P has the same x)
    sums2 = {}
    for P in pts:
        for Qp in pts:
            Rr = add(F, a2, P, Qp)
            if Rr is not None: sums2.setdefault(Rr[0], set()).add((P[0], Qp[0]))
    for xr, prs in sums2.items():
        # P1+P2 = R, P3+P4 = -R  (same x)
        for (x1, x2) in prs:
            for (x3, x4) in prs:
                G4.add((x1, x2, x3, x4))
    zero4 = set(zip(*[a.tolist() for a in np.nonzero(v4 == 0)]))
    missing4 = [t for t in G4 if t not in zero4]
    return dict(n=n, a2=a2, a6=a6, npts=len(pts), S3_zeros=len(Z3), S3_rational_tuples=len(G3), S3_rational_not_zero=len(missing3),
                S4_zeros=nz4, S4_rational_tuples=len(G4), S4_rational_not_zero=len(missing4),
                S4_zeros_unexplained_by_Fq_points=nz4 - len(G4 & zero4))

if __name__ == "__main__":
    res = []
    for n, seed in [(3, 1), (4, 1), (4, 2), (5, 1), (5, 2)]:
        r = run(n, seed); print(r); res.append(r)
        assert r["S3_rational_not_zero"] == 0 and r["S4_rational_not_zero"] == 0
    import json; json.dump(res, open("verify_semantics_results.json", "w"), indent=1)
    print("OK: every F_q-rational relation P1+..+Pk=O gives a zero of S_k")
