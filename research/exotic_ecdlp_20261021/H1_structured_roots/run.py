# H1 -- symmetrized Semaev S_3 small roots over F_p (conjecture C1), plain-Python Sage program.
# Executed on isolab. Pre-registration: PROTOCOL.md in this directory is the authority.
#
# What is measured
#   * curve E/F_p, p = next_prime(2^bits), with full rational 2-torsion and a rational 4-torsion point
#   * f(X1,X2) = S_3(X1,X2,x(R)) (raw, bidegree (2,2), total degree 4)
#   * g(e1,e2): f rewritten in e1=X1+X2, e2=X1*X2 (swap symmetrization, total degree 2)
#   * h(s1,s2): FGHR E[2]-translation invariants xi(P)=sum_T x(P+T), then swap symmetrization
#   * REACH: largest beta such that a hand-built modular Coppersmith lattice (x^i y^j f^k p^(m-k))
#     recovers a planted true root with x_i in [1, p^beta]  (raw: box (B,B); sym: box (2B, B^2))
#   * COUNT CONTROL (two smallest p): exhaustive count of box-ceil(p^(1/3)) 3-decompositions of random R
from sage.all import *

import os, sys, time, json, math, traceback, random, statistics
from itertools import combinations

OUT = os.environ.get('ISOLAB_OUTPUT_DIR', '.')
SEED = 20261022
BITS = [16, 20, 24, 28]
BETA_LO, BETA_HI, BETA_STEP = 0.05, 0.40, 0.0125
TRIALS = 3            # planted instances per beta level (level passes if >= 2 recovered)
NMAX = 180            # max lattice dimension
MMAX = 9              # max multiplicity
MIN_MARGIN = -2.5     # predicted-margin floor (bits) for attempting a lattice
CELL_BUDGET = 640.0   # seconds per cell
GLOBAL_BUDGET = 2850.0
HGLOG = 0.0312        # log2 of root-Hermite factor for delta=0.99 LLL (per dimension)

T0 = time.time()
LOG = []


def log(*a):
    s = ' '.join(str(t) for t in a)
    LOG.append(s)
    print(s)
    sys.stdout.flush()


RES = {
    'experiment': 'EXP-EXOTIC-H1-001',
    'seed': SEED,
    'cells': {},
    'validation': {},
    'errors': [],
    'beta_sym': None,
    'verdict': 'inconclusive',
}


def save():
    RES['elapsed_s'] = round(time.time() - T0, 1)
    RES['log_tail'] = LOG[-150:]
    try:
        os.makedirs(OUT, exist_ok=True)
        with open(os.path.join(OUT, 'result.json'), 'w') as fh:
            json.dump(RES, fh, default=str)
    except Exception as e:  # never fail on save
        print('SAVE FAILED', e)


class CellTimeout(Exception):
    pass


set_random_seed(SEED)
random.seed(SEED)
PZ = PolynomialRing(ZZ, 'x,y')
XV, YV = PZ.gens()

# ---------------------------------------------------------------------------------------------
# Semaev S_3
# ---------------------------------------------------------------------------------------------


def S3(X1, X2, X3, a, b):
    return (X1 - X2) ** 2 * X3 ** 2 - 2 * ((X1 + X2) * (X1 * X2 + a) + 2 * b) * X3 + ((X1 * X2 - a) ** 2 - 4 * b * (X1 + X2))


def verify_S3(E, ntests=40):
    a, b = E.a4(), E.a6()
    ok = tot = 0
    neg_nonzero = 0
    for _ in range(ntests):
        P1 = E.random_point()
        P2 = E.random_point()
        if P1.is_zero() or P2.is_zero() or P1[0] == P2[0]:
            continue
        X3 = (-(P1 + P2))[0]
        tot += 1
        if S3(P1[0], P2[0], X3, a, b) == 0:
            ok += 1
        if S3(P1[0], P2[0], X3 + 1, a, b) != 0:
            neg_nonzero += 1
    return ok, tot, neg_nonzero


# ---------------------------------------------------------------------------------------------
# curve with full rational 2-torsion and a rational point of order 4 (full E[4] when p = 1 mod 4)
# ---------------------------------------------------------------------------------------------


def build_curve(p):
    F = GF(p)
    want_full4 = (int(p) % 4 == 1)
    tries = 0
    while True:
        tries += 1
        r1 = F.random_element()
        r2 = F.random_element()
        r3 = -(r1 + r2)
        rs = [r1, r2, r3]
        if len(set(rs)) < 3:
            continue
        halv = []
        for i in range(3):
            j, k = [t for t in range(3) if t != i]
            halv.append(bool((rs[i] - rs[j]).is_square() and (rs[i] - rs[k]).is_square()))
        if not any(halv):
            continue
        if want_full4 and (not all(halv)) and tries < 4000:
            continue
        a = r1 * r2 + r1 * r3 + r2 * r3
        b = -r1 * r2 * r3
        if a == 0 or b == 0 or 4 * a ** 3 + 27 * b ** 2 == 0:
            continue
        E = EllipticCurve(F, [a, b])
        invs = [int(d) for d in E.abelian_group().invariants()]
        z4 = any(d % 4 == 0 for d in invs)
        full2 = (len(invs) == 2 and invs[0] % 2 == 0)
        full4 = (len(invs) == 2 and all(d % 4 == 0 for d in invs))
        if not (z4 and full2):
            continue
        info = {'invariants': invs, 'order': int(E.cardinality()), 'a': int(a), 'b': int(b),
                'roots_of_cubic': [int(r) for r in rs], 'four_torsion_ok': bool(z4 and full2),
                'full_E4': bool(full4), 'p_mod_4': int(p) % 4, 'curve_search_tries': tries}
        return E, rs, info


# ---------------------------------------------------------------------------------------------
# lattice machinery
# ---------------------------------------------------------------------------------------------


def row_k(A, B, m, LM):
    k = B // LM[1]
    if LM[0] > 0:
        k = min(k, A // LM[0])
    return min(k, m)


SYSTEMS = {
    'raw': {'LM': (2, 2), 'verts': [(0, 0), (2, 0), (0, 2), (2, 2)]},
    'sym2': {'LM': (0, 2), 'verts': [(0, 0), (2, 0), (0, 2)]},
    'fghr': {'LM': (0, 2), 'verts': [(0, 0), (2, 0), (0, 2)]},
}


_SHAPES = {}


def shapes_for(system):
    """(name, key_weights, support_list) of downward-closed monomial supports (cached)."""
    if system not in _SHAPES:
        _SHAPES[system] = _shapes_for(system)
    return _SHAPES[system]


def _shapes_for(system):
    out = []
    if system == 'raw':
        for D in range(2, 40):
            S = [(A, B) for A in range(D + 1) for B in range(D + 1 - A)]
            if len(S) > NMAX:
                break
            out.append(('tri%d' % D, (1, 1), S))
        for c in range(2, 14):
            S = [(A, B) for A in range(c + 1) for B in range(c + 1)]
            if len(S) > NMAX:
                break
            out.append(('rect%d' % c, (1, 1), S))
    else:
        for (wa, wb) in [(1, 1), (1, 2), (2, 3), (1, 3), (2, 5), (1, 4), (3, 5)]:
            for D in range(2, 90):
                S = [(A, B) for A in range(D // wa + 1) for B in range((D - wa * A) // wb + 1)]
                if len(S) > NMAX:
                    break
                out.append(('w%d%d_D%d' % (wa, wb, D), (wa, wb), S))
    return out


_CAND_CACHE = {}


def gen_candidates(system, p, X, Y):
    key = (system, int(p), X, Y)
    if key in _CAND_CACHE:
        return _CAND_CACHE[key]
    lp = math.log2(float(p))
    lx = math.log2(max(X, 1))
    ly = math.log2(max(Y, 1))
    LM = SYSTEMS[system]['LM']
    verts = SYSTEMS[system]['verts']
    allc = []
    for name, (wa, wb), S in shapes_for(system):
        n = len(S)
        Sset = set(S)
        for m in range(1, MMAX + 1):
            ok = True
            ld = 0.0
            for (A, B) in S:
                k = row_k(A, B, m, LM)
                i = A - LM[0] * k
                j = B - LM[1] * k
                for (vx, vy) in verts:
                    if (i + k * vx, j + k * vy) not in Sset:
                        ok = False
                        break
                if not ok:
                    break
                ld += (m - k) * lp + A * lx + B * ly
            if not ok:
                continue
            margin = m * lp - 0.5 * math.log2(n) - (ld / n + HGLOG * n)
            allc.append({'name': name, 'wts': (wa, wb), 'S': S, 'n': n, 'm': m, 'margin': margin})
    good = [c for c in allc if c['margin'] >= MIN_MARGIN]
    chosen = []
    A1 = sorted([c for c in good if c['margin'] >= 1.0], key=lambda c: (c['n'], -c['margin']))
    A2 = sorted(good, key=lambda c: -c['margin'])
    for c in A1[:1] + A2[:2]:
        if all(not (c['name'] == d['name'] and c['m'] == d['m']) for d in chosen):
            chosen.append(c)
    best_margin = max([c['margin'] for c in allc]) if allc else None
    _CAND_CACHE[key] = (chosen, best_margin)
    return chosen, best_margin


def build_matrix(inst, cand, X, Y, p):
    m = cand['m']
    wa, wb = cand['wts']
    S = sorted(cand['S'], key=lambda t: (wa * t[0] + wb * t[1], t[1]))
    idx = {mon: i for i, mon in enumerate(S)}
    n = len(S)
    LM = inst['LM']
    fkd = inst['fk'](m)
    rows = [[0] * n for _ in range(n)]
    for r, (A, B) in enumerate(S):
        k = row_k(A, B, m, LM)
        i = A - LM[0] * k
        j = B - LM[1] * k
        mult = p ** (m - k)
        for (a_, b_), c in fkd[k].items():
            col = idx[(a_ + i, b_ + j)]
            rows[r][col] = mult * c * (X ** (a_ + i)) * (Y ** (b_ + j))
    return matrix(ZZ, rows), S


def eval_modp(terms, x, y, p):
    s = 0
    for (a_, b_, c) in terms:
        s += c * pow(x, a_, p) * pow(y, b_, p)
    return s % p


def recover(polys, X, Y, inst, p):
    found = set()
    pairs = [(i, j) for i in range(len(polys)) for j in range(i + 1, len(polys))][:6]
    for (i, j) in pairs:
        try:
            r = polys[i].resultant(polys[j], YV)
        except Exception:
            continue
        if r == 0 or r.is_constant():
            continue
        up = r.univariate_polynomial()
        xs = [xc for xc in range(0, X + 1) if up(xc) == 0]
        for xc in xs:
            for h in polys[:6]:
                s = h(xc, YV)
                if s == 0:
                    continue
                if s.is_constant():
                    break
                ys = [int(t) for t in s.univariate_polynomial().roots(ring=ZZ, multiplicities=False)]
                for yc in ys:
                    if 0 <= yc <= Y and eval_modp(inst['terms'], xc, yc, p) == 0:
                        found.add((xc, yc))
                break
        if inst['root'] in found:
            break
    return found


def attempt(system, inst, X, Y, p, deadline):
    """Try up to 3 candidate lattices; return dict with success flag."""
    cands, best_margin = gen_candidates(system, p, X, Y)
    out = {'success': False, 'tried': [], 'best_margin': best_margin}
    for cand in cands:
        if time.time() > deadline:
            raise CellTimeout()
        t0 = time.time()
        rec = {'shape': cand['name'], 'm': cand['m'], 'n': cand['n'], 'margin': round(cand['margin'], 2)}
        try:
            M, S = build_matrix(inst, cand, X, Y, p)
            t1 = time.time()
            Mr = M.LLL()
            t_lll = time.time() - t1
            n = cand['n']
            bound2 = (p ** cand['m']) ** 2
            polys = []
            nvan = 0
            x0, y0 = inst['root']
            for row in Mr.rows()[:10]:
                vals = [int(v) for v in row]
                nrm2 = sum(v * v for v in vals)
                d = {}
                good = True
                for col, (A, B) in enumerate(S):
                    v = vals[col]
                    if v == 0:
                        continue
                    den = (X ** A) * (Y ** B)
                    if v % den != 0:
                        good = False
                        break
                    d[(A, B)] = v // den
                if not good or not d:
                    continue
                poly = PZ(d)
                if poly(x0, y0) == 0:
                    nvan += 1
                if nrm2 * n < bound2:   # Howgrave-Graham: ||h||^2 * n < p^(2m)
                    polys.append(poly)
            rec['n_hg_valid'] = len(polys)
            rec['n_vanish_top10'] = nvan
            rec['lll_s'] = round(t_lll, 2)
            if len(polys) >= 2:
                found = recover(polys[:6], X, Y, inst, p)
                rec['n_found'] = len(found)
                if inst['root'] in found:
                    rec['success'] = True
                    out['success'] = True
                    out['time'] = time.time() - t0
                    out['tried'].append(rec)
                    return out
            rec['success'] = False
        except CellTimeout:
            raise
        except Exception as e:
            rec['error'] = repr(e)[:200]
        out['tried'].append(rec)
    return out


# ---------------------------------------------------------------------------------------------
# instances
# ---------------------------------------------------------------------------------------------


def lift_c(c, p):
    v = int(c)
    return v - p if v > p // 2 else v


def make_inst(system, Fpoly, root, p):
    terms = [(int(e[0]), int(e[1]), int(c)) for e, c in Fpoly.dict().items()]
    LM = SYSTEMS[system]['LM']
    assert any((a_, b_) == LM and c == 1 for (a_, b_, c) in terms), 'leading coefficient is not 1'
    fZ = PZ({(a_, b_): lift_c(c, p) for (a_, b_, c) in terms if c % p != 0})
    cache = {}

    def fk(m):
        if m not in cache:
            pw = [PZ(1)]
            for _ in range(m):
                pw.append(pw[-1] * fZ)
            cache[m] = [{(int(e[0]), int(e[1])): int(c) for e, c in q.dict().items()} for q in pw]
        return cache[m]

    inst = {'terms': terms, 'LM': LM, 'root': (int(root[0]), int(root[1])), 'fk': fk}
    assert eval_modp(terms, inst['root'][0], inst['root'][1], p) == 0, 'planted root is not a root'
    return inst


class Ctx:
    pass


def make_ctx(p, E, rs):
    c = Ctx()
    c.p = int(p)
    c.F = GF(p)
    c.E = E
    c.a = E.a4()
    c.b = E.a6()
    c.rs = rs
    c.PF = PolynomialRing(c.F, 'x,y')
    c.PG = PolynomialRing(c.F, 'u,v')
    c.vx = []         # valid x in [1, vx_upto] (x^3+ax+b square)
    c.vx_upto = 0
    c.xi_list = []    # (xi, x) with xi small integer
    c.xi_upto = 0
    r1, r2, r3 = rs
    c.cs = [(rs[i] - rs[j]) * (rs[i] - rs[k]) for (i, j, k) in [(0, 1, 2), (1, 0, 2), (2, 0, 1)]]
    Pt = PolynomialRing(c.F, 't')
    t = Pt.gen()
    c.Pt = Pt
    c.Dpoly = (t - r1) * (t - r2) * (t - r3)
    c.Npoly = t * c.Dpoly + c.cs[0] * (t - r2) * (t - r3) + c.cs[1] * (t - r1) * (t - r3) + c.cs[2] * (t - r1) * (t - r2)
    return c


def xi_of(ctx, x):
    s = x
    for r, c in zip(ctx.rs, ctx.cs):
        s += c / (x - r)
    return s


def extend_vx(ctx, B0):
    F = ctx.F
    while ctx.vx_upto < B0:
        ctx.vx_upto += 1
        x = F(ctx.vx_upto)
        if (x ** 3 + ctx.a * x + ctx.b).is_square():
            ctx.vx.append(ctx.vx_upto)


def extend_xi(ctx, B0, deadline):
    F = ctx.F
    t = ctx.Pt.gen()
    while ctx.xi_upto < B0:
        ctx.xi_upto += 1
        if time.time() > deadline:
            raise CellTimeout()
        xi = ctx.xi_upto
        for xr in (ctx.Npoly - F(xi) * ctx.Dpoly).roots(multiplicities=False):
            if any(xr == r for r in ctx.rs):
                continue
            if (xr ** 3 + ctx.a * xr + ctx.b).is_square():
                ctx.xi_list.append((xi, xr))
                break


def plant_x(ctx, bits, B0, tr):
    extend_vx(ctx, B0)
    vxb = [t for t in ctx.vx if t <= B0]
    if len(vxb) < 2:
        return None
    rng = random.Random(SEED * 1000003 + bits * 10007 + B0 * 101 + tr)
    x1, x2 = rng.sample(vxb, 2)
    P1 = ctx.E.lift_x(ctx.F(x1))
    P2 = ctx.E.lift_x(ctx.F(x2))
    if rng.random() < 0.5:
        P1 = -P1
    if rng.random() < 0.5:
        P2 = -P2
    R = P1 + P2
    return {'x1': x1, 'x2': x2, 'x3': R[0]}


def inst_raw_sym2(ctx, plant, system):
    p = ctx.p
    x, y = ctx.PF.gens()
    f = S3(x, y, plant['x3'], ctx.a, ctx.b)
    if system == 'raw':
        return make_inst('raw', f, (plant['x1'], plant['x2']), p)
    u, v = ctx.PG.gens()
    c = plant['x3']
    g = (u ** 2 - 4 * v) * c ** 2 - 2 * (u * (v + ctx.a) + 2 * ctx.b) * c + (v - ctx.a) ** 2 - 4 * ctx.b * u
    return make_inst('sym2', g, (plant['x1'] + plant['x2'], plant['x1'] * plant['x2']), p)


def inst_fghr(ctx, bits, B0, tr, deadline):
    """FGHR: factor base = {P : xi(P) in [1,B0]}; H(xi1,xi2)=0 iff P1+P2 in +-R + E[2]; swap-symmetrize."""
    p = ctx.p
    F = ctx.F
    extend_xi(ctx, B0, deadline)
    L = [e for e in ctx.xi_list if e[0] <= B0]
    if len(L) < 2:
        return None
    rng = random.Random(SEED * 1000033 + bits * 10007 + B0 * 101 + tr)
    (xi1, xa), (xi2, xb) = rng.sample(L, 2)
    E = ctx.E
    P1 = E.lift_x(xa)
    P2 = E.lift_x(xb)
    R = P1 + P2
    if R.is_zero():
        return None
    # sample (xi(P1'), xi(P2')) with P1'+P2' = R + T, T in E[2]
    T2 = [E(0)] + [E(r, 0) for r in ctx.rs]
    rows = []
    guard = 0
    while len(rows) < 16 and guard < 400:
        guard += 1
        Q1 = E.random_point()
        T = T2[rng.randrange(4)]
        Q2 = R - Q1 + T
        if Q1.is_zero() or Q2.is_zero():
            continue
        if any(Q1[0] == r or Q2[0] == r for r in ctx.rs):
            continue
        s1 = xi_of(ctx, Q1[0])
        s2 = xi_of(ctx, Q2[0])
        rows.append([s1 ** a_ * s2 ** b_ for a_ in range(3) for b_ in range(3)])
    K = matrix(F, rows).right_kernel()
    if K.dimension() != 1:
        raise ValueError('FGHR kernel dimension %d (expected 1)' % K.dimension())
    h = K.basis()[0]
    H = {(a_, b_): h[3 * a_ + b_] for a_ in range(3) for b_ in range(3)}
    if any(H[(a_, b_)] != H[(b_, a_)] for a_ in range(3) for b_ in range(3)):
        raise ValueError('FGHR H not symmetric')
    if H[(2, 2)] == 0:
        raise ValueError('FGHR h22 == 0')
    # sanity: planted pair is a root of H
    s1, s2 = F(xi1), F(xi2)
    if sum(H[k] * s1 ** k[0] * s2 ** k[1] for k in H) != 0:
        raise ValueError('FGHR planted pair not a root of H')
    u, v = ctx.PG.gens()
    g4 = (H[(2, 2)] * v ** 2 + H[(2, 1)] * u * v + H[(2, 0)] * (u ** 2 - 2 * v) + H[(1, 1)] * v
          + H[(1, 0)] * u + H[(0, 0)])
    g4 = g4 / H[(2, 2)]
    inst = make_inst('fghr', g4, (xi1 + xi2, xi1 * xi2), p)
    return inst


# ---------------------------------------------------------------------------------------------
# count control
# ---------------------------------------------------------------------------------------------


def count_control(E, p, nR=200):
    F = E.base_field()
    a, b = E.a4(), E.a6()
    N = int(E.cardinality())
    B = int(math.ceil(float(p) ** (1.0 / 3.0) - 1e-9))
    xs = [x for x in range(0, B + 1) if (F(x) ** 3 + a * F(x) + b).is_square()]
    pts = [E.lift_x(F(x)) for x in xs]
    V = len(xs)
    hit = {}
    ntrip = 0
    for i, j, k in combinations(range(V), 3):
        ntrip += 1
        sums = set()
        for sj in (1, -1):
            for sk in (1, -1):
                S = pts[i] + sj * pts[j] + sk * pts[k]
                if not S.is_zero():
                    sums.add(int(S[0]))
        for s in sums:
            hit[s] = hit.get(s, 0) + 1
    model = ntrip * 8.0 / N
    obs = []
    for _ in range(nR):
        R = E.random_point()
        while R.is_zero():
            R = E.random_point()
        obs.append(hit.get(int(R[0]), 0))
    mean200 = sum(obs) / float(nR)
    big = []
    nbig = 40000
    cnt = 0
    while cnt < nbig:
        x = F.random_element()
        if (x ** 3 + a * x + b).is_square():
            big.append(hit.get(int(x), 0))
            cnt += 1
    meanbig = sum(big) / float(nbig)
    return {'B': B, 'V_valid_x': V, 'triples': ntrip, 'N': N, 'model_mean_per_R': model,
            'obs_mean_200R': mean200, 'ratio_200R': (mean200 / model) if model > 0 else None,
            'obs_mean_40000R': meanbig, 'ratio_40000R': (meanbig / model) if model > 0 else None,
            'ratio_to_one_sixth_40000R': meanbig * 6.0,
            'frac_R_with_decomp_200R': sum(1 for o in obs if o > 0) / float(nR)}


# ---------------------------------------------------------------------------------------------
# sweep
# ---------------------------------------------------------------------------------------------


def sweep(system, ctx, bits, deadline):
    p = ctx.p
    levels = []
    best = None
    best_att = None
    times = []
    consec = 0
    cache = {}
    beta = BETA_LO
    errors = []
    censored = False
    try:
        while beta <= BETA_HI + 1e-9:
            if time.time() > deadline:
                raise CellTimeout()
            B0 = int(math.ceil(float(p) ** beta - 1e-9))
            if system == 'raw':
                X, Y = B0, B0
            else:
                X, Y = 2 * B0, B0 * B0
            if B0 in cache:
                lv = dict(cache[B0])
                lv['beta'] = round(beta, 4)
                lv['reused'] = True
                levels.append(lv)
                if lv['pass']:
                    best = beta
                    best_att = lv.get('att')
                    consec = 0
                elif lv['tested']:
                    consec += 1
                if consec >= 6:
                    break
                beta += BETA_STEP
                continue
            cands, best_margin = gen_candidates(system, p, X, Y)
            lv = {'beta': round(beta, 4), 'B0': B0, 'X': X, 'Y': Y, 'best_margin': None if best_margin is None else round(best_margin, 2),
                  'n_cands': len(cands), 'ok': 0, 'tested': 0, 'pass': False}
            if not cands:
                lv['tested'] = TRIALS
                lv['note'] = 'no lattice with predicted margin >= %.1f' % MIN_MARGIN
            else:
                for tr in range(TRIALS):
                    try:
                        if system == 'fghr':
                            inst = inst_fghr(ctx, bits, B0, tr, deadline)
                        else:
                            plant = plant_x(ctx, bits, B0, tr)
                            inst = None if plant is None else inst_raw_sym2(ctx, plant, system)
                    except CellTimeout:
                        raise
                    except Exception as e:
                        errors.append('%s B0=%d tr=%d: %s' % (system, B0, tr, repr(e)[:160]))
                        inst = None
                    if inst is None:
                        continue
                    lv['tested'] += 1
                    r = attempt(system, inst, X, Y, p, deadline)
                    if r['success']:
                        lv['ok'] += 1
                        times.append(r['time'])
                        lv['att'] = r['tried'][-1]
                    elif tr == 0 or 'tried0' not in lv:
                        lv['tried0'] = r['tried']
                    if lv['ok'] >= 2 or (lv['tested'] - lv['ok']) >= 2:
                        break
                lv['pass'] = (lv['ok'] >= 2)
            cache[B0] = {k: v for k, v in lv.items()}
            levels.append(lv)
            if lv['pass']:
                best = beta
                best_att = lv.get('att')
                consec = 0
            elif lv['tested']:
                consec += 1
            if consec >= 6:
                break
            beta += BETA_STEP
    except CellTimeout:
        censored = True
    return {'beta': best, 'levels': levels, 'times': times, 'best_attempt': best_att,
            'censored': censored, 'errors': errors}


# ---------------------------------------------------------------------------------------------
# main per-cell driver
# ---------------------------------------------------------------------------------------------


def run_cell(bits, cell_deadline):
    p = next_prime(2 ** bits)
    pi = int(p)
    cell = {'bits': bits, 'p': pi, 'censored': False}
    t_cell = time.time()
    E, rs, cinfo = build_curve(p)
    cell.update({'four_torsion_ok': cinfo['four_torsion_ok'], 'full_E4': cinfo['full_E4'], 'curve': cinfo})
    ok, tot, neg = verify_S3(E)
    cell['S3_verified'] = bool(tot > 0 and ok == tot and neg == tot)
    cell['S3_checks'] = {'ok': ok, 'total': tot, 'neg_control_nonzero': neg}
    ctx = make_ctx(p, E, rs)
    # degrees on a random target + identity check g(e1,e2)=f(x1,x2) + xi rational-function check
    R = E.random_point()
    x3 = R[0]
    x, y = ctx.PF.gens()
    f = S3(x, y, x3, ctx.a, ctx.b)
    u, v = ctx.PG.gens()
    g = (u ** 2 - 4 * v) * x3 ** 2 - 2 * (u * (v + ctx.a) + 2 * ctx.b) * x3 + (v - ctx.a) ** 2 - 4 * ctx.b * u
    idok = True
    for _ in range(10):
        x1, x2 = ctx.F.random_element(), ctx.F.random_element()
        if f(x1, x2) != g(x1 + x2, x1 * x2):
            idok = False
    t0_ = ctx.F.random_element()
    xichk = (ctx.Npoly(t0_) / ctx.Dpoly(t0_) == xi_of(ctx, t0_))
    cell['sym_identity_verified'] = bool(idok)
    cell['xi_rational_function_verified'] = bool(xichk)
    cell['degrees'] = {'raw_total': int(f.total_degree()), 'raw_bidegree': [int(f.degree(x)), int(f.degree(y))],
                       'sym_e_total': int(g.total_degree()), 'sym_e_weighted_1_2': 4,
                       'fghr_xi_bidegree': [2, 2], 'fghr_sigma_total': 2}
    cell['symmetrized_degree'] = int(g.total_degree())
    cell['beta_generic'] = 1.0 / f.total_degree()
    cell['beta_generic_product_box_raw'] = 1.0 / (2 * f.total_degree())
    cell['beta_generic_sym_boxequiv'] = 1.0 / (3 * g.total_degree())
    log('[cell %d] p=%d N=%d inv=%s S3=%s deg raw/sym=%d/%d' % (bits, pi, cinfo['order'], cinfo['invariants'],
                                                              cell['S3_verified'], f.total_degree(), g.total_degree()))
    save_cell(bits, cell)
    # count control (two smallest p)
    if bits in BITS[:2]:
        try:
            cc = count_control(E, p)
            cell['count_control'] = cc
            cell['count_control_ratio'] = cc['ratio_200R']
            log('[cell %d] count control: model=%.4f obs200=%.4f ratio200=%s ratio40000=%.3f V=%d' % (
                bits, cc['model_mean_per_R'], cc['obs_mean_200R'], cc['ratio_200R'], cc['ratio_40000R'], cc['V_valid_x']))
        except Exception as e:
            RES['errors'].append('count_control %d: %s' % (bits, traceback.format_exc()[-600:]))
            cell['count_control_ratio'] = None
    else:
        cell['count_control_ratio'] = None
    save_cell(bits, cell)
    # reach sweeps; split remaining budget 3 ways
    systems = ['raw', 'sym2', 'fghr']
    sw = {}
    for si, system in enumerate(systems):
        remaining = cell_deadline - time.time()
        share = remaining / (len(systems) - si)
        dl = time.time() + max(share, 1.0)
        try:
            r = sweep(system, ctx, bits, dl)
        except Exception as e:
            RES['errors'].append('sweep %s %d: %s' % (system, bits, traceback.format_exc()[-800:]))
            r = {'beta': None, 'levels': [], 'times': [], 'best_attempt': None, 'censored': False, 'errors': [repr(e)]}
        sw[system] = r
        log('[cell %d] %s: beta=%s censored=%s levels=%d errs=%d' % (bits, system, r['beta'], r['censored'],
                                                                       len(r['levels']), len(r['errors'])))
        cell['beta_' + system] = r['beta']
        cell['levels_' + system] = r['levels']
        cell['best_attempt_' + system] = r['best_attempt']
        cell['errors_' + system] = r['errors'][:6]
        if r['censored']:
            cell['censored_' + system] = True
        save_cell(bits, cell)
    cell['beta_raw'] = sw['raw']['beta']
    cell['beta_sym_swap'] = sw['sym2']['beta']
    cell['beta_sym_fghr'] = sw['fghr']['beta']
    cands = [b for b in (sw['sym2']['beta'], sw['fghr']['beta']) if b is not None]
    cell['beta_sym'] = max(cands) if cands else None
    allt = sw['raw']['times'] + sw['sym2']['times'] + sw['fghr']['times']
    cell['median_lattice_time_s'] = statistics.median(allt) if allt else None
    cell['median_lattice_time_s_sym'] = statistics.median(sw['sym2']['times'] + sw['fghr']['times']) if (sw['sym2']['times'] + sw['fghr']['times']) else None
    # relation cost at 131 bits: per relation need ~6 * p^(1-3beta) targets, one lattice call each
    bs = cell['beta_sym'] if cell['beta_sym'] is not None else 0.0
    cell['implied_relation_cost_bits_at_131'] = max(math.log2(6.0), (1.0 - 3.0 * bs) * 131.0 + math.log2(6.0))
    cell['implied_relation_cost_bits_at_131_raw'] = max(math.log2(6.0), (1.0 - 3.0 * (cell['beta_raw'] or 0.0)) * 131.0 + math.log2(6.0))
    cell['censored'] = any(sw[s]['censored'] for s in systems)
    cell['elapsed_s'] = round(time.time() - t_cell, 1)
    return cell


def save_cell(bits, cell):
    RES['cells'][str(bits)] = cell
    save()


def compute_verdict():
    cells = [RES['cells'][k] for k in sorted(RES['cells'], key=lambda s: int(s))]
    fin = [c for c in cells if not c.get('censored') and 'beta_generic' in c and 'beta_raw' in c]
    # primary: largest p that finished
    if fin:
        RES['beta_sym'] = fin[-1].get('beta_sym') if fin[-1].get('beta_sym') is not None else 0.0
        RES['beta_sym_cell_bits'] = fin[-1]['bits']
    else:
        part = [c for c in cells if c.get('beta_sym') is not None]
        RES['beta_sym'] = part[-1]['beta_sym'] if part else 0.0
        RES['beta_sym_cell_bits'] = part[-1]['bits'] if part else None
        RES['note_primary'] = 'no cell finished uncensored; beta_sym is from a censored cell'
    # validation
    val = {}
    val['S3_verified_all'] = bool(cells and all(c.get('S3_verified') for c in cells))
    val['sym_identity_verified_all'] = bool(cells and all(c.get('sym_identity_verified') for c in cells))
    val['four_torsion_ok_all'] = bool(cells and all(c.get('four_torsion_ok') for c in cells))
    cc = [c for c in cells if c.get('count_control')]
    val['count_control_cells'] = [c['bits'] for c in cc]
    val['count_control_ratio_200R'] = {str(c['bits']): c['count_control']['ratio_200R'] for c in cc}
    val['count_control_ratio_40000R'] = {str(c['bits']): c['count_control']['ratio_40000R'] for c in cc}
    val['count_control_within_2x'] = bool(cc and all(c['count_control']['ratio_200R'] is not None and 0.5 <= c['count_control']['ratio_200R'] <= 2.0
                                                     and 0.5 <= c['count_control']['ratio_40000R'] <= 2.0 for c in cc))
    allbetas = [c.get('beta_raw') for c in cells] + [c.get('beta_sym_swap') for c in cells] + [c.get('beta_sym_fghr') for c in cells]
    val['machinery_recovers_some_root'] = bool(any(b is not None for b in allbetas))
    val['fghr_available'] = bool(any(c.get('beta_sym_fghr') is not None for c in cells))
    RES['validation'] = val
    # trend
    rows = [(c['bits'], c.get('beta_sym') if c.get('beta_sym') is not None else 0.0, c['beta_generic']) for c in fin]
    RES['trend'] = [{'bits': b, 'beta_sym': s, 'beta_generic': g, 'beta_sym_minus_third': s - 1.0 / 3} for b, s, g in rows]
    if len(rows) >= 2:
        xs_ = [r[0] for r in rows]
        ys_ = [r[1] for r in rows]
        mx = sum(xs_) / float(len(xs_))
        my = sum(ys_) / float(len(ys_))
        den = sum((x_ - mx) ** 2 for x_ in xs_)
        slope = sum((x_ - mx) * (y_ - my) for x_, y_ in zip(xs_, ys_)) / den if den else 0.0
        icpt = my - slope * mx
        RES['fit'] = {'slope_beta_per_bit': slope, 'intercept': icpt, 'beta_at_131_linear_clipped': min(max(icpt + slope * 131, 0.0), 1.0 / 3)}
        b131 = RES['fit']['beta_at_131_linear_clipped']
        RES['implied_cost_bits_at_131_fit'] = max(math.log2(6.0), (1 - 3 * b131) * 131 + math.log2(6.0))
    invalid = not (val['S3_verified_all'] and val['sym_identity_verified_all'] and val['four_torsion_ok_all']
                   and val['count_control_within_2x'] and val['machinery_recovers_some_root'])
    verdict = 'inconclusive'
    if rows and not invalid:
        if all(s <= g + 0.02 for _, s, g in rows):
            verdict = 'kill'
        else:
            exc = [s - 1.0 / 3 for _, s, _g in rows]
            if len(rows) >= 3 and all(e > 0 for e in exc) and all(exc[i + 1] > exc[i] for i in range(len(exc) - 1)):
                verdict = 'win'
    RES['verdict'] = verdict
    RES['invalid_controls'] = bool(invalid)
    RES['implied_relation_cost_bits_at_131'] = fin[-1]['implied_relation_cost_bits_at_131'] if fin else None
    RES['gate_bits'] = 21
    RES['rho_bits'] = 65
    RES['null_box_enumeration_bits_at_131'] = 131.0 / 3


def main():
    for bits in BITS:
        elapsed = time.time() - T0
        remaining = GLOBAL_BUDGET - elapsed
        if remaining < 60:
            RES['cells'][str(bits)] = {'bits': bits, 'censored': True, 'reason': 'global budget exhausted'}
            continue
        cells_left = len([b for b in BITS if b >= bits])
        cell_deadline = time.time() + min(remaining - 20, max(CELL_BUDGET, remaining / float(cells_left)))
        try:
            cell = run_cell(bits, cell_deadline)
            save_cell(bits, cell)
        except Exception as e:
            RES['errors'].append('cell %d: %s' % (bits, traceback.format_exc()[-1200:]))
            c = RES['cells'].get(str(bits), {'bits': bits})
            c['censored'] = True
            c['reason'] = 'exception: ' + repr(e)[:200]
            RES['cells'][str(bits)] = c
            log('[cell %d] EXCEPTION %s' % (bits, repr(e)[:200]))
        save()


try:
    main()
except Exception as e:
    RES['errors'].append('main: ' + traceback.format_exc()[-1500:])
finally:
    try:
        compute_verdict()
    except Exception as e:
        RES['errors'].append('verdict: ' + traceback.format_exc()[-1500:])
    save()
    log('DONE verdict=%s beta_sym=%s elapsed=%.0fs' % (RES.get('verdict'), RES.get('beta_sym'), time.time() - T0))
