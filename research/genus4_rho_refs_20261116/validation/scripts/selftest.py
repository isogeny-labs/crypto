import random, sys
import os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from jac import *
random.seed(7)
def count(p, f, k):
    # points on y^2=f over F_p (k=1) incl. infinity
    n = 1
    for x in range(p):
        y2 = peval(f, x, p)
        n += 1 if y2 == 0 else (2 if pow(y2, (p-1)//2, p) == 1 else 0)
    return n
def count2(p, f):
    # over F_{p^2} = F_p[s]/(s^2 - nu)
    nu = next(a for a in range(2, p) if pow(a, (p-1)//2, p) == p-1)
    n = 1
    for a in range(p):
        for b in range(p):
            ra, rb = 0, 0
            for c in reversed(f):
                ra, rb = (ra*a + nu*rb*b + c) % p, (ra*b + rb*a) % p
            if ra == 0 and rb == 0: n += 1
            else:
                norm = (ra*ra - nu*rb*rb) % p
                n += 2 if pow(norm, (p-1)//2, p) == 1 else 0
    return n
def rand_div(J):
    D = J.zero
    for _ in range(J.g):
        while True:
            x = random.randrange(J.p); s = sqrt_ts(peval(J.f, x, J.p), J.p)
            if s is not None: break
        D = J.add(D, J.point(x, s if random.random()<.5 else (-s) % J.p))
    return D
ok = True; ntest = 0
for p in (31, 43, 101):
    for trial in range(3):
        f = [random.randrange(p) for _ in range(5)] + [1]
        J = Jac(p, f)
        if not J.squarefree: continue
        N1, N2 = count(p, f, 1), count2(p, f)
        order = (N1*N1 + N2)//2 - p; ntest += 1; print("g2", p, f, "N1", N1, "N2", N2, "#J", order)
        for _ in range(10):
            D = rand_div(J); assert J.valid(D)
            if not J.is_zero(J.mul(D, order)): ok = False; print('FAIL order', p, f, order)
            E, F = rand_div(J), rand_div(J)
            if J.add(J.add(D, E), F) != J.add(D, J.add(E, F)): ok = False; print('FAIL assoc')
            if J.add(D, E) != J.add(E, D): ok = False; print('FAIL comm')
            if not J.is_zero(J.add(D, J.neg(D))): ok = False; print('FAIL inverse')
            a, b = random.randrange(1, 10**6), random.randrange(1, 10**6)
            if J.add(J.mul(D, a), J.mul(D, b)) != J.mul(D, a+b): ok = False; print('FAIL linear')
# genus 4 consistency at a moderate prime
p = 2053
f = random_f(p, 4, 40); J = Jac(p, f)
for _ in range(5):
    D, E, F = rand_div(J), rand_div(J), rand_div(J)
    assert J.valid(D) and J.valid(J.add(D, E)) and J.valid(J.add(D, D))
    if J.add(J.add(D, E), F) != J.add(D, J.add(E, F)): ok = False; print('FAIL g4 assoc')
    a, b = random.randrange(1, 10**12), random.randrange(1, 10**12)
    if J.add(J.mul(D, a), J.mul(D, b)) != J.mul(D, a+b): ok = False; print('FAIL g4 linear')
print('selftest', 'PASS' if ok else 'FAIL')
