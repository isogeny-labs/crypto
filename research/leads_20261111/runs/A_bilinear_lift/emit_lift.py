#!/usr/bin/env python3
"""Emit Boolean systems for LEAD-A-bilinear-lift-size-degree (lane A, Lead A5) in polyclose.c format.

Instances are lfd_small.make_instance(n, seed) (research/lfd_lower_bound_20261111/lfd_small.py), the
same generator as the NOTE.md tables.  Variables of the original system in (A,C) coordinates:
a_0..a_{k-1} = x_0..x_{k-1}, c_0..c_{k-1} = x_k..x_{2k-1} (X1 = A, X2 = A + C; lfd_small.descend 'ac').
Each coordinate function f_s is  sum_ij M_s[i][j] a_i c_j + lin_s(a) + lin_s(c) + const  (checked).

Modes (stdout = polyclose input; a one-line JSON header goes to stderr):
  orig            the n descent equations (a,c), N = 2k variables
  bil  [--S ...]  bilinear lift: variables y_ij (i in S, all j) appended after x_{2k-1};
                  every a_i c_j with i in S replaced by y_ij; axioms y_ij + a_i c_j.  S default = all i.
  ext  --D d      Lemma 2 lift exactly as ext_check.build_ext (reduced basis, degree <= d monomials)
  rand / randbil  negative control: n random affine forms of the same shape (random M_s, lin, const),
                  seed = --rseed; 'rand' is the bilinear system, 'randbil' its full lift.
Options --S takes a list of a-indices for a partial lift (mode bil and randbil); --P i,j ... adds single
lifted pairs (exploratory, not preregistered).
"""
import argparse, itertools, json, os, random, sys
import numpy as np
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'lfd_lower_bound_20261111'))
from lfd_small import make_instance, descend, reduced_basis, popcount  # noqa: E402
from ext_check import build_ext  # noqa: E402


def bits(m):
    out = []
    while m:
        low = m & -m
        out.append(low.bit_length() - 1)
        m ^= low
    return out


def split_bilinear(polys, k):
    """polys: list of monomial-mask lists over 2k vars -> list of (Mset, linset, const)."""
    out = []
    amask = (1 << k) - 1
    for p in polys:
        M, lin, const = set(), set(), 0
        for m in p:
            d = popcount(m)
            if d == 0:
                const ^= 1
            elif d == 1:
                lin ^= {bits(m)[0]}
            elif d == 2:
                i, j = bits(m)
                if not (i < k <= j):
                    raise SystemExit('not bilinear in (a,c): monomial x%d x%d' % (i, j))
                M ^= {(i, j - k)}
            else:
                raise SystemExit('degree > 2 in descent')
        out.append((M, lin, const))
    return out


def write_system(NV, polys):
    """polys: list of lists of monomials, monomial = tuple of indices."""
    lines = ['%d %d' % (NV, len(polys))]
    for p in polys:
        lines.append(' '.join([str(len(p))] + [' '.join(map(str, (len(m),) + tuple(m))) for m in p]))
    sys.stdout.write('\n'.join(lines) + '\n')


def lift(parts, k, S, pairs=None):
    S = sorted(S)
    yidx = {}
    for i in S:
        for j in range(k):
            yidx[(i, j)] = 2 * k + len(yidx)
    for (i, j) in (pairs or []):
        if (i, j) not in yidx:
            yidx[(i, j)] = 2 * k + len(yidx)
    NV = 2 * k + len(yidx)
    polys = []
    for M, lin, const in parts:
        p = []
        for (i, j) in sorted(M):
            p.append((yidx[(i, j)],) if (i, j) in yidx else (i, k + j))
        p += [(v,) for v in sorted(lin)]
        if const:
            p.append(())
        polys.append(p)
    for (i, j), y in yidx.items():
        polys.append([(y,), (i, k + j)])
    return NV, polys


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('mode', choices=['orig', 'bil', 'ext', 'rand', 'randbil'])
    ap.add_argument('--n', type=int, required=True)
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--S', type=int, nargs='*', default=None)
    ap.add_argument('--D', type=int, default=None)
    ap.add_argument('--rseed', type=int, default=None)
    ap.add_argument('--P', nargs='*', default=None, help='explicit lifted pairs i,j (exploratory; with --S for the rows)')
    a = ap.parse_args()
    inst = make_instance(a.n, a.seed)
    k = inst['k']
    hdr = dict(mode=a.mode, n=a.n, k=k, seed=a.seed, a6=inst['a6'], z=inst['z'], V=inst['V'])
    if a.mode in ('orig', 'bil'):
        polys, N, nz = descend(inst, 'ac')
        _, _, nz_ab = descend(inst, 'ab')
        assert nz == nz_ab
        hdr['sols'] = nz
        parts = split_bilinear(polys, k)
        if a.mode == 'orig':
            NV, P = lift(parts, k, [])
        else:
            S = list(range(k)) if (a.S is None and a.P is None) else (a.S or [])
            pairs = [tuple(map(int, t.split(','))) for t in (a.P or [])]
            hdr['S'] = S
            hdr['P'] = pairs
            NV, P = lift(parts, k, S, pairs)
    elif a.mode == 'ext':
        polys, N, nz = descend(inst, 'ab')
        hdr['sols'] = nz
        gens = reduced_basis(polys, N)
        ext, NV, nax = build_ext(gens, N, a.D)
        hdr.update(D_lift=a.D, axioms=nax)
        P = [[tuple(bits(m)) for m in g] for g in ext]
    else:
        rng = random.Random(a.rseed)
        parts = []
        for _ in range(a.n):
            M = {(i, j) for i in range(k) for j in range(k) if rng.randrange(2)}
            lin = {v for v in range(2 * k) if rng.randrange(2)}
            parts.append((M, lin, rng.randrange(2)))
        # brute-force solution count over 2^(2k)
        N = 2 * k
        idx = np.arange(1 << N, dtype=np.int64)
        xb = [((idx >> v) & 1).astype(np.uint8) for v in range(N)]
        alive = np.ones(1 << N, dtype=bool)
        for M, lin, c in parts:
            val = np.full(1 << N, c, dtype=np.uint8)
            for (i, j) in M:
                val ^= xb[i] & xb[k + j]
            for v in lin:
                val ^= xb[v]
            alive &= (val == 0)
        hdr.update(rseed=a.rseed, sols=int(alive.sum()))
        S = (list(range(k)) if (a.S is None and a.P is None) else (a.S or [])) if a.mode == 'randbil' else []
        pairs = [tuple(map(int, t.split(','))) for t in (a.P or [])] if a.mode == 'randbil' else []
        hdr['S'] = S
        hdr['P'] = pairs
        NV, P = lift(parts, k, S, pairs)
    hdr['NV'] = NV
    hdr['NP'] = len(P)
    sys.stderr.write(json.dumps(hdr) + '\n')
    write_system(NV, P)


if __name__ == '__main__':
    main()
