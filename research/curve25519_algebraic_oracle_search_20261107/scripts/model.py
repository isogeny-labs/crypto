"""Combine the measured decision costs (data/d*.json) into the parity model of
curve25519_rho_parity_bound_20261104/scripts/screen.py, extended with setup and linear algebra.
S(w) = [G_setup + w N_setup + T (1 + g + w c) + w N_LA] / sqrt(r),  T = (F + 1) r / C.
P_G: group elements computed by an oracle are group operations (g counts them, c excludes their mults).
P_N: the screen's convention (those multiplications are native, in c; g = 0)."""
import json, math, sys, pathlib, random
sys.path.insert(0, str(pathlib.Path(__file__).parent))
D = pathlib.Path(__file__).resolve().parents[1]
r = 131203; sr = math.sqrt(r); RHO = {0: 1.600, 0.1: 1.716, 1: 2.764}; W = (0, 0.1, 1)
L = lambda n: json.load(open(D / "data" / n))
d1, d2, d3, d5, d7 = L("d1_m2.json"), L("d2_coset.json"), L("d3_torsion.json"), L("d5_m3.json"), L("d7_lattice.json")

def S(F, C, g=0.0, c=0.0, Gs=0.0, Ns=0.0, NLA=0.0):
    T = (F + 1) * r / C
    return T, {w: (Gs + w * Ns + T * (1 + g + w * c) + w * NLA) / sr for w in W}

rows = []
def add(design, variant, cls, pricing, F, C, src, exp_r, **k):
    T, s = S(F, C, **k)
    _, s0 = S(F, C, g=k.get('g', 0), c=k.get('c', 0))  # setup- and LA-free (screen convention)
    rows.append(dict(design=design, variant=variant, cls=cls, pricing=pricing, F=F, C=round(C), C_over_r=C / r,
                     trials=T, g=k.get('g', 0), c=k.get('c', 0), S=s, gap={w: s[w] / RHO[w] for w in W},
                     gap_setup_free={w: s0[w] / RHO[w] for w in W}, source=src, S_growth_exponent_in_r=exp_r))

LA = lambda m, F: (m + 1) * F * F          # derived: sparse Wiedemann/Lanczos, (m+1) nonzeros per row
BASE_N = lambda F: 25 * F                   # derived: one sqrt (~25 mults) per decoded base point
BASE_G = lambda F: 3 * F                    # derived: cofactor clearing, 3 doublings per base point

# --- screen reference row (derived inputs, reproduced) -------------------------------------
add("screen", "m=2 F=256 root-find, c=25F (screen row)", "generic (D1 identity)", "P_N", 256, 2 * 256 ** 2,
    "ALGEBRAIC_SCREEN.md / screen.py", 0.5, c=256 * 25)

# --- D1: m = 2 random base; best F per pricing over the measured rows -----------------------
best = {}
for row in d1['rows']:
    F, C = row['F'], row['C']
    for pr, kw in (("P_N", dict(c=row['c_add'])), ("P_G", dict(g=2 * F, c=0))):
        T, s = S(F, C, Gs=BASE_G(F), Ns=BASE_N(F), NLA=LA(2, F), **kw)
        if pr not in best or s[0.1] < best[pr][0]: best[pr] = (s[0.1], row, kw)
    if F == 256:
        add("D1", "m=2 F=256, screen-style quadratic + sqrt per base point (measured 82/base)", "generic", "P_N", F, C,
            "data/d1_m2.json", 0.5, c=row['c_quad'], Gs=BASE_G(F), Ns=BASE_N(F), NLA=LA(2, F))
for pr, (_, row, kw) in best.items():
    F = row['F']
    add("D1", f"m=2 shared-inversion additions, best F = {F}", "generic", pr, F, row['C'], "data/d1_m2.json", 0.5,
        Gs=BASE_G(F), Ns=BASE_N(F), NLA=LA(2, F), **kw)

# --- D2: coset base ---------------------------------------------------------------------------
cos = {(x['d'], x['g']): x for x in d2['rows']}; pc = {x['d']: x['c_poly'] for x in d2['poly_cost']}
for dd in (1022, 2394):
    x = cos[(dd, 5)]
    add("D2", f"coset g*mu_d, d={dd}, norm/resultant + gcd in F_p[X]/(X^d-c)", "algebraic", "P_G", x['unknowns'], x['C'],
        "data/d2_coset.json", None, c=pc[dd], Ns=BASE_N(x['unknowns']), NLA=LA(2, x['unknowns']))
x = cos[(2394, 5)]
add("D2", "coset d=2394, per-point additions", "generic", "P_G", x['unknowns'], x['C'], "data/d2_coset.json", 0.5,
    g=2 * x['base_points'], Ns=BASE_N(x['unknowns']), NLA=LA(2, x['unknowns']))
x = cos[(2394, 1)]
add("D2", "coset d=2394, g=1 (closed under x->1/x), per-point additions", "generic", "P_G", x['unknowns'], x['C'],
    "data/d2_coset.json", 0.5, g=2 * x['base_points'], Ns=BASE_N(x['unknowns']), NLA=LA(2, x['unknowns']))
# growth exponent of the polynomial decision: fit c(d) ~ d^e
ds = [126, 252, 1022, 2394]; cs = [d2['poly_check'][0]['c_poly_last'], d2['poly_check'][1]['c_poly_last'], pc[1022], pc[2394]]
e_poly = (math.log(cs[-1]) - math.log(cs[0])) / (math.log(ds[-1]) - math.log(ds[0]))
rows[-3]['S_growth_exponent_in_r'] = rows[-4]['S_growth_exponent_in_r'] = 0.5 * e_poly + 0.5  # F~sqrt r, d~F

# --- D3: E[8]-closed base (same unknowns) ------------------------------------------------------
x = [t for t in d3 if t['F_unknowns'] == 384][0]
for v in ("closure_reps", "closure_all_points"):
    F = 384; C = x[v]['hit_rate'] * r; npts = F if v == "closure_reps" else 8 * F
    add("D3", f"E[8]-closed base, F'=384 unknowns, {v}", "generic", "P_G", F, C, "data/d3_torsion.json", 0.5,
        g=2 * npts, Gs=BASE_G(F), Ns=BASE_N(F), NLA=LA(2, F))

# --- D4: eliminant of the covered set ----------------------------------------------------------
cov = lambda F: r * (1 - math.exp(-2 * F * F / r))  # calibrated: matches d1 C/r within 1 % (see README)
def best_hash():
    out = None
    for F in range(8, 400):
        T, s = S(F, cov(F), Gs=F * (F + 1) + BASE_G(F), Ns=BASE_N(F), NLA=LA(2, F))
        if out is None or s[0.1] < out[0]: out = (s[0.1], F)
    return out[1]
F = best_hash()
add("D4", f"precomputed pair-sum table (hash eliminant), best F = {F}", "generic (BSGS-shaped)", "P_G", F, cov(F),
    "derived from data/d1_m2.json coverage law", 1 / 6, Gs=F * (F + 1) + BASE_G(F), Ns=BASE_N(F), NLA=LA(2, F))

from functools import lru_cache
@lru_cache(maxsize=None)
def kara(n, m):  # exact mult count of fp-style Karatsuba (threshold 24), dense inputs
    if n == 0 or m == 0: return 0
    if min(n, m) < 24: return n * m
    k = max(n, m) // 2
    n0, n1 = min(n, k), max(n - k, 0); m0, m1 = min(m, k), max(m - k, 0)
    return kara(n0, m0) + kara(n1, m1) + kara(max(n0, n1), max(m0, m1))
@lru_cache(maxsize=None)
def tree(n):  # subproduct tree of n linear factors
    tot = 0; size = 1; cnt = n
    while cnt > 1:
        tot += (cnt // 2) * kara(size + 1, size + 1); size *= 2; cnt = (cnt + 1) // 2
    return tot
def multipoint_per_trial(n, b):  # degree-n Phi at a batch of b points, Newton-division remainder tree
    batch = tree(b) + (n // b + 1) * 4 * kara(b, b) + sum((b >> l) * 4 * kara(1 << l, 1 << l) for l in range(int(math.log2(b)) + 1))
    return batch / b
best = None
for F in (16, 24, 32, 48, 64, 96, 128):
    C = cov(F); n = int(C / 2); T = (F + 1) * r / C; b = max(2, min(n, int(T)))
    c = multipoint_per_trial(n, b)
    Ns = tree(n) + BASE_N(F); Gs = F * (F + 1) + BASE_G(F)
    _, s = S(F, C, c=c, Gs=Gs + (F + 1) * 2 * F, Ns=Ns, NLA=LA(2, F))
    if best is None or s[0.1] < best[0]: best = (s[0.1], F, C, c, Gs + (F + 1) * 2 * F, Ns)
_, F, C, c, Gs, Ns = best
add("D4", f"eliminant polynomial Phi + batched multipoint evaluation, best F = {F}", "algebraic decision, generic setup",
    "P_G", F, C, "derived: counted Karatsuba recursion (validated in model.py)", 1 / 6, c=c, Gs=Gs, Ns=Ns, NLA=LA(2, F))

# --- D5: m = 3 -----------------------------------------------------------------------------------
for row in d5:
    if row['F'] != 56: continue
    F, C = row['F'], row['C']
    add("D5", f"m=3 direct (pairs per trial), F={F}", "generic", "P_N", F, C, "data/d5_m3.json", 0.5,
        c=row['c_direct'], Ns=BASE_N(F), Gs=BASE_G(F), NLA=LA(3, F))
    add("D5", f"m=3 direct, F={F}", "generic", "P_G", F, C, "data/d5_m3.json", 0.5,
        g=2 * F + 2 * F * (F + 1), Ns=BASE_N(F), Gs=BASE_G(F), NLA=LA(3, F))
    add("D5", f"m=3 meet-in-the-middle pair table, F={F}", "generic (BSGS-shaped)", "P_N", F, C, "data/d5_m3.json", 1 / 6,
        c=row['c_mitm'], Ns=BASE_N(F) + 7 * F * (F + 1), Gs=BASE_G(F), NLA=LA(3, F))
    add("D5", f"m=3 meet-in-the-middle pair table, F={F}", "generic (BSGS-shaped)", "P_G", F, C, "data/d5_m3.json", 1 / 6,
        g=2 * F, Gs=F * (F + 1) + BASE_G(F), Ns=BASE_N(F), NLA=LA(3, F))
add("D5", "m=3 F=46, F4 on S4 (T3 cost at F=8 as lower bound)", "algebraic", "P_G", 46, d5[3]['C'], "screen.py (T3 measured) + data/d5_m3.json coverage",
    None, c=17592 * 2297, NLA=LA(3, 46))
add("D5", "same, I2 linear 2-torsion quotient: F4 cost / 100 (optimistic, derived)", "algebraic", "P_G", 46, d5[3]['C'],
    "derived from STRUCTURE-R4 I2 (D 6->5)", None, c=17592 * 2297 / 100, NLA=LA(3, 46))

# --- D7: lattice small-abscissa base ----------------------------------------------------------------
x = [t for t in d7['rows'] if t['X'] == 32][0]
add("D7", "small abscissa X=32, LLL oracle (measured recovery 0/40: no relations)", "algebraic", "P_G", x['base_points'],
    max(x['C_points'] * x['recovered'] / x['planted'], 1e-9), "data/d7_lattice.json", None, c=x['lll_ops_per_trial'])
add("D7", "same, IF every covered R were recovered (ideal lattice, X=32)", "algebraic", "P_G", x['base_points'], x['C_points'],
    "data/d7_lattice.json (coverage) + derived", 0.4, c=x['lll_ops_per_trial'])

# --- growth: D4 optimum re-solved at other r (coverage law and counted Karatsuba scale with r) ---------
def d4_best(rr, poly):
    srr = math.sqrt(rr); best = None
    Fs = range(4, 6000) if not poly else [int(2 ** (k / 4)) for k in range(12, 60)]
    for F in Fs:
        C = rr * (1 - math.exp(-2 * F * F / rr)); T = (F + 1) * rr / C
        if poly:
            n = int(C / 2); b = max(2, min(n, int(T))); c = multipoint_per_trial(n, b)
            G = F * (F + 1) + 3 * F + (F + 1) * 2 * F; N = tree(n) + 25 * F + 3 * F * F
        else:
            c = 0; G = F * (F + 1) + 3 * F; N = 25 * F + 3 * F * F
        v = (G + 0.1 * N + T * (1 + 0.1 * c)) / srr
        if best is None or v < best: best = v
    return best
growth = {}
for name, poly in (("D4 hash", False), ("D4 poly", True)):
    pts = [(rr, d4_best(rr, poly)) for rr in (2 ** 17, 2 ** 20, 2 ** 24, 2 ** 28)]
    growth[name] = dict(points=pts, exponent=(math.log(pts[-1][1]) - math.log(pts[0][1])) / (math.log(pts[-1][0]) - math.log(pts[0][0])))
    print(name, growth[name])
for row in rows:
    if row['design'] == 'D4':
        row['S_growth_exponent_in_r'] = growth['D4 poly' if 'polynomial' in row['variant'] else 'D4 hash']['exponent']
    if 'F4' in row['variant']:
        row['S_growth_exponent_in_r'] = 0.53  # README section 3: modelled F4 cost r^1.03 (L10)

for row in rows:
    print(f"{row['design']:6s} {row['pricing']:3s} {row['variant'][:70]:70s} F={row['F']:5d} C/r={row['C_over_r']:.3g} "
          f"gap {row['gap'][0]:9.3g} {row['gap'][0.1]:9.3g} {row['gap'][1]:9.3g}  (setup-free 0.1: {row['gap_setup_free'][0.1]:.3g})")
json.dump(dict(r=r, rho=RHO, poly_cost_exponent_in_d=e_poly, growth=growth, rows=rows), open(D / "data/table.json", "w"), indent=1, default=str)

if __name__ == "__main__":  # validate kara() against the counted Karatsuba in d2_coset.py
    import fp
    from d2_coset import pmul
    rng = random.Random(1)
    for n in (30, 100, 257):
        u = [rng.randrange(1, fp.p) for _ in range(n)]; v = [rng.randrange(1, fp.p) for _ in range(n)]
        fp.reset(); pmul(u, v); print("kara check", n, fp.CNT['M'], kara(n, n))
