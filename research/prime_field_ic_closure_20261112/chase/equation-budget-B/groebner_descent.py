#!/usr/bin/env python3
"""Groebner side of the descent toys (sympy over GF(p)).  [pcc:unverified]

G2: k=2, m=2. S3(x1,x2,X) with X = u+v*t, t^2=n.  Split into two GF(p) equations,
    lex Groebner basis, eliminant in x2: degree (= #solutions over closure, if radical
    & in shape position) vs #roots in GF(p) vs brute-force GF(p) solution count.
G3: k=3, m=3. S4 = Res_y(S3(x1,x2,y), S3(x3,y,X)), X in GF(p^3)\\GF(p), t^3 = c.
    Split into 3 GF(p) equations in 3 unknowns; Groebner basis; count GF(p) points.
E5: subspace base V = {x = s*t : s in GF(p)} in GF(p^2) (x not in GF(p)).
    Yield of m=2 decompositions per random target in E(GF(p^2)); base size.
"""
import sys, time, random, json, itertools
from sympy import symbols, groebner, Poly, resultant, expand, factor_list
sys.path.insert(0, '.')
from descent import Ext, Curve, O, count_base_points

x1, x2, x3, y, t = symbols('x1 x2 x3 y t')

def S3(a1, a2, a3, a, b):
    return (a1 - a2) ** 2 * a3 ** 2 - 2 * ((a1 + a2) * (a1 * a2 + a) + 2 * b) * a3 + (a1 * a2 - a) ** 2 - 4 * b * (a1 + a2)

def split(expr, modpoly, k, gens, p):
    P = Poly(expand(expr), t, *gens, modulus=p)
    R = P.rem(Poly(modpoly, t, *gens, modulus=p))
    comps = [0] * k
    for mon, c in R.terms():
        comps[mon[0]] += c * __import__('sympy').prod([g ** e for g, e in zip(gens, mon[1:])])
    return [expand(c) for c in comps]

def froots(poly_expr, var, p):
    P = Poly(poly_expr, var, modulus=p)
    return [r for r in range(p) if P.eval(r) % p == 0]

def main():
    rng = random.Random(7)
    out = {}
    # ---------------- G2 ----------------
    p = 101; a, b = 26, 64; n = 2   # same curve as descent.py p=101 run
    rows = []
    for trial in range(6):
        u, v = rng.randrange(p), rng.randrange(1, p)
        X = u + v * t
        eqs = split(S3(x1, x2, X, a, b), t ** 2 - n, 2, (x1, x2), p)
        t0 = time.time()
        G = groebner(eqs, x1, x2, order='lex', modulus=p)
        dt = time.time() - t0
        elim = [g for g in G.exprs if g.free_symbols <= {x2}]
        deg = Poly(elim[0], x2, modulus=p).degree() if elim else None
        r2 = froots(elim[0], x2, p) if elim else []
        brute = sum(1 for s1 in range(p) for s2 in range(p)
                    if all(Poly(e, x1, x2, modulus=p).eval({x1: s1, x2: s2}) % p == 0 for e in eqs)) if trial < 2 else None
        rows.append({"X": [u, v], "gb_len": len(G.exprs), "elim_deg_x2": deg, "Fp_roots_x2": r2,
                     "brute_Fp_solutions(x1,x2)": brute, "gb_sec": round(dt, 3)})
    out["G2_k2_m2_p101"] = rows
    print(json.dumps(out["G2_k2_m2_p101"]), flush=True)

    # ---------------- G3 ----------------
    p = 31
    # find c with t^3 - c irreducible: c not a cube, needs p = 1 mod 3 (31 = 1 mod 3)
    c = next(z for z in range(2, p) if pow(z, (p - 1) // 3, p) != 1)
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b ** 2) % p: break
    t0 = time.time()
    S4 = resultant(S3(x1, x2, y, a, b), S3(x3, y, t, a, b).subs(t, symbols('X')), y)
    Xs = symbols('X')
    S4p = Poly(S4, x1, x2, x3, Xs, modulus=p)
    out["G3_S4_build_sec"] = round(time.time() - t0, 2)
    out["G3_S4_total_degree"] = S4p.total_degree()
    out["G3_S4_degree_in_each_xi"] = [S4p.degree(v) for v in (x1, x2, x3)]
    print(json.dumps({k: out[k] for k in out if k.startswith('G3')}), flush=True)
    rows = []
    F = Ext(p, [(-c) % p, 0, 0]); E = Curve(F, a, b)
    for trial in range(3):
        u, v, w = rng.randrange(p), rng.randrange(1, p), rng.randrange(p)
        Xv = u + v * t + w * t ** 2
        eqs = split(S4p.as_expr().subs(Xs, Xv), t ** 3 - c, 3, (x1, x2, x3), p)
        eqs = [e for e in eqs if e != 0]
        # GF(p)-point count by brute force on the 3 descended equations
        Ps = [Poly(e, x1, x2, x3, modulus=p) for e in eqs]
        t0 = time.time()
        cnt = 0
        for s in itertools.product(range(p), repeat=3):
            if all(P.eval(s) % p == 0 for P in Ps): cnt += 1
        bt = time.time() - t0
        # Groebner (degrevlex) to get dimension info; may be slow
        t0 = time.time()
        try:
            G = groebner(eqs + [x1 ** p - x1, x2 ** p - x2, x3 ** p - x3], x1, x2, x3, order='grevlex', modulus=p)
            gb = [str(g) for g in G.exprs][:3]; gbs = round(time.time() - t0, 2)
        except Exception as ex:
            gb, gbs = f"error {ex}", None
        # is the target point on E with x = X? (only checks rhs is a square in GF(p^3))
        xX = (u, v, w); on = F.sqrt(E.rhs(xX)) is not None
        rows.append({"X": [u, v, w], "x_is_on_curve_over_Fp3": on, "Fp_solutions_brute": cnt,
                     "brute_sec": round(bt, 1), "GB_with_field_eqs": gb, "gb_sec": gbs})
        print(json.dumps(rows[-1]), flush=True)
    out["G3_k3_m3_p31"] = {"curve": [a, b], "c": c, "rows": rows}

    # ---------------- E5 subspace base ----------------
    e5 = []
    for p in (101, 211):
        n = next(z for z in range(2, p) if pow(z, (p - 1) // 2, p) == p - 1)
        while True:
            a, b = rng.randrange(p), rng.randrange(p)
            if (4 * a ** 3 + 27 * b ** 2) % p: break
        F = Ext(p, [(-n) % p, 0]); E = Curve(F, a, b)
        base = []
        for s in range(1, p):
            x = (0, s); yy = F.sqrt(E.rhs(x))
            if yy is None: continue
            base.append((x, yy)); base.append((x, F.neg(yy)))
        sums = {}
        for i, P in enumerate(base):
            for Q in base[i:]:
                S = E.add(P, Q)
                if S is not O: sums[S] = sums.get(S, 0) + 1
        NE = count_base_points(p, a, b); tr = p + 1 - NE
        NE2 = p * p + 1 - (tr * tr - 2 * p)
        hits = 0; trials = 400
        for _ in range(trials):
            while True:
                x = (rng.randrange(p), rng.randrange(p)); yy = F.sqrt(E.rhs(x))
                if yy is not None: break
            if (x, yy) in sums: hits += 1
        e5.append({"p": p, "#E(Fp)": NE, "#E(Fp2)": NE2, "|V-base|": len(base),
                   "distinct_2sums": len(sums), "m2_yield_per_target": hits / trials,
                   "relations_needed~|base|/2(neg-folded)": len(base) // 2,
                   "sqrt(#E(Fp))": round(NE ** 0.5, 2)})
        print(json.dumps(e5[-1]), flush=True)
    out["E5"] = e5
    json.dump(out, open('groebner_descent.json', 'w'), indent=1)

if __name__ == '__main__':
    main()
