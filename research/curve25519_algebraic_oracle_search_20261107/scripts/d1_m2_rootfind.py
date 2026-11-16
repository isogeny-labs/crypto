"""D1: m = 2 full-coverage random base; root-find per base point.
(1) identity: roots in x2 of S3(x1, x2, xR) = {x(R+P1), x(R-P1)} on 2,000 random instances;
(2) exact coverage C(F) by enumerating all pair sums;
(3) counted decision cost per trial: (a) screen-style quadratic solve with sqrt,
    (b) shared-denominator affine additions with batch inversion; hit rate and relation check."""
import json, random, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import fp
from fp import p, A, r, mul, inv, batch_inv, CNT
D = pathlib.Path(__file__).resolve().parents[1]
rng = random.Random(20261107)

# (1) identity
bad = 0
for _ in range(2000):
    P1 = fp.u_rand_Er(rng); R = fp.u_rand_Er(rng)
    xs = {fp.u_add(R, P1)[0], fp.u_add(R, fp.u_neg(P1))[0]}
    for x2 in xs:
        if fp.S3(P1[0], x2, R[0]) != 0: bad += 1
    # quadratic in x2 has exactly these roots: check discriminant square root form
    a = (P1[0] - R[0]) ** 2 % p
    s = 2 * ((P1[0] + R[0]) * (P1[0] * R[0] + 1) + 2 * A * P1[0] * R[0]) * pow(a, p - 2, p) % p
    if sum(xs) % p != s: bad += 1
identity = dict(instances=2000, failures=bad)
print("identity failures", bad)

def base(F, rng):
    B = []; seen = set()
    while len(B) < F:
        P = fp.u_rand_Er(rng)
        if P[0] not in seen: seen.add(P[0]); B.append(P)
    return B

def coverage(B):
    xs = set()
    for i in range(len(B)):
        for j in range(i, len(B)):
            for Q in (fp.u_add(B[i], B[j]), fp.u_add(B[i], fp.u_neg(B[j]))):
                if Q is not None: xs.add(Q[0])
    return 2 * len(xs)  # points: +-

def decide_add(R, B, Bx):
    xR, yR = R
    invs = batch_inv([(xR - x) % p for x, _ in B])
    rel = []
    for (x1, y1), iv in zip(B, invs):
        for sgn in (1, -1):  # slope for R + sgn*P1
            lam = mul((yR - sgn * y1) % p, iv)
            x3 = (mul(lam, lam) - A - xR - x1) % p
            if x3 in Bx: rel.append((x1, y1, sgn, x3))
    return rel

def csqrt(a):  # counted Atkin sqrt (one exponentiation (p-5)/8 plus 4 mults)
    def cpow(b, e):
        res = 1; first = True
        for bit in bin(e)[2:]:
            if first: res = b; first = False; continue
            res = mul(res, res)
            if bit == '1': res = mul(res, b)
        return res
    b = cpow(2 * a % p, (p - 5) // 8); i = mul(mul(2 * a % p, b), b)
    return mul(mul(a, b), (i - 1) % p)

def decide_quad(R, B, Bx):  # screen-style: solve S3(x1, X, xR) = 0 per base point
    xR = R[0]; rel = []
    for (x1, y1) in B:
        t = mul(x1, xR); a = mul(x1 - xR, x1 - xR)
        b = (-2 * (mul(x1 + xR, t + 1) + 2 * A * t)) % p  # 2A*t: scalar, counted below
        CNT['M'] += 1
        e = mul(t - 1, t - 1)
        disc = (mul(b, b) - 4 * mul(a, e)) % p
        s = csqrt(disc); ia = inv(2 * a % p)
        for sg in (1, -1):
            x3 = mul((-b + sg * s) % p, ia)
            if x3 in Bx: rel.append((x1, y1, sg, x3))
    return rel

out = dict(identity=identity, rows=[])
for F in (128, 192, 256, 320, 384, 448):
    rb = random.Random(1000 + F); B = base(F, rb); Bx = {P[0] for P in B}
    C = coverage(B)
    trials = 300; hits = 0; cost = 0; ok = 0
    for _ in range(trials):
        R = fp.u_rand_Er(rng); fp.reset(); rel = decide_add(R, B, Bx); cost += CNT['M']
        if rel:
            hits += 1
            x1, y1, sgn, x3 = rel[0]; Q = fp.u_add(R, (x1, sgn * y1 % p))
            ok += (Q is not None and Q[0] == x3)
    cq = 0; qa = 0
    for _ in range(20):
        R = fp.u_rand_Er(rng); fp.reset(); rq = decide_quad(R, B, Bx); cq += CNT['M'] / 20
        fp.reset(); ra = decide_add(R, B, Bx); qa += (sorted(x[3] for x in rq) == sorted(x[3] for x in ra))
    row = dict(F=F, C=C, C_over_r=C / r, trials=trials, hit_rate=hits / trials, relations_checked_ok=ok,
               c_add=cost / trials, c_add_per_base=cost / trials / F, c_quad=cq, quad_matches_add_of_20=qa, c_quad_per_base=cq / F,
               group_ops_per_trial_PG=2 * F)
    out['rows'].append(row); print(row)
json.dump(out, open(D / "data/d1_m2.json", "w"), indent=1)
