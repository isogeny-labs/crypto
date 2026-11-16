"""Parameter count for the "small-x factor base through the RR parametrisation" idea.

E: y^2 = x^3 + A x + B over F_p, m = 3 summands, f = x^2 + a x + b y + c in L(4O),
c pinned by f(-R) = 0.  Norm(f) = (x^2 + a x + c)^2 - b^2 (x^3 + A x + B)
= (x - x_R) F_{a,b}(x), with F monic cubic, coefficients c_k(a,b) quadratic in (a,b).

Relation iff F_{a,b} splits with roots x_1,x_2,x_3 in [0,B] that are abscissas.
Necessary box condition: c_1 = -e_1 in [-3B, 0], c_2 = e_2 in [0, 3B^2], c_3 = -e_3 in [-B^3, 0].

This script enumerates every (a,b) in F_p^2 for small p, counts
  N_box  : pairs whose (c_1,c_2,c_3) mod p lie in the boxes (what a lattice can see),
  N_split: pairs whose F splits over F_p with all roots in [0,B] (true Vieta image),
  N_rel  : pairs giving a genuine relation (roots are abscissas of points),
per random target, and compares N_box with the heuristic p^2 * prod(|box_k|/p).
"""
import sys, json, random
import numpy as np

def legendre_sqrt(v, p):
    # p % 4 == 3 assumed
    r = pow(v, (p + 1) // 4, p)
    return r if (r * r) % p == v % p else None

def run(p, A, Bc, seed, targets):
    rng = random.Random(seed)
    Bb = int(round(p ** (1.0 / 3)))
    box1 = 3 * Bb + 1          # e1 in [0,3B]
    box2 = 3 * Bb * Bb + 1     # e2 in [0,3B^2]
    box3 = Bb ** 3 + 1         # e3 in [0,B^3]
    # factor base abscissas: x in [0,B] with x^3+Ax+Bc a square
    fb = [x for x in range(0, Bb + 1) if legendre_sqrt((x ** 3 + A * x + Bc) % p, p) is not None]
    # table of monic cubics X^3 - e1 X^2 + e2 X - e3 with roots in [0,B] (ordered-free sets with repetition)
    vieta = {}
    xs = list(range(0, Bb + 1))
    for i in range(len(xs)):
        for j in range(i, len(xs)):
            for k in range(j, len(xs)):
                x1, x2, x3 = xs[i], xs[j], xs[k]
                key = ((x1 + x2 + x3) % p, (x1 * x2 + x1 * x3 + x2 * x3) % p, (x1 * x2 * x3) % p)
                vieta.setdefault(key, []).append((x1, x2, x3))
    fbset = set(fb)
    a = np.arange(p, dtype=np.int64)
    aa, bb = np.meshgrid(a, a, indexing='ij')
    aa = aa.ravel(); bb = bb.ravel()
    rows = []
    for _ in range(targets):
        # random target point with x_R not an integer in [0,B] (a genuine target)
        while True:
            xr = rng.randrange(Bb + 1, p)
            yr = legendre_sqrt((xr ** 3 + A * xr + Bc) % p, p)
            if yr is not None:
                break
        # c = -(xr^2 + a xr - b yr)  (f(-R)=0 with -R=(xr,-yr))
        c = (-(xr * xr) - aa * xr + bb * yr) % p
        # Norm(f) = (x^2 + a x + c)^2 - b^2 (x^3 + A x + Bc) = x^4 + 2a x^3 + (a^2+2c - b^2) x^2 + (2ac - A b^2) x + (c^2 - Bc b^2)
        n3 = (2 * aa - bb * bb) % p
        n2 = (aa * aa + 2 * c) % p
        n1 = (2 * aa * c - A * bb * bb) % p
        n0 = (c * c - Bc * bb * bb) % p
        # divide by (x - xr): F = x^3 + f2 x^2 + f1 x + f0
        f2 = (n3 + xr) % p
        f1 = (n2 + xr * f2) % p
        f0 = (n1 + xr * f1) % p
        rem = (n0 + xr * f0) % p
        assert not rem.any()
        e1 = (-f2) % p; e2 = f1 % p; e3 = (-f0) % p
        inbox = (e1 < box1) & (e2 < box2) & (e3 < box3)
        n_box = int(inbox.sum())
        n_split = 0; n_rel = 0
        for idx in np.nonzero(inbox)[0]:
            key = (int(e1[idx]), int(e2[idx]), int(e3[idx]))
            if key in vieta:
                n_split += 1
                if any(all(x in fbset for x in trip) for trip in vieta[key]):
                    n_rel += 1
        rows.append((n_box, n_split, n_rel))
    heur = p * p * (box1 / p) * (box2 / p) * (min(box3, p) / p)
    r = np.array(rows, dtype=float)
    return {
        'p': p, 'B': Bb, 'factor_base_size': len(fb), 'boxes': [box1, box2, box3],
        'targets': targets,
        'N_box_mean': float(r[:, 0].mean()), 'N_box_heuristic': heur,
        'N_split_mean': float(r[:, 1].mean()), 'N_rel_mean': float(r[:, 2].mean()),
        'N_rel_heuristic_C(fb,3)/p': (len(fb) * (len(fb) - 1) * (len(fb) - 2) / 6) / p,
        'selectivity_loss_box_over_split': float(r[:, 0].mean() / max(r[:, 1].mean(), 1e-9)),
        'p^((m-1)/2)': p ** 1.0,
    }

if __name__ == '__main__':
    out = []
    for p in (503, 1019, 2027, 4091, 8191):
        # p % 4 == 3 for easy sqrt
        assert p % 4 == 3
        res = run(p, A=2, Bc=3, seed=20261115 + p, targets=8)
        print(json.dumps(res))
        out.append(res)
    json.dump(out, open(sys.argv[1], 'w') if len(sys.argv) > 1 else sys.stdout, indent=1)
