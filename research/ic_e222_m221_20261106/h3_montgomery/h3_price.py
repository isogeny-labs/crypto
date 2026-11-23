#!/usr/bin/env python3
"""H3 pricing on the real M-221 curve: exact counted field operations for the
relation-search step x(aG + bQ).  Python only; no wall time.  Run as
    python3 -I h3_price.py --out results.json
Unvalidated, pre-gate.  Counted ops: M (mul), S (sqr), I (inv), D (mul by a
small constant), A (add/sub/double).  Comparisons and conditional swaps are
not counted.  Curve: other/M-221 from docs/curves/standards/parameters.json."""
import argparse, json, math, random, statistics, sys, importlib.util, os

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))

# ---- registry parameters (other/M-221) read from the repository file -------
reg = json.load(open(os.path.join(REPO, "docs/curves/standards/parameters.json")))
ent = [c for c in (reg if isinstance(reg, list) else reg.get("curves", reg.values())) if isinstance(c, dict) and c.get("source_id") == "other/M-221"][0]
P = int(ent["field"]["p"], 16)
A = int(ent["params"]["a"]["raw"], 16)
Bc = int(ent["params"]["b"]["raw"], 16)
GX = int(ent["generator"]["x"]["raw"], 16)
GY = int(ent["generator"]["y"]["raw"], 16)
R = int(ent["order"], 16)
H = int(ent["cofactor"], 16)
assert P == 2**221 - 3 and A == 117050 and Bc == 1 and GX == 4 and H == 8
A24 = (A + 2) // 4
assert (A + 2) % 4 == 0 and A24 == 29263

# ---- counting field -----------------------------------------------------------
class Ctr:
    M = S = I = D = A = 0
    @classmethod
    def reset(cls): cls.M = cls.S = cls.I = cls.D = cls.A = 0
    @classmethod
    def snap(cls): return dict(M=cls.M, S=cls.S, I=cls.I, D=cls.D, A=cls.A)

def mul(a, b): Ctr.M += 1; return a * b % P
def sqr(a): Ctr.S += 1; return a * a % P
def inv(a): Ctr.I += 1; return pow(a, -1, P)
def dmul(a, k): Ctr.D += 1; return a * k % P
def add(a, b): Ctr.A += 1; return (a + b) % P
def sub(a, b): Ctr.A += 1; return (a - b) % P

# ---- reference (uncounted) affine Montgomery arithmetic ----------------------
def ref_add(p1, p2):
    if p1 is None: return p2
    if p2 is None: return p1
    x1, y1 = p1; x2, y2 = p2
    if x1 == x2:
        if (y1 + y2) % P == 0: return None
        l = (3 * x1 * x1 + 2 * A * x1 + 1) * pow(2 * y1, -1, P) % P
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, P) % P
    x3 = (l * l - A - x1 - x2) % P
    return (x3, (l * (x1 - x3) - y1) % P)
def ref_neg(p1): return None if p1 is None else (p1[0], (-p1[1]) % P)
def ref_mul(k, p1):
    r = None
    if k < 0: k, p1 = -k, ref_neg(p1)
    while k:
        if k & 1: r = ref_add(r, p1)
        p1 = ref_add(p1, p1); k >>= 1
    return r
def on_mont(p1): return (p1[1] ** 2 - (p1[0] ** 3 + A * p1[0] ** 2 + p1[0])) % P == 0

G = (GX, GY)
assert on_mont(G)

# ---- Weierstrass image (map from gen_instances.py) ---------------------------
spec = importlib.util.spec_from_file_location("gen_instances", os.path.join(REPO, "research/ic_e222_m221_20261106/gen_instances.py"))
gi = importlib.util.module_from_spec(spec); spec.loader.exec_module(gi)
AW, BW = gi.mont_to_weier(A, 1, P)
SHIFT = A * pow(3, -1, P) % P          # u = x + A/3, v = y  (B = 1)
def to_w(p1): return None if p1 is None else ((p1[0] + SHIFT) % P, p1[1])
def from_w(p1): return None if p1 is None else ((p1[0] - SHIFT) % P, p1[1])
def on_w(p1): return (p1[1] ** 2 - (p1[0] ** 3 + AW * p1[0] + BW)) % P == 0
assert on_w(to_w(G))

# ---- x-only Montgomery arithmetic (projective X:Z) ---------------------------
def xdbladd(Xp, Zp, Xq, Zq, xd):
    """(2P, P+Q) from x(P), x(Q), affine x(Q-P)=xd.  Montgomery 1987;
    Costello-Smith 2017 Alg. 'xDBLADD': 5M + 4S + 1D (difference affine)."""
    t0 = add(Xp, Zp); t1 = sub(Xp, Zp)
    X2 = sqr(t0)
    t2 = sub(Xq, Zq); t3 = add(Xq, Zq)
    t0 = mul(t0, t2)
    Z2 = sqr(t1)
    t1 = mul(t1, t3)
    t2 = sub(X2, Z2)
    X2 = mul(X2, Z2)
    t3 = dmul(t2, A24)
    Zs = sub(t0, t1)
    t0 = add(t0, t1)
    Z2 = add(t3, Z2)
    Xs = sqr(t0)
    Zs = sqr(Zs)
    Z2 = mul(Z2, t2)
    Zs = mul(Zs, xd)
    return X2, Z2, Xs, Zs

def ladder(k, x0):
    """x-only Montgomery ladder over every bit of k; returns (R0, R1) = (kP, (k+1)P) projective."""
    R0 = (1, 0); R1 = (x0, 1)
    for i in range(k.bit_length() - 1, -1, -1):
        bit = (k >> i) & 1
        if bit: R0, R1 = R1, R0
        X2, Z2, Xs, Zs = xdbladd(R0[0], R0[1], R1[0], R1[1], x0)
        R0, R1 = (X2, Z2), (Xs, Zs)
        if bit: R0, R1 = R1, R0
    return R0, R1

def xonly_affine(Xz):
    return mul(Xz[0], inv(Xz[1]))

# ---- Weierstrass Jacobian (y^2 = x^3 + AW x + BW) -----------------------------
INF = None
def jdbl(pt):
    """dbl-2007-bl with a full-size 'a': 2M + 8S."""
    if pt is None: return None
    X1, Y1, Z1 = pt
    if Y1 == 0: return None
    XX = sqr(X1); YY = sqr(Y1); YYYY = sqr(YY); ZZ = sqr(Z1)
    t = sqr(add(X1, YY))
    S_ = sub(sub(t, XX), YYYY); S_ = add(S_, S_)
    M_ = add(add(XX, XX), XX); M_ = add(M_, mul(AW, sqr(ZZ)))
    T = sub(sub(sqr(M_), S_), S_)
    e = add(YYYY, YYYY); e = add(e, e); e = add(e, e)
    Y3 = sub(mul(M_, sub(S_, T)), e)
    Z3 = sub(sub(sqr(add(Y1, Z1)), YY), ZZ)
    return (T, Y3, Z3)

def jmadd(pt, q):
    """madd-2007-bl (Z2 = 1): 7M + 4S."""
    if q is None: return pt
    x2, y2 = q
    if pt is None: return (x2, y2, 1)
    X1, Y1, Z1 = pt
    Z1Z1 = sqr(Z1)
    U2 = mul(x2, Z1Z1)
    S2 = mul(mul(y2, Z1), Z1Z1)
    Hh = sub(U2, X1)
    HH = sqr(Hh)
    I_ = add(HH, HH); I_ = add(I_, I_)
    J = mul(Hh, I_)
    r = sub(S2, Y1); r = add(r, r)
    if Hh == 0:
        return jdbl(pt) if r == 0 else None
    V = mul(X1, I_)
    X3 = sub(sub(sub(sqr(r), J), V), V)
    t = mul(Y1, J)
    Y3 = sub(mul(r, sub(V, X3)), add(t, t))
    Z3 = sub(sub(sqr(add(Z1, Hh)), Z1Z1), HH)
    return (X3, Y3, Z3)

def jadd(p1, p2):
    """add-2007-bl: 11M + 5S."""
    if p1 is None: return p2
    if p2 is None: return p1
    X1, Y1, Z1 = p1; X2, Y2, Z2 = p2
    Z1Z1 = sqr(Z1); Z2Z2 = sqr(Z2)
    U1 = mul(X1, Z2Z2); U2 = mul(X2, Z1Z1)
    S1 = mul(mul(Y1, Z2), Z2Z2); S2 = mul(mul(Y2, Z1), Z1Z1)
    Hh = sub(U2, U1)
    r = sub(S2, S1); r = add(r, r)
    if Hh == 0:
        return jdbl(p1) if r == 0 else None
    t = add(Hh, Hh); I_ = sqr(t)
    J = mul(Hh, I_)
    V = mul(U1, I_)
    X3 = sub(sub(sub(sqr(r), J), V), V)
    t = mul(S1, J)
    Y3 = sub(mul(r, sub(V, X3)), add(t, t))
    Z3 = mul(sub(sub(sqr(add(Z1, Z2)), Z1Z1), Z2Z2), Hh)
    return (X3, Y3, Z3)

def jsmul(k, base):
    """MSB-first double-and-add with mixed addition of an affine base."""
    acc = None
    for i in range(k.bit_length() - 1, -1, -1):
        acc = jdbl(acc)
        if (k >> i) & 1:
            acc = jmadd(acc, base)
    return acc

def jx_affine(pt):
    zi = inv(pt[2]); return mul(pt[0], sqr(zi))
def jto_affine(pt):
    zi = inv(pt[2]); zi2 = sqr(zi); zi3 = mul(zi2, zi)
    return (mul(pt[0], zi2), mul(pt[1], zi3))
def aff_add_w(p1, p2):
    """affine add on the Weierstrass image: 1I + 2M + 1S."""
    l = mul(sub(p2[1], p1[1]), inv(sub(p2[0], p1[0])))
    x3 = sub(sub(sqr(l), p1[0]), p2[0])
    return (x3, sub(mul(l, sub(p1[0], x3)), p1[1]))

# ---- price ---------------------------------------------------------------
def price(c, ir, s=0.8, d=0.1):
    return c["M"] + s * c["S"] + d * c["D"] + ir * c["I"]

def mean_counts(cs):
    return {k: statistics.mean(c[k] for c in cs) for k in "MSIDA"}

# ---- route (a): Weierstrass-Jacobian, two scalar mults + one full add -----------
def route_a(a, b, Gw, Qw):
    Ctr.reset()
    Ra = jsmul(a, Gw); Rb = jsmul(b, Qw)
    S_ = jadd(Ra, Rb)
    x = jx_affine(S_)
    return x, Ctr.snap()

# producer shape: each scalar mult converted to affine, affine add (full point)
def route_a0(a, b, Gw, Qw):
    Ctr.reset()
    pa = jto_affine(jsmul(a, Gw)); pb = jto_affine(jsmul(b, Qw))
    s = aff_add_w(pa, pb)
    return s[0], Ctr.snap()

# route (c): Shamir interleaved on Jacobian; table G, Q, G+Q affine (precomputed once)
def route_c(a, b, Gw, Qw, GQw):
    Ctr.reset()
    tab = {(1, 0): Gw, (0, 1): Qw, (1, 1): GQw}
    n = max(a.bit_length(), b.bit_length())
    acc = None
    for i in range(n - 1, -1, -1):
        acc = jdbl(acc)
        sel = ((a >> i) & 1, (b >> i) & 1)
        if sel != (0, 0): acc = jmadd(acc, tab[sel])
    x = jx_affine(acc)
    return x, Ctr.snap()

# route (b1): two x-only ladders, y-recovery, one affine Montgomery add (all counted)
Y2G = GY * GY % P
def recover_y(xP, yP, y2P, inv2y, k1, k2):
    """y of kP from x(kP) = k1 = (X1,Z1), x((k+1)P) = k2 = (X2,Z2), P = (xP, yP), B = 1.
    (y-yk)^2 = (x_{k+1}+A+xk+x)(x-xk)^2 and yk^2 = xk^3+A xk^2+xk give
    yk = (yk^2 + y^2 - RHS)/(2y).  Batch inversion of Z1, Z2: 1I + 3M."""
    X1, Z1 = k1; X2, Z2 = k2
    z12 = mul(Z1, Z2); iz = inv(z12)
    zi1 = mul(iz, Z2); zi2 = mul(iz, Z1)
    xk = mul(X1, zi1); xk1 = mul(X2, zi2)
    xk2 = sqr(xk)
    yk2 = add(add(mul(xk2, xk), dmul(xk2, A)), xk)
    dx = sub(xP, xk)
    rhs = mul(add(add(add(xk1, A), xk), xP), sqr(dx))
    num = sub(add(yk2, y2P), rhs)
    return xk, mul(num, inv2y)

def route_b1(a, b, Gm, Qm, ctx):
    Ctr.reset()
    (Ga0, Ga1), _ = ladder(a, Gm[0]), None
    (Qb0, Qb1), _ = ladder(b, Qm[0]), None
    xa, ya = recover_y(Gm[0], Gm[1], ctx["y2G"], ctx["inv2yG"], Ga0, Ga1)
    xb, yb = recover_y(Qm[0], Qm[1], ctx["y2Q"], ctx["inv2yQ"], Qb0, Qb1)
    l = mul(sub(yb, ya), inv(sub(xb, xa)))
    x3 = sub(sub(sub(sqr(l), A), xa), xb)
    return x3, Ctr.snap()

# route (b2): Euclidean differential chain (Montgomery 1992 / subtractive form), x-only
def dadd(P1, P2, Dd):
    """differential addition, 4M + 2S (3M + 2S when the difference has Z = 1).
    Montgomery 1987.  Points are (X, Z, affine_flag)."""
    X1, Z1, _ = P1; X2, Z2, _ = P2; Xd, Zd, aff = Dd
    t0 = add(X1, Z1); t1 = sub(X1, Z1)
    t2 = sub(X2, Z2); t3 = add(X2, Z2)
    u = mul(t0, t2); v = mul(t1, t3)
    s = sqr(add(u, v)); d = sqr(sub(u, v))
    X = s if aff else mul(s, Zd)
    Z = mul(d, Xd)
    return (X, Z, False)

def euclid_chain(a, b, Gx, Qx, Dx):
    """x-only a*G + b*Q for coprime a, b > 0, given x(G), x(Q), x(G-Q) (all affine).
    State (P1, P2, D = P1 - P2, a, b) with target a*P1 + b*P2; one dADD per step."""
    P1 = (Gx, 1, True); P2 = (Qx, 1, True); Dd = (Dx, 1, True)
    steps = 0
    while a > 0 and b > 0:
        S_ = dadd(P1, P2, Dd)
        if a >= b:   # a P1 + b P2 = (a-b) P1 + b (P1+P2); pair (P1, P1+P2), difference P2
            a -= b; P1, P2, Dd = P1, S_, P2
        else:        # = a (P1+P2) + (b-a) P2; pair (P1+P2, P2), difference P1
            b -= a; P1, P2, Dd = S_, P2, P1
        steps += 1
    Pf = P2 if a == 0 else P1
    assert (a == 0 and b == 1) or (b == 0 and a == 1), "gcd(a,b) != 1"
    return Pf, steps

def euclid_steps(a, b):
    """number of subtractive steps (= sum of partial quotients) for coprime a, b."""
    s = 0
    while a and b:
        if a >= b: q, a = divmod(a, b)
        else: q, b = divmod(b, a)
        s += q
    return s

PHI = (1 + 5 ** 0.5) / 2
def golden_pair(rng, nbits, K):
    """Random a of nbits bits; b among K candidates around a/phi with the fewest Euclid steps."""
    a = rng.randrange(1 << (nbits - 1), 1 << nbits)
    from fractions import Fraction
    # exact a/phi to full precision via integer sqrt: b0 = floor(a*(sqrt5-1)/2)
    s5 = math.isqrt(5 * a * a * 4)  # = floor(2 a sqrt5)
    b0 = (s5 - 2 * a) // 4
    best = None
    for dlt in range(-K // 2, K // 2 + 1):
        b = b0 + dlt
        if b < 1 or math.gcd(a, b) != 1: continue
        st = euclid_steps(a, b)
        if best is None or st < best[0]: best = (st, b)
    return a, best[1], best[0]

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--pairs", type=int, default=200)
    ap.add_argument("--golden-pairs", type=int, default=100)
    ap.add_argument("--golden-k", type=int, default=2000)
    ap.add_argument("--seed", default="h3-v1")
    args = ap.parse_args()
    rng = random.Random(args.seed)
    out = {"status": "unvalidated (pre-gate)", "curve": "other/M-221", "p_bits": P.bit_length(),
           "r_bits": R.bit_length(), "seed": args.seed}

    # ---- correctness: [r]G = O both ways --------------------------------------
    rg = ref_mul(R, G)
    (r0, r1) = ladder(R, GX)
    ladder_inf = (r0[1] == 0)
    jac = jsmul(R, to_w(G))
    out["order_check"] = {"affine_mont_rG_is_O": rg is None, "xonly_ladder_Z_is_0": bool(ladder_inf),
                          "weierstrass_jacobian_rG_is_O": jac is None}
    if not (rg is None and ladder_inf and jac is None):
        out["order_check"]["conclusion"] = "registry generator/order inconsistent; stopped"
        json.dump(out, open(args.out, "w"), indent=1); print(json.dumps(out["order_check"])); return 1

    # ---- correctness: ladder x([k]G) = Weierstrass x under the map --------------
    ok_lad = 0
    for _ in range(20):
        k = rng.randrange(1, R)
        (X0, Z0), _r1 = ladder(k, GX)
        xl = X0 * pow(Z0, -1, P) % P
        jx = jsmul(k, to_w(G)); zi = pow(jx[2], -1, P)
        xw = jx[0] * zi * zi % P
        ok_lad += (xw - SHIFT) % P == xl and ref_mul(k, G)[0] == xl
    out["ladder_vs_weierstrass_20"] = ok_lad

    # ---- fixed Q (a random multiple of G; the log is never given to a solver) -------
    kq = rng.randrange(2, R)
    Q = ref_mul(kq, G)
    Gw, Qw = to_w(G), to_w(Q)
    GQ = ref_add(G, Q); GQw = to_w(GQ)
    Dm = ref_add(G, ref_neg(Q)); Dx = Dm[0]
    ctx = {"y2G": GY * GY % P, "inv2yG": pow(2 * GY, -1, P), "y2Q": Q[1] * Q[1] % P, "inv2yQ": pow(2 * Q[1], -1, P)}

    # ---- uniform random pairs: routes a0, a, c, b1 ---------------------------------
    rows = {"a0": [], "a": [], "c": [], "b1": []}
    agree = 0
    for _ in range(args.pairs):
        a = rng.randrange(1, R); b = rng.randrange(1, R)
        ref = ref_add(ref_mul(a, G), ref_mul(b, Q)); xr = ref[0]
        x0, c0 = route_a0(a, b, Gw, Qw)
        xa_, ca = route_a(a, b, Gw, Qw)
        xc, cc = route_c(a, b, Gw, Qw, GQw)
        xb1, cb = route_b1(a, b, (GX, GY), Q, ctx)
        agree += all(v == xr or (v - SHIFT) % P == xr for v in (x0, xa_, xc)) and xb1 == xr
        rows["a0"].append(c0); rows["a"].append(ca); rows["c"].append(cc); rows["b1"].append(cb)
    out["route_agreement_with_reference"] = f"{agree}/{args.pairs}"

    # ---- euclid chain: correctness (execute), step counts ------------------------
    # executed on reduced-size pairs and on golden-selected full-size pairs
    ok_eu = 0; tot_eu = 0
    for nb in (16, 32, 64):
        for _ in range(5):
            while True:
                a = rng.randrange(1, 1 << nb); b = rng.randrange(1, 1 << nb)
                if math.gcd(a, b) == 1: break
            Ctr.reset()
            Pf, st = euclid_chain(a, b, GX, Q[0], Dx)
            xf = Pf[0] * pow(Pf[1], -1, P) % P
            ref = ref_add(ref_mul(a, G), ref_mul(b, Q))[0]
            ok_eu += xf == ref; tot_eu += 1
            c = Ctr.snap(); exp_M = 4 * st - 1 - 0  # first step has affine difference (3M), later 4M unless affine
    out["euclid_exec_check_small"] = f"{ok_eu}/{tot_eu}"

    gold = []
    for _ in range(args.golden_pairs):
        a, b, st = golden_pair(rng, R.bit_length() - 1, args.golden_k)
        Ctr.reset()
        Pf, st2 = euclid_chain(a, b, GX, Q[0], Dx)
        xf = Pf[0] * pow(Pf[1], -1, P) % P
        c = Ctr.snap(); c["I"] += 1; c["M"] += 1   # final affine conversion (do it for real below)
        refx = ref_add(ref_mul(a, G), ref_mul(b, Q))[0]
        gold.append((st, st2, xf == refx, c, a.bit_length(), b.bit_length()))
    out["euclid_golden"] = {"k_candidates": args.golden_k, "pairs": args.golden_pairs,
        "steps_mean": statistics.mean(g[0] for g in gold), "steps_min": min(g[0] for g in gold), "steps_max": max(g[0] for g in gold),
        "steps_formula_equals_executed": all(g[0] == g[1] for g in gold), "x_matches_reference": f"{sum(g[2] for g in gold)}/{len(gold)}",
        "log_phi_bound_steps": math.log(1 << (R.bit_length() - 1)) / math.log(PHI)}
    rows["b2_golden"] = [g[3] for g in gold]

    # uniform random pairs: step counts from the integer Euclid (formula, equal to executed count)
    uni = []
    for _ in range(args.pairs):
        while True:
            a = rng.randrange(1, R); b = rng.randrange(1, R)
            if math.gcd(a, b) == 1: break
        uni.append(euclid_steps(a, b))
    out["euclid_uniform"] = {"pairs": args.pairs, "steps_median": statistics.median(uni), "steps_mean": statistics.mean(uni),
                             "steps_min": min(uni), "steps_max": max(uni)}
    # per-step cost, measured: one executed dADD with projective difference and one with affine
    Ctr.reset(); dadd((5, 7, False), (11, 13, False), (17, 19, False)); out["dadd_projective_diff"] = Ctr.snap()
    Ctr.reset(); dadd((5, 1, True), (11, 1, True), (17, 1, True)); out["dadd_affine_diff"] = Ctr.snap()
    Ctr.reset(); xdbladd(5, 1, 11, 1, 17); out["xdbladd_affine_diff"] = Ctr.snap()
    Ctr.reset(); jdbl((5, 7, 11)); out["jdbl"] = Ctr.snap()
    Ctr.reset(); jmadd((5, 7, 11), (13, 17)); out["jmadd"] = Ctr.snap()
    Ctr.reset(); jadd((5, 7, 11), (13, 17, 19)); out["jadd"] = Ctr.snap()

    # uniform-pair Euclid counts: steps * per-step (first step affine diff)
    per_proj = out["dadd_projective_diff"]; per_aff = out["dadd_affine_diff"]
    def euclid_counts(steps):
        c = {k: steps * per_proj[k] for k in "MSIDA"}
        for k in "MSIDA": c[k] -= (per_proj[k] - per_aff[k])        # first step has an affine difference
        c["I"] += 1; c["M"] += 1                                      # final x = X/Z
        return c
    rows["b2_uniform"] = [euclid_counts(s) for s in uni]

    # ---- summary ---------------------------------------------------------------------
    names = {"a0": "(a0) producer shape: 2 scalar mults to affine + affine add",
             "a": "(a) Jacobian: 2 scalar mults + 1 Jacobian add, x only",
             "c": "(c) Jacobian Shamir interleaved (table G, Q, G+Q)",
             "b1": "(b1) 2 x-only ladders + y-recovery + 1 affine add",
             "b2_golden": "(b2) Euclid differential chain, golden-selected pair",
             "b2_uniform": "(b2u) Euclid differential chain, uniform random pair"}
    out["summary"] = {}
    for key, lst in rows.items():
        m = mean_counts(lst)
        out["summary"][key] = {"label": names[key], "counts_mean": m,
            "m_eq": {f"I={r}M": price(m, r) for r in (20, 80, 100)}, "n": len(lst)}
    out["pricing_convention"] = "M-eq = M + 0.8*S + 0.1*D + (I/M ratio)*I ; A (add/sub) not priced; scalars uniform in [1, r-1] (%d-bit order)" % R.bit_length()
    json.dump(out, open(args.out, "w"), indent=1)
    print(json.dumps({k: out[k] for k in out if k not in ("summary",)}, indent=1)[:1800])
    for key, v in out["summary"].items():
        m = v["counts_mean"]
        print(f'{key:11s} M={m["M"]:8.1f} S={m["S"]:8.1f} I={m["I"]:4.1f} D={m["D"]:6.1f} A={m["A"]:8.1f}  '
              + "  ".join(f"{k}:{x:8.0f}" for k, x in v["m_eq"].items()))
    return 0

if __name__ == "__main__":
    sys.exit(main())
