#!/usr/bin/env python3
"""L2 part (b1): desk verification and summation polynomials S^f(a, b, c).

Per curve k and coordinate function f:
  1. rational fibre-size distribution over all of E(F_p) (max = deg f expected);
  2. parity and symmetries f o s = +-f for s : P -> eps P + Q, Q in E(F_p)[8];
  3. split-fibre count: for values a, b whose fibres are fully rational, the number
     of distinct f(-(A_i + B_j)) = degree of S^f in one variable (independent check);
  4. interpolation: the minimal symmetric polynomial vanishing on sampled zeros
     (f(P1), f(P2), f(P3)), P1 + P2 + P3 = O, in the orbit-sum basis of a box of
     per-variable degree D, D increased until the kernel is non-zero (odd f: both
     total-degree parities tried). Reports per-variable degree, total degree, terms,
     kernel dimension; verifies on fresh zeros and on random non-zeros;
  5. for f = x: proportionality to the explicit Semaev S3 (positive control PC3).
"""
import argparse, itertools, json, sys, time
import numpy as np
import common as C_
from linalg import nullspace


def orbits(D, parity=None):
    out = []
    for i in range(D + 1):
        for j in range(i, D + 1):
            for k in range(j, D + 1):
                if parity is not None and (i + j + k) % 2 != parity:
                    continue
                out.append((i, j, k))
    return out


def orbit_matrix(orbs, a, b, c, p, D):
    pw = []
    for z in (a, b, c):
        P = np.ones((z.size, D + 1), dtype=np.int64)
        for e in range(1, D + 1):
            P[:, e] = P[:, e - 1] * z % p
        pw.append(P)
    M = np.zeros((a.size, len(orbs)), dtype=np.int64)
    for col, o in enumerate(orbs):
        acc = np.zeros(a.size, dtype=np.int64)
        for perm in set(itertools.permutations(o)):
            acc = (acc + pw[0][:, perm[0]] * pw[1][:, perm[1]] % p * pw[2][:, perm[2]]) % p
        M[:, col] = acc
    return M


def sample_zeros(C, val, ok, N, rng):
    """N triples (f(P1), f(P2), f(P3)) with P1 + P2 + P3 = O, all f finite."""
    n = C.n
    A, B, Cc = [], [], []
    full = np.full(n, -1, dtype=np.int64); full[1:] = val
    okf = np.zeros(n, dtype=bool); okf[1:] = ok
    while len(A) < N:
        l1 = rng.integers(1, n, size=4 * N); l2 = rng.integers(1, n, size=4 * N)
        l3 = (-(l1 + l2)) % n
        g = okf[l1] & okf[l2] & okf[l3]
        A += list(full[l1[g]]); B += list(full[l2[g]]); Cc += list(full[l3[g]])
    return (np.array(A[:N]), np.array(B[:N]), np.array(Cc[:N]))


def fibres(C, val, ok):
    v = val[ok]; logs = C.logs[ok]
    o = np.argsort(v, kind='stable'); v = v[o]; logs = logs[o]
    u, start, cnt = np.unique(v, return_index=True, return_counts=True)
    return u, start, cnt, logs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--k', type=int, nargs='+', default=[16, 18, 20])
    ap.add_argument('--funcs', nargs='+', default=C_.FUNCS)
    ap.add_argument('--Dmax', type=int, default=27)
    ap.add_argument('--verify', type=int, default=1000)
    ap.add_argument('--out', required=True)
    a = ap.parse_args()
    t0 = time.time()
    res = {}
    for k in a.k:
        C = C_.Curve(k); p, n, r = C.p, C.n, C.r
        res[k] = {'facts': C.facts}
        fullU = C.U; fullV = C.V
        for f in a.funcs:
            rng = np.random.default_rng(C_.dseed(k, 'sumpoly', f))
            val, ok = C.fvals(f)
            R = {}
            # 1. fibre sizes
            u, start, cnt, flogs = fibres(C, val, ok)
            R['max_rational_fibre'] = int(cnt.max())
            R['fibre_size_freq'] = {int(s): float((cnt == s).sum() / p) for s in np.unique(cnt)}
            R['points_per_value'] = float(cnt.sum() / p)
            # 2. parity and symmetries over E(F_p)[8] (logs j r)
            fullval = np.full(n, -1, dtype=np.int64); fullval[1:] = np.where(ok, val, -1)
            L = rng.integers(1, n, size=4000)
            sym = []
            for eps in (1, -1):
                for j in range(8):
                    img = (eps * L + j * r) % n
                    g = (fullval[L] >= 0) & (fullval[img] >= 0) & (img != 0)
                    fa, fb = fullval[L][g], fullval[img][g]
                    if np.all(fa == fb):
                        sym.append(dict(eps=eps, Q_log=f'{j}r', sign='+'))
                    elif np.all((fa + fb) % p == 0):
                        sym.append(dict(eps=eps, Q_log=f'{j}r', sign='-'))
            R['symmetries_rational'] = sym
            # 3. split-fibre count
            d = C_.DESK[f]['deg']
            full_split = np.nonzero(cnt == d)[0]
            if full_split.size >= 2:
                counts = []
                for _ in range(20):
                    ia, ib = rng.choice(full_split, 2, replace=False)
                    As = flogs[start[ia]:start[ia] + d]; Bs = flogs[start[ib]:start[ib] + d]
                    s = (-(As[:, None] + Bs[None, :])) % n
                    vals = fullval[s.ravel()]
                    counts.append(int(np.unique(vals[(s.ravel() != 0)]).size))
                R['split_fibre_distinct_values'] = dict(max=max(counts), min=min(counts), trials=len(counts))
            else:
                R['split_fibre_distinct_values'] = None  # no fully rational fibre of size deg f
            # 4. interpolation
            parities = [None] if C_.DESK[f]['parity'] == 'even' else [0, 1]
            found = None
            for D in range(1, a.Dmax + 1):
                for par in parities:
                    orbs = orbits(D, par)
                    N = len(orbs) + 40
                    A_, B_, Cc_ = sample_zeros(C, val, ok, N, rng)
                    M = orbit_matrix(orbs, A_, B_, Cc_, p, D)
                    ker = nullspace(M, p)
                    if ker:
                        found = (D, par, orbs, ker)
                        break
                if found:
                    break
                if time.time() - t0 > 3 * 3600:
                    break
            if found:
                D, par, orbs, ker = found
                s = ker[0]
                nz = [i for i in range(len(orbs)) if s[i]]
                tot = max(sum(orbs[i]) for i in nz)
                terms = sum(len(set(itertools.permutations(orbs[i]))) for i in nz)
                # normalise: first nonzero coefficient = 1
                s = s * pow(int(s[nz[0]]), p - 2, p) % p
                Av, Bv, Cv = sample_zeros(C, val, ok, a.verify, np.random.default_rng(C_.dseed(k, 'verify', f)))
                ev = orbit_matrix([orbs[i] for i in nz], Av, Bv, Cv, p, D) @ s[nz] % p
                rnd = np.random.default_rng(C_.dseed(k, 'nonzero', f)).integers(0, p, size=(3, a.verify))
                evr = orbit_matrix([orbs[i] for i in nz], rnd[0], rnd[1], rnd[2], p, D) @ s[nz] % p
                R['S'] = dict(pervar_degree=D, total_parity=par, total_degree=int(tot), terms=int(terms),
                              orbit_terms=len(nz), kernel_dim=len(ker),
                              verify_zeros_vanish=int((ev == 0).sum()), verify_n=a.verify,
                              random_nonvanish=int((evr != 0).sum()),
                              coeffs=[[*orbs[i], int(s[i])] for i in nz])
                if f == 'x':
                    # PC3: explicit Weierstrass S3 with a4, a6 of the engine's model
                    a4 = (1 - A_sq(C) * pow(3, -1, p)) % p
                    a6 = (C.A * (2 * C.A * C.A - 9) * pow(27, -1, p)) % p
                    x1, x2, x3 = rnd
                    S3 = ((x1 - x2) ** 2 % p * x3 % p * x3 - 2 * (((x1 + x2) * ((x1 * x2 + a4) % p) + 2 * a6) % p) * x3
                          + (x1 * x2 - a4) ** 2 - 4 * a6 * (x1 + x2)) % p
                    ratio = evr * np.array([pow(int(z), p - 2, p) if z else 0 for z in S3]) % p
                    R['PC3_S3_proportional'] = bool(np.unique(ratio[S3 != 0]).size == 1)
            else:
                R['S'] = None
            res[k][f] = R
            print(f'k={k} f={f} maxfib={R["max_rational_fibre"]} split={R["split_fibre_distinct_values"]} '
                  f'S={None if not R["S"] else {x: R["S"][x] for x in ("pervar_degree","total_degree","terms","kernel_dim","verify_zeros_vanish","random_nonvanish")}} '
                  f'{time.time()-t0:.0f}s', file=sys.stderr)
    json.dump(res, open(a.out, 'w'), indent=1, default=int)


def A_sq(C):
    return C.A * C.A % C.p


if __name__ == '__main__':
    main()
    import resource
    print(f'maxrss_MB {resource.getrusage(resource.RUSAGE_SELF).ru_maxrss/1024:.0f}', file=sys.stderr)
