"""Shared arithmetic, deterministic encodings, and measured toy DLP solvers."""
import math
import random
from time import perf_counter
from sage.all import GF, Integer, factor, set_random_seed


def seed_all(seed):
    random.seed(seed)
    set_random_seed(seed)


def field_data(K):
    return {'characteristic': int(K.characteristic()), 'degree': int(K.degree()),
            'order': int(K.cardinality()), 'modulus': str(K.modulus())}


def ec_key(P):
    return () if P.is_zero() else (P[0], P[1])


def point_data(P):
    if P.is_zero():
        return {'infinity': True}
    return {'x': str(P[0]), 'y': str(P[1])}


def project_prime_order(P, annihilator, r):
    """Project via the full r-primary component, then reduce to order r.

    Multiplication by N/r can kill all r-torsion in a noncyclic group with
    r^2 | N. Remove only the factor coprime to r, then descend the r-power
    order. This works for both cyclic and rank-two elliptic-curve groups.
    """
    N=Integer(annihilator); r=Integer(r); exponent=N.valuation(r)
    if not exponent: raise ValueError('Prime does not divide the annihilator')
    S=(N//r**exponent)*P
    for _ in range(exponent):
        if S.is_zero(): return S
        T=r*S
        if T.is_zero(): return S
        S=T
    raise ValueError('Supplied annihilator does not annihilate the point')


def prime_generator(E, N=None, exclude=(), minimum=5):
    N = Integer(E.cardinality() if N is None else N)
    candidates = [int(r) for r, e in factor(N) if r >= minimum and r not in exclude]
    for r in sorted(candidates, reverse=True):
        for _ in range(100):
            P = project_prime_order(E.random_point(),N,r)
            if not P.is_zero() and (r * P).is_zero():
                return P, r
    raise ValueError('No suitable prime-order point found')


def frobenius(P, power):
    if P.is_zero():
        return P
    return P.curve()([P[0]**power, P[1]**power])


def trace_point(P, q, degree):
    T = P.curve()(0)
    U = P
    for _ in range(degree):
        T += U
        U = frobenius(U, q)
    return T


def min_field_degree(P, p, n):
    return next(d for d in range(1, n+1) if n % d == 0 and
                frobenius(P, p**d) == P)


def bsgs_ec(P, Q, order):
    """BSGS in <P>, with explicit counts; Q is never given its secret scalar."""
    start = perf_counter()
    width = math.isqrt(int(order)-1) + 1
    table = {}
    U = P.curve()(0)
    adds = 0
    for j in range(width):
        table.setdefault(ec_key(U), j)
        U += P
        adds += 1
    step = width * P
    V = Q
    for i in range(width+1):
        j = table.get(ec_key(V))
        if j is not None:
            answer = (i*width+j) % int(order)
            if answer*P == Q:
                return {'scalar': answer, 'seconds': perf_counter()-start,
                        'group_additions': adds, 'baby_steps': len(table),
                        'algorithm': 'BSGS', 'interval_size': int(order)}
        V -= step
        adds += 1
    raise ValueError('Target is not in the supplied cyclic subgroup')


def bsgs_generic(P, Q, order):
    """BSGS for Jacobian points. Verify equality after a representation match.

    Hess reduced divisors provide deterministic representations for the examples
    used here. Equality is checked, so a representation collision is harmless.
    """
    start = perf_counter()
    width = math.isqrt(int(order)-1)+1
    table = {}
    U = 0 * P
    adds = 0
    for j in range(width):
        table.setdefault(str(U), []).append((j, U))
        U += P
        adds += 1
    step = width * P
    V = Q
    for i in range(width+1):
        for j, W in table.get(str(V), []):
            if V == W:
                answer = (i*width+j) % int(order)
                if answer*P == Q:
                    return {'scalar': answer, 'seconds': perf_counter()-start,
                            'group_additions': adds, 'baby_steps': len(table),
                            'algorithm': 'BSGS', 'interval_size': int(order)}
        V -= step
        adds += 1
    raise ValueError('No logarithm found with the supplied bound/representation')


def timed(call):
    start = perf_counter()
    value = call()
    return value, perf_counter() - start
