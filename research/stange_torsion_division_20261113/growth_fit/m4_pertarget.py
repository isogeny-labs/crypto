"""Per-target replicate of sym_m4/m4_sym.py for growth fits (arms D, Bs, Dp, M, Mt).

Same curve (y^2 = x^3 + 5x + 7), same factor bases (dense FD = n smallest valid x,
binomial mu_n, Dp = FD split round-robin in 3 parts), same msolve encodings and the
same timing rule (min of 3 msolve wall times minus a trivial-system baseline), but:
  - every per-target time and relation count is saved (the original saved only
    median/min/max), so the fit can bootstrap over targets;
  - M and Mt count point additions (calls to padd), so their growth can also be
    fitted in an exact unit;
  - ground truth uses a faster brute force (univariate in x3 per (x1, x2) pair),
    cross-checked against direct evaluation of S4 on random triples.
Assertions as in m4_sym.py: every msolve basis is [1] iff brute force finds no
solution, and M / Mt return exactly the brute-force set of valid relation multisets.
Usage: python3 m4_pertarget.py p n --arms=D,Bs,Dp,M,Mt [--targets=8] [--seed=11]
Exploratory; not a bench.record run.
"""
import sys, time, subprocess, random, statistics as st, tempfile, os, json
from sympy import symbols, resultant, Poly, expand
from sympy.polys.polyfuncs import symmetrize

p = int(sys.argv[1]); n = int(sys.argv[2])
ARMS = next((a.split('=', 1)[1].split(',') for a in sys.argv if a.startswith('--arms=')), ['D', 'Bs', 'Dp', 'M', 'Mt'])
NT = int(next((a.split('=', 1)[1] for a in sys.argv if a.startswith('--targets=')), 8))
SEED = int(next((a.split('=', 1)[1] for a in sys.argv if a.startswith('--seed=')), 11))
assert (p - 1) % n == 0
A, B = 5, 7; assert (4 * A**3 + 27 * B * B) % p
x1, x2, x3, x4, X, e1, e2, e3 = symbols('x1 x2 x3 x4 X e1 e2 e3')
def S3(a, b, c): return (a - b)**2 * c**2 - 2 * ((a + b) * (a * b + A) + 2 * B) * c + (a * b - A)**2 - 4 * B * (a + b)
t0 = time.time()
S4 = Poly(resultant(S3(x1, x2, X), S3(x3, x4, X), X), x1, x2, x3, x4, modulus=p)
S4T = [(m, int(c) % p) for m, c in S4.terms()]
print(f"# S4 built ({len(S4T)} terms) {time.time()-t0:.1f}s", file=sys.stderr)
issq = lambda z: z % p == 0 or pow(z % p, (p - 1) // 2, p) == 1
def S4f(a, b, c, d):
    return sum(co * pow(a, i, p) * pow(b, j, p) * pow(c, k, p) * pow(d, l, p) for (i, j, k, l), co in S4T) % p
def isprime(q): return q > 1 and all(q % k for k in range(2, int(q**.5) + 1))
g = next(z for z in range(2, p) if all(pow(z, (p - 1) // q, p) != 1 for q in range(2, p) if (p - 1) % q == 0 and isprime(q)))
MU = sorted({pow(g, (p - 1) // n * k, p) for k in range(n)}); assert len(MU) == n
FD = [x for x in range(p) if issq(x**3 + A * x + B)][:n]
def msqrt(a):
    a %= p
    if a == 0: return 0
    q, s_ = p - 1, 0  # Tonelli-Shanks, as in m4_sym.py (an O(p) search here would distort M/Mt timings)
    while q % 2 == 0: q //= 2; s_ += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    M_, c, t_, r = s_, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t_ != 1:
        i, tt = 0, t_
        while tt != 1: tt = tt * tt % p; i += 1
        b = pow(c, 1 << (M_ - i - 1), p); M_, c, t_, r = i, b * b % p, t_ * b * b % p, r * b % p
    assert r * r % p == a
    return r
ADDS = [0]
def padd(P, Q):
    ADDS[0] += 1
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0] and (P[1] + Q[1]) % p == 0: return None
    l = (3 * P[0] * P[0] + A) * pow(2 * P[1], -1, p) % p if P == Q else (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p; return (x, (l * (P[0] - x) - P[1]) % p)
def pneg(P): return None if P is None else (P[0], (-P[1]) % p)
PTS = [(x, msqrt(x**3 + A * x + B)) for x in FD]
PART = [FD[i::3] for i in range(3)]
def build_pair_table():
    tab = {}
    for i in range(len(PTS)):
        for j in range(i, len(PTS)):
            for S in (padd(PTS[i], PTS[j]), padd(PTS[i], pneg(PTS[j]))):
                tab.setdefault('O' if S is None else S[0], set()).add((PTS[i][0], PTS[j][0]))
    return tab
def mitm_table(xR, tab):
    R_ = (xR, msqrt(xR**3 + A * xR + B)); out = set()
    for P3 in PTS:
        for T in (padd(R_, pneg(P3)), padd(R_, P3)):
            for (a, b) in tab.get('O' if T is None else T[0], ()):
                out.add(tuple(sorted((a, b, P3[0]))))
    return out
def mitm(xR):
    R_ = (xR, msqrt(xR**3 + A * xR + B)); want = {}
    for P3 in PTS:
        for T in (padd(R_, pneg(P3)), padd(R_, P3)):
            want.setdefault('O' if T is None else T[0], set()).add(P3[0])
    out = set()
    for i in range(len(PTS)):
        for j in range(i, len(PTS)):
            for S in (padd(PTS[i], PTS[j]), padd(PTS[i], pneg(PTS[j]))):
                for z in want.get('O' if S is None else S[0], ()):
                    out.add(tuple(sorted((PTS[i][0], PTS[j][0], z))))
    return out
def ms(P, vs):
    out = []
    for mon, c in P.terms():
        c = int(c) % p
        if c == 0: continue
        out.append(str(c) + ''.join(f"*{v}^{e}" for v, e in zip(vs, mon) if e))
    return '+'.join(out) if out else '0'
SCR = os.environ.get('GF_SCRATCH', tempfile.gettempdir())
def run(polys, vs):
    with tempfile.TemporaryDirectory(dir=SCR) as d:
        fi = os.path.join(d, 'i'); fo = os.path.join(d, 'o')
        open(fi, 'w').write(",".join(vs) + f"\n{p}\n" + ",\n".join(polys) + "\n")
        best = None
        for _ in range(3):
            t1 = time.perf_counter(); r = subprocess.run(['msolve', '-g', '2', '-t', '1', '-f', fi, '-o', fo], capture_output=True, text=True); dt = time.perf_counter() - t1
            if r.returncode != 0: raise SystemExit(f"msolve rc={r.returncode}")
            best = dt if best is None else min(best, dt)
        o = open(fo).read()
    body = o[o.rfind('#---') + 4:].replace('\n', '').replace(' ', '')
    return best, body in ('[1]:', '[1*1]:') or body.startswith('[1]') or body.startswith('[1*1]')
base_runs = [run(["u-1", "v-2"], ["u", "v"])[0] for _ in range(7)]
base = min(base_runs)
def esym(expr):
    expr_s, rem, defs = symmetrize(expand(expr), [x1, x2, x3], formal=True)
    assert rem == 0
    rep = {d[0]: v for d, v in zip(defs, [e1, e2, e3])}
    return expand(expr_s.subs(rep))
bs_cons = None
if 'Bs' in ARMS:
    P_ = [3]; E_ = [1, e1, e2, e3]
    for k in range(1, 2 * n + 1):
        s = sum((-1)**(i - 1) * E_[i] * (P_[k - i] if k - i > 0 else k) for i in range(1, min(k, 3) + 1))
        P_.append(Poly(expand(s), e1, e2, e3, modulus=p).as_expr())
    pn, p2n = P_[n], P_[2 * n]
    bs_cons = [Poly(e3**n - 1, e1, e2, e3, modulus=p), Poly(pn - 3, e1, e2, e3, modulus=p),
               Poly(expand((pn**2 - p2n) * pow(2, -1, p)) - 3, e1, e2, e3, modulus=p)]
    _r = random.Random(5)
    for _ in range(3):
        xs = [_r.randrange(p) for _ in range(3)]
        ev = {e1: sum(xs) % p, e2: (xs[0]*xs[1] + xs[0]*xs[2] + xs[1]*xs[2]) % p, e3: xs[0]*xs[1]*xs[2] % p}
        for k in (n, 2 * n): assert Poly(P_[k], e1, e2, e3, modulus=p).eval(ev) % p == sum(pow(z, k, p) for z in xs) % p
R = random.Random(SEED)
TAB = None; TAB_s = None; TAB_adds = None
if 'Mt' in ARMS:
    ADDS[0] = 0; t1 = time.perf_counter(); TAB = build_pair_table(); TAB_s = time.perf_counter() - t1; TAB_adds = ADDS[0]
def rand_xE():
    while True:
        z = R.randrange(p)
        if issq(z**3 + A * z + B): return z
def brute(S4R_terms, xs1, xs2, xs3):
    # all ordered (a,b,c) with S4R(a,b,c)=0: for each (a,b) reduce to a quartic in c
    out = []
    for a in xs1:
        pa = [pow(a, i, p) for i in range(5)]
        for b in xs2:
            pb = [pow(b, j, p) for j in range(5)]
            cf = [0] * 5
            for (i, j, k), co in S4R_terms: cf[k] = (cf[k] + co * pa[i] * pb[j]) % p
            for c in xs3:
                if (cf[0] + c * (cf[1] + c * (cf[2] + c * (cf[3] + c * cf[4])))) % p == 0: out.append((a, b, c))
    return out
per = []
for ti in range(NT):
    xR = rand_xE()
    S4R = Poly(S4.as_expr().subs(x4, xR), x1, x2, x3, modulus=p)
    S4RT = [(m, int(c) % p) for m, c in S4R.terms()]
    _q = random.Random(ti)
    for _ in range(5):  # cross-check fast brute evaluator against direct S4
        a, b, c = (_q.randrange(p) for _ in range(3))
        assert (sum(co * pow(a, i, p) * pow(b, j, p) * pow(c, k, p) for (i, j, k), co in S4RT) - S4f(a, b, c, xR)) % p == 0
    valid = lambda tr: all(issq(z**3 + A * z + B) for z in tr)
    solD = brute(S4RT, FD, FD, FD)
    solP = [s for s in solD if s[0] in PART[0] and s[1] in PART[1] and s[2] in PART[2]]
    assert solP == brute(S4RT, PART[0], PART[1], PART[2])
    truth = {tuple(sorted(s)) for s in solD if valid(s)}
    rec = {'target': ti, 'xR': xR, 'time_s': {}, 'rel_dense_multisets': len(truth),
           'rel_Dp_multisets': len({tuple(sorted(s)) for s in solP if valid(s)})}
    fD = Poly(expand(eval('*'.join(f'(X-{c})' for c in FD))), X, modulus=p)
    uniD = lambda v: ms(Poly(fD.as_expr().subs(X, symbols(v)), symbols(v), modulus=p), [v])
    V3 = ['x1', 'x2', 'x3']
    arms = []
    if 'D' in ARMS: arms.append(('D', [ms(S4R, V3), uniD('x1'), uniD('x2'), uniD('x3')], V3, bool(solD)))
    if 'Bs' in ARMS:
        solB = brute(S4RT, MU, MU, MU)
        rec['rel_binomial_multisets'] = len({tuple(sorted(s)) for s in solB if valid(s)})
        S4sym = Poly(esym(S4R.as_expr()), e1, e2, e3, modulus=p)
        arms.append(('Bs', [ms(S4sym, ['e1', 'e2', 'e3'])] + [ms(c, ['e1', 'e2', 'e3']) for c in bs_cons], ['e1', 'e2', 'e3'], bool(solB)))
    if 'Dp' in ARMS:
        fP = [Poly(expand(eval('*'.join(f'(X-{c})' for c in Pk))), X, modulus=p) for Pk in PART]
        arms.append(('Dp', [ms(S4R, V3)] + [ms(Poly(fP[k].as_expr().subs(X, symbols(v)), symbols(v), modulus=p), [v]) for k, v in enumerate(V3)], V3, bool(solP)))
    if 'M' in ARMS:
        best = None
        for _ in range(3):
            ADDS[0] = 0; t1 = time.perf_counter(); mm = mitm(xR); dt = time.perf_counter() - t1; best = dt if best is None else min(best, dt)
        rec['time_s']['M'] = best; rec['adds_M'] = ADDS[0]
        assert mm == truth, ('mitm mismatch', xR)
    if 'Mt' in ARMS:
        best = None
        for _ in range(3):
            ADDS[0] = 0; t1 = time.perf_counter(); mt = mitm_table(xR, TAB); dt = time.perf_counter() - t1; best = dt if best is None else min(best, dt)
        rec['time_s']['Mt'] = best; rec['adds_Mt'] = ADDS[0]
        assert mt == truth, ('table mismatch', xR)
    R.shuffle(arms)
    for k, polys, vs, has in arms:
        dt, trivial = run(polys, vs); rec['time_s'][k] = dt - base
        assert trivial == (not has), (k, ti, xR, trivial, has)
    per.append(rec)
    print(f"# target {ti} xR={xR} {rec['time_s']}", file=sys.stderr, flush=True)
res = {'p': p, 'n': n, 'arms': ARMS, 'seed': SEED, 'targets': NT, 'msolve': '0.6.5',
       'baseline_s': base, 'baseline_runs_s': base_runs, 'Mt_table_build_s': TAB_s, 'Mt_table_adds': TAB_adds,
       'timing': 'msolve arms: min of 3 wall times minus min-of-7 trivial baseline; M/Mt: in-process python min of 3',
       'per_target': per}
print(json.dumps(res))
