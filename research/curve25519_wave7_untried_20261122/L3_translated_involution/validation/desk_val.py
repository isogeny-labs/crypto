#!/usr/bin/env python3
"""Validator's independent desk checks for wave-7 L3 (own code; imports only vcore + stdlib/numpy)."""
import json, random, sys, time
import numpy as np
from vcore import *

OUT = {}
CH = []


def check(name, ok, detail=''):
    CH.append(dict(name=name, ok=bool(ok), detail=detail))
    print(('PASS ' if ok else 'FAIL ') + name + ('  ' + str(detail) if detail else ''), flush=True)


def curve_facts(k, full=False):
    p, A, n, r, E = build(k)
    t0 = time.time()
    check(f'k{k}_p_prime', isprime(p))
    check(f'k{k}_r_prime', isprime(r) and n == 8 * r)
    nE = E.count()
    check(f'k{k}_nE_charsum', nE == n, nE)
    check(f'k{k}_A2m4_nonsquare', leg(A * A - 4, p) == -1)
    G = Group(E, n, r, False, labels=full)
    check(f'k{k}_E_cyclic_order_n_generator', G.C.mul(n, G.g) is None and G.C.mul(n // 2, G.g) is not None
          and G.C.mul(n // r, G.g) is not None, G.g)
    # Velu with kernel (0,0)
    EV, phi = velu2(E, 0)
    check(f'k{k}_velu_codomain', (EV.a2, EV.a4, EV.a6) == (A % p, (-4) % p, (-4 * A) % p),
          (EV.a2, EV.a4, EV.a6))
    # shift X = x + A : (x+A)(x-2)(x+2) -> X (X - (A+2)) (X - (A-2))
    al, be = (A + 2) % p, (A - 2) % p
    Ep = Curve(p, -(al + be), al * be, 0)
    rng = random.Random(99 + k)
    ok = True
    for _ in range(300):
        x = rng.randrange(p)
        if (EV.f(x) - Ep.f((x + A) % p)) % p:
            ok = False
    check(f'k{k}_shifted_model_identity', ok)
    nEp = Ep.count()
    check(f'k{k}_nEp_eq_nE', nEp == n, nEp)
    roots = [x for x in (0, al, be) if Ep.f(x) == 0]
    check(f'k{k}_Ep_full_rational_2torsion_and_v2_3', len(set(roots)) == 3 and n % 8 == 0 and (n // 8) % 2 == 1)
    Gp = Group(Ep, n, r, True, labels=full)
    check(f'k{k}_Ep_Z2xZ4r_generators', Ep.mul(n // 2, Gp.g) is None and Gp.h is not None)

    def phis(P):
        Q = phi(P)
        return None if Q is None else ((Q[0] + A) % p, Q[1])
    # homomorphism and E[r] -> E'[r]
    ok_h = ok_on = True
    for _ in range(400):
        P = G.pt(rng.randrange(1, n))
        Q = G.pt(rng.randrange(1, n))
        a, b, c = phis(P), phis(Q), phis(E.add(P, Q))
        ok_on &= Ep.on(a) and Ep.on(b)
        ok_h &= Ep.add(a, b) == c
    check(f'k{k}_phi_on_Ep_and_homomorphism_400', ok_on and ok_h)
    # phi on E[r]: E[r] = labels k = 0 mod 8 ; image in E'[r] (r * image = O), injective
    if full:
        idx = range(8, n, 8)
    else:
        idx = [8 * j for j in rng.sample(range(1, r), 1500)]
    imgs, okr = set(), True
    for L in idx:
        P = G.pt(L)
        assert E.mul(r, P) is None
        Q = phis(P)
        okr &= Q is not None and Ep.mul(r, Q) is None
        imgs.add(Q)
    check(f'k{k}_phi_Er_into_Epr_injective', okr and len(imgs) == len(list(idx)), f'{len(imgs)} images')
    OUT[f'k{k}_facts'] = dict(p=p, A=A, n=n, r=r, nE=nE, nEp=nEp, twist=2 * p + 2 - n, g=G.g, gp=Gp.g, hp=Gp.h,
                              s=round(time.time() - t0, 1))
    return p, A, n, r, E, G, Ep, Gp, phis


def main():
    for k in (18, 20):
        curve_facts(k)
    p, A, n, r, E, G, Ep, Gp, phis = curve_facts(16, full=True)
    al, be = (A + 2) % p, (A - 2) % p
    rng = random.Random(4242)

    # ---------------- Lemma 3 on E: 2E(F_p) by explicit doubling of every point
    twoE = set()
    for L in range(1, n):
        twoE.add(E.add(G.pt(L), G.pt(L)))
    allpts = [G.pt(L) for L in range(1, n)]
    ok = len(twoE) == n // 2   # includes O
    bad = 0
    for L in range(1, n):
        P = G.pt(L)
        in2 = P in twoE
        sq = leg(P[0], p) >= 0      # x = 0 is T0 = (0,0)
        if not (in2 == (L % 2 == 0) == sq):
            bad += 1
    check('E_2E_iff_log_even_iff_x_square_exhaustive', ok and bad == 0, f'|2E|={len(twoE)} mismatches={bad}')
    T0 = (0, 0)
    halves_T0 = [P for P in allpts if E.add(P, P) == T0]
    check('E_T0_has_rational_halves_x_pm1', len(halves_T0) == 2 and all(P[0] in (1, p - 1) for P in halves_T0),
          halves_T0)

    # ---------------- E' : 2E' and classes, Kummer map via explicit F_{p^2} halving
    twoEp = set(Ep.add(Gp.pt(L), Gp.pt(L)) for L in range(1, n))
    es = [0, al, be]

    def half_crit(P):
        x = P[0]
        vals = []
        for i, e in enumerate(es):
            d = (x - e) % p
            if d == 0:
                o = [es[j] for j in range(3) if j != i]
                d = (x - o[0]) * (x - o[1]) % p
            vals.append(leg(d, p) == 1)
        return all(vals)
    bad = 0
    for L in range(1, n):
        P = Gp.pt(L)
        kk, e = L % Gp.M, L // Gp.M
        cls00 = (kk % 2 == 0 and e == 0)
        if not ((P in twoEp) == cls00 == half_crit(P)):
            bad += 1
    check('Ep_2E_iff_class00_iff_X_minus_e_squares_exhaustive', len(twoEp) == n // 4 and bad == 0,
          f'|2Ep|={len(twoEp)} mismatches={bad}')
    e0 = [e for e in es if (e, 0) in twoEp]
    check('Ep_exactly_one_2torsion_in_2Ep', len(e0) == 1, f'e0={e0} (A+2={al}, A-2={be})')
    e0 = e0[0]
    OUT['Ep_e0'] = e0

    # F_{p^2} = F_p[i]/(i^2 - nr)
    nr = 2
    while leg(nr, p) != -1:
        nr += 1

    def fm(a, b):
        return ((a[0] * b[0] + nr * a[1] * b[1]) % p, (a[0] * b[1] + a[1] * b[0]) % p)

    def fa(a, b):
        return ((a[0] + b[0]) % p, (a[1] + b[1]) % p)

    def fs(a, b):
        return ((a[0] - b[0]) % p, (a[1] - b[1]) % p)

    def fpow(a, e):
        r_ = (1, 0)
        while e:
            if e & 1:
                r_ = fm(r_, a)
            a = fm(a, a)
            e >>= 1
        return r_

    def finv(a):
        nn = (a[0] * a[0] - nr * a[1] * a[1]) % p
        iv = pow(nn, -1, p)
        return (a[0] * iv % p, (-a[1]) * iv % p)

    def fsqrt(a):
        if a == (0, 0):
            return (0, 0)
        q, s = p * p - 1, 0
        while q % 2 == 0:
            q //= 2
            s += 1
        if fpow(a, (p * p - 1) // 2) != (1, 0):
            return None
        z = (1, 1)
        while fpow(z, (p * p - 1) // 2) == (1, 0):
            z = (z[0] + 1, 1)
        c = fpow(z, q)
        x = fpow(a, (q + 1) // 2)
        t = fpow(a, q)
        m = s
        while t != (1, 0):
            i, tt = 0, t
            while tt != (1, 0):
                tt = fm(tt, tt)
                i += 1
            b = fpow(c, 1 << (m - i - 1))
            x = fm(x, b)
            c = fm(b, b)
            t = fm(t, c)
            m = i
        assert fm(x, x) == a
        return x

    def f2add(C, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if fa(y1, y2) == (0, 0):
                return None
            num = fa(fa(fm((3, 0), fm(x1, x1)), fm((2 * C.a2, 0), x1)), (C.a4, 0))
            l = fm(num, finv(fm((2, 0), y1)))
        else:
            l = fm(fs(y2, y1), finv(fs(x2, x1)))
        x3 = fs(fs(fs(fm(l, l), (C.a2, 0)), x1), x2)
        return (x3, fs(fm(l, fs(x1, x3)), y1))

    def f2rhs(C, x):
        return fa(fa(fa(fm(fm(x, x), x), fm((C.a2, 0), fm(x, x))), fm((C.a4, 0), x)), (C.a6, 0))

    def conj(a):
        return (a[0], (-a[1]) % p)

    def half_fp2(C, T, roots3):
        """Halve T on y^2 = prod (x - e_i) over F_{p^2} via x_S = x_T + sum_{i<j} r_i r_j with r_i^2 = x_T - e_i."""
        xT, yT = T
        rs = [fsqrt(((xT - e) % p, 0)) for e in roots3]
        if any(v is None for v in rs):
            return None
        # sign choice: r1 r2 r3 = -yT or yT; try sign patterns, verify by doubling
        for s1 in (1, -1):
            for s2 in (1, -1):
                r1 = rs[0] if s1 == 1 else fs((0, 0), rs[0])
                r2 = rs[1] if s2 == 1 else fs((0, 0), rs[1])
                r3 = rs[2]
                xS = fa(fa(fa((xT % p, 0), fm(r1, r2)), fm(r1, r3)), fm(r2, r3))
                yy = fsqrt(f2rhs(C, xS))
                if yy is None:
                    continue
                for y in (yy, fs((0, 0), yy)):
                    S = (xS, y)
                    if f2add(C, S, S) == ((xT % p, 0), (yT % p, 0)):
                        return S
        return None
    kum = {}
    ok_all = True
    for L in [rng.randrange(1, n) for _ in range(400)]:
        P = Gp.pt(L)
        if P[1] == 0:
            continue
        S = half_fp2(Ep, P, es)
        if S is None:
            ok_all = False
            continue
        piS = (conj(S[0]), conj(S[1]))
        dlt = f2add(Ep, piS, (S[0], fs((0, 0), S[1])))
        assert dlt is None or (dlt[0][1] == 0 and dlt[1] == (0, 0))
        w = None if dlt is None else dlt[0][0]
        kk, e = L % Gp.M, L // Gp.M
        cls = (kk % 2, e)
        # canonical class labels: (0,1) = class of the 2-torsion points outside 2E'
        kum.setdefault(str(cls), set()).add(w)
    kum = {c: sorted(str(v) for v in s) for c, s in kum.items()}
    single = all(len(v) == 1 for v in kum.values()) and len(set(v[0] for v in kum.values())) == 4
    check('Ep_every_T_has_half_over_Fp2_and_Kummer_map_bijective', ok_all and single, kum)
    check('Ep_class01_maps_to_e0_=_(A+2,0)', kum.get('(0, 1)') == [str(e0)] and e0 == al, f'class01->{kum.get("(0, 1)")}, e0={e0}, A+2={al}')
    OUT['kummer_own_labels'] = kum

    # ---------------- E: odd-log T has no half over F_{p^2}; halves over F_{p^4}
    # Montgomery doubling x(2S) = (x^2-1)^2 / (4 x (x^2 + A x + 1)); halving quartic H(x) = (x^2-1)^2 - 4 xT x (x^2+Ax+1)
    def pmod(a, m):
        a = a[:]
        while len(a) >= len(m):
            if a[-1] % p:
                c = a[-1] * pow(m[-1], -1, p) % p
                sh = len(a) - len(m)
                for i in range(len(m)):
                    a[sh + i] = (a[sh + i] - c * m[i]) % p
            a.pop()
        while a and a[-1] % p == 0:
            a.pop()
        return a

    def pmul(a, b, m):
        if not a or not b:
            return []
        r_ = [0] * (len(a) + len(b) - 1)
        for i, x in enumerate(a):
            for j, y in enumerate(b):
                r_[i + j] = (r_[i + j] + x * y) % p
        return pmod(r_, m)

    def ppow(a, e, m):
        r_ = [1]
        while e:
            if e & 1:
                r_ = pmul(r_, a, m)
            a = pmul(a, a, m)
            e >>= 1
        return r_

    def pgcd(a, b):
        while b:
            a, b = b, pmod(a, b)
        return a

    def psub(a, b):
        n_ = max(len(a), len(b))
        r_ = [((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n_)]
        while r_ and r_[-1] == 0:
            r_.pop()
        return r_
    stats = dict(odd=dict(n=0, irreducible=0, no_root_Fp2=0, half_over_Fp4=0),
                 even=dict(n=0, rational_half=0))
    for _ in range(300):
        L = rng.randrange(1, n)
        P = G.pt(L)
        if P[1] == 0:
            continue
        xT = P[0]
        # H = x^4 - 2x^2 + 1 - 4 xT (x^3 + A x^2 + x)
        H = [1, (-4 * xT) % p, (-2 - 4 * xT * A) % p, (-4 * xT) % p, 1]
        xp = ppow([0, 1], p, H)
        xp2 = ppow([0, 1], p * p, H)
        g1 = pgcd(H, psub(xp, [0, 1]))
        g2 = pgcd(H, psub(xp2, [0, 1]))
        if L % 2 == 1:
            st = stats['odd']
            st['n'] += 1
            st['no_root_Fp2'] += (len(g2) - 1 == 0)
            # irreducible: no roots in F_p, F_p2 (no linear or quadratic factors) => irreducible quartic
            st['irreducible'] += (len(g1) - 1 == 0 and len(g2) - 1 == 0)
            if len(g2) - 1 == 0:
                rhs = [0, 1, A % p, 1]          # x + A x^2 + x^3 as poly in x mod H
                rhs = pmod(rhs, H)
                st['half_over_Fp4'] += (ppow(rhs, (p ** 4 - 1) // 2, H) == [1])
        else:
            st = stats['even']
            st['n'] += 1
            # rational half: a rational root x_S of H with f(x_S) a square
            xs_ = np.arange(p, dtype=np.int64)
            hv = np.zeros(p, dtype=np.int64)
            for c_ in reversed(H):
                hv = (hv * xs_ + c_) % p
            roots = [int(x) for x in np.nonzero(hv == 0)[0]]
            st['rational_half'] += any(leg(E.f(x), p) >= 0 for x in roots)
    so, se = stats['odd'], stats['even']
    check('E_odd_T_halving_quartic_irreducible_no_half_over_Fp2', so['irreducible'] == so['n'] == so['no_root_Fp2'], so)
    check('E_odd_T_halves_defined_over_Fp4', so['half_over_Fp4'] == so['n'], so)
    check('E_even_T_has_rational_half', se['rational_half'] == se['n'], se)

    # ---------------- every degree-2 function is iota_T invariant (fibre-sum test over all F_p values)
    lab = G.label

    def fibre_test(vals, Texp):
        """vals: dict label -> value in F_p (poles absent). Fibres must have <= 2 points, constant sum T."""
        fib = {}
        for L, v in vals.items():
            fib.setdefault(v, []).append(L)
        sizes = np.bincount([len(f) for f in fib.values()])
        sums = set()
        ones = []
        for f in fib.values():
            if len(f) == 2:
                sums.add(int(G.add(f[0], f[1])))
            elif len(f) == 1:
                ones.append(f[0])
        T = sums.pop() if len(sums) == 1 else None
        okT = T is not None and T == Texp and len(sums) == 0
        # 1-point fibres are exactly the rational halves of T
        LL = np.arange(n)
        halves = np.nonzero(G.add(LL, LL) == (T if T is not None else -1))[0].tolist()
        okh = sorted(ones) == sorted(halves)
        return okT and max(len(f) for f in fib.values()) <= 2 and okh, dict(sizes=sizes.tolist(), T=T, Texp=Texp,
                                                                         n1=len(ones), nhalves=len(halves))

    def vals_of(func):
        out = {}
        for L in range(0, n):
            P = G.pt(L)
            v = func(P)
            if v is not None:
                out[L] = v
        return out

    def compose(F, func):
        a_, b_, c_, d_ = F
        inf_img = a_ * pow(c_, -1, p) % p

        def g(P):
            v = func(P)
            if v is None:
                return inf_img          # the polar fibre goes to mu(inf)
            return mobius(F, v, p)      # None only at v = mu^{-1}(inf)
        return g

    def x_fn(P):
        return None if P is None else P[0]

    def hQ(Q):
        xQ, yQ = Q

        def h(P):
            if P is None or P == Q:
                return None
            if P == E.neg(Q):
                if yQ == 0:
                    return None
                return (3 * xQ * xQ + 2 * A * xQ + 1) * pow((-2 * yQ) % p, -1, p) % p
            if P[0] == xQ:
                return None
            return (P[1] + yQ) * pow((P[0] - xQ) % p, -1, p) % p
        return h

    def line_fn(al_, be_):
        """f = (y - al x - be)/q(x), q = cubic/(x - x_R0), R0 a rational intersection; T expected = R0."""
        xs = np.arange(p, dtype=np.int64)
        cub = (E.f(xs) - (al_ * xs + be_) ** 2) % p
        rts = [int(x) for x in np.nonzero(cub == 0)[0]]
        if not rts:
            return None, None
        x0 = rts[0]
        R0 = (x0, (al_ * x0 + be_) % p)
        # q(x) = cubic / (x - x0): cubic = x^3 + (A - al^2) x^2 + (1 - 2 al be) x - be^2
        c2, c1 = (A - al_ * al_) % p, (1 - 2 * al_ * be_) % p
        q2, q1 = 1, (c2 + x0) % p
        q0 = (c1 + x0 * q1) % p

        def f(P):
            if P is None:
                return 0
            x, y = P
            d = (x * x + q1 * x + q0) % p
            if d == 0:
                d2 = (y + al_ * x + be_) % p      # f = (x - x0)/(y + al x + be) away from its zeros
                if d2 == 0:
                    return None                   # pole (P1 or P2)
                return (x - x0) * pow(d2, -1, p) % p
            return (y - al_ * x - be_) * pow(d, -1, p) % p
        return f, R0
    results = []
    # T = O : x and Moebius of x
    for _ in range(2):
        F = rand_mob(rng, p)
        ok, d = fibre_test(vals_of(compose(F, x_fn)), 0)
        results.append(('mobius_x', ok, d))
    # D = O + Q : h_Q for random Q (incl. Q = T0)
    for Q in [G.pt(rng.randrange(1, n)) for _ in range(3)] + [T0]:
        F = rand_mob(rng, p)
        ok, d = fibre_test(vals_of(compose(F, hQ(Q))), lab(Q))
        results.append(('h_Q', ok, d))
    # D = P1 + P2 (rational, conjugate, or 2P) via lines
    kinds = {}
    tries = 0
    while len(results) < 18 and tries < 200:
        tries += 1
        a_, b_ = rng.randrange(p), rng.randrange(p)
        f, R0 = line_fn(a_, b_)
        if f is None:
            continue
        xs = np.arange(p, dtype=np.int64)
        nroots = int(((E.f(xs) - (a_ * xs + b_) ** 2) % p == 0).sum())
        kind = {1: 'conjugate_pair', 3: 'rational_pair', 2: 'double'}[nroots]
        if kinds.get(kind, 0) >= 5:
            continue
        kinds[kind] = kinds.get(kind, 0) + 1
        F = rand_mob(rng, p)
        ok, d = fibre_test(vals_of(compose(F, f)), lab(R0))
        results.append(('line_' + kind, ok, d))
    good = all(ok for _, ok, _ in results)
    check('every_degree2_function_constant_fibre_sum_T_rational_and_halves_are_1point_fibres',
          good, [(k_, ok, d['T'], d['Texp'], d['n1'], d['nhalves']) for k_, ok, d in results])
    OUT['degree2_kinds'] = kinds

    # ---------------- Theorem 1: rational half -> translated x-frame, relation table
    Lr = list(range(n))
    xs_lab = np.array([-1] + [G.pt(L)[0] for L in range(1, n)], dtype=np.int64)
    ok1 = True
    det1 = []
    for trial in range(6):
        while True:
            Ls = rng.randrange(1, n)
            LT = int(G.add(Ls, Ls))
            if LT != 0 and G.pt(LT)[1] != 0:
                break
        S, T = G.pt(Ls), G.pt(LT)
        F = rand_mob(rng, p)
        h = compose(F, hQ(T))
        # fit mu with h(P) = mu(x(P - S)) on 3 points, verify on all points
        pairs, allp = [], []
        for L in Lr:
            P = G.pt(L)
            v = h(P)
            Q = E.sub(P, S)
            if v is None or Q is None:
                continue
            allp.append((Q[0], v))
        seen, pr = set(), []
        for x, v in allp:
            if x not in seen:
                seen.add(x)
                pr.append((x, v))
            if len(pr) == 3:
                break
        mu = fit_mobius(pr, p)
        okfit = mu is not None and all(mobius(mu, x, p) == v for x, v in allp)
        # base identity and relation sets on random V
        Fv = rand_mob(rng, p)
        V = set(v for v in (mobius(Fv, y, p) for y in range(64)) if v is not None)
        Bf = sorted(L for L in Lr if h(G.pt(L)) in V)
        mui = mobius_inv(mu, p)
        Vx = set()
        withO = False
        for v in V:
            u = mobius(mui, v, p)
            if u is None:
                withO = True
            else:
                Vx.add(u)
        Bx = [L for L in range(1, n) if xs_lab[L] in Vx] + ([0] if withO else [])
        okbase = sorted(int(G.add(Ls, L)) for L in Bx) == Bf
        negc = set(int(G.sub(L, Ls)) for L in Bf) == set(int(G.neg(G.sub(L, Ls))) for L in Bf)
        Bfa, Bxa = np.array(Bf), np.array(Bx)
        Df = set(G.sub(Bfa[:, None], Bfa[None, :]).ravel().tolist())
        Sx = set(G.sub(Bxa[:, None], Bxa[None, :]).ravel().tolist()) | set(G.add(Bxa[:, None], Bxa[None, :]).ravel().tolist())
        sgn = set()
        for s1 in (1, -1):
            for s2 in (1, -1):
                A1 = Bfa if s1 == 1 else G.neg(Bfa)
                A2 = Bfa if s2 == 1 else G.neg(Bfa)
                sgn |= set(G.add(A1[:, None], A2[None, :]).ravel().tolist())
        union = Sx | set(int(v) for v in G.add(np.array(sorted(Sx)), LT)) | set(int(v) for v in G.sub(np.array(sorted(Sx)), LT))
        okrel = (Df == Sx) and (sgn == union)
        # same-sign relation shift: P_i + P_j = T + (Q_i + Q_j) pointwise on all pairs (group law, not labels)
        okpt = True
        for _ in range(50):
            i, j = rng.choice(Bf), rng.choice(Bf)
            Pi, Pj = G.pt(i), G.pt(j)
            Qi, Qj = E.sub(Pi, S), E.sub(Pj, S)
            okpt &= E.add(Pi, Pj) == E.add(T, E.add(Qi, Qj)) and E.sub(Pi, Pj) == E.sub(Qi, Qj)
        ok1 &= okfit and okbase and negc and okrel and okpt
        det1.append(dict(fit=okfit, base=okbase, negclosed=negc, sets=okrel, pointwise=okpt, nB=len(Bf)))
    check('Theorem1_translated_xframe_base_and_relation_sets', ok1, det1)

    # ---------------- Lemma 2 for odd T, and Theorem 4 (no within-fibre E[r] differences)
    ok2, det2 = True, []
    for trial in range(6):
        while True:
            LT = rng.randrange(1, n)
            if LT % 2 == 1:
                break
        T = G.pt(LT)
        F = rand_mob(rng, p)
        h = compose(F, hQ(T))
        Fv = rand_mob(rng, p)
        V = set(v for v in (mobius(Fv, y, p) for y in range(64)) if v is not None)
        Bf = np.array(sorted(L for L in Lr if h(G.pt(L)) in V))
        closed = set(Bf.tolist()) == set(int(v) for v in G.sub(LT, Bf))
        D = set(G.sub(Bf[:, None], Bf[None, :]).ravel().tolist())
        sgn = set()
        for s1 in (1, -1):
            for s2 in (1, -1):
                A1 = Bf if s1 == 1 else G.neg(Bf)
                A2 = Bf if s2 == 1 else G.neg(Bf)
                sgn |= set(G.add(A1[:, None], A2[None, :]).ravel().tolist())
        Da = np.array(sorted(D))
        union = D | set(G.add(Da, LT).tolist()) | set(G.sub(Da, LT).tolist())
        ok2 &= closed and sgn == union
        det2.append(dict(closed=closed, signed_eq_union=sgn == union, nB=len(Bf)))
    check('Lemma2_B_eq_T_minus_B_and_signed_set_union_oddT', ok2, det2)
    # Theorem 4 over all points a, for 40 odd T (labels) + pointwise group-law spot checks; even T contrast
    Ln = np.arange(1, n)
    cnt_odd, cnt_even = 0, []
    for _ in range(40):
        LT = 2 * rng.randrange(n // 2) + 1
        d = G.sub(G.add(Ln, Ln), LT)
        cnt_odd += int((G.insub(d) & (d != 0)).sum())
        LT2 = 2 * rng.randrange(1, n // 2)
        d2 = G.sub(G.add(Ln, Ln), LT2)
        cnt_even.append(int((G.insub(d2) & (d2 != 0)).sum()))
    spot = True
    for _ in range(300):
        LT = 2 * rng.randrange(n // 2) + 1
        a = G.pt(rng.randrange(1, n))
        T = G.pt(LT)
        R = E.sub(E.add(a, a), T)
        spot &= not (R is not None and E.mul(r, R) is None)
    check('Theorem4_no_within_fibre_Er_difference_for_odd_T', cnt_odd == 0 and spot and min(cnt_even) > 0,
          f'odd: {cnt_odd} hits over 40 T x all a; even T hits per T: {sorted(set(cnt_even))}')
    # E': within-fibre for no-half classes, and x-frame self-pair cosets (4 of 8 on E', 2 of 8 on E)
    Lnp = np.arange(1, n)
    hits = {}
    for cls in ((0, 1), (1, 0), (1, 1), (0, 0)):
        tot = 0
        for _ in range(10):
            LT = int(Gp.mk(2 * rng.randrange(Gp.M // 2) + cls[0], cls[1]))
            d = Gp.sub(Gp.add(Lnp, Lnp), LT)
            tot += int((Gp.insub(d) & (d != 0)).sum())
        hits[str(cls)] = tot
    selfE = int((G.insub(G.add(Ln, Ln)) & (G.add(Ln, Ln) != 0)).sum()) / (n - 1)
    selfEp = int((Gp.insub(Gp.add(Lnp, Lnp)) & (Gp.add(Lnp, Lnp) != 0)).sum()) / (n - 1)
    check('Theorem4_Ep_no_half_classes_and_selfpair_fractions',
          hits['(0, 1)'] == hits['(1, 0)'] == hits['(1, 1)'] == 0 and hits['(0, 0)'] > 0
          and abs(selfE - 0.25) < 0.001 and abs(selfEp - 0.5) < 0.001,
          dict(hits=hits, selfpair_frac_E=round(selfE, 4), selfpair_frac_Ep=round(selfEp, 4)))

    # ---------------- Huff model on E'
    others = [e for e in es if e != e0]
    okh = None
    for ea, eb in ((others[0], others[1]), (others[1], others[0])):
        a2_, b2_ = (e0 - ea) % p, (e0 - eb) % p
        a_, b_ = sqrtp(a2_, p), sqrtp(b2_, p)
        if a_ is None or b_ is None:
            continue
        okh = (ea, eb, a_, b_)
        break
    ea, eb, ha, hb = okh

    def huff(P, which):
        if P is None:
            return 0                        # zero at O (excluded from bases)
        X, Y = P
        u = (X - e0) % p
        if Y == 0:
            if which == 'x':
                return 0 if u == (-ha * ha) % p else None
            return 0 if u == (-hb * hb) % p else None
        iv = pow(Y, -1, p)
        if which == 'x':
            return hb * (u + ha * ha) % p * iv % p
        return ha * (u + hb * hb) % p * iv % p
    okeq = True
    for L in range(1, n):
        P = Gp.pt(L)
        if P[1] == 0:
            continue
        x, y = huff(P, 'x'), huff(P, 'y')
        okeq &= (ha * x * (y * y - 1) - hb * y * (x * x - 1)) % p == 0
    check('Huff_equation_all_points', okeq, dict(e0=e0, ea=ea, eb=eb, a=ha, b=hb))
    for which in ('x', 'y'):
        fib = {}
        for L in range(1, n):
            v = huff(Gp.pt(L), which)
            if v is not None:
                fib.setdefault(v, []).append(L)
        sizes = np.bincount([len(f) for f in fib.values()]).tolist()
        sums = set(int(Gp.add(f[0], f[1])) for f in fib.values() if len(f) == 2)
        Tl = sums.pop() if len(sums) == 1 else None
        TP = Gp.pt(Tl) if Tl is not None else None
        cls = None if Tl is None else (int(Tl % Gp.M) % 2, int(Tl // Gp.M))
        exp_T = (ea if which == 'x' else eb, 0)
        ok = (Tl is not None and not sums and max(len(f) for f in fib.values()) <= 2 and TP == exp_T
              and TP[1] == 0 and TP not in twoEp and cls == (0, 1))
        check(f'Huff_{which}_degree2_iotaT_T_2torsion_outside_2Ep_class01', ok,
              dict(fibre_sizes=sizes, T=TP, expected=exp_T, cls=cls))

    # ---------------- Jacobi intersection on E'
    def ji(P, which):
        if P is None:
            return None
        X, Y = P
        den = (X * X - al * be) % p
        if den == 0:
            return None
        iv = pow(den, -1, p)
        if which == 's':
            return 2 * Y * iv % p
        if which == 'c':
            return (X * X - 2 * al * X + al * be) * iv % p
        return (X * X - 2 * be * X + al * be) * iv % p
    okj = True
    for L in range(1, n):
        P = Gp.pt(L)
        s, c, d = ji(P, 's'), ji(P, 'c'), ji(P, 'd')
        if s is None:
            continue
        okj &= (al * s * s + c * c - 1) % p == 0 and (be * s * s + d * d - 1) % p == 0
    check('JI_equations_all_points', okj)
    E2 = {e: Gp.label((e, 0)) for e in es}
    Lall = np.arange(1, n)
    for which, grp in (('c', [('neg', None), ('tau', be), ('iota', be)]),
                       ('d', [('neg', None), ('tau', al), ('iota', al)]),
                       ('s', [('tau', 0), ('iota', al), ('iota', be)])):
        vals = np.array([-1 if ji(Gp.pt(L), which) is None else ji(Gp.pt(L), which) for L in range(n)])
        fib = np.bincount(np.bincount(vals[vals >= 0], minlength=p)).tolist()
        okg = True
        for op, e in grp:
            if op == 'neg':
                img = Gp.neg(Lall)
            elif op == 'tau':
                img = Gp.add(Lall, E2[e])
            else:
                img = Gp.sub(E2[e], Lall)
            m = (vals[Lall] >= 0) & (vals[img] >= 0)
            okg &= bool(np.all(vals[Lall][m] == vals[img][m]))
        # not invariant under negation for s; not under other translations
        extra = ''
        if which == 's':
            img = Gp.neg(Lall)
            m = (vals[Lall] >= 0) & (vals[img] >= 0)
            extra = f'neg-invariant={bool(np.all(vals[Lall][m] == vals[img][m]))}'
        n4 = fib[4] if len(fib) > 4 else 0
        check(f'JI_{which}_degree4_and_Klein_invariance', okg and n4 > 0 and len(fib) == 5,
              f'fibre-size histogram {fib}; {extra}')

    # pullbacks: c = Moebius o x o phi_{e_beta}, d = Moebius o x o phi_{e_alpha}; s = g o phihat, phihat kernel (0,0)
    for which, e in (('c', be), ('d', al), ('s', 0)):
        C2, ph = velu2(Ep, e)
        allv, pr, seen = [], [], set()
        for L in range(1, n):
            P = Gp.pt(L)
            v = ji(P, which)
            Q = ph(P)
            if v is None or Q is None:
                continue
            allv.append((Q, v))
        okon = all(C2.on(Q) for Q, _ in allv[:500])
        if which in ('c', 'd'):
            for Q, v in allv:
                if Q[0] not in seen:
                    seen.add(Q[0])
                    pr.append((Q[0], v))
                if len(pr) == 3:
                    break
            mu = fit_mobius(pr, p)
            ok = okon and mu is not None and all(mobius(mu, Q[0], p) == v for Q, v in allv)
            check(f'JI_{which}_is_mobius_of_x_on_Ep/<e>_pulled_back', ok, f'kernel X={e}')
        else:
            # g(Q) well defined on the codomain, degree 2, g = mu o x o tau_{-S}, S rational of order 4, 2S = T_g
            g = {}
            okwd = True
            for Q, v in allv:
                if Q in g and g[Q] != v:
                    okwd = False
                g[Q] = v
            # codomain group: points of C2 (scan); fibre sums of g
            cpts = C2.points()
            nC = len(cpts) + 1
            fibg = {}
            for Q, v in g.items():
                fibg.setdefault(v, []).append(Q)
            sizes = np.bincount([len(f) for f in fibg.values()]).tolist()
            sums = set()
            for f in fibg.values():
                if len(f) == 2:
                    sums.add(C2.add(f[0], f[1]))
            Tg = sums.pop() if len(sums) == 1 else 'many'
            okT = Tg != 'many' and Tg is not None and Tg[1] == 0
            # rational halves of Tg on C2
            hs = [Q for Q in cpts if C2.add(Q, Q) == Tg] if okT else []
            okmu = False
            if hs:
                S = hs[0]
                allp, seen2, pr2 = [], set(), []
                for Q, v in g.items():
                    R = C2.sub(Q, S)
                    if R is None:
                        continue
                    allp.append((R[0], v))
                for x, v in allp:
                    if x not in seen2:
                        seen2.add(x)
                        pr2.append((x, v))
                    if len(pr2) == 3:
                        break
                mu = fit_mobius(pr2, p)
                okmu = mu is not None and all(mobius(mu, x, p) == v for x, v in allp)
            # is codomain isomorphic to E? j-invariant comparison
            def jinv(C):
                a2, a4, a6 = C.a2, C.a4, C.a6
                b2, b4, b6 = 4 * a2 % p, 2 * a4 % p, 4 * a6 % p
                b8 = (a2 * 4 * a6 - a4 * a4) % p  # a1=a3=0: b8 = 4 a2 a6 - a4^2
                c4 = (b2 * b2 - 24 * b4) % p
                disc = (-b2 * b2 * b8 - 8 * b4 ** 3 - 27 * b6 * b6 + 9 * b2 * b4 * b6) % p
                return c4 ** 3 * pow(disc, -1, p) % p
            isoE = jinv(C2) == jinv(E)
            ok = okon and okwd and okT and len(hs) == 2 and okmu and nC == n and isoE
            check('JI_s_is_translated_xframe_on_E_pulled_back_by_dual_isogeny', ok,
                  dict(well_defined=okwd, g_fibre_sizes=sizes, Tg=Tg, rational_halves=hs, mobius_fit=okmu,
                       codomain_order=nC, j_equal_E=isoE))
    OUT['checks'] = CH
    OUT['n_pass'] = sum(c['ok'] for c in CH)
    OUT['n_total'] = len(CH)
    json.dump(OUT, open(sys.argv[1], 'w'), indent=1, default=str)
    print(f'{OUT["n_pass"]}/{OUT["n_total"]} pass')


if __name__ == '__main__':
    main()
