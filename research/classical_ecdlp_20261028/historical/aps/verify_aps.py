"""Independent, tiny-field checks of the APS relation mechanism.

Uses exhaustive root/sign enumeration, NOT a Groebner-basis implementation
or a reproduction of the paper's performance experiments. Standard library only.
"""
from itertools import combinations, product
from math import exp
from random import Random

p, A, B, r = 17, 2, 2, 19
P = (5, 1)


def add(U, V):
    if U is None:
        return V
    if V is None:
        return U
    x, y = U
    z, v = V
    if x == z and (y + v) % p == 0:
        return None
    k = ((v-y)*pow(z-x, -1, p) if x != z
         else (3*x*x+A)*pow(2*y, -1, p)) % p
    X = (k*k-x-z) % p
    return X, (k*(x-X)-y) % p


def neg(U):
    return None if U is None else (U[0], -U[1] % p)


def mul(n, U):
    n %= r
    R = None
    while n:
        if n & 1:
            R = add(R, U)
        U = add(U, U)
        n >>= 1
    return R


def S3(x, y, z):
    return ((x-y)**2*z*z - 2*((x+y)*(x*y+A)+2*B)*z
            + (x*y-A)**2 - 4*B*(x+y)) % p


points = [None] + [(x, y) for x in range(p) for y in range(p)
                   if (y*y-x*x*x-A*x-B) % p == 0]
assert len(points) == r
assert len({mul(k, P) for k in range(r)}) == r

# Every triple of DISTINCT rational x-coordinates: polynomial zero iff
# some choice of signs for the represented rational points sums to O.
by_x = {R[0]: R for R in points if R is not None}
roots = 0
for xyz in combinations(by_x, 3):
    U, V, W = [by_x[x] for x in xyz]
    signed_zero = any(add(add(U, V if e == 1 else neg(V)),
                         W if f == 1 else neg(W)) is None
                      for e, f in product((1, -1), repeat=2))
    assert (S3(*xyz) == 0) == signed_zero
    roots += signed_zero
print(f'S3/sign equivalence: all {len(list(combinations(by_x, 3)))} triples passed; {roots} roots')

# A concrete labeled relation, with w=7 used only to generate Q.
Q = (0, 6)
labels = [(8, 2), (3, 3), (1, 1)]
Rs = [add(mul(u, P), mul(v, Q)) for u, v in labels]
assert add(add(Rs[0], Rs[1]), neg(Rs[2])) is None
assert S3(*(R[0] for R in Rs)) == 0
lam = (labels[0][0] + labels[1][0] - labels[2][0]) % r
mu = (labels[0][1] + labels[1][1] - labels[2][1]) % r
w = -lam * pow(mu, -1, r) % r
assert mul(w, P) == Q
print(f'Toy example: Q={Q}, labeled points={list(zip(labels, Rs))}')
print(f'R1+R2-R3=O; lambda={lam}, mu={mu}; recovered w={w}')

# For fixed R1=3P, R2=5P, R3=8P, test every mask triple, for every w.
# Conditioning on the points leaves the v labels uniform. Exactly 1/r
# of masks have mu=0; every other mask must recover w.
extracted = failures = 0
for secret in range(r):
    for v1, v2, v3 in product(range(r), repeat=3):
        u1, u2, u3 = ((3-secret*v1) % r, (5-secret*v2) % r,
                      (8-secret*v3) % r)
        lam, mu = (u1+u2-u3) % r, (v1+v2-v3) % r
        if not mu:
            assert lam == 0
            failures += 1
        else:
            assert -lam*pow(mu, -1, r) % r == secret
            extracted += 1
assert failures * r == extracted + failures
print(f'Mask audit: {extracted} valid extractions, {failures} zero denominators; fraction=1/{r}')

# A separate scalar-group experiment checks only the random-sum heuristic,
# not polynomial solving cost. All chosen points have different x-classes +/-R.
rng = Random(20261025)
order, t, trials = 10007, 7, 10000
hits = total_relations = 0
for _ in range(trials):
    samples = rng.sample(range(1, (order+1)//2), 3*t)
    V1, V2, V3 = samples[:t], samples[t:2*t], set(samples[2*t:])
    count = 0
    for a in V1:
        for b in V2:
            for e in (1, -1):
                c = (-a-e*b) % order
                if min(c, order-c) in V3:
                    count += 1
    hits += count > 0
    total_relations += count
L = 4*t**3/order
print(f'Random-sum audit: r={order}, total s={3*t}, m=3, trials={trials}')
print(f'Mean signed relations={total_relations/trials:.4f}; heuristic={L:.4f}')
print(f'Success fraction={hits/trials:.4f}; Poisson heuristic={1-exp(-L):.4f}')
