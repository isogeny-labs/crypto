import itertools
import os
import sys
import unittest

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from l01.rng import mix64, SM64, stream
from l01.field import Counter, F
from l01.instance import (random_f, squarefree, point_counts, jac_order, Tables, chitab_plain)
from l01.curve import Curve, IDENT
from l01 import native, pencil, verify, linalg
from l01.rows import canon


def mk(p, g, s):
    f = random_f(p, g, s)
    T = Tables(p, g, f)
    return f, T, Curve(p, g, f, T)


def full_allowed(T):
    al = [False] * T.p
    for x in T.B + T.Z:
        al[x] = True
    return al


def relations_of(T, g, anchors):
    ctr = Counter()
    Fq = F(T.p, ctr)
    info, pts, cnt = pencil.sieve_tuple(T, Fq, g, anchors, full_allowed(T), T.p - 1, True)
    cands = [lam for lam in range(T.p) if cnt[lam] == g + 1]
    return info, pts, cands, ctr


class T01(unittest.TestCase):
    def test01_mix64(self):
        self.assertEqual(mix64(0), 0xe220a8397b1dcdaf)
        self.assertEqual(mix64(1), 0x910a2dec89025cc1)

    def test02_random_f(self):
        self.assertEqual(random_f(1031, 2, 1), [85, 928, 641, 833, 893, 1])
        self.assertEqual(random_f(131, 2, 1), [31, 15, 114, 98, 48, 1])
        self.assertEqual(random_f(13, 2, 1), [3, 3, 1, 0, 9, 1])
        self.assertEqual(random_f(7, 3, 1), [2, 2, 4, 0, 0, 2, 4, 1])
        self.assertIs(squarefree(random_f(11, 2, 1), 11), False)
        self.assertIs(squarefree(random_f(11, 2, 2), 11), True)

    def test03_jacobian_order(self):
        cases = [(13, 2, 1, [16, 170], 200), (11, 2, 2, [10, 114], 96),
                 (7, 3, 1, [11, 55, 323], 555), (7, 3, 2, [7, 51, 388], 316)]
        for p, g, s, N, J in cases:
            f, T, C = mk(p, g, s)
            Nk = point_counts(f, p, g)
            self.assertEqual(Nk, N)
            self.assertEqual(jac_order(Nk, p, g), J)
            # brute force count of reduced Mumford pairs
            cnt = 0
            vc = Curve(p, g, f, T)
            for d in range(g + 1):
                for uc in itertools.product(range(p), repeat=d):
                    u = list(uc) + [1]
                    for vcoef in itertools.product(range(p), repeat=d):
                        v = list(vcoef)
                        while v and v[-1] == 0:
                            v.pop()
                        t = vc.k_sub(f, vc.k_mul(v, v))
                        if vc.k_rem(t, u) == []:
                            cnt += 1
            self.assertEqual(cnt, J, (p, g, s))
            rng = SM64(12345)
            for _ in range(8):
                D = C.random_divisor(rng)
                self.assertEqual(C.mul_ltr(D, J), IDENT)

    def test04_batch_inv(self):
        c = Counter()
        Fq = F(101, c)
        vals = [3, 17, 55, 99, 100]
        out = Fq.batch_inv(vals)
        for v, o in zip(vals, out):
            self.assertEqual(v * o % 101, 1)
        self.assertEqual((c.mul, c.inv), (12, 1))
        c2 = Counter()
        F(101, c2).batch_inv([7])
        self.assertEqual((c2.mul, c2.inv), (0, 1))

    def test05_interpolate(self):
        for (p, g, s, anchors, a0, Pi, mulc) in [
            (13, 2, 1, [(0, 4), (1, 2)], [4, 11], [0, 12, 1], 10),
            (101, 2, 1, [(0, 43), (3, 5)], [43, 21], [0, 98, 1], 10),
            (101, 3, 1, [(1, 44), (2, 13), (3, 50)], [42, 69, 34], [95, 11, 95, 1], 27)]:
            c = Counter()
            Fq = F(p, c)
            ra0, rPi = pencil.interpolate(Fq, g, anchors)
            self.assertEqual(ra0, a0)
            self.assertEqual(rPi, Pi)
            self.assertEqual((c.mul, c.inv), (mulc, 1))

    def test06_fd(self):
        p = 101
        coef = [5, 17, 99, 3]
        c = Counter()
        Fq = F(p, c)
        fd = pencil.FD(Fq, coef)
        self.assertEqual(c.mul, 12)
        h = F(p, Counter())
        for x in range(101):
            self.assertEqual(fd.D[0], h.horner(coef, x))
            before = c.fd_add
            fd.step()
            self.assertEqual(c.fd_add - before, 3)

    def _pencil_example(self, p, g, anchors):
        f, T, C = mk(p, g, 1)
        info, pts, cands, ctr = relations_of(T, g, anchors)
        return f, T, C, info, pts, cands, ctr

    def test07_08_pencil_examples(self):
        for (p, g, anchors, nrel) in [(13, 2, [(0, 4), (1, 2)], 1),
                                      (101, 2, [(0, 43), (3, 5)], 12),
                                      (101, 3, [(1, 44), (2, 13), (3, 50)], 3)]:
            f, T, C, info, pts, cands, ctr = self._pencil_example(p, g, anchors)
            self.assertEqual(len(cands), nrel)
            # count formula (test 8)
            expect = pencil.interp_mul(g) + pencil.fd_init_mul(g) + 3 * (info["nX"] - 1) + info["npts"]
            self.assertEqual(info["mul"], expect)
            self.assertEqual(info["inv"], 2)
            vc = Curve(p, g, f, T)
            rel = set()
            for lam in cands:
                points = list(anchors) + sorted(pts[lam])
                rel.add((lam, tuple(points)))
                self.assertTrue(verify.v1_pencil(p, f, g, info["a0"], info["Pi"], lam, points))
                self.assertEqual(verify.v2_sum(vc, points), IDENT)
            if p == 13:
                self.assertEqual(cands, [10])
                self.assertEqual(sorted(pts[10]), [(6, 6), (8, 2), (11, 3)])
                lam = 10
                a = [(info["a0"][k] + lam * info["Pi"][k]) % p for k in range(g)] + [lam]
                self.assertEqual(a, [4, 1, 10])
            nv = pencil.p_naive1(T, g, 1, 1, anchors)
            self.assertEqual(nv["relset"], rel)

    def test09_native(self):
        chi13, chi7 = chitab_plain(13), chitab_plain(7)

        def dc(p, chi, u):
            n = Counter()
            r = native.disc_character(p, u, chi, n)
            return r, n

        r, n = dc(13, chi13, [2, 10, 1]); self.assertEqual((r, n.mul, n.add), (1, 7, 2))
        n = Counter(); self.assertTrue(native.splits_completely(13, [2, 10, 1], n)); self.assertEqual(n.mul, 22)
        r, n = dc(7, chi7, [1, 0, 1]); self.assertEqual((r, n.mul), (-1, 7))
        n = Counter(); self.assertFalse(native.splits_completely(7, [1, 0, 1], n)); self.assertEqual(n.mul, 16)
        r, n = dc(13, chi13, [1, 11, 1]); self.assertEqual((r, n.mul), (0, 7))
        r, n = dc(13, chi13, [6, 2, 7, 1]); self.assertEqual((r, n.mul, n.add), (1, 17, 6))
        n = Counter(); self.assertFalse(native.splits_completely(13, [6, 2, 7, 1], n)); self.assertEqual(n.mul, 51)
        r, n = dc(13, chi13, [7, 11, 7, 1]); self.assertEqual((r, n.mul), (1, 17))
        n = Counter(); self.assertTrue(native.splits_completely(13, [7, 11, 7, 1], n)); self.assertEqual(n.mul, 51)
        n = Counter()
        self.assertEqual(sorted(native.roots_of(13, [7, 11, 7, 1], SM64(1), n)), [1, 2, 3])

    def test10_cantor(self):
        for (p, g) in [(13, 2), (7, 3)]:
            f, T, C = mk(p, g, 1)
            rng = SM64(99)
            for _ in range(20):
                A, B, D = (C.random_divisor(rng) for _ in range(3))
                self.assertEqual(C.cantor(C.cantor(A, B), D), C.cantor(A, C.cantor(B, D)))
            for _ in range(10):
                A = C.random_divisor(rng)
                self.assertEqual(C.cantor(A, C.negate(A)), IDENT)
            pts = [(x, T.sq[T.fx[x]]) for x in T.B]
            for P, Q in [(pts[0], pts[1]), (pts[1], pts[2])]:
                x1, y1 = P
                x2, y2 = Q
                D = C.cantor(C.point(*P), C.point(*Q))
                u = C.k_mul([(-x1) % p, 1], [(-x2) % p, 1])
                slope = (y2 - y1) * pow(x2 - x1, p - 2, p) % p
                v = [(y1 - slope * x1) % p, slope]
                while v and v[-1] == 0:
                    v.pop()
                self.assertEqual(D, (tuple(u), tuple(v)))

    def test11_sparse(self):
        r = linalg.sparse_eliminate([((0, 1), (1, 1)), ((1, 1), (2, 1)), ((0, 1), (2, 1))], 3)
        self.assertEqual((r["pivots"], r["mul"], r["inv"], r["initial_nnz"], r["peak_nnz"],
                          r["uncovered_cols"]), (3, 4, 3, 6, 6, 0))

    def test12_2lp_full(self):
        p, g = 101, 2
        anchors = [(0, 43), (3, 5)]
        f, T, C = mk(p, g, 1)
        info, pts, cands, ctr = relations_of(T, g, anchors)
        rel = {tuple(sorted(list(anchors) + sorted(pts[l]))) for l in cands}
        vc = Curve(p, g, f, T)
        r = pencil.p_2lp(T, g, 1, 1, vc, nb=T.n, tuples_override=[anchors], keep=True)
        zero = {pts_ for (lam, lp, pts_) in r["kept"] if lp == 0}
        self.assertEqual(zero, rel)
        self.assertEqual(len(zero), 12)

    def test13_canon(self):
        p = 101
        pts = [(3, 5), (7, 20), (9, 100)]
        neg = [(x, (p - y) % p) for x, y in pts]
        self.assertEqual(canon(pts, p), canon(neg, p))


if __name__ == "__main__":
    unittest.main()
