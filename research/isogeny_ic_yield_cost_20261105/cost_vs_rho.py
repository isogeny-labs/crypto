from sage.all import *
# End-to-end cost of Semaev-style index calculus vs Pollard rho on toy Curve25519-like curves.
# Levers: (1) factor-base size B = c * p^(1/m), (2) cofactor-coset lookup on/off,
#         (3) decomposition strategy: FULL table of all m-sums vs MITM (table of ceil(m/2)-sums, loop over floor(m/2)-sums).
# All costs are counted in elliptic-curve group additions (one affine add/double = 1 op); LA reported separately.
# usage: sage -python cost_vs_rho.py BITS M NSCAL OUT.jsonl
import sys, json, time, random, math, itertools

BITS, M, NSCAL, OUT = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
MAX_COMBOS = 6_000_000
set_random_seed(BITS * 100 + 7)

# ---------- curve: Montgomery y^2 = x^3 + A x^2 + x, p = 5 mod 8, #E = 8q, #twist = 4q' ----------
p = next_prime(2**BITS)
while True:
    while p % 8 != 5: p = next_prime(p + 1)
    F = GF(p); found = None
    for A in range(486662 % p, min(p, 486662 % p + 4 * 4000), 4):
        if (A * A - 4) % p == 0: continue
        EM = EllipticCurve(F, [0, A, 0, 1, 0]); N = EM.order(); Nt = 2 * (p + 1) - N
        if N % 8 == 0 and is_prime(N // 8) and Nt % 4 == 0 and is_prime(Nt // 4):
            found = A; break
    if found is not None: break
    p = next_prime(p + 1)
p = int(p); A_M = found; h = 8; q = int(N // h)
E0 = EM.short_weierstrass_model(); a4 = int(E0.a4()); a6 = int(E0.a6())

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
def tup(P): return None if P.is_zero() else (int(P[0]), int(P[1]))

G = tup(h * E0.random_point())
while G is None: G = tup(h * E0.random_point())
rng = random.Random(BITS * 1000 + M)
scalars = [rng.randrange(1, q) for _ in range(NSCAL)]
Qs = [tup(k * E0(G[0], G[1])) for k in scalars]          # inputs, not counted
TORS = sorted({tup(q * E0.random_point()) for _ in range(300)}, key=str)
assert len(TORS) == h

R_STEPS = 64
steps_rng = random.Random(99)
STEP_S = [steps_rng.randrange(1, q) for _ in range(R_STEPS)]
OPS[0] = 0
STEP_P = [mul(s, G) for s in STEP_S]                      # shared precomputation, counted once below
STEP_COST = OPS[0]

# ---------- Pollard rho (r-adding walk + Brent cycle finding), no negation map ----------
def rho(Q, seed):
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
            if (b - sb) % q == 0:
                return rho(Q, seed + 7919)
            k = (sa - a) * pow(b - sb, -1, q) % q
            return k, OPS[0], setup, steps
        if lam == power:
            saved, sa, sb, power, lam = X, a, b, power * 2, 0

# ---------- factor base + tables ----------
def factor_base(c):
    B = max(2, int(c * p ** (1.0 / M)))
    fb = []
    for x in range(B):
        if E0.is_x_coord(F(x)): fb.append(tup(E0.lift_x(F(x))))
    return B, fb

def signed_list(fb):
    s = []
    for j, P in enumerate(fb):
        s.append((P, (j, 1)))
        if P[1] != 0: s.append((neg(P), (j, -1)))
    return s

def build_sums(signed, k):
    """dict: point -> one decomposition (list of (j, sign)) over all multisets of AT MOST k signed FB points
    (an 'empty slot' element allows fewer points); built incrementally so each real tree edge costs one add."""
    items = [(None, None)] + signed
    T = {}; OPS[0] = 0
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

def ncombos(n, k): return math.comb(n + k - 1, k)

# ---------- one configuration ----------
def run_config(c, coset):
    B, fb = factor_base(c); S = signed_list(fb); nF = len(fb)
    if ncombos(len(S), M) > MAX_COMBOS: return None
    if nF < 4 or ncombos(len(S), M) < 0.01 * h * q: return "tiny"
    tors = TORS if coset else [None]
    t0 = time.time()
    T, full_build = build_sums(S, M)
    # MITM structures for cost accounting: table of ceil(M/2)-sums, list of floor(M/2)-sums
    kt, kl = (M + 1) // 2, M // 2
    Tm, mitm_build_t = build_sums(S, kt)
    Lm, mitm_build_l = build_sums(S, kl)
    mitm_build = mitm_build_t + mitm_build_l
    mitm_per_lookup = len(Lm)            # one add per element of the loop list
    build_s = time.time() - t0

    def lookup(R):
        for Tt in tors:
            X = R if Tt is None else add(R, Tt)
            if X in T: return T[X]
        return None
    # MITM equivalence check on 200 random targets (uncounted)
    chk = random.Random(5); agree = 0
    for _ in range(200):
        R = mul(chk.randrange(1, q), G)
        full = R in T
        mitm = any(add(R, neg(s)) in Tm for s in Lm)
        agree += (full == mitm)
        if full != mitm and BITS <= 14: print("    MITM mismatch: full", full, "mitm", mitm, flush=True)

    # precomputation: relations a*G = sum e_j P_j  (mod cofactor), FB logs by linear algebra
    walk = random.Random(int(1000 * c) + coset)
    OPS[0] = 0
    a = walk.randrange(1, q); R = mul(a, G)
    rels = []; trials = 0
    def used_cols(): return len({j for (_, d) in rels for (j, _) in d})
    L = None; la_s = 0.0; la_fieldops = 0; last_rank = [-1]
    while trials < 3_000_000:
        while len(rels) < used_cols() + 8:
            j = walk.randrange(R_STEPS); a = (a + STEP_S[j]) % q; R = add(R, STEP_P[j]); trials += 1
            if R is None: continue
            d = lookup(R)
            if d is not None: rels.append((a, d))
        ops_rel = OPS[0]
        cols = sorted({j for (_, d) in rels for (j, _) in d}); ci = {j: i for i, j in enumerate(cols)}
        Mx = matrix(GF(q), len(rels), len(cols)); v = vector(GF(q), [x for x, _ in rels])
        nnz = 0
        for r_, (_, d) in enumerate(rels):
            for (j, s) in d: Mx[r_, ci[j]] += s; nnz += 1
        t1 = time.time()
        # m=2 without cosets: relations force T_i + T_j = 0 on cofactor classes, so every odd class function
        # f(-T) = -f(T) lies in the kernel (one dim per class pair {T,-T}, T != -T). Descent decompositions obey the
        # same constraint, so k is unaffected; allow exactly that deficiency (k is verified below anyway).
        defic = 0
        if M == 2 and not coset:
            saved_ops = OPS[0]
            cls = {mul(q, fb[j]) for j in cols}
            OPS[0] = saved_ops
            pairs = {frozenset((str(t), str(neg(t)))) for t in cls if t is not None and t != neg(t) and neg(t) in cls}
            defic = len(pairs)
        rk = Mx.rank()
        stalled = len(rels) >= 3 * len(cols) + 24 and rk == last_rank[0]
        last_rank[0] = rk
        if rk == len(cols) - defic or stalled:
            sol = Mx.solve_right(v); la_s += time.time() - t1
            la_fieldops = 2 * len(cols) * nnz      # Lanczos/Wiedemann-style estimate
            L = {j: int(sol[ci[j]]) for j in cols}
            OPS[0] = ops_rel
            break
        la_s += time.time() - t1
        OPS[0] = ops_rel
        for _ in range(8):
            j = walk.randrange(R_STEPS); a = (a + STEP_S[j]) % q; R = add(R, STEP_P[j]); trials += 1
            if R is None: continue
            d = lookup(R)
            if d is not None: rels.append((a, d))
    precomp_rel_ops = OPS[0]
    yield_ = len(rels) / trials

    # descent per scalar: Q + b*G walk until decomposable with only logged FB elements
    dres = []
    for si, (k, Q) in enumerate(zip(scalars, Qs)):
        w = random.Random(7 * si + 1); OPS[0] = 0
        b = w.randrange(1, q); R = add(Q, mul(b, G)); tr = 0; wrong = 0
        while True:
            j = w.randrange(R_STEPS); b = (b + STEP_S[j]) % q; R = add(R, STEP_P[j]); tr += 1
            if R is None: continue
            d = lookup(R)
            if d is not None and all(jj in L for (jj, _) in d):
                kk = (sum(s * L[jj] for (jj, s) in d) - b) % q
                if mul(kk, G) == Q: break          # counted verification
                wrong += 1
        dres.append(dict(ops=OPS[0], trials=tr, ok=int(kk == k), wrong=wrong))
    desc_ops = [x["ops"] for x in dres]; desc_tr = [x["trials"] for x in dres]
    lookups_per_trial = len(tors)
    precomp_trials = trials
    # MITM accounting: same decomposition set, so same trial counts; each lookup costs |Lm| adds instead of 1 table probe
    mitm_precomp = STEP_COST + mitm_build + precomp_trials * (1 + lookups_per_trial * mitm_per_lookup)
    mitm_desc = [t * (1 + lookups_per_trial * mitm_per_lookup) + 4 * math.log2(q) for t in desc_tr]
    full_precomp = STEP_COST + full_build + precomp_rel_ops
    assert all(x['ok'] for x in dres), 'descent produced wrong k'
    return dict(bits=BITS, p=p, A=A_M, q=q, m=M, c=c, coset=coset, B=B, fb=nF, signed=len(S),
                table_entries=len(T), full_build_ops=full_build, mitm_table_entries=len(Tm), mitm_list=len(Lm),
                mitm_build_ops=mitm_build, mitm_check_agree=agree, build_s=build_s,
                precomp_trials=precomp_trials, relations=len(rels), yield_=yield_, precomp_rel_ops=precomp_rel_ops,
                la_cols=len(L), la_s=la_s, la_fieldops_est=la_fieldops,
                full_precomp_ops=full_precomp, mitm_precomp_ops=mitm_precomp,
                desc_ops_mean=sum(desc_ops) / len(desc_ops), desc_ops_min=min(desc_ops), desc_ops_max=max(desc_ops),
                desc_trials_mean=sum(desc_tr) / len(desc_tr),
                mitm_desc_ops_mean=sum(mitm_desc) / len(mitm_desc),
                recovered=sum(x["ok"] for x in dres), nscal=len(dres), wrong_candidates=sum(x["wrong"] for x in dres),
                rank_deficiency=int(len(cols) - Mx.rank()))

out = open(OUT, "a")
# rho baseline (same scalars)
rr = []
RHO_N = int(__import__("os").environ.get("RHO_N", "0")) or len(scalars)
rs_rng = random.Random(BITS * 77)
rho_targets = [(k, Q) for k, Q in zip(scalars, Qs)]
while len(rho_targets) < RHO_N:
    k = rs_rng.randrange(1, q); rho_targets.append((k, tup(k * E0(G[0], G[1]))))
rho_steps = []; rho_setup = []
for si, (k, Q) in enumerate(rho_targets):
    kk, ops, su, stp = rho(Q, 1000 + si); rr.append((ops, int(kk == k))); rho_steps.append(stp); rho_setup.append(su)
rho_ops = [o for o, _ in rr]
out.write(json.dumps(dict(kind="rho", bits=BITS, p=p, q=q, m=M, ops_mean=sum(rho_ops) / len(rho_ops),
                          ops_min=min(rho_ops), ops_max=max(rho_ops), ops_sd=float(np.std(rho_ops)) if False else None,
                          ops_all=rho_ops, walk_steps_all=rho_steps, setup_mean=sum(rho_setup)/len(rho_setup), recovered=sum(ok for _, ok in rr), n=len(rr),
                          sqrt_pi_q_over_2=math.sqrt(math.pi * q / 2))) + "\n"); out.flush()
print(f"bits={BITS} p={p} q={q} m={M} rho mean ops={sum(rho_ops)/len(rho_ops):.0f} rec={sum(ok for _, ok in rr)}/{len(rr)}", flush=True)
if __import__("os").environ.get("RHO_ONLY"):
    print("rho walk steps mean", sum(rho_steps)/len(rho_steps), "setup", sum(rho_setup)/len(rho_setup), flush=True); sys.exit(0)
for c in (0.5, 1.0, 2.0):
    for coset in (0, 1):
        r = run_config(c, coset)
        if r is None or r == "tiny":
            print(f"  m={M} c={c} coset={coset}: skipped ({'table too large' if r is None else 'factor base too small'})", flush=True); continue
        r["kind"] = "ic"; out.write(json.dumps(r) + "\n"); out.flush()
        print(f"  m={M} c={c} coset={coset} |FB|={r['fb']} |T|={r['table_entries']} yield={r['yield_']:.3f} "
              f"full: pre={r['full_precomp_ops']:.0f} desc={r['desc_ops_mean']:.0f} | mitm: pre={r['mitm_precomp_ops']:.0f} "
              f"desc={r['mitm_desc_ops_mean']:.0f} | LA {r['la_cols']} cols {r['la_s']:.2f}s | mitm_chk={r['mitm_check_agree']}/200 "
              f"rec={r['recovered']}/{r['nscal']}", flush=True)
print("done", flush=True)
