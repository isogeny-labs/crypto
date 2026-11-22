#!/usr/bin/env python3
"""Exact checks of the curve facts and the desk lemmas of DESK.md, for one family miniature.

  python3 desk_checks.py --k 16 --out ../smoke/desk_k16.json

Every item is an exact computation (no sampling statistics); sampled items use a fixed seed.
"""
import argparse, json, random, sys, time
import numpy as np
import curves as cv

# ------------------------------------------------------------ F_{p^2} arithmetic (for E' halves)

class Fp2:
    def __init__(self, p):
        self.p = p
        nr = 2
        while cv.legendre(nr, p) != -1:
            nr += 1
        self.nr = nr                       # i^2 = nr

    def mul(self, a, b):
        p = self.p
        return ((a[0] * b[0] + self.nr * a[1] * b[1]) % p, (a[0] * b[1] + a[1] * b[0]) % p)

    def add(self, a, b):
        return ((a[0] + b[0]) % self.p, (a[1] + b[1]) % self.p)

    def sub(self, a, b):
        return ((a[0] - b[0]) % self.p, (a[1] - b[1]) % self.p)

    def inv(self, a):
        p = self.p
        nrm = (a[0] * a[0] - self.nr * a[1] * a[1]) % p
        ni = pow(nrm, -1, p)
        return (a[0] * ni % p, (-a[1]) * ni % p)

    def pow(self, a, e):
        r = (1, 0)
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r

    def sqrt(self, a):
        """Square root in F_{p^2} (p^2 = 1 mod 8 here): Tonelli-Shanks in the field of order q = p^2."""
        q = self.p * self.p
        if a == (0, 0):
            return (0, 0)
        if self.pow(a, (q - 1) // 2) != (1, 0):
            return None
        Q, S = q - 1, 0
        while Q % 2 == 0:
            Q //= 2; S += 1
        z = (0, 1)
        while self.pow(z, (q - 1) // 2) == (1, 0):
            z = (z[0] + 1, z[1])
        M, c, t, R = S, self.pow(z, Q), self.pow(a, Q), self.pow(a, (Q + 1) // 2)
        while t != (1, 0):
            i, tt = 0, t
            while tt != (1, 0):
                tt = self.mul(tt, tt); i += 1
            b = self.pow(c, 1 << (M - i - 1))
            M, c, t, R = i, self.mul(b, b), self.mul(t, self.mul(b, b)), self.mul(R, b)
        return R

    def frob(self, a):
        return (a[0], (-a[1]) % self.p)


def ec2_add(K, C, P, Q):
    """Addition on y^2 = x^3 + a2 x^2 + a4 x + a6 over F_{p^2}; points as pairs of Fp2 or None."""
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    a2, a4 = (C.a2, 0), (C.a4, 0)
    if x1 == x2:
        if K.add(y1, y2) == (0, 0):
            return None
        num = K.add(K.add(K.mul((3, 0), K.mul(x1, x1)), K.mul((2, 0), K.mul(a2, x1))), a4)
        lam = K.mul(num, K.inv(K.mul((2, 0), y1)))
    else:
        lam = K.mul(K.sub(y2, y1), K.inv(K.sub(x2, x1)))
    x3 = K.sub(K.sub(K.sub(K.mul(lam, lam), a2), x1), x2)
    y3 = K.sub(K.mul(lam, K.sub(x1, x3)), y1)
    return (x3, y3)


def half_over_fp2(K, C, T, es):
    """A point S in E'(F_{p^2}) with 2S = T, by the 2-descent halving formula
    x_S = x_T + r1 r2 + r1 r3 + r2 r3, r_i = sqrt(x_T - e_i) in F_{p^2}."""
    xT, yT = T
    rs = [K.sqrt(((xT - e) % K.p, 0)) for e in es]
    if any(r is None for r in rs):
        return None
    for s1 in (1, -1):
        for s2 in (1, -1):
            r1 = rs[0]
            r2 = K.mul(rs[1], (s1 % K.p, 0))
            r3 = K.mul(rs[2], (s2 % K.p, 0))
            xS = K.add((xT, 0), K.add(K.mul(r1, r2), K.add(K.mul(r1, r3), K.mul(r2, r3))))
            rhs = K.add(K.add(K.mul(K.mul(xS, xS), xS), K.mul((C.a2, 0), K.mul(xS, xS))), K.mul((C.a4, 0), xS))
            yS = K.sqrt(rhs)
            if yS is None:
                continue
            for sy in (1, -1):
                S = (xS, K.mul(yS, (sy % K.p, 0)))
                if ec2_add(K, C, S, S) == ((xT, 0), (yT, 0)):
                    return S
    return None

# ------------------------------------------------------------ helpers

def structure_of(G, vals, nsample=None):
    """Degree (fibre histogram) and invariance elements of a coordinate function given per point."""
    ok = np.nonzero(vals >= 0)[0]
    o = np.argsort(vals[ok], kind='stable')
    sv, si = vals[ok][o], ok[o]
    cuts = np.nonzero(np.diff(sv))[0] + 1
    groups = np.split(si, cuts)
    hist = np.bincount([len(g) for g in groups]).tolist()
    deg = max(len(g) for g in groups)
    trans, refl = None, None
    for g in groups:
        if len(g) != deg:
            continue
        P = g[0]
        tset = set(int(x) for x in G.subi(g, P))          # Q - P
        rset = set(int(x) for x in G.addi(g, P))          # Q + P
        trans = tset if trans is None else trans & tset
        refl = rset if refl is None else refl & rset
    return dict(fibre_size_hist=hist, degree=deg,
                translations=sorted(trans), reflection_sums=sorted(refl))


def describe(G, idx):
    k, e = (int(idx) % G.M, int(idx) // G.M)
    P = G.point(idx)
    return dict(idx=int(idx), k=k, eps=e, point=P, coset=[k % G.m2, e],
                two_torsion=bool(P is not None and P[1] == 0), in_2E=bool(e == 0 and k % 2 == 0))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--k', type=int, required=True)
    ap.add_argument('--samples', type=int, default=2000)
    ap.add_argument('--out', required=True)
    a = ap.parse_args()
    t0 = time.time()
    B = cv.build(a.k)
    p, A, n, r = B['p'], B['A'], B['n'], B['r']
    GE, GEp, E, Ep = B['GE'], B['GEp'], B['E'], B['Ep']
    rng = random.Random(20261122 + a.k)
    out = dict(k=a.k, p=p, A=A, n=n, r=r)
    fails = []

    def check(name, cond):
        out.setdefault('checks', {})[name] = bool(cond)
        if not cond:
            fails.append(name)

    # 1. curve facts --------------------------------------------------------
    check('p_prime', cv.is_prime(p))
    check('r_prime', cv.is_prime(r))
    check('n_eq_8r', n == 8 * r)
    check('p_mod_8_eq_5', p % 8 == 5)
    nE = cv.count_points(p, A, 1, 0)
    out['nE_charsum'] = nE
    check('nE_eq_n', nE == n)
    check('A2m4_nonsquare', cv.legendre(A * A - 4, p) == -1)
    check('E_cyclic_gen_order_8r', GE.M == 8 * r)   # Group() asserted a point of order M exists
    nEp = GEp.n
    out['nEp_charsum'] = nEp
    check('nEp_eq_nE', nEp == n)
    check('Ep_struct_Z2xZ4r', GEp.M == 4 * r and GEp.noncyclic)
    tw = 2 * (p + 1) - n
    out['twist_order'] = tw
    b2, b4, b6, v = B['velu']
    out['velu_EV'] = dict(a2=b2, a4=b4, a6=b6, v=v,
                          note='y^2 = x^3 + A x^2 - 4x - 4A = (x+A)(x-2)(x+2); X = x + A')
    check('velu_coeffs', (b2, b4, b6) == (A % p, (1 - 5) % p, (-4 * A) % p))
    # 2. the isogeny ------------------------------------------------------------
    ok_on, ok_hom, ok_sub = True, True, True
    G = GE.point(8)          # 8g generates E[r]
    phiG = cv.phi(G, p, A)
    check('phi_G_in_Ep_r', Ep.on(phiG) and Ep.mul(r, phiG) is None and phiG is not None)
    img = set()
    for _ in range(a.samples):
        P, Q = E.random_point(rng), E.random_point(rng)
        fP, fQ = cv.phi(P, p, A), cv.phi(Q, p, A)
        ok_on &= Ep.on(fP) and Ep.on(fQ)
        ok_hom &= cv.phi(E.add(P, Q), p, A) == Ep.add(fP, fQ)
        R = GE.point(8 * rng.randrange(1, r))
        iR = GEp.index_of(cv.phi(R, p, A))
        ok_sub &= bool(GEp.in_sub(iR))
        img.add((R, iR))
    check('phi_maps_E_to_Ep', ok_on)
    check('phi_homomorphism_samples', ok_hom)
    check('phi_Er_into_Epr', ok_sub)
    Rs = {}
    inj = True
    for R, iR in img:
        if iR in Rs and Rs[iR] != R:
            inj = False
        Rs[iR] = R
    check('phi_injective_on_Er_samples', inj)
    # 3. halving lemma on E -----------------------------------------------------
    lem_E, nof2 = True, True
    for _ in range(a.samples):
        idx = rng.randrange(1, GE.M)
        T = GE.point(idx)
        if T[1] == 0:
            continue
        has_half = idx % 2 == 0
        lem_E &= has_half == (cv.legendre(T[0], p) == 1)
        if not has_half:
            nof2 &= cv.legendre(T[0] * T[0] + A * T[0] + 1, p) == -1   # norm of x_T - e1
    check('E_half_iff_log_even_iff_xT_square', lem_E)
    check('E_odd_T_no_half_over_Fp2', nof2)
    out['E_T0'] = describe(GE, GE.M // 2)
    check('E_T0_in_2E', GE.M // 2 % 2 == 0)
    # 4. halving on E': classes and F_{p^2} halves ----------------------------------
    K = Fp2(p)
    es = [0, B['al'], B['be']]
    cls_e, lem_Ep, f2ok = {}, True, True
    for _ in range(min(a.samples, 400)):
        kk, ee = rng.randrange(GEp.M), rng.randrange(2)
        idx = int(GEp.enc(kk, ee))
        T = GEp.point(idx)
        if T is None:
            continue
        cls = (kk % 2, ee)
        allsq = all(cv.legendre(T[0] - e, p) >= 0 for e in es)
        lem_Ep &= (cls == (0, 0)) == allsq
        S = half_over_fp2(K, Ep, T, es)
        if S is None:
            f2ok = False
            continue
        rational = S[0][1] == 0 and S[1][1] == 0
        piS = (K.frob(S[0]), K.frob(S[1]))
        negS = (S[0], K.sub((0, 0), S[1]))
        e = ec2_add(K, Ep, piS, negS)
        if cls == (0, 0):
            f2ok &= e is None or rational or (e[1] == (0, 0))   # some half is rational (checked via class)
            continue
        f2ok &= (not rational) and e is not None and e[1] == (0, 0) and e[0][1] == 0
        if e is not None:
            cls_e.setdefault(str(cls), set()).add(e[0][0])
    check('Ep_half_iff_class00_iff_all_X_minus_e_square', lem_Ep)
    check('Ep_every_T_has_half_over_Fp2_with_pi_S_minus_S_in_Ep2', f2ok)
    out['Ep_class_to_e'] = {c: sorted(v) for c, v in cls_e.items()}
    check('Ep_class_to_e_injective', all(len(v) == 1 for v in cls_e.values())
          and len(set(min(v) for v in cls_e.values())) == len(cls_e))
    out['Ep_two_torsion'] = {str(e): describe(GEp, GEp.index_of((e, 0))) for e in es}
    # 5. coordinate functions: degree, invariance group ---------------------------------
    funcs = {}
    hT0 = np.full(GE.n, -1, dtype=np.int64)
    okm = (GE.X > 0)
    hT0[okm] = GE.Y[okm] * cv.vinv(GE.X[okm], p) % p
    funcs['E.x'] = (GE, cv.coord_x(GE))
    funcs['E.y_over_x'] = (GE, hT0)
    funcs['Ep.x'] = (GEp, cv.coord_x(GEp))
    for w in 'xy':
        funcs['Ep.huff_' + w] = (GEp, cv.coord_huff(B, w))
    for w in 'scd':
        funcs['Ep.ji_' + w] = (GEp, cv.coord_ji(B, w))
    H = cv.huff_params(B)
    out['huff_params'] = H
    # Huff curve equation and JI equations hold on every affine point
    Gp = GEp
    hx, hy = cv.coord_huff(B, 'x'), cv.coord_huff(B, 'y')
    m = (hx >= 0) & (hy >= 0)
    lhs = H['a'] * hx[m] % p * ((hy[m] * hy[m] - 1) % p) % p
    rhs = H['b'] * hy[m] % p * ((hx[m] * hx[m] - 1) % p) % p
    check('huff_equation_all_points', bool(np.all(lhs == rhs)))
    js, jc, jd = (cv.coord_ji(B, w) for w in 'scd')
    m = (js >= 0) & (jc >= 0) & (jd >= 0)
    e1 = (B['al'] * js[m] % p * js[m] + jc[m] * jc[m]) % p
    e2 = (B['be'] * js[m] % p * js[m] + jd[m] * jd[m]) % p
    check('ji_equations_all_points', bool(np.all(e1 == 1) and np.all(e2 == 1)))
    st = {}
    for name, (Gx, vals) in funcs.items():
        s = structure_of(Gx, vals)
        s['translations'] = [describe(Gx, i) for i in s['translations']]
        s['reflection_sums_T'] = [describe(Gx, i) for i in s['reflection_sums']]
        del s['reflection_sums']
        st[name] = s
    out['functions'] = st
    check('huff_degree_2', st['Ep.huff_x']['degree'] == 2 and st['Ep.huff_y']['degree'] == 2)
    check('ji_degree_4', all(st['Ep.ji_' + w]['degree'] == 4 for w in 'scd'))
    for w in 'xy':
        Ts = st['Ep.huff_' + w]['reflection_sums_T']
        check(f'huff_{w}_iota_T_with_T_2torsion_not_in_2E', len(Ts) == 1 and Ts[0]['two_torsion'] and not Ts[0]['in_2E'])
    # 6. translated-frame identity: h_T = M o x o tau_{-S} for T = 2S rational ------------------
    for (Gx, label) in ((GE, 'E'), (GEp, 'Ep')):
        allok = True
        for trial in range(5):
            while True:
                iS = rng.randrange(1, Gx.n)
                iT = int(Gx.addi(iS, iS))
                Tpt = Gx.point(iT)
                if Tpt is not None and Tpt[1] != 0:
                    break
            V = [rng.randrange(p) for _ in range(400)]
            base, nf, pairs = cv.slope_base(Gx, Tpt, V)
            Q = Gx.subi(base, iS)
            allok &= set(int(x) for x in Q) == set(int(x) for x in Gx.neg(Q))
            xs = [(int(Gx.X[int(Gx.subi(i, iS))]), lam) for i, lam in pairs if int(Gx.subi(i, iS)) != 0]
            Mf = fit_mobius(xs, p)
            allok &= Mf is not None and all(apply_mobius(Mf, x, p) == lam for x, lam in xs)
        check(f'{label}_translated_frame_identity', allok)
    # 7. JI s factors through the dual isogeny (invariant under translation by ker phi-hat = (0,0))
    T0p = GEp.index_of((0, 0))
    check('ji_s_invariant_under_tau_T0p', any(d['idx'] == T0p for d in st['Ep.ji_s']['translations']))
    out['elapsed_s'] = round(time.time() - t0, 2)
    out['failed'] = fails
    json.dump(out, open(a.out, 'w'), indent=1, default=str)
    print(f'k={a.k} desk checks: {len(out["checks"]) - len(fails)}/{len(out["checks"])} pass; failed={fails}; {out["elapsed_s"]}s')
    return 1 if fails else 0


def fit_mobius(xs, p):
    """Möbius M with M(x) = lam from >= 3 pairs with distinct x: solve a x + b - lam c x - lam d = 0."""
    seen, pts = set(), []
    for x, lam in xs:
        if x not in seen:
            seen.add(x); pts.append((x, lam))
        if len(pts) == 3:
            break
    if len(pts) < 3:
        return None
    rows = [[x % p, 1, (-lam * x) % p, (-lam) % p] for x, lam in pts]
    # nullspace of 3x4 matrix mod p
    m = [r[:] for r in rows]
    piv, row = [], 0
    for col in range(4):
        pr = next((i for i in range(row, 3) if m[i][col] % p), None)
        if pr is None:
            continue
        m[row], m[pr] = m[pr], m[row]
        iv = pow(m[row][col], -1, p)
        m[row] = [v * iv % p for v in m[row]]
        for i in range(3):
            if i != row and m[i][col]:
                f = m[i][col]
                m[i] = [(vi - f * vr) % p for vi, vr in zip(m[i], m[row])]
        piv.append(col); row += 1
        if row == 3:
            break
    free = [c for c in range(4) if c not in piv]
    if len(free) != 1:
        return None
    sol = [0] * 4
    sol[free[0]] = 1
    for i, c in enumerate(piv):
        sol[c] = (-m[i][free[0]]) % p
    a, b, c, d = sol
    if (a * d - b * c) % p == 0:
        return None
    return (a, b, c, d)


def apply_mobius(Mf, x, p):
    a, b, c, d = Mf
    den = (c * x + d) % p
    if den == 0:
        return None
    return (a * x + b) * pow(den, -1, p) % p


if __name__ == '__main__':
    sys.exit(main())
