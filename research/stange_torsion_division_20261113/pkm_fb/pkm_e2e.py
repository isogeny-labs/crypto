"""Toy end-to-end index calculus with PKM's p-1 subgroup factor base (m = 2) vs store-all rho.

Curve y^2 = x^3 + A x + B over F_p with prime order r (brute point count, seeded), p the smallest
prime >= 2^(2k) with 2^k | p - 1 (PKM rule; p < 2^16 here, outside the msolve crash zone).
Factor base F: one point per x in V = mu_{2^k} that is an x-coordinate (sign fixed). Targets
R = aP + bQ with fresh random a, b (scalar multiplication uncounted; charged 1 point addition each,
as in ../ic_e2e). Decomposition: msolve 0.6.5 reduced DRL
basis of the chosen encoding (T2 tower, PKM system (4); or D: X^n - 1), solutions read from the
basis (candidates v1 in V, v2 from the quadratic S3(v1, X2, xR) = 0, kept iff every basis element
vanishes at the lifted point; count asserted equal to the quotient dimension). Each solution with
both coordinates in the x-set gives R = s1 F_i + s2 F_j (signs found by trial). Collect |F| + 1
relations (more on rank deficiency), solve mod r, assert log == true w.
Rho: rho_store_neg from ../ic_e2e/ic_mt.py (stores every point, x-coordinate matching = negation
map), point additions, mean of 20 runs, every log checked.
Units: IC walk additions and msolve calls / seconds; rho additions and Python wall seconds. One
msolve call per target, single run (nondeterministic time). Exploratory; not a bench.record run.
Usage: pkm_e2e.py k seed arm
"""
import sys, os, time, json, random, math, re, subprocess, tempfile
from sympy import isprime
k = int(sys.argv[1]); SEED = int(sys.argv[2]); ARM = sys.argv[3]; n = 2 ** k
rng = random.Random(SEED)
p = 2 ** (2 * k) + 1
while not ((p - 1) % n == 0 and isprime(p)):
    p += 1
assert p < 2 ** 16
leg = lambda a: 0 if a % p == 0 else (1 if pow(a % p, (p - 1) // 2, p) == 1 else -1)
while True:
    A, B = rng.randrange(1, p), rng.randrange(1, p)
    if (4 * A ** 3 + 27 * B * B) % p == 0:
        continue
    r = p + 1 + sum(leg(x * x * x + A * x + B) for x in range(p))
    if isprime(r):
        break
def msqrt(a):
    a %= p
    if a == 0: return 0
    q, s = p - 1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while leg(z) != -1: z += 1
    M, c, t, R_ = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt * tt % p; i += 1
        b = pow(c, 1 << (M - i - 1), p); M, c, t, R_ = i, b * b % p, t * b * b % p, R_ * b % p
    return R_
OPS = [0]
def add(P, Q):
    OPS[0] += 1
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0] and (P[1] + Q[1]) % p == 0: return None
    l = (3 * P[0] * P[0] + A) * pow(2 * P[1], -1, p) % p if P == Q else (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p; return (x, (l * (P[0] - x) - P[1]) % p)
neg = lambda P: None if P is None else (P[0], (-P[1]) % p)
def mul(e, P):
    sv = OPS[0]; R_ = None
    while e:
        if e & 1: R_ = add(R_, P)
        P = add(P, P); e >>= 1
    OPS[0] = sv; return R_
def rand_point():
    while True:
        x = rng.randrange(p); v = (x ** 3 + A * x + B) % p
        if leg(v) == 1: return (x, msqrt(v))
P = rand_point(); w = rng.randrange(1, r); Q = mul(w, P)

def rho_store_neg():
    OPS[0] = 0; K = 20; coef = [(rng.randrange(r), rng.randrange(r)) for _ in range(K)]
    M = [add(mul(a, P), mul(b, Q)) for a, b in coef]
    a, b = rng.randrange(r), rng.randrange(r); X = add(mul(a, P), mul(b, Q)); OPS[0] = 0; seen = {}
    while True:
        key = X[0] if X is not None else 'inf'
        if key in seen:
            a0, b0, X0 = seen[key]
            da, db = ((a0 - a) % r, (b - b0) % r) if X0 == X else ((-a0 - a) % r, (b + b0) % r)
            return (da * pow(db, -1, r) % r if db else None), OPS[0]
        seen[key] = (a, b, X)
        i = hash(X) % K if X is not None else 0; X = add(X, M[i]); a = (a + coef[i][0]) % r; b = (b + coef[i][1]) % r

g = 2
while leg(g) != -1: g += 1
zeta = pow(g, (p - 1) // n, p); MU = [pow(zeta, j, p) for j in range(n)]; MUS = set(MU)
FB = []
for v in MU:
    if leg(v ** 3 + A * v + B) == 1: FB.append((v, msqrt(v ** 3 + A * v + B)))
FBI = {pt[0]: i for i, pt in enumerate(FB)}

def S3c(a, c):  # S3(a, X, c) as quadratic in X: coefficients (q2, q1, q0)
    # S3(a,b,c) = (a-b)^2 c^2 - 2((a+b)(ab+A)+2B) c + (ab-A)^2 - 4B(a+b)
    q2 = (c * c - 2 * a * c + a * a) % p
    q1 = (-2 * a * c * c - 2 * (a * a + A) * c - 2 * a * A - 4 * B) % p
    q0 = (a * a * c * c - 2 * (a * A + 2 * B) * c + A * A - 4 * B * a) % p
    return q2, q1, q0
def S3(a, b, c): return ((a - b) ** 2 * c ** 2 - 2 * ((a + b) * (a * b + A) + 2 * B) * c + (a * b - A) ** 2 - 4 * B * (a + b)) % p

def system(xR):
    # S3(x1, x2, xR) with xR substituted, as msolve text
    from sympy import symbols, Poly
    a, b = symbols('a b'); Pl = Poly((a - b) ** 2 * xR ** 2 - 2 * ((a + b) * (a * b + A) + 2 * B) * xR + (a * b - A) ** 2 - 4 * B * (a + b), a, b, modulus=p)
    if ARM == 'D':
        vs = ['x1', 'x2']; top = vs; eqs = [f'x1^{n}-1', f'x2^{n}-1']; lift = lambda v1, v2: [v1, v2]
    else:
        vs = [f'x{i}_{j}' for i in (1, 2) for j in range(1, k + 1)]; top = ['x1_1', 'x2_1']; eqs = []
        for i in (1, 2):
            eqs += [f'x{i}_{j+1}-x{i}_{j}^2' for j in range(1, k)] + [f'1-x{i}_{k}^2']
        lift = lambda v1, v2: [pow(v1, 2 ** j, p) for j in range(k)] + [pow(v2, 2 ** j, p) for j in range(k)]
    sp = '+'.join(f"{int(c) % p}" + ''.join(f"*{v}^{e}" for v, e in zip(top, mon) if e) for mon, c in Pl.terms())
    return vs, [sp] + eqs, lift

def run_ms(vs, eqs):
    with tempfile.TemporaryDirectory() as dd:
        fi = os.path.join(dd, 'i'); fo = os.path.join(dd, 'o')
        open(fi, 'w').write(",".join(vs) + f"\n{p}\n" + ",\n".join(eqs) + "\n")
        t1 = time.perf_counter(); rr = subprocess.run(['msolve', '-g', '2', '-t', '1', '-f', fi, '-o', fo], capture_output=True, text=True)
        dt = time.perf_counter() - t1
        assert rr.returncode == 0, rr.returncode
        return dt, open(fo).read()
def parse_gb(out, vs):
    body = out[out.rfind('#---') + 4:].replace('\n', '').replace(' ', '').rstrip(':')
    polys = []
    for ps in body[1:-1].split(','):
        terms = {}
        for t in re.findall(r'[+-]?[^+-]+', ps):
            parts = t.split('*'); c = int(parts[0]) % p; e = [0] * len(vs)
            for f in parts[1:]:
                v, a = (f.split('^') + ['1'])[:2]; e[vs.index(v)] += int(a)
            terms[tuple(e)] = (terms.get(tuple(e), 0) + c) % p
        polys.append({e: c for e, c in terms.items() if c})
    return polys
def evalp(Pd, pt):
    s = 0
    for e, c in Pd.items():
        t = c
        for v, a in zip(pt, e):
            if a: t = t * pow(v, a, p) % p
        s += t
    return s % p

def decompose(xR):
    vs, eqs, lift = system(xR); dt, out = run_ms(vs, eqs); gb = parse_gb(out, vs)
    if len(gb) == 1 and list(gb[0].keys()) == [tuple([0] * len(vs))]:
        return dt, []
    sols = []
    for v1 in MU:
        q2, q1, q0 = S3c(v1, xR)
        cands = set()
        if q2 == 0:
            if q1: cands.add(-q0 * pow(q1, -1, p) % p)
        else:
            disc = (q1 * q1 - 4 * q2 * q0) % p
            if leg(disc) >= 0:
                s = msqrt(disc); inv = pow(2 * q2, -1, p)
                cands |= {(-q1 + s) * inv % p, (-q1 - s) * inv % p}
        for v2 in cands:
            if v2 in MUS:
                assert S3(v1, v2, xR) == 0
                if all(evalp(Pd, lift(v1, v2)) == 0 for Pd in gb): sols.append((v1, v2))
    return dt, sols

def nullvec(rows, ncols):
    """left kernel vector of the relation matrix mod r (Gaussian elimination on the transpose)"""
    nr = len(rows)
    M = [[rows[i][c] % r for i in range(nr)] + [0] for c in range(ncols)]
    piv = []; rr_ = 0
    for c in range(nr):
        pr = next((i for i in range(rr_, ncols) if M[i][c]), None)
        if pr is None: continue
        M[rr_], M[pr] = M[pr], M[rr_]; iv = pow(M[rr_][c], -1, r)
        M[rr_] = [x * iv % r for x in M[rr_]]
        for i in range(ncols):
            if i != rr_ and M[i][c]:
                f = M[i][c]; M[i] = [(x - f * y) % r for x, y in zip(M[i], M[rr_])]
        piv.append(c); rr_ += 1
    free = [c for c in range(nr) if c not in piv]
    if not free: return None
    fc = free[0]; lam = [0] * nr; lam[fc] = 1
    for i, c in enumerate(piv): lam[c] = (-M[i][fc]) % r
    return lam

t0 = time.time()
K = 20; coef = [(rng.randrange(r), rng.randrange(r)) for _ in range(K)]
Mw = [add(mul(a, P), mul(b, Q)) for a, b in coef]
a, b = rng.randrange(r), rng.randrange(r); X = add(mul(a, P), mul(b, Q)); OPS[0] = 0
rows = []; targets = 0; ms_s = 0.0; ms_calls = 0; sign_adds = 0; need = len(FB) + 1; wi = None; retries = 0
while wi is None:
    while len(rows) < need:
        # fresh random target R = aP + bQ (an r-adding walk cycles after ~sqrt(r) steps at these tiny r,
        # repeating targets); scalar multiplication uncounted, charged 1 addition per target as in ../ic_e2e
        a, b = rng.randrange(r), rng.randrange(r); X = add(mul(a, P), mul(b, Q)); targets += 1
        if X is None: continue
        dt, sols = decompose(X[0]); ms_s += dt; ms_calls += 1
        for v1, v2 in {tuple(sorted(s)) for s in sols}:
            if v1 in FBI and v2 in FBI:
                F1, F2 = FB[FBI[v1]], FB[FBI[v2]]
                for s1 in (1, -1):
                    for s2 in (1, -1):
                        if add(F1 if s1 == 1 else neg(F1), F2 if s2 == 1 else neg(F2)) == X: rel = (s1, s2)
                sign_adds += 1   # charged one addition per hit, as in ../ic_e2e
                row = [0] * len(FB); row[FBI[v1]] += rel[0]; row[FBI[v2]] += rel[1]
                rows.append((row, a, b))   # sum row_i F_i = aP + bQ
    lam = nullvec([rw for rw, _, _ in rows], len(FB))
    if lam is not None:
        sa = sum(l * rows[i][1] for i, l in enumerate(lam)) % r; sb = sum(l * rows[i][2] for i, l in enumerate(lam)) % r
        if sb: wi = (-sa * pow(sb, -1, r)) % r; break
    need += 1; retries += 1
assert wi == w, ('ic wrong', wi, w)
walk_adds = OPS[0]
rho = []; t1 = time.perf_counter()
for _ in range(20):
    wr, ops = rho_store_neg(); assert wr is None or wr == w, 'rho wrong'; rho.append((ops, wr is not None))
rho_s = (time.perf_counter() - t1) / 20
print(json.dumps({'k': k, 'n': n, 'p': p, 'r': r, 'A': A, 'B': B, 'seed': SEED, 'arm': ARM, 'fb_size': len(FB),
                  'ic': {'log_ok': True, 'targets': targets, 'walk_and_sign_adds': walk_adds, 'msolve_calls': ms_calls,
                         'msolve_wall_s_total': ms_s, 'relations': len(rows), 'retries': retries, 'wall_s': time.time() - t0},
                  'rho_store_neg': {'ops_mean': sum(o for o, _ in rho) / len(rho), 'ops_runs': [o for o, _ in rho],
                                    'solved': sum(s for _, s in rho), 'python_wall_s_mean': rho_s}}))
