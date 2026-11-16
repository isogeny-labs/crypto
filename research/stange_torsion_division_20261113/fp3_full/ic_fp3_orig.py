#!/usr/bin/env python3
"""Gaudry index calculus on E/F_{p^3} (n = 3) versus matched Pollard rho.

F_{p^3} = F_p[t]/(t^3 - t - c), p = 3 mod 4. E: y^2 = x^3 + a x + b, #E(F_{p^3}) = r prime.
Factor base: points with x in F_p. Target R from a walk with RNG-chosen multipliers (repeats skipped).
Decomposition R = s1 F1 + s2 F2 + s3 F3: S4(e1, e2, e3; x(R)) = 0 (S4 symmetrized, total degree 4,
35 monomials, from s4_sym.json) written over F_p by Weil descent: 3 equations in e1, e2, e3,
solved by msolve 0.6.5 (rational parametrization over F_p), F_p roots of the eliminating polynomial,
then T^3 - e1 T^2 + e2 T - e3 must have 3 distinct F_p roots, all in the factor base.
Linear algebra: incremental sparse Gaussian elimination mod r (exact); Z_r multiplications counted.
Modes replayed on the same relation stream: all (factor base = every x in F_p), and with small set
x < B (B = p/2): full:B (3 small), 1lp:B (>= 2 small), 2lp:B (>= 1 small).

Counted (F_p mults M, inversions I): rho point additions; IC walk additions, building the descended
system, the root finding of the eliminating polynomial (gcd(z^p - z, f) and splitting), the
cubic roots, factor-base tests, y lifts and sign additions. NOT counted: msolve's Groebner basis /
parametrization work (reported as wall time, and as the break-even budget it would have to meet).
F_{p^3} mult: schoolbook 9 M + 2 M reduction (c), sqr 6 + 2 M; inversion priced 12 M + 1 I (adjugate).
"""
import json, math, os, random, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ic_fp2 as F   # F_p counters, polynomial root finding, Z_r helpers

C = F.C
p = 0
cmod = 0
HERE = os.path.dirname(os.path.abspath(__file__))
SCR = os.environ.get('SCR', '/tmp')


def setp(pp):
    global p
    p = pp; F.p = pp


def red(c5):
    # c5 coefficients deg 0..4, t^3 = t + c, t^4 = t^2 + c t
    c0, c1, c2, c3, c4 = c5
    c0 += cmod * c3; c1 += c3
    c1 += cmod * c4; c2 += c4
    return (c0 % p, c1 % p, c2 % p)


def m3(x, y):
    C[0] += 11
    a0, a1, a2 = x; b0, b1, b2 = y
    return red((a0*b0, a0*b1 + a1*b0, a0*b2 + a1*b1 + a2*b0, a1*b2 + a2*b1, a2*b2))


def s3(x):
    C[0] += 8
    a0, a1, a2 = x
    return red((a0*a0, 2*a0*a1, 2*a0*a2 + a1*a1, 2*a1*a2, a2*a2))


def ad(x, y):
    return ((x[0]+y[0]) % p, (x[1]+y[1]) % p, (x[2]+y[2]) % p)


def sb(x, y):
    return ((x[0]-y[0]) % p, (x[1]-y[1]) % p, (x[2]-y[2]) % p)


def ng(x):
    return ((-x[0]) % p, (-x[1]) % p, (-x[2]) % p)


def mfp(x, k):
    C[0] += 3
    return (x[0]*k % p, x[1]*k % p, x[2]*k % p)


def pw_raw(x, e):
    r = (1, 0, 0)
    for bit in bin(e)[2:]:
        r = red_raw_mul(r, r)
        if bit == '1':
            r = red_raw_mul(r, x)
    return r


def red_raw_mul(x, y):
    a0, a1, a2 = x; b0, b1, b2 = y
    return red((a0*b0, a0*b1 + a1*b0, a0*b2 + a1*b1 + a2*b0, a1*b2 + a2*b1, a2*b2))


def i3(x):
    C[0] += 12; C[1] += 1
    return pw_raw(x, p**3 - 2)


def pw(x, e):
    r = None
    for bit in bin(e)[2:]:
        if r is not None:
            r = s3(r)
        if bit == '1':
            r = x if r is None else m3(r, x)
    return r


def norm_raw(x):
    xp = pw_raw(x, p); xpp = pw_raw(xp, p)
    n = red_raw_mul(red_raw_mul(x, xp), xpp)
    assert n[1] == 0 and n[2] == 0
    return n[0]


def sqrt3(x):
    """q = p^3 = 3 mod 4: x^((q+1)/4), counted"""
    if x == (0, 0, 0):
        return x
    r = pw(x, (p**3 + 1) // 4)
    if s3(r) != x:
        return None
    return r


def padd(P, Q, a):
    if P is None:
        return Q
    if Q is None:
        return P
    if P[0] == Q[0]:
        if ad(P[1], Q[1]) == (0, 0, 0):
            return None
        x2 = s3(P[0])
        lam = m3(ad(ad(ad(x2, x2), x2), a), i3(ad(P[1], P[1])))
    else:
        lam = m3(sb(Q[1], P[1]), i3(sb(Q[0], P[0])))
    x3 = sb(sb(s3(lam), P[0]), Q[0])
    return (x3, sb(m3(lam, sb(P[0], x3)), P[1]))


def pneg(P):
    return None if P is None else (P[0], ng(P[1]))


def smul(k, P, a):
    R = None
    for bit in bin(k)[2:]:
        R = padd(R, R, a)
        if bit == '1':
            R = padd(R, P, a)
    return R


def rand_point(a, b, rng):
    while True:
        x = (rng.randrange(p), rng.randrange(p), rng.randrange(p))
        rhs = ad(ad(m3(s3(x), x), m3(a, x)), b)
        y = sqrt3(rhs)
        if y is not None:
            return (x, y)


def find_field():
    global cmod
    for c in range(1, p):
        if all((x**3 - x - c) % p for x in range(p)):
            cmod = c
            return c


def find_curve(rng):
    q = p**3
    sq = math.isqrt(q) + 1
    while True:
        a = (rng.randrange(p), rng.randrange(p), rng.randrange(p))
        b = (rng.randrange(p), rng.randrange(p), rng.randrange(p))
        P = rand_point(a, b, rng)
        L = q + 1 - 2 * sq
        W = 4 * sq + 1
        m = math.isqrt(W) + 1
        baby = {}
        R = None
        for j in range(m):
            baby.setdefault(R, j)
            R = padd(R, P, a)
        mP = R
        G = smul(L, P, a)
        found = None
        for i in range(m + 1):
            for key, sgn in ((pneg(G), 1), (G, -1)):
                if key in baby:
                    N = L + i * m + sgn * baby[key]
                    if abs(N - q - 1) <= 2 * math.isqrt(q) + 1 and F.is_prime(N):
                        found = N
                        break
            if found:
                break
            G = padd(G, mP, a)
        if found and smul(found, P, a) is None:
            return a, b, found

# ------------------------------------------------------------------------- S4 system

S4 = json.load(open(os.path.join(HERE, 's4_sym.json')))['terms']


class Sys:
    def __init__(self, a, b):
        # per-curve precomputation (uncounted, done once): a^i b^j
        self.ab = {}
        for _, _, _, cps in S4:
            for ex4, ea, eb, v in cps:
                if (ea, eb) not in self.ab:
                    self.ab[(ea, eb)] = red_raw_mul(pw_raw(a, ea), pw_raw(b, eb))
        self.maxx4 = max(t[0] for _, _, _, cps in S4 for t in cps)

    def build(self, x4):
        """coefficients of e1^i e2^j e3^k as F_{p^3} elements -> three F_p polynomials (counted)."""
        pows = [(1, 0, 0), x4]
        for _ in range(self.maxx4 - 1):
            pows.append(m3(pows[-1], x4))
        polys = [[], [], []]
        for i, j, k, cps in S4:
            acc = (0, 0, 0)
            for ex4, ea, eb, v in cps:
                term = m3(pows[ex4], self.ab[(ea, eb)]) if ex4 and (ea or eb) else (pows[ex4] if not (ea or eb) else self.ab[(ea, eb)])
                C[0] += 3
                acc = ad(acc, ((term[0]*v) % p, (term[1]*v) % p, (term[2]*v) % p))
            for comp in range(3):
                if acc[comp]:
                    polys[comp].append((i, j, k, acc[comp]))
        return polys


def msolve(polys, tag):
    names = ['e1', 'e2', 'e3']
    lines = []
    for P in polys:
        if not P:
            continue
        terms = []
        for i, j, k, c in P:
            mono = '*'.join([f'{n}^{d}' for n, d in zip(names, (i, j, k)) if d])
            terms.append(f'{c}' + (('*' + mono) if mono else ''))
        lines.append('+'.join(terms))
    fin = os.path.join(SCR, f'ms_{tag}.in'); fout = os.path.join(SCR, f'ms_{tag}.out')
    with open(fin, 'w') as fh:
        fh.write('e1,e2,e3\n%d\n' % p + ',\n'.join(lines) + '\n')
    t0 = time.time()
    rc = subprocess.run(['msolve', '-f', fin, '-o', fout], capture_output=True).returncode
    dt = time.time() - t0
    if rc != 0:
        return None, dt
    s = open(fout).read().strip().rstrip(':')
    d = eval(s)
    return d, dt


def eval_poly(c, z):
    return sum(v * pow(z, i, p) for i, v in enumerate(c)) % p


def solve_sys(polys, rng, tag):
    """F_p solutions (e1, e2, e3); also returns (deg elim, msolve sec, status)"""
    d, dt = msolve(polys, tag)
    if d is None:
        return [], 0, dt, 'msolve_error'
    if d[0] == -1 or d == [-1] or (isinstance(d, list) and len(d) == 1 and d[0] == -1):
        return [], 0, dt, 'nosol'
    body = d[1]
    if len(body) < 6 or body[2] == -1:
        return [], 0, dt, 'posdim'
    nv, deg, vars_, lin = body[1], body[2], body[3], body[4]
    if list(lin) != [0, 0, 1] or vars_ != ['e1', 'e2', 'e3']:
        return [], deg, dt, 'nongeneric'
    par = body[5][1]
    elim = par[0][1]; den = par[1][1]
    nums = [par[2][0][0][1], par[2][1][0][1]]
    # counted root finding of the eliminating polynomial (same routine as n = 2)
    roots = F.proots(F.ptrim(list(elim)), rng) if len(elim) > 1 else []
    sols = []
    for z in roots:
        dz = eval_poly(den, z)
        C[0] += 3 * len(elim)  # evaluating den and two numerators (Horner)
        if dz == 0:
            continue
        idz = F.inv(dz)
        e1 = (-eval_poly(nums[0], z) * idz) % p; e2 = (-eval_poly(nums[1], z) * idz) % p
        C[0] += 2
        sols.append((e1, e2, z))
    # check against the system (uncounted verification)
    good = []
    for e in sols:
        ok = all(sum(c * pow(e[0], i, p) * pow(e[1], j, p) * pow(e[2], k, p) for i, j, k, c in P) % p == 0 for P in polys)
        if ok:
            good.append(e)
    return good, deg, dt, 'ok' if len(good) == len(sols) else 'badparam'


def canon(y):
    n = ng(y)
    return y if y <= n else n


def decompose(R, a, b, S, rng, tag, stats):
    polys = S.build(R[0])
    sols, deg, dt, st = solve_sys(polys, rng, tag)
    stats['msolve_sec'] += dt; stats['calls'] += 1; stats['deg'].append(deg); stats['status'][st] = stats['status'].get(st, 0) + 1
    out = []
    for e1, e2, e3 in sols:
        cub = [(-e3) % p, e2 % p, (-e1) % p, 1]
        xs = F.proots(cub, rng)
        if len(xs) != 3:
            continue
        pts = []
        for X in xs:
            fx = ad(ad(mfp((X*X*X % p, 0, 0), 1), mfp(a, X)), b)
            C[0] += 2
            C[0] += 22  # norm (two Frobenius-twisted products)
            if F.legendre(norm_raw(fx)) != 1:
                break
            pts.append(((X, 0, 0), canon(sqrt3(fx))))
        if len(pts) < 3:
            continue
        found = False
        for s2 in (1, -1):
            T = padd(pts[0], pts[1] if s2 == 1 else pneg(pts[1]), a)
            for s3_ in (1, -1):
                U = padd(T, pts[2] if s3_ == 1 else pneg(pts[2]), a)
                if U is not None and U[0] == R[0]:
                    s1 = 1 if U[1] == R[1] else -1
                    out.append(((xs[0], s1), (xs[1], s1 * s2), (xs[2], s1 * s3_)))
                    found = True
                    break
            if found:
                break
        assert found, "S4 solution without a sign pattern"
    return out


class Elim:
    """incremental elimination mod r of rows {x: coef} = A P + B Q"""

    def __init__(self, r):
        self.r = r; self.piv = {}

    def add(self, row, A, B):
        r = self.r
        row = {k: v % r for k, v in row.items() if v % r}
        while row:
            k = max(row)
            if k not in self.piv:
                c = pow(row[k], -1, r); C[2] += 3
                self.piv[k] = ({kk: v * c % r for kk, v in row.items()}, A * c % r, B * c % r)
                C[2] += len(row) + 2
                return None
            prow, pA, pB = self.piv[k]
            c = row[k]
            for kk, v in prow.items():
                nv = (row.get(kk, 0) - c * v) % r
                if nv:
                    row[kk] = nv
                else:
                    row.pop(kk, None)
            A = (A - c * pA) % r; B = (B - c * pB) % r
            C[2] += len(prow) + 2
        return (A, B)


def rho_run(P, Q, a, r, w, rng):
    C[0] = C[1] = C[2] = 0
    W = F.Walk.__new__(F.Walk)
    mult = []
    for _ in range(20):
        c = rng.randrange(r); d = rng.randrange(r)
        mult.append((padd(smul(c, P, a), smul(d, Q, a), a), c, d))
    c = rng.randrange(r); d = rng.randrange(r)
    R = padd(smul(c, P, a), smul(d, Q, a), a)
    C[0] = C[1] = C[2] = 0
    seen = {}; steps = 0; t0 = time.time()
    while True:
        h = (R[0][0] * 7 + R[0][1] * 13 + R[0][2] * 5 + R[1][0]) % 20
        M, cc, dd = mult[h]
        R = padd(R, M, a); c = (c + cc) % r; d = (d + dd) % r; steps += 1
        if R is None:
            continue
        if R[0] in seen:
            y, c0, d0 = seen[R[0]]
            A, B = ((c - c0), (d - d0)) if y == R[1] else ((c + c0), (d + d0))
            ww = F.solve_eq(A % r, B % r, r)
            if ww is not None:
                assert ww == w, 'rho wrong'
                return dict(steps=steps, M=C[0], I=C[1], sec=time.time() - t0)
        else:
            seen[R[0]] = (R[1], c, d)


def ic_run(P, Q, a, b, r, w, rng, tag, cap=4.0):
    C[0] = C[1] = C[2] = 0
    mult = []
    for _ in range(20):
        c = rng.randrange(r); d = rng.randrange(r)
        mult.append((padd(smul(c, P, a), smul(d, Q, a), a), c, d))
    c = rng.randrange(r); d = rng.randrange(r)
    R = padd(smul(c, P, a), smul(d, Q, a), a)
    S = Sys(a, b)
    C[0] = C[1] = C[2] = 0
    B = p // 2
    modes = {'all': 0, 'full:B': 3, '1lp:B': 2, '2lp:B': 1}
    el = {m: Elim(r) for m in modes}
    zr = {m: 0 for m in modes}
    kept = {m: 0 for m in modes}
    done = {}
    stats = dict(msolve_sec=0.0, calls=0, deg=[], status={})
    seen = set(); trials = 0; rels = 0; t0 = time.time(); first = None
    while len(done) < len(modes):
        M_, cc, dd = mult[rng.randrange(20)]
        R = padd(R, M_, a); c = (c + cc) % r; d = (d + dd) % r; trials += 1
        if R is None or R[0] in seen:
            continue
        seen.add(R[0])
        if R[0][1] == 0 and R[0][2] == 0:
            continue
        for rel in decompose(R, a, b, S, rng, tag, stats):
            rels += 1
            nsmall = sum(1 for (x, s) in rel if x < B)
            for m, need in modes.items():
                if m in done or nsmall < need:
                    continue
                kept[m] += 1
                row = {}
                for x, s in rel:
                    row[x] = row.get(x, 0) + s
                z0 = C[2]
                eq = el[m].add(row, c, d)
                zr[m] += C[2] - z0
                if eq is not None:
                    ww = F.solve_eq(eq[0], eq[1], r)
                    if ww is not None:
                        assert ww == w, 'IC wrong log'
                        done[m] = dict(trials=trials, rels_seen=rels, rels_kept=kept[m], M=C[0], I=C[1], Zr=zr[m],
                                       msolve_calls=stats['calls'], msolve_sec=stats['msolve_sec'], sec=time.time() - t0)
        if done and first is None:
            first = trials
        if first and trials > cap * first:
            break
    for m in modes:
        done.setdefault(m, dict(censored=True, trials=trials))
    stats['deg_mean'] = sum(stats['deg']) / max(1, len(stats['deg'])); del stats['deg']
    return done, stats


def main():
    pp = int(sys.argv[1]); seed = int(sys.argv[2]); ntarg = int(sys.argv[3]); nrho = int(sys.argv[4]); out = sys.argv[5]
    setp(pp); assert p % 4 == 3
    find_field()
    rng = random.Random(seed)
    t0 = time.time()
    a, b, r = find_curve(rng)
    G = rand_point(a, b, rng)
    res = dict(p=p, n=3, seed=seed, cmod=cmod, a=a, b=b, r=r, log2r=math.log2(r), gen_sec=time.time() - t0, targets=[])
    for k in range(ntarg):
        w = rng.randrange(1, r); Q = smul(w, G, a)
        rho = [rho_run(G, Q, a, r, w, rng) for _ in range(nrho)]
        ic, st = ic_run(G, Q, a, b, r, w, rng, f'{p}_{seed}_{k}')
        res['targets'].append(dict(w=w, rho=rho, ic=ic, decomposition=st))
        print(p, seed, k, 'rho', [x['steps'] for x in rho], {m: (v['trials'], v.get('M')) for m, v in ic.items()}, st, flush=True)
    json.dump(res, open(out, 'w'), indent=1)


if __name__ == '__main__':
    main()
