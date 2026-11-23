"""PKM 2016 section 3.3 composed-map (tower) factor base vs dense encoding of the SAME factor base.

Factor base V = mu_n, the subgroup of order n = 2^k in F_p^*, with 2^k | p - 1 (PKM's choice:
smallest such p with p >= 2^(m k)). Target xR = x-coordinate of a random point of
E: y^2 = x^3 + A x + B (A, B seeded). Decompose R = sum_{i=1..m} +-P_i with x(P_i) in V via the
summation polynomial S_{m+1}(X_1..X_m, xR) = 0.

Encodings (identical ideal after elimination, identical F_p-solutions):
  D   dense  : vars X_i;           S_{m+1}(X, xR), prod_{v in V}(X_i - v) = X_i^n - 1
  T2  tower  : vars X_{i,1..k};    S_{m+1}(X_{.,1}, xR), X_{i,j+1} - X_{i,j}^2, 1 - X_{i,k}^2   (PKM system (4))
  T4  tower with degree-4 steps (k even only): X_{i,j+1} - X_{i,j}^4, 1 - X_{i,k/2}^4

Checks asserted on every target, every arm:
  * all solutions over the algebraic closure lie in F_p (X^n = 1 splits since n | p - 1), so the
    quotient dimension dim K[X]/I (staircase of msolve's reduced DRL basis) must equal the number
    of ordered tuples (v_1..v_m) in V^m with S_{m+1}(v, xR) = 0 found by brute force (numpy);
  * every brute-force tuple (lifted along the tower) is a zero of every basis element.
  Together (with |V(I)| <= dim) this means msolve's F_p-solution set equals brute force.
Time per arm and target: min of REPS (default 3) msolve 0.6.5 (-g 2 -t 1) wall times minus a trivial-system
baseline. One extra untimed run with -v 2 gives msolve's largest F4 step degree ("max deg").
msolve is nondeterministic in run time; inputs are regenerable from (m, k, seed) and the
per-target records saved here. Exploratory; not a bench.record run.
  --fb=isog : PKM section 3.4 instead: V = x(T + <G>) on an auxiliary curve with G of order 2^k;
  arms D (dense prod(X - v), a full degree-n polynomial) and I2 (tower of Velu 2-isogeny x-maps).
Usage: pkm_tower.py m k [--targets=T] [--arms=D,T2,T4] [--seed=S] [--timeout=SEC] [--reps=R] [--pmin=P | --p=P]
"""
import sys, os, time, json, random, subprocess, tempfile, re, statistics as st
import numpy as np
from sympy import symbols, resultant, Poly, isprime

m = int(sys.argv[1]); k = int(sys.argv[2]); n = 2 ** k
opt = {a.split('=', 1)[0][2:]: a.split('=', 1)[1] for a in sys.argv[3:] if a.startswith('--') and '=' in a}
T = int(opt.get('targets', 8)); SEED = int(opt.get('seed', 20261115)); TO = float(opt.get('timeout', 300)); REPS = int(opt.get('reps', 3))
ARMS = opt.get('arms', 'D,T2' + (',T4' if k % 2 == 0 and k >= 4 else '')).split(',')
rng = random.Random(SEED * 1000 + 10 * m + k)

# PKM rule: smallest prime p >= 2^(m k) with 2^k | p - 1. msolve 0.6.5 segfaults (rc -11) on these
# systems for every prime tried in (2^16, 2^18) (65537, 70000+, 100609, 133121, 201473), so a p
# landing there is bumped to >= 2^18. --p=P fixes the prime (fixed-p grid).
p = int(opt.get('pmin', 2 ** (m * k) + 1))
if 2 ** 16 < p < 2 ** 18:
    p = 2 ** 18 + 1
while not ((p - 1) % n == 0 and isprime(p)):
    p += 1
if 'p' in opt:
    p = int(opt['p']); assert isprime(p) and (p - 1) % n == 0
while True:
    A, B = rng.randrange(1, p), rng.randrange(1, p)
    if (4 * A ** 3 + 27 * B * B) % p:
        break

# summation polynomial S_{m+1}(x1..xm, X) as a dict {exponents: coeff}
x = symbols('x1:5'); Xs = symbols('X')
def S3(a, b, c):
    return (a - b) ** 2 * c ** 2 - 2 * ((a + b) * (a * b + A) + 2 * B) * c + (a * b - A) ** 2 - 4 * B * (a + b)
if m == 2:
    Spol = Poly(S3(x[0], x[1], x[2]), x[0], x[1], x[2], modulus=p)
elif m == 3:
    Spol = Poly(resultant(S3(x[0], x[1], Xs), S3(x[2], x[3], Xs), Xs), *x, modulus=p)
else:
    raise SystemExit('m must be 2 or 3')
STERMS = [(mon, int(c) % p) for mon, c in Spol.terms()]

def S_specialize(xR):
    """coefficients of S(x1..xm, xR) as {exps(m): c}"""
    d = {}
    for mon, c in STERMS:
        e = mon[:m]; d[e] = (d.get(e, 0) + c * pow(xR, mon[m], p)) % p
    return {e: c for e, c in d.items() if c}

g = 2
while any(pow(g, (p - 1) // q, p) == 1 for q in set(f for f in range(2, 64) if (p - 1) % f == 0 and isprime(f))) or pow(g, (p - 1) // 2, p) == 1:
    g += 1
zeta = pow(g, (p - 1) // n, p)
MU = [pow(zeta, j, p) for j in range(n)]; assert len(set(MU)) == n and pow(zeta, n, p) == 1
FBK = opt.get('fb', 'subgroup')
CHAIN = []   # isogeny mode: [(x0_j, t_j)] for the 2-isogeny steps; CHI = x-coordinate of phi(T)
if FBK == 'isog':
    # PKM section 3.4 with B = 2: auxiliary curve E': y^2 = x^3 + a' x + b' with a point G of order 2^k;
    # phi = phi_k o ... o phi_1, phi_j a Velu 2-isogeny; V = x(T + <G>) for T outside <G> with
    # phi(T) not 2-torsion. L_j(x) = x + t_j/(x - x0_j) (degree 2 after clearing the denominator).
    import numpy as _np
    irng = random.Random(SEED * 7919 + 10 * m + k)
    def _ecadd(P1, P2, a_):
        if P1 is None: return P2
        if P2 is None: return P1
        if P1[0] == P2[0] and (P1[1] + P2[1]) % p == 0: return None
        l = (3 * P1[0] ** 2 + a_) * pow(2 * P1[1], -1, p) % p if P1 == P2 else (P2[1] - P1[1]) * pow(P2[0] - P1[0], -1, p) % p
        x3 = (l * l - P1[0] - P2[0]) % p; return (x3, (l * (P1[0] - x3) - P1[1]) % p)
    def _ecmul(e, P1, a_):
        R_ = None
        while e:
            if e & 1: R_ = _ecadd(R_, P1, a_)
            P1 = _ecadd(P1, P1, a_); e >>= 1
        return R_
    def _sqrt(z):
        z %= p
        for _ in range(200):
            c = irng.randrange(p)
            if pow((c * c - z) % p, (p - 1) // 2, p) == p - 1: break
        w2 = (c * c - z) % p; e = (p + 1) // 2; r0, r1 = 1, 0; b0, b1 = c, 1
        while e:   # Cipolla
            if e & 1: r0, r1 = (r0 * b0 + r1 * b1 % p * w2) % p, (r0 * b1 + r1 * b0) % p
            b0, b1 = (b0 * b0 + b1 * b1 % p * w2) % p, (2 * b0 * b1) % p; e >>= 1
        assert r0 * r0 % p == z; return r0
    xs = _np.arange(p, dtype=_np.int64)
    def _count(a_, b_):
        v = (xs * xs % p * xs % p + a_ * xs + b_) % p; acc = _np.ones(p, dtype=_np.int64); base_ = v.copy(); e = (p - 1) // 2
        while e:
            if e & 1: acc = acc * base_ % p
            base_ = base_ * base_ % p; e >>= 1
        return p + 1 + int(_np.sum(_np.where(v == 0, 0, _np.where(acc == 1, 1, -1))))
    while True:
        a_, b_ = irng.randrange(1, p), irng.randrange(1, p)
        if (4 * a_ ** 3 + 27 * b_ * b_) % p == 0: continue
        N_ = _count(a_, b_)
        if N_ % n: continue
        G = None
        for _ in range(20):
            while True:
                xx = irng.randrange(p); vv = (xx ** 3 + a_ * xx + b_) % p
                if vv and pow(vv, (p - 1) // 2, p) == 1: break
            Pr = _ecmul(N_ // n, (xx, _sqrt(vv)), a_)
            if Pr is not None and _ecmul(n // 2, Pr, a_) is not None: G = Pr; break
        if G is None: continue
        H = [None]; cur = G
        for _ in range(n - 1): H.append(cur); cur = _ecadd(cur, G, a_)
        assert cur is None
        T_ = None
        for _ in range(50):
            while True:
                xx = irng.randrange(p); vv = (xx ** 3 + a_ * xx + b_) % p
                if vv and pow(vv, (p - 1) // 2, p) == 1: break
            Tc = (xx, _sqrt(vv))
            if Tc in H or _ecmul(n, Tc, a_) is None and False: continue
            coset = [_ecadd(Tc, h, a_) for h in H]
            if None in coset: continue
            V_ = sorted({c[0] for c in coset})
            if len(V_) == n: T_ = Tc; break
        if T_ is None: continue
        break
    # chain of Velu 2-isogenies; kernel of step j is generated by 2^(k-j) * (image of G)
    aa, bb, Gc, Tc = a_, b_, G, T_
    def _xmap(xv, x0, t): return (xv + t * pow((xv - x0) % p, -1, p)) % p
    for j in range(k):
        K2 = _ecmul(2 ** (k - 1 - j), Gc, aa); assert K2 is not None and K2[1] == 0
        x0 = K2[0]; t = (3 * x0 * x0 + aa) % p; wv = x0 * t % p
        CHAIN.append((x0, t))
        def _phi(Pt, x0=x0, t=t):
            if Pt is None or Pt[0] == x0: return None
            u = pow((Pt[0] - x0) % p, -1, p)
            return ((Pt[0] + t * u) % p, Pt[1] * (1 - t * u * u) % p)
        Gc = _phi(Gc); Tc = _phi(Tc); aa, bb = (aa - 5 * t) % p, (bb - 7 * wv) % p
        if Gc is not None: assert (Gc[1] ** 2 - Gc[0] ** 3 - aa * Gc[0] - bb) % p == 0
    assert Gc is None and Tc is not None and Tc[1] != 0
    CHI = Tc[0]
    for v in V_:   # every element of V maps to CHI along the chain
        z = v
        for x0, t in CHAIN: z = _xmap(z, x0, t)
        assert z == CHI
    MU = V_; ISO = {'aux_curve': [a_, b_], 'aux_order': N_, 'G': G, 'T': T_, 'chain': CHAIN, 'chi': CHI}
    DENSE = [1]   # coefficients of prod (X - v), highest degree first
    for v in V_:
        DENSE = [(c - v * d) % p for c, d in zip(DENSE + [0], [0] + DENSE)]
issq = lambda z: z % p == 0 or pow(z % p, (p - 1) // 2, p) == 1
XCOORD = {v for v in MU if issq(v ** 3 + A * v + B)}

def brute(xR):
    """ordered tuples in MU^m with S(v, xR) = 0 (numpy, exact mod p)"""
    Sd = S_specialize(xR); mu = np.array(MU, dtype=np.int64)
    maxe = max(max(e) for e in Sd) if Sd else 0
    pw = [np.ones(n, dtype=np.int64)]
    for _ in range(maxe):
        pw.append(pw[-1] * mu % p)
    if m == 2:
        acc = np.zeros((n, n), dtype=np.int64)
        for (a, b), c in Sd.items():
            acc = (acc + c * (pw[a][:, None] * pw[b][None, :] % p)) % p
        idx = np.argwhere(acc == 0)
    else:
        acc = np.zeros((n, n, n), dtype=np.int64)
        for (a, b, cc), c in Sd.items():
            acc = (acc + c * ((pw[a][:, None] * pw[b][None, :] % p)[:, :, None] * pw[cc][None, None, :] % p)) % p
        idx = np.argwhere(acc == 0)
    return [tuple(MU[i] for i in row) for row in idx]

def msterm(e, vs):
    return ''.join(f"*{v}^{a}" for v, a in zip(vs, e) if a)

def system(arm, xR):
    Sd = S_specialize(xR)
    if arm == 'D':
        vs = [f"x{i}" for i in range(1, m + 1)]; top = vs
        if FBK == 'isog':
            eqs = ['+'.join(f"{c}*{v}^{n - e}" if n - e else f"{c}" for e, c in enumerate(DENSE) if c) for v in vs]
        else:
            eqs = [f"{v}^{n}-1" for v in vs]
        lift = lambda t: list(t)
    elif arm == 'I2':
        vs = [f"x{i}_{j}" for i in range(1, m + 1) for j in range(1, k + 1)]
        top = [f"x{i}_1" for i in range(1, m + 1)]
        eqs = []
        for i in range(1, m + 1):
            for j in range(1, k + 1):
                x0, t = CHAIN[j - 1]; xj = f"x{i}_{j}"
                # nxt*(x - x0) - x^2 + x0*x - t = 0, nxt = x_{j+1} (or CHI at the last step); like terms combined
                if j < k:
                    nx = f"x{i}_{j+1}"
                    eqs.append(f"1*{nx}*{xj}+{(-x0) % p}*{nx}+{p - 1}*{xj}^2+{x0}*{xj}+{(-t) % p}")
                else:
                    eqs.append(f"{p - 1}*{xj}^2+{(CHI + x0) % p}*{xj}+{(-x0 * CHI - t) % p}")
        def lift(tup):
            out_ = []
            for v in tup:
                z = v
                for x0, t in CHAIN:
                    out_.append(z); z = (z + t * pow((z - x0) % p, -1, p)) % p
            return out_
    else:
        d = int(arm[1:]); levels = 0; q = 1
        while q < n:
            q *= d; levels += 1
        assert q == n, (arm, n)
        vs = [f"x{i}_{j}" for i in range(1, m + 1) for j in range(1, levels + 1)]
        top = [f"x{i}_1" for i in range(1, m + 1)]
        eqs = []
        for i in range(1, m + 1):
            for j in range(1, levels):
                eqs.append(f"x{i}_{j+1}-x{i}_{j}^{d}")
            eqs.append(f"1-x{i}_{levels}^{d}")
        lift = lambda t, d=d, L=levels: [pow(v, d ** j, p) for v in t for j in range(L)]
    sp = '+'.join(f"{c}" + msterm(e, top) for e, c in Sd.items())
    return vs, [sp] + eqs, lift

def run_ms(vs, eqs, verbose=False):
    with tempfile.TemporaryDirectory() as dd:
        fi = os.path.join(dd, 'i'); fo = os.path.join(dd, 'o')
        open(fi, 'w').write(",".join(vs) + f"\n{p}\n" + ",\n".join(eqs) + "\n")
        cmd = ['msolve', '-g', '2', '-t', '1', '-f', fi, '-o', fo] + (['-v', '2'] if verbose else [])
        t1 = time.perf_counter()
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=TO)
        dt = time.perf_counter() - t1
        out = open(fo).read() if os.path.exists(fo) else ''
    return dt, r.returncode, out, r.stdout

def parse_gb(out, vs):
    body = out[out.rfind('#---') + 4:].replace('\n', '').replace(' ', '').rstrip(':')
    assert body.startswith('[') and body.endswith(']'), body[:200]
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

def grevlex_key(e):
    return (sum(e), tuple(-a for a in reversed(e)))

def staircase(lms, nv, cap=100000):
    if any(sum(l) == 0 for l in lms):
        return 0
    for i in range(nv):  # zero-dimensional needs a pure power of each variable
        assert any(l[i] > 0 and sum(l) == l[i] for l in lms), 'not zero-dimensional'
    seen = {tuple([0] * nv)}; todo = [tuple([0] * nv)]
    while todo:
        e = todo.pop()
        for i in range(nv):
            f = list(e); f[i] += 1; f = tuple(f)
            if f in seen or any(all(f[j] >= l[j] for j in range(nv)) for l in lms):
                continue
            seen.add(f); todo.append(f)
            assert len(seen) < cap
    return len(seen)

def evalp(P, pt):
    s = 0
    for e, c in P.items():
        t = c
        for v, a in zip(pt, e):
            if a:
                t = t * pow(v, a, p) % p
        s += t
    return s % p

def maxdeg(stdout):
    ds = [int(mm.group(1)) for mm in re.finditer(r'^\s*(\d+)\s+\d+\s+\d+\s+\d+\s+x\s+\d+', stdout, re.M)]
    return max(ds) if ds else None

def rand_target():
    while True:
        xr = rng.randrange(p)
        if issq(xr ** 3 + A * xr + B):
            return xr

base = min(run_ms(['u', 'v'], ['u-1', 'v-2'])[0] for _ in range(7))
recs = []; failures = []
for t in range(T):
    xR = rand_target(); bf = brute(xR)
    rel = sorted({tuple(sorted(s)) for s in bf if all(v in XCOORD for v in s)})
    rec = {'target': t, 'xR': xR, 'brute_ordered_tuples': len(bf), 'relation_multisets_xcoord': len(rel), 'arms': {}}
    order = ARMS[:]; rng.shuffle(order)
    for arm in order:
        vs, eqs, lift = system(arm, xR)
        times = []; out = None; rcs = []
        for _ in range(REPS):
            try:
                dt, rc, o, _so = run_ms(vs, eqs)
            except subprocess.TimeoutExpired:
                rcs.append('timeout'); break
            rcs.append(rc)
            if rc == 0:
                times.append(dt); out = o
        if not times:
            failures.append({'target': t, 'arm': arm, 'rcs': rcs}); rec['arms'][arm] = {'fail': rcs}
            sd = os.environ.get('PKM_SCRATCH')
            if sd:
                open(os.path.join(sd, f'fail_m{m}_k{k}_p{p}_t{t}_{arm}.ms'), 'w').write(",".join(vs) + f"\n{p}\n" + ",\n".join(eqs) + "\n")
            print(f"# FAIL m={m} k={k} t={t} {arm} rcs={rcs}", file=sys.stderr); continue
        gb = parse_gb(out, vs)
        lms = [max(P, key=grevlex_key) for P in gb]
        dim = staircase(lms, len(vs))
        assert dim == len(bf), (arm, t, dim, len(bf))
        for s in bf:
            pt = lift(s); assert all(evalp(P, pt) == 0 for P in gb), (arm, t, s)
        md = None
        try:
            _, rcv, _, so = run_ms(vs, eqs, verbose=True); md = maxdeg(so) if rcv == 0 else None
        except subprocess.TimeoutExpired:
            pass
        rec['arms'][arm] = {'time_s': min(times) - base, 'raw_min_s': min(times), 'runs_ok': len(times), 'rcs': rcs,
                            'gb_len': len(gb), 'quotient_dim': dim, 'max_step_deg': md, 'nvars': len(vs), 'neqs': len(eqs)}
    recs.append(rec)
    print(f"# m={m} k={k} t={t} bf={len(bf)} rel={len(rel)} " + ' '.join(
        f"{a}:{r.get('time_s', float('nan')):.4f}s/d{r.get('max_step_deg')}" for a, r in rec['arms'].items()), file=sys.stderr)

summ = {}
for arm in ARMS:
    ts = [r['arms'][arm]['time_s'] for r in recs if 'time_s' in r['arms'].get(arm, {})]
    ds = [r['arms'][arm]['max_step_deg'] for r in recs if r['arms'].get(arm, {}).get('max_step_deg') is not None]
    summ[arm] = {'median_s': st.median(ts) if ts else None, 'n_ok': len(ts), 'max_step_deg_values': sorted(set(ds))}
json.dump({'m': m, 'k': k, 'n': n, 'p': p, 'A': A, 'B': B, 'zeta': zeta, 'seed': SEED, 'targets': T, 'arms': ARMS, 'reps': REPS,
           'baseline_s': base, 'msolve': '0.6.5 (-g 2 -t 1); nondeterministic run time', 'summary': summ,
           'failures': failures, 'records': recs, 'factor_base': FBK, 'isogeny': ISO if FBK == 'isog' else None, 'xcoord_fraction_of_V': len(XCOORD) / n}, sys.stdout, indent=1)
