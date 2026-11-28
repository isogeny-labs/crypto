"""Validator desk check for L2 (independent code; imports nothing from the L2 scripts).

Per curve k and chart function f:
  * degree of f : E -> P^1 from the function field (polynomial gcd mod p);
  * exact rational fibre-size table, max fibre, points per value;
  * signed symmetries P -> eps P + Q (Q in E[8]) checked on EVERY point;
  * the translation subgroup H of the + stabiliser and which 4-torsion point is rational;
  * split-fibre count of distinct f(-(A_i + B_j)) over 200 pairs of fully rational fibres;
  * own interpolation of S^f in the ELEMENTARY-SYMMETRIC basis e1^i e2^j e3^k
    (box i+j+k <= D, i.e. per-variable degree <= D; parity classes for odd f),
    kernel dimension at D-1 and at D, total degree = max(i + 2j + 3k);
  * verification on 1000 fresh zeros and 1000 random triples;
  * proportionality to the producer's recorded polynomial (data/sumpoly.json).
"""
import itertools, json, sys, time
import numpy as np
from vcurve import VCurve, npow

LEAD = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L2_highdeg_coords'
FUNCS = ['x', 'ye', 'xe', 'v', 'q', 'xeye', 'jac6', 'uv']


# ---------------- univariate polynomials mod p (lists, low degree first)
def ptrim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def pmul(a, b, p):
    if not a or not b:
        return []
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = (out[i + j] + x * y) % p
    return ptrim(out)


def pdivmod(a, b, p):
    a = a[:]; q = [0] * max(len(a) - len(b) + 1, 1)
    inv = pow(b[-1], p - 2, p)
    while len(a) >= len(b) and a:
        c = a[-1] * inv % p; d = len(a) - len(b)
        q[d] = c
        for i, y in enumerate(b):
            a[i + d] = (a[i + d] - c * y) % p
        ptrim(a)
    return ptrim(q), a


def pgcd(a, b, p):
    a, b = ptrim(a[:]), ptrim(b[:])
    while b:
        a, b = b, pdivmod(a, b, p)[1]
    return a


def rdeg(num, den, p):
    """degree of the reduced rational function num/den (max of degrees)."""
    g = pgcd(num, den, p)
    n2 = pdivmod(num, g, p)[0]; d2 = pdivmod(den, g, p)[0]
    return max(len(n2) - 1, len(d2) - 1)


def degree_of(f, C):
    p, A = C.p, C.A
    A3 = C.A3
    w = [0, 1, A % p, 1]                     # u^3 + A u^2 + u
    U = [0, 1]; X = [A3, 1]
    one = [1]
    if f == 'x':
        return 2 * rdeg(X, one, p)
    if f == 'ye':
        return 2 * rdeg([(-1) % p, 1], [1, 1], p)
    if f == 'jac6':
        return 2 * rdeg(w, pmul(pmul(X, X, p), X, p), p)
    # odd f = h(u) v : deg f = deg_u(h^2 w)
    if f == 'v':
        hn, hd = one, one
    elif f == 'q':          # x / v = x v / w
        hn, hd = X, w
    elif f == 'uv':
        hn, hd = U, one
    elif f == 'xe':         # u / v = u v / w
        hn, hd = U, w
    elif f == 'xeye':       # u(u-1) / (v (u+1)) = u(u-1) v / (w (u+1))
        hn, hd = pmul(U, [(-1) % p, 1], p), pmul(w, [1, 1], p)
    num = pmul(pmul(hn, hn, p), w, p)
    den = pmul(hd, hd, p)
    return rdeg(num, den, p)


# ---------------- linear algebra mod p
def echelon(M, p):
    M = M % p
    R, Cc = M.shape
    piv = []; r = 0
    for c in range(Cc):
        if r == R:
            break
        nz = np.flatnonzero(M[r:, c])
        if nz.size == 0:
            continue
        i = r + int(nz[0])
        if i != r:
            M[[r, i]] = M[[i, r]]
        inv = pow(int(M[r, c]), p - 2, p)
        M[r, c:] = M[r, c:] * inv % p
        below = M[r + 1:, c]
        nzb = np.flatnonzero(below)
        if nzb.size:
            rows = r + 1 + nzb
            M[rows, c:] = (M[rows, c:] - below[nzb, None] * M[r, c:][None, :]) % p
        piv.append(c); r += 1
    return M[:r], piv


def kernel_vector(E, piv, ncols, fc, p):
    x = np.zeros(ncols, dtype=np.int64); x[fc] = 1
    for i in range(len(piv) - 1, -1, -1):
        s = int((E[i] * x % p).sum() % p)   # E[i, piv_i] = 1 and x[piv_i] = 0 so far
        x[piv[i]] = (-s) % p
    return x


def ebasis(D, parity):
    cols = []
    for s in range(D + 1):
        for k in range(s + 1):
            for j in range(s - k + 1):
                i = s - k - j
                if parity is not None and (i + 2 * j + 3 * k) % 2 != parity:
                    continue
                cols.append((i, j, k))
    return cols


def eval_ebasis(cols, a, b, c, p, D):
    e1 = (a + b + c) % p
    e2 = (a * b % p + b * c % p + c * a % p) % p
    e3 = a * b % p * c % p
    P = []
    for e in (e1, e2, e3):
        T = np.ones((e.size, D + 1), dtype=np.int64)
        for t in range(1, D + 1):
            T[:, t] = T[:, t - 1] * e % p
        P.append(T)
    M = np.empty((a.size, len(cols)), dtype=np.int64)
    for ci, (i, j, k) in enumerate(cols):
        M[:, ci] = P[0][:, i] * P[1][:, j] % p * P[2][:, k] % p
    return M


def sample_zeros(C, val, ok, N, rng):
    n = C.n
    out = [[], [], []]
    while len(out[0]) < N:
        l1 = rng.integers(1, n, size=4 * N); l2 = rng.integers(1, n, size=4 * N)
        l3 = (-(l1 + l2)) % n
        g = ok[l1] & ok[l2] & ok[l3] & (l3 != 0)
        for o, l in zip(out, (l1, l2, l3)):
            o.extend(val[l[g]].tolist())
    return [np.array(o[:N], dtype=np.int64) for o in out]


def eval_producer(coeffs, a, b, c, p):
    D = max(max(t[:3]) for t in coeffs)
    pw = []
    for z in (a, b, c):
        T = np.ones((z.size, D + 1), dtype=np.int64)
        for t in range(1, D + 1):
            T[:, t] = T[:, t - 1] * z % p
        pw.append(T)
    acc = np.zeros(a.size, dtype=np.int64)
    terms = 0; tot = 0; pv = 0
    for (i, j, k, cf) in coeffs:
        perms = set(itertools.permutations((i, j, k)))
        terms += len(perms); tot = max(tot, i + j + k); pv = max(pv, i, j, k)
        s = np.zeros(a.size, dtype=np.int64)
        for (x, y, z) in perms:
            s = (s + pw[0][:, x] * pw[1][:, y] % p * pw[2][:, z]) % p
        acc = (acc + cf % p * s) % p
    return acc, terms, tot, pv


def main():
    ks = [int(x) for x in sys.argv[1].split(',')] if len(sys.argv) > 1 else [16, 18, 20]
    funcs = sys.argv[2].split(',') if len(sys.argv) > 2 else FUNCS
    sp = json.load(open(f'{LEAD}/data/sumpoly.json'))
    out = {}
    t0 = time.time()
    for k in ks:
        C = VCurve(k)
        p, n, r = C.p, C.n, C.r
        res = {'facts': C.facts, 'generator_uv': list(C.gen_uv)}
        # which 4-torsion point is rational: logs 2r and 6r
        res['u_of_rational_4torsion'] = sorted({int(C.U[2 * r]), int(C.U[6 * r])})
        res['T0_log'] = n // 2
        print(f'k={k} table {time.time()-t0:.1f}s facts={C.facts} u(4-tors)={res["u_of_rational_4torsion"]}', file=sys.stderr, flush=True)
        rng = np.random.default_rng(777000 + k)
        allL = np.arange(1, n, dtype=np.int64)
        for f in funcs:
            R = {}
            val, ok = C.fvals(f)
            R['degree_function_field'] = degree_of(f, C)
            v_ok = val[ok]
            uvals, cnts = np.unique(v_ok, return_counts=True)
            R['max_fibre'] = int(cnts.max())
            R['fibre_hist'] = {int(s): int((cnts == s).sum()) for s in np.unique(cnts)}
            R['points_per_value'] = float(ok.sum() / p)
            R['poles_affine'] = int((~ok[1:]).sum())
            # symmetries on every point
            sym = []
            for eps in (1, -1):
                for j in range(8):
                    img = (eps * allL + j * r) % n
                    g = ok[allL] & ok[img] & (img != 0)
                    fa, fb = val[allL][g], val[img][g]
                    if np.array_equal(fa, fb):
                        sym.append([eps, j, '+'])
                    elif np.all((fa + fb) % p == 0):
                        sym.append([eps, j, '-'])
            R['symmetries'] = sym
            R['H_translations_plus'] = sorted({j for (e, j, s) in sym if s == '+'})
            # split-fibre count
            d = R['degree_function_field']
            order = np.argsort(v_ok, kind='stable')
            logs_sorted = np.flatnonzero(ok)[order]
            starts = np.concatenate([[0], np.cumsum(cnts)[:-1]])
            full = np.flatnonzero(cnts == d)
            R['n_fully_rational_fibres'] = int(full.size)
            if full.size >= 2:
                cs = []
                for _ in range(200):
                    ia, ib = rng.choice(full, 2, replace=False)
                    As = logs_sorted[starts[ia]:starts[ia] + d]; Bs = logs_sorted[starts[ib]:starts[ib] + d]
                    s = (-(As[:, None] + Bs[None, :])).ravel() % n
                    s = s[(s != 0)]
                    s = s[ok[s]]
                    cs.append(int(np.unique(val[s]).size))
                R['split_fibre'] = [min(cs), max(cs)]
            else:
                R['split_fibre'] = None
            # own interpolation in the e-basis
            Dc = R['split_fibre'][1] if R['split_fibre'] else None
            if Dc is None:   # xeye at k = 18, 20: use the k = 16 value
                Dc = out.get(16, {}).get(f, {}).get('S_own', {}).get('pervar') or 4
            odd = bool(any(e == -1 and j == 0 and s == '-' for (e, j, s) in sym))
            pars = [0, 1] if odd else [None]
            kers = {}
            found = None
            for D in (Dc - 1, Dc):
                for par in pars:
                    cols = ebasis(D, par)
                    a_, b_, c_ = sample_zeros(C, val, ok, len(cols) + 60, rng)
                    M = eval_ebasis(cols, a_, b_, c_, p, D)
                    E, piv = echelon(M, p)
                    kd = len(cols) - len(piv)
                    kers[f'D{D}_par{par}'] = dict(cols=len(cols), rank=len(piv), kernel_dim=kd)
                    if D == Dc and kd == 1:
                        fc = [c for c in range(len(cols)) if c not in set(piv)][0]
                        x = kernel_vector(E, piv, len(cols), fc, p)
                        assert not np.any(M @ x % p) if M.shape[1] < 400 else True
                        found = (D, par, cols, x)
            R['kernels'] = kers
            if found:
                D, par, cols, x = found
                nz = np.flatnonzero(x)
                pervar = max(sum(cols[i]) for i in nz)
                tot = max(cols[i][0] + 2 * cols[i][1] + 3 * cols[i][2] for i in nz)
                a_, b_, c_ = sample_zeros(C, val, ok, 1000, np.random.default_rng(888000 + k))
                ev = eval_ebasis([cols[i] for i in nz], a_, b_, c_, p, D) @ x[nz] % p
                rnd = np.random.default_rng(999000 + k).integers(0, p, size=(3, 1000))
                evr = eval_ebasis([cols[i] for i in nz], rnd[0], rnd[1], rnd[2], p, D) @ x[nz] % p
                # producer's polynomial on the same points
                S = sp[str(k)][f]['S']
                pz, pterms, ptot, ppv = eval_producer(S['coeffs'], a_, b_, c_, p)
                pr, _, _, _ = eval_producer(S['coeffs'], rnd[0], rnd[1], rnd[2], p)
                both = (evr != 0) & (pr != 0)
                ratio = evr[both] * npow(pr[both], p - 2, p) % p
                R['S_own'] = dict(pervar=int(pervar), total=int(tot), parity=par, ebasis_terms=int(nz.size),
                                  fresh_zeros_vanish=int((ev == 0).sum()), random_nonvanish=int((evr != 0).sum()))
                R['S_producer_check'] = dict(producer_zero_on_my_fresh_zeros=int((pz == 0).sum()),
                                             producer_terms_recount=pterms, producer_total_recount=ptot,
                                             producer_pervar_recount=ppv,
                                             proportional=bool(np.unique(ratio).size == 1 and both.sum() > 900
                                                               and np.array_equal(evr == 0, pr == 0)),
                                             recorded=dict(pervar=S['pervar_degree'], total=S['total_degree'], terms=S['terms'],
                                                           kernel_dim=S['kernel_dim']))
            else:
                R['S_own'] = None
            out.setdefault(k, {})[f] = R
            print(f'k={k} f={f} deg={R["degree_function_field"]} maxfib={R["max_fibre"]} ppv={R["points_per_value"]:.4f} '
                  f'H+={R["H_translations_plus"]} split={R["split_fibre"]} kers={kers} S_own={R["S_own"]} '
                  f'prod={R.get("S_producer_check")} {time.time()-t0:.0f}s', file=sys.stderr, flush=True)
        out[k]['_curve'] = res
    tag = '_'.join(map(str, ks)) + '_' + '-'.join(funcs)
    json.dump(out, open(f'desk_{tag}.json', 'w'), indent=1, default=str)


if __name__ == '__main__':
    main()
