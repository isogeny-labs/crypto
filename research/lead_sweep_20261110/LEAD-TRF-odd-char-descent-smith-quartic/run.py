#!/usr/bin/env python3
"""Toy for LEAD-TRF-odd-char-descent-smith-quartic. stdlib only, fixed seeds.
Question: do smooth plane quartics over F_q exist whose Jacobian has the Frobenius
polynomial x^6 - t x^3 + q^3 of a Weil restriction Res_{F_q3/F_q}(E), for (nearly)
all admissible traces t?  (a1=a2=0 <=> N1=q+1, N2=q^2+1.)"""
import sys, json, random, time, itertools
SMOKE = "--smoke" in sys.argv
T0 = time.time()
MONOS = [(i, j, 4 - i - j) for i in range(5) for j in range(5 - i)]  # 15 degree-4 monomials

class Fld:
    def __init__(s, q, k):
        s.q, s.k = q, k
        s.mod = None
        if k > 1:
            for c in itertools.product(range(q), repeat=k):  # x^k + c[k-1]x^{k-1}+..+c[0]
                if all((sum(c[i] * pow(r, i, q) for i in range(k)) + pow(r, k, q)) % q for r in range(q)):
                    s.mod = c; break
    def mul(s, a, b):
        q, k = s.q, s.k
        if k == 1: return ((a[0] * b[0]) % q,)
        r = [0] * (2 * k - 1)
        for i, x in enumerate(a):
            if x:
                for j, y in enumerate(b): r[i + j] += x * y
        for d in range(2 * k - 2, k - 1, -1):
            c = r[d]
            if c:
                for i in range(k): r[d - k + i] -= c * s.mod[i]
                r[d] = 0
        return tuple(x % q for x in r[:k])
    def elems(s): return list(itertools.product(range(s.q), repeat=s.k))
    def add(s, a, b): return tuple((x + y) % s.q for x, y in zip(a, b))
    def sc(s, c, a): return tuple((c * x) % s.q for x in a)

def points(F):
    E = F.elems(); one = tuple([1] + [0] * (F.k - 1)); zero = tuple([0] * F.k)
    P = [(x, y, one) for x in E for y in E] + [(x, one, zero) for x in E] + [(one, zero, zero)]
    return P

def planes(F, P):
    """per point: k byte-strings (digit planes) of the 15 monomial values"""
    out = []
    one = tuple([1] + [0] * (F.k - 1))
    for p in P:
        pw = []
        for c in p:
            l = [one]
            for _ in range(4): l.append(F.mul(l[-1], c))
            pw.append(l)
        vals = [F.mul(F.mul(pw[0][i], pw[1][j]), pw[2][l]) for i, j, l in MONOS]
        out.append(tuple(bytes(v[d] for v in vals) for d in range(F.k)))
    return out

def count(c, PL, q):
    n = 0
    for pl in PL:
        for pk in pl:
            if sum(map(int.__mul__, c, pk)) % q: break
        else: n += 1
    return n

def singular(c, F, P, PL, q):
    """True if some point of P is singular on f=sum c_m x^i y^j z^l (checks f and f_x,f_y,f_z)."""
    zero = tuple([0] * F.k); one = tuple([1] + [0] * (F.k - 1))
    def pw(a, e):
        r = one
        for _ in range(e): r = F.mul(r, a)
        return r
    for p, pl in zip(P, PL):
        if any(sum(map(int.__mul__, c, pk)) % q for pk in pl): continue
        tot = [zero] * 4
        for cm, (i, j, l) in zip(c, MONOS):
            if not cm: continue
            m = F.mul(F.mul(pw(p[0], i), pw(p[1], j)), pw(p[2], l))
            tot[0] = F.add(tot[0], F.sc(cm, m))
            for idx, (e, ee) in enumerate(((i, (i - 1, j, l)), (j, (i, j - 1, l)), (l, (i, j, l - 1)))):
                if e % F.q:
                    d = F.mul(F.mul(pw(p[0], ee[0]), pw(p[1], ee[1])), pw(p[2], ee[2]))
                    tot[idx + 1] = F.add(tot[idx + 1], F.sc(cm * e, d))
        if all(t == zero for t in tot): return True
    return False

def arm(q, nsamp, seed, ctx):
    rng = random.Random(seed)
    F1, F2, F3 = ctx['F']
    PL1, PL2 = ctx['PL1'], ctx['PL2']
    ts, hits, nonsmooth, notdiv, n1ok = [], 0, 0, 0, 0
    for _ in range(nsamp):
        c = [rng.randrange(q) for _ in range(15)]
        if not any(c): continue
        if count(c, PL1, q) != q + 1: continue
        n1ok += 1
        if count(c, PL2, q) != q * q + 1: continue
        hits += 1
        if 'PL3' not in ctx: ctx['PL3'] = planes(F3, ctx['P3'])
        if singular(c, F2, ctx['P2'], PL2, q) or singular(c, F3, ctx['P3'], ctx['PL3'], q):
            nonsmooth += 1; continue
        s3 = q ** 3 + 1 - count(c, ctx['PL3'], q)
        if s3 % 3: notdiv += 1; continue
        ts.append(s3 // 3)
    return dict(q=q, samples=nsamp, seed=seed, n1_pass=n1ok, a1a2_zero=hits,
                nonsmooth_rejected=nonsmooth, s3_not_div3=notdiv, t_values=sorted(ts))

def make_ctx(q):
    F = [Fld(q, 1), Fld(q, 2), Fld(q, 3)]
    P = [points(f) for f in F]
    return dict(F=F, P2=P[1], P3=P[2], PL1=planes(F[0], P[0]), PL2=planes(F[1], P[1]))

def coverage(q, ts):
    W = [t for t in range(-int(q ** 1.5), int(q ** 1.5) + 1) if t % q]
    got = set(ts)
    return len([t for t in W if t in got]) / len(W), len(W)

def pos_control(q, ctx, rng):
    """machinery check: N1 from planes vs integer brute force; Hasse-Weil on N1,N2 for 150 quartics"""
    bad = 0; hw = 0
    for _ in range(150):
        c = [rng.randrange(q) for _ in range(15)]
        n = 0
        pts = [(x, y, 1) for x in range(q) for y in range(q)] + [(x, 1, 0) for x in range(q)] + [(1, 0, 0)]
        for x, y, z in pts:
            if sum(cm * x ** i * y ** j * z ** l for cm, (i, j, l) in zip(c, MONOS)) % q == 0: n += 1
        if n != count(c, ctx['PL1'], q): bad += 1
    return dict(n1_mismatch_vs_bruteforce=bad)

def null_cov(q, nhits, seed):
    """null reference: t drawn uniformly from the Weil window (upper reference for coverage)"""
    rng = random.Random(seed)
    W = [t for t in range(-int(2 * q ** 1.5), int(2 * q ** 1.5) + 1) if t % q]
    return coverage(q, [rng.choice(W) for _ in range(nhits)])[0]

def main():
    cfg = dict(q5=(300, 300), q7=0) if SMOKE else dict(q5=(20000, 20000), q7=12000)
    out = dict(lead="LEAD-TRF-odd-char-descent-smith-quartic", smoke=SMOKE, seeds=dict(q5_A=20261110, q5_B=20261111, q7=7001, ctrl=99),
               params=cfg)
    ctx5 = make_ctx(5)
    out['positive_control_q5'] = pos_control(5, ctx5, random.Random(99))
    A = arm(5, cfg['q5'][0], 20261110, ctx5)
    B = arm(5, cfg['q5'][1], 20261111, ctx5)
    cA, nW = coverage(5, A['t_values']); cB, _ = coverage(5, B['t_values'])
    pooled = A['t_values'] + B['t_values']
    cP, _ = coverage(5, pooled)
    out.update(arm_A={k: v for k, v in A.items() if k != 't_values'}, arm_B={k: v for k, v in B.items() if k != 't_values'},
               window_size_q5=nW, coverage_A=cA, coverage_B=cB, coverage_pooled=cP, aa_abs_diff=abs(cA - cB),
               hits_pooled=len(pooled), null_uniform_coverage=null_cov(5, len(pooled), 5), t_pooled_distinct=sorted(set(pooled)))
    if cfg['q7'] and time.time() - T0 < 330:
        ctx7 = make_ctx(7)
        R = arm(7, cfg['q7'], 7001, ctx7)
        c7, n7 = coverage(7, R['t_values'])
        out['q7_descriptive'] = dict({k: v for k, v in R.items() if k != 't_values'}, coverage=c7, window=n7, hits_smooth=len(R['t_values']))
    out['elapsed_s'] = round(time.time() - T0, 1)
    bad = out['positive_control_q5']['n1_mismatch_vs_bruteforce'] or A['s3_not_div3'] or B['s3_not_div3']
    if SMOKE or bad or len(pooled) < 30: v = "inconclusive"
    elif cP >= 0.8 and abs(cA - cB) <= 0.15: v = "consistent_with_prediction"
    elif cP < 0.5: v = "falsified"
    else: v = "inconclusive"
    out['verdict'] = v
    print(json.dumps(out))

main()
