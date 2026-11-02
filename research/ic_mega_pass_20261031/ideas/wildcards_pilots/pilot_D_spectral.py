#!/usr/bin/env python3
"""Pilot D (MP-wildcards-3): spectral gap of the coarse-grained translation-by-P
operator on a toy Koblitz curve as a universal bound on x-functional filter gain.

Enumerate the prime-order subgroup <P> of K_0: y^2+xy=x^3+1 over F_{2^n}
(a=0, cofactor 4).  Partition points by C=2^w buckets defined by
  (L) w random F_2-linear functionals of x(R)             (trace filters)
  (Q) w random F_2-linear functionals of x(R)^3 + c x(R)  (cubic filter)
  (R) random partition with the same bucket sizes          (null)
Build the C x C row-stochastic transition matrix T[c][c'] of R -> R+P,
report |lambda_2|(T), the single-bucket gain G = C * max_c,c' T[c][c'], and the
Frobenius-norm distance ||T - J/C||_F, against the random-partition null.
"""
import json, sys, random, time
import numpy as np
import cypari2
pari = cypari2.Pari()
pari.allocatemem(2**29)

def setup(n):
    g = pari(f"ffgen(Mod(1,2)*ffinit(2,{n}),'g)")
    A1 = {11: 1, 17: 1}.get(n, 0); cof = 2 if A1 else 4
    E = pari.ellinit([1, g**0 if A1 else 0, 0, 0, g**0])
    card = int(pari.ellcard(E))
    r = card // cof
    assert pari.isprime(r), (n, card)
    while True:
        Q = pari.random(E)
        P = pari.ellmul(E, Q, cof)
        if P != 0 and int(pari.ellorder(E, P)) == r:
            break
    return g, E, P, r

def ff_to_int(x, n=None):
    sx = str(x)
    if sx in ('0', '1'):
        return int(sx)
    pol = pari('(%s).pol' % sx)
    return int(pari.fromdigits(pari.Vec(pari.lift(pol)), 2))

def enumerate_x(E, P, r, n):
    xs = np.zeros(r, dtype=np.int64)
    R = P
    for j in range(1, r):
        xs[j] = ff_to_int(R[0], n)
        R = pari.elladd(E, R, P)
    assert R == 0
    return xs  # xs[0] = point at infinity (unused)

def lin_functionals(xs, n, w, rng):
    # bucket(x) = bits of L x for random w x n F_2 matrix
    L = [rng.getrandbits(n) for _ in range(w)]
    b = np.zeros_like(xs)
    for i, row in enumerate(L):
        par = np.zeros_like(xs)
        v = xs & row
        # parity of popcount
        for k in range(n):
            par ^= (v >> k) & 1
        b |= par << i
    return b

def gf2n_mul_table(n):
    # irreducible modulus from PARI ffinit is unknown as int; instead use PARI for cubing.
    pass

def cube_plus_cx(E, P, r, n, c):
    xs = np.zeros(r, dtype=np.int64)
    R = P
    cc = c
    for j in range(1, r):
        x = R[0]
        xs[j] = ff_to_int(x**3 + cc * x, n)
        R = pari.elladd(E, R, P)
    return xs

def stats(buckets, C):
    r = len(buckets) - 1
    a = buckets[1:r]
    bnext = buckets[2:r + 1]
    T = np.zeros((C, C))
    np.add.at(T, (a, bnext), 1.0)
    rows = T.sum(1, keepdims=True)
    rows[rows == 0] = 1
    Tn = T / rows
    ev = np.linalg.eigvals(Tn)
    ev = sorted(np.abs(ev), reverse=True)
    lam2 = float(ev[1]) if len(ev) > 1 else 0.0
    gain = float(C * Tn.max())
    dist = float(np.linalg.norm(Tn - 1.0 / C))
    return dict(lambda2=lam2, gain=gain, frob_dist=dist, n_pairs=int(T.sum()))

def main():
    ns = [int(s) for s in sys.argv[1:]] or [13, 15, 17]
    rng = random.Random(7)
    out = []
    for n in ns:
        t0 = time.time()
        g, E, P, r = setup(n)
        xs = enumerate_x(E, P, r, n)
        c = g**5
        xs3 = cube_plus_cx(E, P, r, n, c)
        for w in [3, 4, 5, 6]:
            C = 1 << w
            if C * C * 4 > r:
                continue
            for name, arr in [("linear_x", xs), ("cubic_x", xs3)]:
                for trial in range(3):
                    b = lin_functionals(arr, n, w, rng)
                    s = stats(b, C); s.update(n=n, r=r, w=w, filter=name, trial=trial)
                    out.append(s); print(json.dumps(s), flush=True)
            # null: random partition matched to bucket sizes of a linear filter
            for trial in range(3):
                b = lin_functionals(xs, n, w, rng)
                # null = random FUNCTION OF x with matched bucket sizes (keeps x(jP)=x((r-j)P) symmetry)
                ux, inv = np.unique(xs[1:], return_inverse=True)
                lab = np.array([b[1:][np.where(inv == i)[0][0]] for i in range(len(ux))], dtype=np.int64)
                lab = lab.copy(); rng.shuffle(lab)
                bnull = np.concatenate([[0], lab[inv]])
                s = stats(bnull, C); s.update(n=n, r=r, w=w, filter="random_null", trial=trial)
                out.append(s); print(json.dumps(s), flush=True)
        print("# n", n, "secs", round(time.time() - t0, 1), flush=True)
    json.dump(out, open(sys.argv[0].replace('.py', '_results.json'), 'w'), indent=1)

if __name__ == "__main__":
    main()
