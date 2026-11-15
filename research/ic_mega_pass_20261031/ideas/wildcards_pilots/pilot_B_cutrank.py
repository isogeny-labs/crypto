#!/usr/bin/env python3
"""Pilot B (MP-wildcards-1): cut rank of bit-encoded group-addition tensors.

For a group law written as a 0/1 tensor T[x][y][z] = [x * y = z] with x,y,z
encoded in b bits each, interleave the 3b bits by significance and cut after
the j least significant bit-planes.  The F_2 rank of the 2^{3j} x 2^{3(b-j)}
flattening is the minimal bond dimension of ANY exact tensor-network
contraction along that cut (over F_2).  Groups compared at matched size:
  xor   : F_2^b                     (DLP trivial, expect O(1) rank)
  addp  : Z/p additive              (carry/wrap, expect O(1))
  mulp  : (Z/p)^* multiplicative    (Bryant-type exponential expected)
  ecx   : x-only E(F_p) via Semaev S_3 (expect exponential, like mulp)
  latin : random relabelling of Z/p additive (random Latin square control)
Output JSON with ranks per (group, b, j).
"""
import json, sys, random, time
import numpy as np

def f2_rank_rows(rows_bits, ncols):
    """rows_bits: list of python ints (bitmasks of length ncols). Returns F_2 rank."""
    piv = {}
    r = 0
    for v in rows_bits:
        while v:
            h = v.bit_length() - 1
            if h in piv:
                v ^= piv[h]
            else:
                piv[h] = v
                r += 1
                break
    return r

def interleave_index(x, y, z, b, j):
    """split (x,y,z) b-bit each into low (j planes) and high (b-j planes) indices."""
    mlo = (1 << j) - 1
    lo = (x & mlo) | ((y & mlo) << j) | ((z & mlo) << (2 * j))
    hb = b - j
    hi = (x >> j) | ((y >> j) << hb) | ((z >> j) << (2 * hb))
    return lo, hi

def triples_xor(b):
    N = 1 << b
    for x in range(N):
        for y in range(N):
            yield x, y, x ^ y

def triples_addp(p):
    for x in range(p):
        for y in range(p):
            yield x, y, (x + y) % p

def triples_mulp(p):
    for x in range(1, p):
        for y in range(1, p):
            yield x, y, (x * y) % p

def triples_latin(p, rng):
    perm = list(range(p)); rng.shuffle(perm)
    for x in range(p):
        for y in range(p):
            yield perm[x], perm[y], perm[(x + y) % p]

def triples_ecx(p, a, bb):
    # S_3(x1,x2,x3) for y^2 = x^3 + a x + bb over F_p, all roots x3 of the quadratic.
    inv2 = pow(2, -1, p)
    sq = {}
    for t in range(p):
        sq.setdefault(t * t % p, []).append(t)
    for x1 in range(p):
        for x2 in range(p):
            A = (x1 - x2) * (x1 - x2) % p
            B = (-2 * ((x1 + x2) * (x1 * x2 + a) + 2 * bb)) % p
            C = ((x1 * x2 - a) ** 2 - 4 * bb * (x1 + x2)) % p
            if A == 0:
                if B != 0:
                    yield x1, x2, (-C * pow(B, -1, p)) % p
                continue
            D = (B * B - 4 * A * C) % p
            for s in sq.get(D, []):
                x3 = (-B + s) * pow(2 * A, -1, p) % p
                yield x1, x2, x3

def cut_rank(triples, b, j):
    ncols = 1 << (3 * (b - j))
    rows = {}
    for x, y, z in triples:
        lo, hi = interleave_index(x, y, z, b, j)
        rows[lo] = rows.get(lo, 0) | (1 << hi)
    nz = len(rows)
    return f2_rank_rows(list(rows.values()), ncols), nz, ncols, 1 << (3 * j)

def prime_near(N):
    import sympy
    return int(sympy.prevprime(N))

def main():
    out = []
    rng = random.Random(1)
    bs = [int(s) for s in sys.argv[1:]] or [4, 5, 6, 7]
    for b in bs:
        p = prime_near(1 << b)
        # curve y^2 = x^3 + 2x + 3 unless singular
        a, bb = 2, 3
        for j in range(1, b):
            if 3 * j > 14 or 3 * (b - j) > 20:
                continue
            for name, gen in [("xor", lambda: triples_xor(b)),
                              ("addp", lambda: triples_addp(p)),
                              ("mulp", lambda: triples_mulp(p)),
                              ("latin", lambda: triples_latin(p, rng)),
                              ("ecx", lambda: triples_ecx(p, a, bb))]:
                t0 = time.time()
                rk, nz, ncols, nrows = cut_rank(gen(), b, j)
                rec = dict(group=name, b=b, p=p, j=j, rank_f2=rk, nonzero_rows=nz,
                           rows=nrows, cols=ncols, log2_rank=float(np.log2(max(rk, 1))),
                           secs=round(time.time() - t0, 2))
                out.append(rec)
                print(json.dumps(rec), flush=True)
    json.dump(out, open(sys.argv[0].replace('.py', '_results.json'), 'w'), indent=1)

if __name__ == "__main__":
    main()
