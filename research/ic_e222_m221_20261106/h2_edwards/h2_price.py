#!/usr/bin/env python3
"""H2 pricing: Edwards unified addition vs Weierstrass Jacobian on the real E-222 curve,
in exact counted field operations.  Unvalidated, pre-gate.  Python only.

Run:  python3 -I h2_price.py --params /home/user/crypto/docs/curves/standards/parameters.json \
          --out <dir> [--scalars 200] [--seed 1]

Op classes: M full mul, S square, D mul by the small constant d (18 bit),
A add/sub/neg/shift-by-add (x2 = 1A, x3 = 2A, x4 = 2A, x8 = 3A), I inversion.
A multiplication by a full-size curve constant (Weierstrass a, Montgomery B) is counted as M.
"""
import argparse, json, random, time, collections, os, sys

ap = argparse.ArgumentParser()
ap.add_argument("--params", required=True); ap.add_argument("--out", required=True)
ap.add_argument("--scalars", type=int, default=200); ap.add_argument("--seed", type=int, default=1)
args = ap.parse_args()

reg = json.load(open(args.params))
items = reg if isinstance(reg, list) else reg.get("curves", reg)
cur = [c for c in (items if isinstance(items, list) else items.values()) if isinstance(c, dict) and c.get("source_id") == "other/E-222"][0]
p = int(cur["field"]["p"], 16)
d = int(cur["params"]["d"]["raw"], 16)
r = int(cur["order"], 16)
h = int(cur["cofactor"], 16)
Gx = int(cur["generator"]["x"]["raw"], 16); Gy = int(cur["generator"]["y"]["raw"], 16)
assert p == 2**222 - 117 and d == 0x27166 and h == 4

C = collections.Counter()
def reset(): C.clear()
def snap(): return {k: C[k] for k in "MSDAI"}
def mul(a, b): C["M"] += 1; return a * b % p
def sqr(a): C["S"] += 1; return a * a % p
def muld(a): C["D"] += 1; return a * d % p
def add(a, b): C["A"] += 1; return (a + b) % p
def sub(a, b): C["A"] += 1; return (a - b) % p
def neg(a): C["A"] += 1; return (-a) % p
def t2(a): C["A"] += 1; return 2 * a % p
def t3(a): C["A"] += 2; return 3 * a % p
def t4(a): C["A"] += 2; return 4 * a % p
def t8(a): C["A"] += 3; return 8 * a % p
def inv(a): C["I"] += 1; return pow(a, -1, p)
def power(a, e):  # left-to-right binary square-and-multiply, counted
    acc = a
    for bit in bin(e)[3:]:
        acc = sqr(acc)
        if bit == "1": acc = mul(acc, a)
    return acc

# ---------- curve maps (uncounted) ----------
inv0 = lambda a: pow(a, -1, p)
Am = 2 * (1 + d) * inv0(1 - d) % p          # Montgomery A, a_Ed = 1
Bm = 4 * inv0(1 - d) % p
a2 = Am * Bm % p; a4 = Bm * Bm % p
Wa = (a4 - a2 * a2 * inv0(3)) % p
Wb = (2 * pow(a2, 3, p) * inv0(27) - a2 * a4 * inv0(3)) % p
shift = a2 * inv0(3) % p
def ed2w(P):
    x, y = P
    u = (1 + y) * inv0(1 - y) % p; v = u * inv0(x) % p
    return ((Bm * u + shift) % p, a4 * v % p)
def w2ed(P):
    X, Y = P
    u = (X - shift) * inv0(Bm) % p; v = Y * inv0(a4) % p
    x = u * inv0(v) % p; y = (u - 1) * inv0(u + 1) % p
    return (x, y)
def on_ed(P): x, y = P; return (x * x + y * y - 1 - d * x * x * y * y) % p == 0
def on_w(P): X, Y = P; return (Y * Y - X ** 3 - Wa * X - Wb) % p == 0
G_ed = (Gx, Gy)
assert on_ed(G_ed), "registry generator not on curve"
G_w = ed2w(G_ed); assert on_w(G_w)
from math import isqrt
legendre = lambda a: pow(a % p, (p - 1) // 2, p)
assert legendre(d) == p - 1, "d must be a non-square for completeness"

# ---------- Edwards affine ----------
ED_ID = (0, 1)
def ed_aff_add(P, Q):
    x1, y1 = P; x2, y2 = Q
    xx = mul(x1, x2); yy = mul(y1, y2); xy = mul(x1, y2); yx = mul(y1, x2)
    t = muld(mul(xx, yy))
    n1 = add(xy, yx); n2 = sub(yy, xx)
    d1 = add(1, t); d2 = sub(1, t)
    m = mul(d1, d2); im = inv(m)
    i1 = mul(d2, im); i2 = mul(d1, im)
    return (mul(n1, i1), mul(n2, i2))
def ed_aff_mul(k, P):
    acc = None
    for bit in bin(k)[2:]:
        if acc is not None: acc = ed_aff_add(acc, acc)
        if bit == "1": acc = P if acc is None else ed_aff_add(acc, P)
    return acc if acc is not None else ED_ID

# ---------- Edwards extended (X:Y:Z:T), HWCD 2008 ----------
def ed_ext_from_aff(P): return (P[0], P[1], 1, P[0] * P[1] % p)   # T for a base point: not counted (amortised)
def ed_ext_add(P, Q):   # add-2008-hwcd, a=1, unified, complete for nonsquare d
    X1, Y1, Z1, T1 = P; X2, Y2, Z2, T2 = Q
    A = mul(X1, X2); B = mul(Y1, Y2); Cc = muld(mul(T1, T2)); Dd = mul(Z1, Z2)
    E = sub(sub(mul(add(X1, Y1), add(X2, Y2)), A), B)
    F = sub(Dd, Cc); Gg = add(Dd, Cc); H = sub(B, A)
    return (mul(E, F), mul(Gg, H), mul(F, Gg), mul(E, H))
def ed_ext_madd(P, Q):  # Z2 = 1 (affine base with T2 = x*y given)
    X1, Y1, Z1, T1 = P; X2, Y2, _, T2 = Q
    A = mul(X1, X2); B = mul(Y1, Y2); Cc = muld(mul(T1, T2)); Dd = Z1
    E = sub(sub(mul(add(X1, Y1), add(X2, Y2)), A), B)
    F = sub(Dd, Cc); Gg = add(Dd, Cc); H = sub(B, A)
    return (mul(E, F), mul(Gg, H), mul(F, Gg), mul(E, H))
def ed_ext_dbl(P):      # dbl-2008-hwcd, a=1
    X1, Y1, Z1, _ = P
    A = sqr(X1); B = sqr(Y1); Cc = t2(sqr(Z1)); D_ = A
    E = sub(sub(sqr(add(X1, Y1)), A), B)
    Gg = add(D_, B); F = sub(Gg, Cc); H = sub(D_, B)
    return (mul(E, F), mul(Gg, H), mul(F, Gg), mul(E, H))
def ed_ext_to_aff(P):
    X, Y, Z, _ = P; zi = inv(Z); return (mul(X, zi), mul(Y, zi))
EXT_ID = (0, 1, 1, 0)
def ed_ext_mul(k, P, mixed=True, to_aff=True):
    Pe = ed_ext_from_aff(P); acc = None
    for bit in bin(k)[2:]:
        if acc is not None: acc = ed_ext_dbl(acc)
        if bit == "1":
            acc = Pe if acc is None else (ed_ext_madd(acc, Pe) if mixed else ed_ext_add(acc, Pe))
    if acc is None: acc = EXT_ID
    return ed_ext_to_aff(acc) if to_aff else acc
def ed_ext_mul_win(k, P, w=4, to_aff=True):
    Pe = ed_ext_from_aff(P)
    tab = [None, Pe, ed_ext_dbl(Pe)]
    for i in range(3, 1 << w): tab.append(ed_ext_madd(tab[i - 1], Pe))
    bits = bin(k)[2:]; bits = "0" * ((-len(bits)) % w) + bits
    digs = [int(bits[i:i + w], 2) for i in range(0, len(bits), w)]
    acc = tab[digs[0]] if digs[0] else None
    for dg in digs[1:]:
        if acc is not None:
            for _ in range(w): acc = ed_ext_dbl(acc)
        if dg: acc = tab[dg] if acc is None else ed_ext_add(acc, tab[dg])
    if acc is None: acc = EXT_ID
    return ed_ext_to_aff(acc) if to_aff else acc

# ---------- Weierstrass affine ----------
def w_aff_add(P, Q):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = mul(add(t3(sqr(x1)), Wa), inv(t2(y1)))   # 3x^2 + a: a is added, not multiplied
        x3_ = sub(sub(sqr(l), x1), x2)
    else:
        l = mul(sub(y2, y1), inv(sub(x2, x1)))
        x3_ = sub(sub(sqr(l), x1), x2)
    return (x3_, sub(mul(l, sub(x1, x3_)), y1))
def w_aff_mul(k, P):
    acc = None
    for bit in bin(k)[2:]:
        if acc is not None: acc = w_aff_add(acc, acc)
        if bit == "1": acc = P if acc is None else w_aff_add(acc, P)
    return acc

# ---------- Weierstrass Jacobian ----------
def j_dbl(P):  # dbl-2007-bl, general a
    if P is None: return None
    X1, Y1, Z1 = P
    if Y1 == 0: return None
    XX = sqr(X1); YY = sqr(Y1); YYYY = sqr(YY); ZZ = sqr(Z1)
    S_ = t2(sub(sub(sqr(add(X1, YY)), XX), YYYY))
    M_ = add(t3(XX), mul(Wa, sqr(ZZ)))          # a * ZZ^2: full-size constant, counted M
    T = sub(sqr(M_), t2(S_))
    Y3 = sub(mul(M_, sub(S_, T)), t8(YYYY))
    Z3 = sub(sub(sqr(add(Y1, Z1)), YY), ZZ)
    return (T, Y3, Z3)
def j_add(P, Q):  # add-2007-bl
    if P is None: return Q
    if Q is None: return P
    X1, Y1, Z1 = P; X2, Y2, Z2 = Q
    Z1Z1 = sqr(Z1); Z2Z2 = sqr(Z2)
    U1 = mul(X1, Z2Z2); U2 = mul(X2, Z1Z1)
    S1 = mul(mul(Y1, Z2), Z2Z2); S2 = mul(mul(Y2, Z1), Z1Z1)
    H = sub(U2, U1)
    if H == 0:
        return j_dbl(P) if S1 == S2 else None
    I_ = sqr(t2(H)); J = mul(H, I_); r_ = t2(sub(S2, S1)); V = mul(U1, I_)
    X3 = sub(sub(sqr(r_), J), t2(V))
    Y3 = sub(mul(r_, sub(V, X3)), t2(mul(S1, J)))
    Z3 = mul(sub(sub(sqr(add(Z1, Z2)), Z1Z1), Z2Z2), H)
    return (X3, Y3, Z3)
def j_madd(P, Q):  # madd-2007-bl, Q affine (Z2 = 1)
    if P is None: return (Q[0], Q[1], 1)
    X1, Y1, Z1 = P; x2_, y2_ = Q
    Z1Z1 = sqr(Z1); U2 = mul(x2_, Z1Z1); S2 = mul(mul(y2_, Z1), Z1Z1)
    H = sub(U2, X1)
    if H == 0:
        return j_dbl(P) if S2 == Y1 else None
    HH = sqr(H); I_ = t4(HH); J = mul(H, I_); r_ = t2(sub(S2, Y1)); V = mul(X1, I_)
    X3 = sub(sub(sqr(r_), J), t2(V))
    Y3 = sub(mul(r_, sub(V, X3)), t2(mul(Y1, J)))
    Z3 = sub(sub(sqr(add(Z1, H)), Z1Z1), HH)
    return (X3, Y3, Z3)
def j_to_aff(P):
    if P is None: return None
    X, Y, Z = P; zi = inv(Z); zi2 = sqr(zi)
    return (mul(X, zi2), mul(mul(Y, zi2), zi))
def j_mul(k, P, mixed=True, to_aff=True):
    acc = None
    for bit in bin(k)[2:]:
        if acc is not None: acc = j_dbl(acc)
        if bit == "1":
            acc = (P[0], P[1], 1) if acc is None else (j_madd(acc, P) if mixed else j_add(acc, (P[0], P[1], 1)))
    return j_to_aff(acc) if to_aff else acc
def j_mul_win(k, P, w=4, to_aff=True):
    Pj = (P[0], P[1], 1)
    tab = [None, Pj, j_dbl(Pj)]
    for i in range(3, 1 << w): tab.append(j_madd(tab[i - 1], P))
    bits = bin(k)[2:]; bits = "0" * ((-len(bits)) % w) + bits
    digs = [int(bits[i:i + w], 2) for i in range(0, len(bits), w)]
    acc = tab[digs[0]] if digs[0] else None
    for dg in digs[1:]:
        if acc is not None:
            for _ in range(w): acc = j_dbl(acc)
        if dg: acc = tab[dg] if acc is None else j_add(acc, tab[dg])
    return j_to_aff(acc) if to_aff else acc

# ---------- (3) correctness ----------
rng = random.Random(args.seed)
out = {"status": "unvalidated pre-gate", "curve": "E-222 (registry other/E-222)", "p": hex(p), "d": d, "r": hex(r), "h": h}
reset()
rG_ed = ed_aff_mul(r, G_ed)
rG_ext = ed_ext_mul(r, G_ed)
rG_w = w_aff_mul(r, G_w)
rG_j = j_mul(r, G_w)
out["rG_is_identity"] = {"edwards_affine": rG_ed == ED_ID, "edwards_ext": rG_ext == ED_ID,
                          "weier_affine": rG_w is None, "weier_jacobian": rG_j is None}
if not all(out["rG_is_identity"].values()):
    out["verdict"] = "registry value inconsistent: [r]G != O on at least one implementation; stopped"
    json.dump(out, open(os.path.join(args.out, "results.json"), "w"), indent=1)
    print(json.dumps(out, indent=1)); sys.exit(2)
agree = []
for _ in range(20):
    k = rng.randrange(1, r)
    res = [ed_aff_mul(k, G_ed), ed_ext_mul(k, G_ed), ed_ext_mul(k, G_ed, mixed=False), ed_ext_mul_win(k, G_ed),
           w2ed(w_aff_mul(k, G_w)), w2ed(j_mul(k, G_w)), w2ed(j_mul(k, G_w, mixed=False)), w2ed(j_mul_win(k, G_w))]
    agree.append(all(x == res[0] for x in res))
    assert on_ed(res[0])
out["agree_20_scalars"] = all(agree); out["agree_count"] = sum(agree)
assert all(agree), "implementations disagree"

# ---------- (4) counts ----------
def mean_counts(fn, n):
    tot = collections.Counter(); scal = [rng.randrange(1, r) for _ in range(n)]
    for k in scal:
        reset(); fn(k)
        tot.update(C)
    return {c: round(tot[c] / n, 3) for c in "MSDAI"}
N = args.scalars
smult = {
 "Edwards affine, binary":                 mean_counts(lambda k: ed_aff_mul(k, G_ed), N),
 "Edwards extended, binary, mixed add":    mean_counts(lambda k: ed_ext_mul(k, G_ed), N),
 "Edwards extended, binary, unified add":  mean_counts(lambda k: ed_ext_mul(k, G_ed, mixed=False), N),
 "Edwards extended, 4-bit window":         mean_counts(lambda k: ed_ext_mul_win(k, G_ed), N),
 "Weierstrass affine, binary":             mean_counts(lambda k: w_aff_mul(k, G_w), N),
 "Weierstrass Jacobian, binary, mixed add (producer path)": mean_counts(lambda k: j_mul(k, G_w), N),
 "Weierstrass Jacobian, binary, general add": mean_counts(lambda k: j_mul(k, G_w, mixed=False), N),
 "Weierstrass Jacobian, 4-bit window":     mean_counts(lambda k: j_mul_win(k, G_w), N),
}
# same, without the final conversion to affine (for composition in the trial)
smult_noaff = {
 "Edwards extended, binary, mixed add":   mean_counts(lambda k: ed_ext_mul(k, G_ed, to_aff=False), N),
 "Weierstrass Jacobian, binary, mixed add": mean_counts(lambda k: j_mul(k, G_w, to_aff=False), N),
}
# single group operations (fixed sample, counted once)
def one(fn):
    reset(); fn(); return snap()
P1 = ed_aff_mul(rng.randrange(2, r), G_ed); P2 = ed_aff_mul(rng.randrange(2, r), G_ed)
E1 = ed_ext_add(ed_ext_from_aff(P1), ed_ext_from_aff(G_ed)); E2 = ed_ext_dbl(ed_ext_from_aff(P1))  # make Z != 1
E1 = ed_ext_from_aff(P1); E2 = ed_ext_from_aff(P2)
ex = ed_ext_dbl(ed_ext_from_aff(P1)); ey = ed_ext_dbl(ed_ext_from_aff(P2))
W1 = ed2w(P1); W2 = ed2w(P2)
jx = j_dbl((W1[0], W1[1], 1)); jy = j_dbl((W2[0], W2[1], 1))
single = {
 "Edwards affine add":                      one(lambda: ed_aff_add(P1, P2)),
 "Edwards extended unified add (HWCD)":     one(lambda: ed_ext_add(ex, ey)),
 "Edwards extended mixed add (Z2=1)":      one(lambda: ed_ext_madd(ex, E2)),
 "Edwards extended dedicated doubling":     one(lambda: ed_ext_dbl(ex)),
 "Weierstrass affine add":                  one(lambda: w_aff_add(W1, W2)),
 "Weierstrass affine doubling":             one(lambda: w_aff_add(W1, W1)),
 "Weierstrass Jacobian add (add-2007-bl)":  one(lambda: j_add(jx, jy)),
 "Weierstrass Jacobian mixed add (madd-2007-bl)": one(lambda: j_madd(jx, W2)),
 "Weierstrass Jacobian doubling (dbl-2007-bl)":   one(lambda: j_dbl(jx)),
}

# ---------- pricing ----------
Dm, Am_, Sm = 0.1, 0.0, 0.8
def price(c, I, A_=Am_, D_=Dm, S_=Sm):
    return c["M"] + S_ * c["S"] + D_ * c["D"] + A_ * c["A"] + I * c["I"]
IR = [1, 20, 80, 100]
# measured python I/M diagnostic
xs = [rng.randrange(2, p) for _ in range(2000)]
t = time.perf_counter()
for _ in range(20):
    for x in xs: x * x % p
tm = (time.perf_counter() - t) / (20 * len(xs))
t = time.perf_counter()
for x in xs: pow(x, -1, p)
ti = (time.perf_counter() - t) / len(xs)
py_ratio = ti / tm

# ---------- (5) IC trial model ----------
# exponent shapes of the producer's sqrt_mod_p for p = 3 mod 4: Euler (p-1)/2 always, root (p+1)/4 if QR
e_euler = (p - 1) >> 1; e_root = (p + 1) >> 2
def exp_counts(e): return {"S": e.bit_length() - 1, "M": bin(e).count("1") - 1}
ce, cr = exp_counts(e_euler), exp_counts(e_root)
def mimic_elem(xr, xi):   # one factor-base element of find_one_relation_counted_with_rng (counted)
    a_, b_ = Wa, Wb
    x1x2 = mul(xr, xi); sm = add(xr, xi); df = sub(xr, xi)
    qa = sqr(df)
    inner = add(mul(sm, add(x1x2, a_)), t2(b_))
    qb = neg(t2(inner))
    part = sub(x1x2, a_)
    qc = sub(sqr(part), mul(t4(b_), sm))
    disc = sub(sqr(qb), mul(t4(qa), qc))
    if power(disc, e_euler) != 1: return False
    s_ = power(disc, e_root)
    ti_ = inv(t2(qa))
    for sign in (0, 1):
        sv = neg(s_) if sign else s_
        mul(add(neg(qb), sv), ti_)
    return True
reset(); NS = 400; qr = 0
for _ in range(NS): qr += mimic_elem(rng.randrange(1, p), rng.randrange(1, 64))
elem = {c: C[c] / NS for c in "MSDAI"}
qr_frac = qr / NS
# producer trial pieces (means from the runs above): two Jacobian mixed scalar mults incl. to_affine, one affine add
add_aff = single["Weierstrass affine add"]
def trial(fb, model):
    prod = collections.Counter()
    for _ in range(2):
        for c, v in smult["Weierstrass Jacobian, binary, mixed add (producer path)"].items(): prod[c] += v
    for c, v in add_aff.items(): prod[c] += v
    smult_part = dict(prod)
    if model == "A":    # directive: one exponentiation 222S + 111M per fb element, plus ~10 small ops
        per = {"M": 111 + 3 + 1 + 2, "S": 222 + 2 + 1, "D": 0, "A": 12, "I": 0.5}
    else:               # B: source-exact (Euler always, root if QR), counted
        per = elem
    total = {c: smult_part.get(c, 0) + fb * per.get(c, 0) for c in "MSDAI"}
    return smult_part, total, per
def h2_part():
    prod = collections.Counter()
    for _ in range(2):
        for c, v in smult_noaff["Edwards extended, binary, mixed add"].items(): prod[c] += v
    for c, v in single["Edwards extended unified add (HWCD)"].items(): prod[c] += v
    # conversion of R to the Weierstrass x used by the Semaev step: u = (Z+Y)/(Z-Y), x_W = B*u + A*B/3
    prod["I"] += 1; prod["A"] += 3; prod["M"] += 2
    return dict(prod)
rows = {}
for model in "AB":
    for fb in (8, 32):
        sp, tot, per = trial(fb, model)
        h2p = h2_part()
        h2tot = {c: h2p.get(c, 0) + fb * per.get(c, 0) for c in "MSDAI"}
        for I in IR:
            cur_p = price(sp, I); cur_t = price(tot, I); h2_p = price(h2p, I); h2_t = price(h2tot, I)
            rows[f"model{model}_fb{fb}_I{I}"] = {"smult_part_cur": round(cur_p, 1), "trial_cur": round(cur_t, 1),
                "smult_share_cur": round(cur_p / cur_t, 4), "smult_part_h2": round(h2_p, 1), "trial_h2": round(h2_t, 1),
                "trial_ratio_h2_over_cur": round(h2_t / cur_t, 4)}
res_s = {"smult": {k: {**v, **{f"M_eq_I{I}": round(price(v, I), 1) for I in IR}, "M_eq_I100_A0.05": round(price(v, 100, A_=0.05), 1)} for k, v in smult.items()},
         "single": {k: {**v, **{f"M_eq_I{I}": round(price(v, I), 2) for I in IR}} for k, v in single.items()}}
out.update({"n_scalars": N, "seed": args.seed, "pricing": {"S": Sm, "D": Dm, "A": Am_, "I_over_M": IR},
            "python_I_over_M_diagnostic": round(py_ratio, 1), "counts": res_s,
            "smult_no_final_affine": smult_noaff, "sqrt_exponent_shapes": {"euler_bits": e_euler.bit_length(), "euler_popcount": bin(e_euler).count("1"),
             "root_bits": e_root.bit_length(), "root_popcount": bin(e_root).count("1"), "euler_counts": ce, "root_counts": cr},
            "fb_element_counted_mean": {k: round(v, 2) for k, v in elem.items()}, "qr_fraction_observed": round(qr_frac, 3),
            "h2_replacement_part": h2_part(), "trial": rows})
os.makedirs(args.out, exist_ok=True)
json.dump(out, open(os.path.join(args.out, "results.json"), "w"), indent=1)
print(json.dumps({k: out[k] for k in ("rG_is_identity", "agree_count", "python_I_over_M_diagnostic", "qr_fraction_observed")}, indent=1))
print("fb element (counted mean):", out["fb_element_counted_mean"])
for k, v in smult.items(): print(f"{k:60s} {v}  Meq(I=80)={price(v, 80):.0f}")
for k, v in single.items(): print(f"{k:50s} {v}")
for k, v in rows.items(): print(k, v)
