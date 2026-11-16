"""D7 (own design): small-abscissa base B_X = {P : x(P) in [1, X]} and a lattice oracle.
S3(x1, x2, xR) in s = x1 + x2, t = x1 x2 is linear in the six monomials (1, s, t, s^2, st, t^2):
  xR^2 (s^2 - 4t) - 2 xR (st + s + 2A t) + t^2 - 2t + 1 = 0 (mod p).
Per trial: one LLL reduction of a weighted 6-dimensional lattice, O(1) work independent of F.
Measured: exact coverage of B_X (enumeration), LLL recovery rate on planted R = +-P1 +- P2 with
x1, x2 <= X, and counted arithmetic operations of the LLL (each integer/rational multiplication or
division = 1 op; multi-precision, so this undercounts native cost)."""
import json, random, sys, pathlib
from fractions import Fraction
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import fp
from fp import p, A, r
D = pathlib.Path(__file__).resolve().parents[1]
rng = random.Random(202611077)
OPS = [0]

def dot(u, v):
    OPS[0] += len(u); return sum(a * b for a, b in zip(u, v))

def lll(B, delta=Fraction(3, 4)):
    B = [list(b) for b in B]; n = len(B)
    def gso():
        Bs = []; mu = [[Fraction(0)] * n for _ in range(n)]; nrm = []
        for i in range(n):
            v = [Fraction(x) for x in B[i]]
            for j in range(i):
                mu[i][j] = Fraction(dot(B[i], Bs[j])) / nrm[j]; OPS[0] += 1
                v = [a - mu[i][j] * b for a, b in zip(v, Bs[j])]; OPS[0] += n
            Bs.append(v); nrm.append(dot(v, v))
        return Bs, mu, nrm
    Bs, mu, nrm = gso(); k = 1
    while k < n:
        for j in range(k - 1, -1, -1):
            q = round(mu[k][j])
            if q:
                B[k] = [a - q * b for a, b in zip(B[k], B[j])]; OPS[0] += n
                Bs, mu, nrm = gso()
        if nrm[k] >= (delta - mu[k][k - 1] ** 2) * nrm[k - 1]:
            k += 1
        else:
            B[k], B[k - 1] = B[k - 1], B[k]; Bs, mu, nrm = gso(); k = max(k - 1, 1)
    return B

def oracle(xR, X):
    c = [1, (-2 * xR) % p, (-4 * xR * xR - 4 * A * xR - 2) % p, xR * xR % p, (-2 * xR) % p, 1]  # coeff of t^2 = 1
    size = [1, 2 * X, X * X, 4 * X * X, 2 * X ** 3, X ** 4]; Kw = X ** 4
    wgt = [Kw // s for s in size]
    basis = []
    for i in range(5):
        v = [0] * 6; v[i] = 1; v[5] = (-c[i]) % p; basis.append(v)
    basis.append([0, 0, 0, 0, 0, p])
    Bw = [[v[i] * wgt[i] for i in range(6)] for v in basis]
    Rd = lll(Bw)
    for v in Rd:
        u = [v[i] // wgt[i] for i in range(6)]
        if u[0] in (1, -1):
            u = [x * u[0] for x in u]
            s, t = u[1], u[2]
            if u[3] == s * s and u[4] == s * t and u[5] == t * t:
                disc = s * s - 4 * t
                if disc >= 0:
                    q = int(round(disc ** 0.5))
                    if q * q == disc and (s + q) % 2 == 0:
                        return ((s + q) // 2, (s - q) // 2)
    return None

def base_X(X):
    return [P for P in (fp.u_lift(x) for x in range(1, X + 1)) if P]

out = []
for X in (2, 3, 4, 6, 8, 12, 16, 32):
    B = base_X(X)
    pts = set()
    for i in range(len(B)):
        for j in range(i, len(B)):
            for a in (B[i], fp.u_neg(B[i])):
                for b in (B[j], fp.u_neg(B[j])):
                    Q = fp.u_add(a, b)
                    if Q is not None and fp.u_mul(r, Q) is None: pts.add(Q)
    pts = list(pts); C = len(pts)
    succ = 0; n = 0; OPS[0] = 0
    for Q in (rng.sample(pts, min(40, len(pts))) if pts else []):
        res = oracle(Q[0], X); n += 1
        if res and fp.S3(res[0], res[1], Q[0]) == 0 and 1 <= min(res) and max(res) <= X: succ += 1
    row = dict(X=X, base_points=len(B), C_points=C, C_over_r=C / r, planted=n, recovered=succ,
               lll_ops_per_trial=OPS[0] / n if n else None)
    out.append(row); print(row)
# positive control: planted vectors with entries in [-3, 3] in the same lattice shape are recovered
pc = 0
for _ in range(20):
    v0 = [1] + [rng.randrange(-3, 4) for _ in range(5)]
    c = [0] + [rng.randrange(p) for _ in range(4)]
    c[0] = (-(sum(c[i] * v0[i] for i in range(1, 5)) + v0[5])) % p
    basis = [[1 if j == i else 0 for j in range(5)] + [(-c[i]) % p] for i in range(5)] + [[0] * 5 + [p]]
    pc += any(b == v0 or b == [-x for x in v0] for b in lll(basis))
# Gaussian-heuristic bound: weighted target ~ sqrt(6) X^4 must be below (p X^14 / 16)^(1/6)
xmax = (p / (16 * 6 ** 3)) ** 0.1
print("positive control", pc, "/20; heuristic X_max", xmax)
json.dump(dict(rows=out, positive_control_recovered_of_20=pc, heuristic_X_max=xmax), open(D / "data/d7_lattice.json", "w"), indent=1)
