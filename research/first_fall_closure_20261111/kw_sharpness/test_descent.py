"""Sanity: the descended system evaluates to S_4 (random points), degree <= 6, top form equals closed formula."""
import random, numpy as np
from kwgf import GF
import ffd, semaev

def rank_indep_nus(F, npr, rnd):
    while True:
        nus = [rnd.randrange(1, F.q) for _ in range(npr)]
        # F_2-independence
        rk = ffd.rank_int([v for v in nus])
        if rk == npr: return nus

def eval_s4(F, a6, c, xs):
    tot = 0
    for mon in ffd.s4_terms():
        t = F.pow(a6, mon[0]) if mon[0] else 1
        for i in range(3): t = F.mul(t, F.pow(xs[i], mon[1 + i]) if mon[1 + i] else 1)
        t = F.mul(t, F.pow(c, mon[4]) if mon[4] else 1)
        tot ^= t
    return tot

if __name__ == '__main__':
    rnd = random.Random(7)
    for n, npr in [(7, 3), (9, 3), (10, 4), (13, 5)]:
        F, a2, a6, c = ffd.random_instance(n, rnd)
        nus = rank_indep_nus(F, npr, rnd)
        N = 3 * npr; M = ffd.masks(N)
        s = ffd.descend(F, nus, c, a6)
        # evaluate at random Boolean points
        for _ in range(200):
            y = [rnd.randrange(2) for _ in range(N)]
            xs = []
            for k in range(3):
                x = 0
                for l in range(npr):
                    if y[k * npr + l]: x ^= nus[l]
                xs.append(x)
            ymask = sum(b << i for i, b in enumerate(y))
            val = 0
            for k in range(n):
                # s_k(y) = parity of monomials mask subset of ymask
                sk = s[k]; par = 0
                # enumerate submasks
                sub = ymask
                while True:
                    par ^= (sk >> sub) & 1
                    if sub == 0: break
                    sub = (sub - 1) & ymask
                val |= par << k
            assert val == eval_s4(F, a6, c, xs), "descent mismatch"
        degs = [ffd.degree(v, M) for v in s]
        tf = ffd.top_formula(F, nus, c)
        ok = all((s[k] & M.deg[6]) == tf[k] for k in range(n))
        print("n=%d n'=%d: descent matches S4 on 200 points; degrees of s_k = %s; degree-6 part equals c*prod(Moore) formula: %s" % (n, npr, degs, ok))
        assert ok and max(degs) <= 6
    print("test_descent OK")
