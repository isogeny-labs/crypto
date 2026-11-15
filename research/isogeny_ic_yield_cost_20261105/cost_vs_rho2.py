from sage.all import *
# v2 of cost_vs_rho.py: everything MEASURED (no analytic MITM / folded-coset accounting).
#  - several curves per size (CURVE_IDX = n-th qualifying (p, A)), 100 scalars per config
#  - IC strategies: FULL (table of all <=m sums, only if it fits) and MITM (table of <=ceil(m/2) sums, loop over <=floor(m/2) sums)
#  - coset: 0 = plain lookup of R; 1 = FOLDED: look up h*R in tables built from h*P_i (3 doublings + one lookup per trial)
#  - rho baselines: (a) r-adding + Brent, no negation, single target; (b) distinguished-point rho with negation map,
#    single target; (c) same, multi-target (Kuhn-Struik): DLPs solved sequentially, DPs of solved targets become known.
# usage: sage -python cost_vs_rho2.py BITS M CURVE_IDX NSCAL OUT.jsonl      (M=0 -> rho only)
import sys, json, time, random, math, os

BITS, M, CIDX, NSCAL, OUT = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
MAX_TABLE = 4_000_000          # max entries in any table we build
MAX_WORK = 2.5e8               # max estimated adds for one IC config
set_random_seed(BITS * 100 + 7 + 1000 * CIDX)
h = 8

# ---------- curve: CIDX-th qualifying Montgomery y^2 = x^3 + A x^2 + x, p = 5 mod 8, #E = 8q, #twist = 4q' ----------
p = next_prime(2 ** BITS); seen = 0; found = None
while found is None:
    while p % 8 != 5: p = next_prime(p + 1)
    F = GF(p)
    for A in range(486662 % p, min(p, 486662 % p + 4 * 4000), 4):
        if (A * A - 4) % p == 0: continue
        EM = EllipticCurve(F, [0, A, 0, 1, 0]); N = EM.order(); Nt = 2 * (p + 1) - N
        if N % 8 == 0 and is_prime(N // 8) and Nt % 4 == 0 and is_prime(Nt // 4):
            if seen == CIDX: found = A; break
            seen += 1
    if found is None: p = next_prime(p + 1)
p = int(p); A_M = found; q = int(N // h)
E0 = EM.short_weierstrass_model(); a4 = int(E0.a4())

OPS = [0]
def add(P, Q):
    OPS[0] += 1
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a4) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)
def neg(P): return None if P is None else (P[0], (-P[1]) % p)
def mul(k, P):
    R = None
    while k:
        if k & 1: R = add(R, P)
        k >>= 1
        if k: P = add(P, P)
    return R
def _h(P):
    P = add(P, P);  P = add(P, P) if P else None;  P = add(P, P) if P else None
    return P
def tup(P): return None if P.is_zero() else (int(P[0]), int(P[1]))

G = tup(h * E0.random_point())
while G is None: G = tup(h * E0.random_point())
rng = random.Random(BITS * 1000 + 17 * CIDX + 5)
scalars = [rng.randrange(1, q) for _ in range(NSCAL)]
Qs = [tup(k * E0(G[0], G[1])) for k in scalars]           # inputs, not counted
log2q = math.log2(q)
out = open(OUT, "a")
meta = dict(bits=BITS, curve=CIDX, p=p, A=A_M, q=q, m=M)

# ======================= rho baselines =======================
def rho_brent(Q, seed):
    r = random.Random(seed); OPS[0] = 0
    ca = [r.randrange(q) for _ in range(32)]; cb = [r.randrange(q) for _ in range(32)]
    cP = [add(mul(x, G), mul(y, Q)) for x, y in zip(ca, cb)]
    a, b = r.randrange(q), r.randrange(q); X = add(mul(a, G), mul(b, Q))
    setup = OPS[0]; steps = 0
    saved, sa, sb, power, lam = X, a, b, 1, 0
    while True:
        j = (X[0] if X else 0) % 32
        X = add(X, cP[j]); a = (a + ca[j]) % q; b = (b + cb[j]) % q; lam += 1; steps += 1
        if X == saved:
            if (b - sb) % q == 0: return rho_brent(Q, seed + 7919)
            return (sa - a) * pow(b - sb, -1, q) % q, OPS[0], setup, steps
        if lam == power: saved, sa, sb, power, lam = X, a, b, power * 2, 0

def canon(X, s, b):
    """negation map: representative with y <= (p-1)/2; X = s*Q + b*G tracked as (s, b)."""
    if X[1] > (p - 1) // 2: return (X[0], p - X[1]), (-s) % q, (-b) % q
    return X, s, b

class DPRho:
    """Distinguished-point rho, negation map, target-independent walk (steps are multiples of G only), so DPs
    can be shared across targets (Kuhn-Struik). 2-cycle reduction: if the next point would use the same index,
    take the next index instead (a function of the point only); longer fruitless cycles are detected in-walk (Brent) and escaped by doubling the cycle's min point;
    walks longer than 20/theta are abandoned (safety cap)."""
    R = 64
    def __init__(self, seed):
        r = random.Random(seed); OPS[0] = 0
        self.cs = [r.randrange(1, q) for _ in range(self.R)]
        self.cP = [mul(c, G) for c in self.cs]
        self.setup = OPS[0]
        self.dbits = max(0, int(round(math.log2(math.sqrt(q)) - 5)))   # theta = 2^-d, ~32 DPs per sqrt(q)
        self.mask = (1 << self.dbits) - 1
        self.known = {}          # DP point -> known discrete log (w.r.t. G)
        self.r = r; self.cycles = 0
    def step(self, X, s, b):
        j = X[0] % self.R
        Y = add(X, self.cP[j])
        if Y is not None and canon(Y, 1, 0)[0][0] % self.R == j:   # 2-cycle reduction (function of X only)
            j = (j + 1) % self.R; Y = add(X, self.cP[j])
        if Y is None: return None, s, b
        return canon(Y, s % q, (b + self.cs[j]) % q)
    def solve(self, Q):
        """returns (k, ops, walks, abandoned); known DPs from earlier targets are reused."""
        OPS[0] = 0; walks = 0; abandoned = 0; mine = {}
        cap = 20 * (self.mask + 1)
        while True:
            b0 = self.r.randrange(1, q); walks += 1
            X0 = add(Q, mul(b0, G))
            if X0 is None: continue
            X, s, b = canon(X0, 1, b0 % q)
            L = 0; dead = False; saved = X; power = 1; lam = 0
            while (X[0] & self.mask) != 0:
                if L >= cap: dead = True; break
                X, s, b = self.step(X, s, b)
                if X is None: dead = True; break
                L += 1; lam += 1
                if X == saved:                        # fruitless cycle (Brent detection inside the walk)
                    cyc = [(X, s, b)]; Y, sy, by = self.step(X, s, b)
                    while Y != X: cyc.append((Y, sy, by)); Y, sy, by = self.step(Y, sy, by)
                    Xm, sm, bm = min(cyc, key=lambda t: t[0])       # deterministic escape: double the min point
                    D2 = add(Xm, Xm)
                    if D2 is None: dead = True; break
                    X, s, b = canon(D2, 2 * sm % q, 2 * bm % q); self.cycles += 1
                    saved = X; power = 1; lam = 0; continue
                if lam == power: saved = X; power *= 2; lam = 0
            if dead: abandoned += 1; continue
            if X in self.known:                       # s*k + b = known  -> k
                k = (self.known[X] - b) * pow(s, -1, q) % q
                logs = {P: (s2 * k + b2) % q for P, (s2, b2) in mine.items()}
                logs[X] = self.known[X]
                self.known.update(logs)
                return k, OPS[0], walks, abandoned
            if X in mine:
                s2, b2 = mine[X]
                if (s - s2) % q:                      # s*k+b = s2*k+b2  -> k = (b2-b)/(s-s2)
                    k = (b2 - b) * pow(s - s2, -1, q) % q
                    logs = {P: (ss * k + bb) % q for P, (ss, bb) in mine.items()}
                    self.known.update(logs)
                    return k, OPS[0], walks, abandoned
                continue                              # useless self collision
            mine[X] = (s, b)

if M == 0:
    t0 = time.time()
    br = [rho_brent(Q, 1000 + i) for i, Q in enumerate(Qs)]
    assert all(k == x[0] for k, x in zip(scalars, br))
    single = []
    for i, Q in enumerate(Qs):                        # fresh DP table per target = single-target DP rho
        d = DPRho(5000 + i); k, ops, w, ab = d.solve(Q); assert k == scalars[i]; single.append((ops, d.setup, w, ab))
    d = DPRho(4242); multi = []
    for i, Q in enumerate(Qs):
        k, ops, w, ab = d.solve(Q); assert k == scalars[i], "multi-target rho wrong"; multi.append((ops, w, ab))
    rec = dict(kind="rho", **meta, n=NSCAL, sqrt_pi_q_over_2=math.sqrt(math.pi * q / 2),
               brent_ops=[x[1] for x in br], brent_setup=[x[2] for x in br], brent_steps=[x[3] for x in br],
               dp_single_ops=[x[0] for x in single], dp_single_setup=single[0][1], dp_single_abandoned=sum(x[3] for x in single),
               dp_multi_ops=[x[0] for x in multi], dp_multi_setup=d.setup, dp_multi_abandoned=sum(x[2] for x in multi),
               dp_bits=d.dbits, dp_multi_cycles=d.cycles, wall_s=time.time() - t0)
    out.write(json.dumps(rec) + "\n"); out.flush()
    bm = sum(rec["brent_ops"]) / NSCAL; sm = sum(rec["dp_single_ops"]) / NSCAL; mm = sum(rec["dp_multi_ops"]) / NSCAL
    print(f"rho bits={BITS} curve={CIDX} q={q}: brent {bm:.0f}  dp-neg single {sm:.0f}(+{rec['dp_single_setup']} setup)  "
          f"dp-neg multi {mm:.0f}/DLP (+{d.setup} once)  all {NSCAL} recovered", flush=True)
    sys.exit(0)

# ======================= index calculus =======================
R_STEPS = 64
steps_rng = random.Random(99)
STEP_S = [steps_rng.randrange(1, q) for _ in range(R_STEPS)]
OPS[0] = 0
STEP_P = [mul(s, G) for s in STEP_S]
STEP_COST = OPS[0]

def factor_base(c):
    B = max(2, int(c * p ** (1.0 / M)))
    fb = [tup(E0.lift_x(F(x))) for x in range(B) if E0.is_x_coord(F(x))]
    return B, fb

def signed_list(pts):
    s = []
    for j, P in enumerate(pts):
        if P is None: continue
        s.append((P, (j, 1)))
        if P[1] != 0: s.append((neg(P), (j, -1)))
    return s

def build_sums(signed, k):
    """point -> one decomposition, over all multisets of AT MOST k signed points. Each tree edge = one add."""
    items = [(None, None)] + signed; T = {}; OPS[0] = 0
    def rec(start, depth, S, dec):
        if depth == k:
            if S not in T: T[S] = dec
            return
        for i in range(start, len(items)):
            P, lab = items[i]
            if P is None: rec(i, depth + 1, S, dec)
            else: rec(i, depth + 1, add(S, P), dec + [lab])
    rec(0, 0, None, [])
    return T, OPS[0]

def nc(n, k): return math.comb(n + k, k)          # multisets of size <= k from n items

def run_config(c, coset, strat):
    B, fb = factor_base(c); nF = len(fb)
    base = [_h(P) for P in fb] if coset else fb      # folded: tables over h*P_i
    OPS[0] = 0
    S = signed_list(base); nS = len(S)
    fold_cost = 3 * nF if coset else 0
    # predicted yield ~ (#<=m sums) / #targets-space ; targets: q (folded) or h*q (plain)
    space = q if coset else h * q
    pred = min(1.0, nc(nS, M) / space)
    if nF < 4 or pred < 0.01: return "tiny"
    est_trials = (nF + 8) / pred + NSCAL / pred
    if strat == "FULL":
        if nc(nS, M) > MAX_TABLE: return None
    else:
        kt, kl = (M + 1) // 2, M // 2
        if nc(nS, kt) > MAX_TABLE or est_trials * nc(nS, kl) > MAX_WORK: return None
    t0 = time.time()
    if strat == "FULL":
        T, build = build_sums(S, M)
        def lookup(R):
            X = _h(R) if coset else R
            if X is None: return None
            return T.get(X)
        tbl, lst = len(T), 0
    else:
        Tm, bt = build_sums(S, kt); Lm, bl = build_sums(S, kl)
        Ln = [(neg(P), d) for P, d in Lm.items() if P is not None]     # negation is free (no add)
        build = bt + bl
        def lookup(R):
            X = _h(R) if coset else R
            if X is None: return None
            d = Tm.get(X)
            if d is not None: return d
            for nP, ds in Ln:
                Y = add(X, nP)
                d = Tm.get(Y)
                if d is not None: return d + ds
            return None
        tbl, lst = len(Tm), len(Lm)
    build_s = time.time() - t0
    build += fold_cost

    walk = random.Random(int(1000 * c) + coset + 7 * CIDX)
    OPS[0] = 0
    a = walk.randrange(1, q); R = mul(a, G)
    rels = []; trials = 0; L = None; la_s = 0.0; last_rank = -1; next_check = 0
    cols_set = set()
    while trials < 5_000_000:
        j = walk.randrange(R_STEPS); a = (a + STEP_S[j]) % q; R = add(R, STEP_P[j]); trials += 1
        if R is None: continue
        d = lookup(R)
        if d is None: continue
        rels.append((a, d)); cols_set.update(jj for jj, _ in d)
        if len(rels) < len(cols_set) + 8 or len(rels) < next_check: continue
        ops_save = OPS[0]
        cols = sorted(cols_set); ci = {jj: i for i, jj in enumerate(cols)}
        Mx = matrix(GF(q), len(rels), len(cols), sparse=False); v = vector(GF(q), [x for x, _ in rels])
        nnz = 0
        for r_, (_, dd) in enumerate(rels):
            for (jj, s) in dd: Mx[r_, ci[jj]] += s; nnz += 1
        t1 = time.time()
        defic = 0
        if M == 2 and not coset:
            cls = {mul(q, fb[jj]) for jj in cols}
            defic = len({frozenset((str(t), str(neg(t)))) for t in cls if t is not None and t != neg(t) and neg(t) in cls})
        rk = Mx.rank()
        stalled = len(rels) >= 3 * len(cols) + 24 and rk == last_rank
        last_rank = rk
        OPS[0] = ops_save
        if rk == len(cols) - defic or stalled:
            sol = Mx.solve_right(v); la_s += time.time() - t1
            L = {jj: int(sol[ci[jj]]) for jj in cols}; la_fieldops = 2 * len(cols) * nnz; rank_def = len(cols) - rk
            break
        la_s += time.time() - t1
        next_check = len(rels) + max(8, len(cols) // 10)
    precomp_rel_ops = OPS[0]; yield_ = len(rels) / trials

    dres = []
    for si, (k, Q) in enumerate(zip(scalars, Qs)):
        w = random.Random(7 * si + 1 + 1000 * CIDX); OPS[0] = 0
        b = w.randrange(1, q); R = add(Q, mul(b, G)); tr = 0; wrong = 0
        while True:
            j = w.randrange(R_STEPS); b = (b + STEP_S[j]) % q; R = add(R, STEP_P[j]); tr += 1
            if R is None: continue
            d = lookup(R)
            if d is not None and all(jj in L for (jj, _) in d):
                kk = (sum(s * L[jj] for (jj, s) in d) - b) % q
                if mul(kk, G) == Q: break
                wrong += 1
        dres.append((OPS[0], tr, int(kk == k), wrong))
    assert all(x[2] for x in dres), "descent produced wrong k"
    dops = [x[0] for x in dres]
    return dict(kind="ic", **meta, strat=strat, c=c, coset=coset, B=B, fb=nF, signed=nS, table_entries=tbl, list_entries=lst,
                build_ops=build, build_s=build_s, step_cost=STEP_COST, precomp_trials=trials, relations=len(rels),
                yield_=yield_, pred_yield=pred, precomp_rel_ops=precomp_rel_ops,
                precomp_ops=STEP_COST + build + precomp_rel_ops, la_cols=len(L), la_s=la_s, la_fieldops_est=la_fieldops,
                rank_deficiency=int(rank_def), desc_ops_all=dops, desc_ops_mean=sum(dops) / len(dops),
                desc_trials_mean=sum(x[1] for x in dres) / len(dres), verify_ops_est=1.5 * log2q,
                recovered=sum(x[2] for x in dres), nscal=len(dres), wrong_candidates=sum(x[3] for x in dres))

CLIST = [float(x) for x in os.environ.get("CLIST", "0.5,1,2").split(",")]
STRATS = os.environ.get("STRATS", "FULL,MITM").split(",")
for strat in STRATS:
    if strat == "FULL" and M == 2 and BITS > 24: continue
    for c in CLIST:
        for coset in (0, 1):
            t0 = time.time()
            r = run_config(c, coset, strat)
            if r is None or r == "tiny":
                print(f"  {strat} m={M} c={c} coset={coset}: skipped ({'over table/work cap' if r is None else 'predicted yield < 1%'})", flush=True)
                continue
            out.write(json.dumps(r) + "\n"); out.flush()
            print(f"  b={BITS} cv={CIDX} {strat} m={M} c={c} coset={coset} |FB|={r['fb']} tbl={r['table_entries']} list={r['list_entries']} "
                  f"yield={r['yield_']:.3f} pre={r['precomp_ops']:.0f} desc={r['desc_ops_mean']:.0f} LA={r['la_cols']} ({r['la_s']:.1f}s) "
                  f"rec={r['recovered']}/{r['nscal']} wall={time.time()-t0:.0f}s", flush=True)
print("done", flush=True)
