import random
from tower_engine import *
from towers import *

def eval_m(mm, xs, p):
    r = 1
    for i, x in enumerate(xs):
        if mm >> i & 1: r = r * x % p
    return r

p = find_prime(); sq = sq_table(p); print('p', p)
rng = random.Random(7)
for t in (3, 4, 5, 6):
    E = build_tower(p, t, rng, sq)
    xi, tau, c = velu_data(p, t, E); V = base_V(p, t, E)
    check_tower(p, t, E, xi, tau, c, V)
    pts = chain_points(p, t, xi, tau, V)
    R = Rules('velu', t, p, xi, tau, c)
    bad = n = 0
    for m in range(1 << t):
        for j in range(t):
            d = R.mult1(m, j)
            for xs in pts:
                lhs = sum(cf * eval_m(mm, xs, p) for mm, cf in d.items()) % p
                n += 1; bad += (lhs != xs[j] * eval_m(m, xs, p) % p)
    print('G3 ok t=%d; G2 products checked %d bad %d' % (t, n, bad), flush=True)
