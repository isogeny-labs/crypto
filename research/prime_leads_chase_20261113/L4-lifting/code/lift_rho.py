"""Q2 supplement: height ratio rho_G of genuine xedni-style lifts at crypto size.

7-coefficient fit  a y^2 + a1 x y + a3 y = b x^3 + a2 x^2 + a4 x + a6  through k = 6 centred integer
lifts of random points of E: y^2 = x^3 + A x + B over F_p (p of PBITS bits). The 1-dim integer kernel
reduces to E mod p automatically. Converted to integral short-ish Weierstrass, then Gram/SVP.
usage: lift_rho.py SEED NCURVES PBITS NDOUBLE OUT.jsonl
"""
import sys, json, random, time
from math import gcd
import sympy
import numpy as np
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from heights import Curve, gram, svp_gram
from xedni_grow import is_prime


def main():
    seed, ncurves, pbits, nd, outp = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
    rng = random.Random(seed)
    fo = open(outp, 'w')
    for ci in range(ncurves):
        t0 = time.time()
        while True:
            p = rng.randrange(1 << (pbits - 1), 1 << pbits)
            if p % 4 == 3 and is_prime(p):
                break
        A, B = rng.randrange(p), rng.randrange(p)
        pts = []
        while len(pts) < 6:
            x = rng.randrange(p)
            f = (x ** 3 + A * x + B) % p
            if pow(f, (p - 1) // 2, p) == 1:
                y = pow(f, (p + 1) // 4, p)
                cl = lambda v: v - p if v > p // 2 else v
                pts.append((cl(x), cl(y)))
        # unknowns (a, a1, a3, b, a2, a4, a6): a y^2 + a1 x y + a3 y - b x^3 - a2 x^2 - a4 x - a6 = 0
        Mx = sympy.Matrix([[y * y, x * y, y, -x ** 3, -x * x, -x, -1] for (x, y) in pts])
        ns = Mx.nullspace()
        if len(ns) != 1:
            continue
        v = ns[0]
        den = sympy.ilcm(*[t.q for t in v])
        v = [int(t * den) for t in v]
        g = 0
        for t in v:
            g = gcd(g, t)
        a, a1, a3, b, a2, a4, a6 = [t // g for t in v]
        # check reduction == E mod p up to scalar: a1=a3=a2=0, b=a, a4=A a, a6=B a (mod p)
        red_ok = all(t % p == 0 for t in (a1, a3, a2)) and (b - a) % p == 0 and (a4 - A * a) % p == 0 and (a6 - B * a) % p == 0
        ab = a * b
        # W^2 + a1 X W + a3' W = X^3 + a2' X^2 + a4' X + a6'
        a3p, a2p, a4p, a6p = a3 * ab, a * a2, a * a4 * ab, a * a6 * ab * ab
        b2 = a1 * a1 + 4 * a2p
        b4 = 2 * a4p + a1 * a3p
        b6 = a3p * a3p + 4 * a6p
        E = Curve(b2, 8 * b4, 16 * b6)
        Q = []
        for (x, y) in pts:
            X = ab * x
            W = ab * a * y
            U = 4 * X
            Z = 8 * W + 4 * a1 * X + 4 * a3p
            Q.append((sympy.Rational(U), sympy.Rational(Z)))
        from fractions import Fraction as Fr
        Q = [(Fr(int(U)), Fr(int(Z))) for (U, Z) in Q]
        assert all(E.on(P) for P in Q), 'transform'
        M, err = gram(E, Q, nd)
        w = np.linalg.eigvalsh(M)
        rec = {'seed': seed, 'pbits': pbits, 'nd': nd, 'red_ok': red_ok, 'coef_bits': max(abs(t).bit_length() for t in (a, a1, a3, b, a2, a4, a6)),
               'model_bits': [abs(E.a2).bit_length(), abs(E.a4).bit_length(), abs(E.a6).bit_length()],
               'heights': np.diag(M).tolist(), 'maxerr': float(err.max()), 'min_eig_rel': float(w.min() / w.max())}
        if w.min() > 0:
            lam, cv = svp_gram(M)
            rec.update({'lambda_G': lam, 'svp': cv.tolist(), 'H': float(np.max(np.diag(M))), 'rho_G': float(np.max(np.diag(M)) / lam)})
            # also rho for the 5-point sub-configuration (s=5 rank, k=6 is the Model-X shape)
        rec['sec'] = time.time() - t0
        fo.write(json.dumps(rec) + '\n'); fo.flush()
        print(json.dumps({k: rec.get(k) for k in ('pbits', 'red_ok', 'coef_bits', 'maxerr', 'min_eig_rel', 'rho_G', 'H', 'lambda_G', 'sec')}), flush=True)


if __name__ == '__main__':
    main()
